mod dashboard;
mod editor;
mod keybindings;
mod lesson_browser;
mod markdown;
mod problem_runner;
mod screens;
mod theme;
mod viz_player;

use std::io;
use std::sync::mpsc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::progress::{ProblemProgress, Progress};
use crate::tracker::Operation;
use crate::visualizer;

use dashboard::DashboardState;
use editor::{CheckMessage, InTuiEditorState};
use lesson_browser::{LessonInfo, LessonListState, LessonReaderState};
use problem_runner::{
    CaseResult, ComplexityMessage, ComplexityViewState, ProblemDetailState, ProblemInfo,
    ProblemListState, ProblemResultState, ProblemRunningState, TestMessage,
};
use screens::{Action, Screen};
use viz_player::{VizPickerState, VizPlayerState};

struct App {
    screen_stack: Vec<Screen>,
    show_help: bool,

    // Shared data
    progress: Progress,
    problems: Vec<ProblemInfo>,
    lessons: Vec<LessonInfo>,

    // Screen states
    dashboard: DashboardState,
    lesson_list: LessonListState,
    lesson_reader: LessonReaderState,
    problem_list: ProblemListState,
    problem_detail: ProblemDetailState,
    problem_running: ProblemRunningState,
    problem_result: ProblemResultState,
    complexity_view: ComplexityViewState,
    in_tui_editor: InTuiEditorState,
    viz_picker: VizPickerState,
    viz_player: VizPlayerState,

    // Async receivers
    test_receiver: Option<mpsc::Receiver<TestMessage>>,
    complexity_receiver: Option<mpsc::Receiver<ComplexityMessage>>,
    check_receiver: Option<mpsc::Receiver<CheckMessage>>,

    // Replay data from last test run
    replay_ops: Vec<Operation>,
    /// After tests complete, go to replay instead of results
    replay_after_tests: Option<usize>,
}

impl App {
    fn new() -> Self {
        let progress = Progress::load();
        let problems = ProblemInfo::load_all();
        let lessons = LessonInfo::load_all();
        let problem_list = ProblemListState::new(&problems);

        Self {
            screen_stack: vec![Screen::Dashboard],
            show_help: false,
            progress,
            problems,
            lessons,
            dashboard: DashboardState::new(),
            lesson_list: LessonListState::new(),
            lesson_reader: LessonReaderState::new(),
            problem_list,
            problem_detail: ProblemDetailState,
            problem_running: ProblemRunningState::new(),
            problem_result: ProblemResultState::new(),
            complexity_view: ComplexityViewState::new(),
            in_tui_editor: InTuiEditorState::new(),
            viz_picker: VizPickerState::new(),
            viz_player: VizPlayerState::new(),
            test_receiver: None,
            complexity_receiver: None,
            check_receiver: None,
            replay_ops: Vec::new(),
            replay_after_tests: None,
        }
    }

    fn current_screen(&self) -> &Screen {
        self.screen_stack.last().unwrap_or(&Screen::Dashboard)
    }

    fn handle_action(
        &mut self,
        action: Action,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> bool {
        match action {
            Action::None => false,
            Action::Quit => true,
            Action::Push(screen) => {
                // When re-running from ProblemResult, pop the old result first
                if matches!(&screen, Screen::ProblemRunning { .. })
                    && matches!(self.current_screen(), Screen::ProblemResult { .. })
                {
                    self.screen_stack.pop();
                }
                // Special handling: when pushing ProblemRunning, start tests
                if let Screen::ProblemRunning { problem_idx } = &screen {
                    self.problem_running.reset();
                    let problem_id = self.problems[*problem_idx].id.clone();
                    self.test_receiver = Some(problem_runner::spawn_test_runner(problem_id));
                }
                // When pushing LessonReader, load the lesson
                if let Screen::LessonReader { lesson_idx } = &screen {
                    self.lesson_reader.load_lesson(*lesson_idx, &self.lessons);
                }
                // When pushing ReplayPlayer: try problem reference viz, then instrumented replay
                if let Screen::ReplayPlayer { problem_idx } = &screen {
                    let pid = &self.problems[*problem_idx].id;
                    if let Some(frames) = visualizer::problem_viz::get_problem_viz(pid) {
                        let name = format!("Viz: {}", pid);
                        self.viz_player.load_replay_frames(frames, name);
                    } else {
                        let name = format!("Replay: {}", pid);
                        let frames = visualizer::instrumented::replay_from_ops(&self.replay_ops);
                        self.viz_player.load_replay_frames(frames, name);
                    }
                }
                // When pushing ComplexityView, start measurement
                if let Screen::ComplexityView { problem_idx } = &screen {
                    self.complexity_view.reset();
                    let problem_id = self.problems[*problem_idx].id.clone();
                    self.complexity_receiver =
                        Some(problem_runner::spawn_complexity_runner(problem_id));
                }
                // When pushing InTuiEditor, load the file and jump to function
                if let Screen::InTuiEditor { problem_idx } = &screen {
                    let topic = self.problems[*problem_idx].topic.clone();
                    let problem_id = self.problems[*problem_idx].id.clone();
                    self.in_tui_editor
                        .load_file(*problem_idx, &topic, &problem_id);
                }
                self.screen_stack.push(screen);
                false
            }
            Action::Pop => {
                if self.screen_stack.len() > 1 {
                    self.screen_stack.pop();
                }
                // Reload progress when returning to dashboard
                if matches!(self.current_screen(), Screen::Dashboard) {
                    self.progress = Progress::load();
                }
                false
            }
            Action::LaunchEditor(path) => {
                if let Err(e) = editor::launch_external_editor(terminal, &path) {
                    let _ = e;
                }
                false
            }
            Action::GoTo(screen) => {
                self.screen_stack.clear();
                self.screen_stack.push(Screen::Dashboard);
                if !matches!(screen, Screen::Dashboard) {
                    // Trigger the Push logic for the target screen
                    return self.handle_action(Action::Push(screen), terminal);
                }
                self.progress = Progress::load();
                false
            }
            Action::SaveRunReplay(problem_idx) => {
                // Pop editor, start test runner, flag to go to replay after
                if matches!(self.current_screen(), Screen::InTuiEditor { .. }) {
                    self.screen_stack.pop();
                }
                self.replay_after_tests = Some(problem_idx);
                self.problem_running.reset();
                let problem_id = self.problems[problem_idx].id.clone();
                self.test_receiver = Some(problem_runner::spawn_test_runner(problem_id));
                self.screen_stack
                    .push(Screen::ProblemRunning { problem_idx });
                false
            }
        }
    }

    fn tick(&mut self) {
        // Poll test execution results
        if let Some(ref rx) = self.test_receiver {
            loop {
                match rx.try_recv() {
                    Ok(TestMessage::CaseResult {
                        index,
                        total,
                        passed,
                        input_desc,
                        expected,
                        actual,
                        comparisons,
                        swaps,
                        ops,
                    }) => {
                        self.problem_running.results.push(CaseResult {
                            index,
                            total,
                            passed,
                            input_desc,
                            expected,
                            actual,
                            comparisons,
                            swaps,
                            ops,
                        });
                    }
                    Ok(TestMessage::Done { replay_ops }) => {
                        self.problem_running.done = true;
                        self.test_receiver = None;
                        self.replay_ops = replay_ops;
                        self.save_test_progress();
                        if let Some(Screen::ProblemRunning { problem_idx }) =
                            self.screen_stack.last().cloned()
                        {
                            self.screen_stack.pop();
                            if let Some(target_idx) = self.replay_after_tests.take() {
                                // Check if there are any useful ops to replay
                                let has_ops = self.replay_ops.iter().any(|op| {
                                    matches!(
                                        op,
                                        crate::tracker::Operation::Compare { .. }
                                            | crate::tracker::Operation::Swap { .. }
                                    )
                                });
                                if has_ops {
                                    let name = format!("Replay: {}", self.problems[target_idx].id);
                                    let frames =
                                        visualizer::instrumented::replay_from_ops(&self.replay_ops);
                                    self.viz_player.load_replay_frames(frames, name);
                                    self.screen_stack.push(Screen::ReplayPlayer {
                                        problem_idx: target_idx,
                                    });
                                } else {
                                    // No ops (e.g. todo!() stubs) — show results instead
                                    self.problem_result.scroll_offset = 0;
                                    self.screen_stack
                                        .push(Screen::ProblemResult { problem_idx });
                                }
                            } else {
                                self.problem_result.scroll_offset = 0;
                                self.screen_stack
                                    .push(Screen::ProblemResult { problem_idx });
                            }
                        }
                        break;
                    }
                    Ok(TestMessage::Panicked(msg)) => {
                        self.problem_running.results.push(CaseResult {
                            index: self.problem_running.results.len(),
                            total: 0,
                            passed: false,
                            input_desc: String::new(),
                            expected: String::new(),
                            actual: format!("PANIC: {}", msg),
                            comparisons: 0,
                            swaps: 0,
                            ops: 0,
                        });
                        self.problem_running.done = true;
                        self.test_receiver = None;
                        if let Some(Screen::ProblemRunning { problem_idx }) =
                            self.screen_stack.last().cloned()
                        {
                            self.problem_result.scroll_offset = 0;
                            self.screen_stack.pop();
                            self.screen_stack
                                .push(Screen::ProblemResult { problem_idx });
                        }
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => {
                        self.problem_running.done = true;
                        self.test_receiver = None;
                        break;
                    }
                }
            }
        }

        // Poll complexity measurement results
        if let Some(ref rx) = self.complexity_receiver {
            match rx.try_recv() {
                Ok(ComplexityMessage::Result {
                    estimated,
                    ascii_plot,
                }) => {
                    self.complexity_view.loading = false;
                    self.complexity_view.estimated = estimated;
                    self.complexity_view.ascii_plot = ascii_plot;
                    self.complexity_receiver = None;
                }
                Ok(ComplexityMessage::Error(msg)) => {
                    self.complexity_view.loading = false;
                    self.complexity_view.error = Some(msg);
                    self.complexity_receiver = None;
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.complexity_view.loading = false;
                    self.complexity_view.error =
                        Some("Measurement thread disconnected".to_string());
                    self.complexity_receiver = None;
                }
            }
        }

        // Poll cargo check results
        if let Some(ref rx) = self.check_receiver {
            match rx.try_recv() {
                Ok(CheckMessage::Done(errors)) => {
                    self.in_tui_editor.apply_check_result(errors);
                    self.check_receiver = None;
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.check_receiver = None;
                }
            }
        }

        // Start cargo check if editor just saved and no check is running
        if self.in_tui_editor.is_checking() && self.check_receiver.is_none() {
            let file_path = self.in_tui_editor.file_path().to_string();
            self.check_receiver = Some(editor::spawn_cargo_check(file_path));
        }

        // Advance auto-play visualization
        self.viz_player.tick();
    }

    fn save_test_progress(&mut self) {
        if let Some(Screen::ProblemRunning { problem_idx }) = self.screen_stack.last() {
            let p = &self.problems[*problem_idx];
            let passed = self
                .problem_running
                .results
                .iter()
                .filter(|r| r.passed)
                .count();
            let total = self.problem_running.results.len();
            let total_ops: usize = self.problem_running.results.iter().map(|r| r.ops).sum();
            let total_cmp: usize = self
                .problem_running
                .results
                .iter()
                .map(|r| r.comparisons)
                .sum();
            let total_swaps: usize = self.problem_running.results.iter().map(|r| r.swaps).sum();
            let solved = passed == total && total > 0;

            let entry = self
                .progress
                .problems
                .entry(p.id.clone())
                .or_insert(ProblemProgress {
                    solved: false,
                    best_comparisons: None,
                    best_swaps: None,
                    best_total_ops: None,
                });

            if solved {
                entry.solved = true;
            }
            if total_ops > 0 {
                entry.best_total_ops =
                    Some(entry.best_total_ops.map_or(total_ops, |b| b.min(total_ops)));
                entry.best_comparisons = Some(
                    entry
                        .best_comparisons
                        .map_or(total_cmp, |b| b.min(total_cmp)),
                );
                entry.best_swaps =
                    Some(entry.best_swaps.map_or(total_swaps, |b| b.min(total_swaps)));
            }
            let _ = self.progress.save();
        }
    }

    fn handle_event(
        &mut self,
        event: Event,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> bool {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return false;
            }

            // Ctrl+C always quits
            if key.code == KeyCode::Char('c')
                && key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
            {
                return true;
            }

            // In-TUI editor handles all keys directly (no help toggle, no '?' intercept)
            if matches!(self.current_screen(), Screen::InTuiEditor { .. }) {
                let action = self.in_tui_editor.handle_key(key);
                return self.handle_action(action, terminal);
            }

            // Help toggle (not in editor)
            if key.code == KeyCode::Char('?') {
                self.show_help = !self.show_help;
                return false;
            }

            if self.show_help && key.code == KeyCode::Esc {
                self.show_help = false;
                return false;
            }

            if self.show_help {
                return false;
            }

            // Global navigation shortcuts (Ctrl+D/L/P) — work from any non-editor screen
            let ctrl = key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL);
            if ctrl {
                let nav = match key.code {
                    KeyCode::Char('d') => Some(Action::GoTo(Screen::Dashboard)),
                    KeyCode::Char('l') => Some(Action::GoTo(Screen::LessonList)),
                    KeyCode::Char('p') => Some(Action::GoTo(Screen::ProblemList)),
                    _ => None,
                };
                if let Some(action) = nav {
                    return self.handle_action(action, terminal);
                }
            }

            let action = match self.current_screen().clone() {
                Screen::Dashboard => self.dashboard.handle_key(key),
                Screen::LessonList => self.lesson_list.handle_key(key, &self.lessons),
                Screen::LessonReader { .. } => {
                    let visible_height = terminal
                        .size()
                        .map(|s| s.height.saturating_sub(4))
                        .unwrap_or(20);
                    self.lesson_reader.handle_key(
                        key,
                        &self.lessons,
                        &mut self.progress,
                        visible_height,
                    )
                }
                Screen::ProblemList => self.problem_list.handle_key(key, &self.problems),
                Screen::ProblemDetail { problem_idx } => {
                    self.problem_detail
                        .handle_key(key, problem_idx, &self.problems)
                }
                Screen::ProblemRunning { .. } => Action::None,
                Screen::ProblemResult { problem_idx } => {
                    let visible_height = terminal
                        .size()
                        .map(|s| s.height.saturating_sub(4))
                        .unwrap_or(20);
                    let total_lines = self.problem_running.results.len() * 2 + 15;
                    self.problem_result.handle_key(
                        key,
                        problem_idx,
                        &self.problems,
                        visible_height,
                        total_lines,
                    )
                }
                Screen::ComplexityView { .. } => {
                    let visible_height = terminal
                        .size()
                        .map(|s| s.height.saturating_sub(4))
                        .unwrap_or(20);
                    self.complexity_view.handle_key(key, visible_height)
                }
                Screen::ReplayPlayer { problem_idx } => {
                    // Replay-specific keys: jump to editor for this problem
                    match key.code {
                        KeyCode::Char('e') => {
                            let path =
                                editor::solution_file_path(&self.problems[problem_idx].topic);
                            Action::LaunchEditor(path.to_string_lossy().to_string())
                        }
                        KeyCode::Char('i') => Action::Push(Screen::InTuiEditor { problem_idx }),
                        _ => self.viz_player.handle_key(key),
                    }
                }
                Screen::InTuiEditor { .. } => unreachable!(), // handled above
                Screen::VizPicker => self.viz_picker.handle_key(key, &self.problems),
            };

            return self.handle_action(action, terminal);
        }
        false
    }

    fn render(&mut self, f: &mut ratatui::Frame) {
        let area = f.area();

        match self.current_screen().clone() {
            Screen::Dashboard => {
                self.dashboard.render(f, area, &self.progress);
            }
            Screen::LessonList => {
                self.lesson_list
                    .render(f, area, &self.lessons, &self.progress);
            }
            Screen::LessonReader { .. } => {
                self.lesson_reader
                    .render(f, area, &self.lessons, &self.progress);
            }
            Screen::ProblemList => {
                self.problem_list
                    .render(f, area, &self.problems, &self.progress);
            }
            Screen::ProblemDetail { problem_idx } => {
                self.problem_detail
                    .render(f, area, problem_idx, &self.problems, &self.progress);
            }
            Screen::ProblemRunning { problem_idx } => {
                self.problem_running
                    .render(f, area, problem_idx, &self.problems);
            }
            Screen::ProblemResult { problem_idx } => {
                self.problem_result.render(
                    f,
                    area,
                    problem_idx,
                    &self.problems,
                    &self.problem_running,
                );
            }
            Screen::ComplexityView { problem_idx } => {
                self.complexity_view
                    .render(f, area, problem_idx, &self.problems);
            }
            Screen::ReplayPlayer { .. } => {
                self.viz_player.render(f, area);
            }
            Screen::InTuiEditor { .. } => {
                self.in_tui_editor.render(f, area);
            }
            Screen::VizPicker => {
                self.viz_picker.render(f, area, &self.problems);
            }
        }

        // Help overlay (not shown in editor)
        if self.show_help && !matches!(self.current_screen(), Screen::InTuiEditor { .. }) {
            keybindings::render_help(f, self.current_screen());
        }
    }
}

/// Entry point: launch the TUI.
pub fn run() -> io::Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Only restore terminal + print for main thread panics.
        // Background threads (test runners) use catch_unwind — suppress their output
        // to avoid corrupting the TUI display.
        let is_main = std::thread::current().name().is_some_and(|n| n == "main");
        if is_main {
            let _ = terminal::disable_raw_mode();
            let _ = crossterm::execute!(io::stdout(), terminal::LeaveAlternateScreen);
            original_hook(panic_info);
        }
    }));

    terminal::enable_raw_mode()?;
    crossterm::execute!(io::stdout(), terminal::EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new();
    let tick_rate = std::time::Duration::from_millis(50);

    loop {
        terminal.draw(|f| app.render(f))?;

        if event::poll(tick_rate)? {
            let event = event::read()?;
            if app.handle_event(event, &mut terminal) {
                break;
            }
        }

        app.tick();
    }

    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), terminal::LeaveAlternateScreen)?;

    Ok(())
}
