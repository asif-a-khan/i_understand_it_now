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

use dashboard::DashboardState;
use lesson_browser::{LessonInfo, LessonListState, LessonReaderState};
use problem_runner::{
    CaseResult, ProblemDetailState, ProblemInfo, ProblemListState, ProblemResultState,
    ProblemRunningState, TestMessage,
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
    viz_picker: VizPickerState,
    viz_player: VizPlayerState,

    // Async test execution
    test_receiver: Option<mpsc::Receiver<TestMessage>>,
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
            viz_picker: VizPickerState::new(),
            viz_player: VizPlayerState::new(),
            test_receiver: None,
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
                if matches!(&screen, Screen::ProblemRunning { .. }) {
                    if matches!(self.current_screen(), Screen::ProblemResult { .. }) {
                        self.screen_stack.pop();
                    }
                }
                // Special handling: when pushing ProblemRunning, start tests
                if let Screen::ProblemRunning { problem_idx } = &screen {
                    self.problem_running.reset();
                    let problem_id = self.problems[*problem_idx].id.clone();
                    self.test_receiver =
                        Some(problem_runner::spawn_test_runner(problem_id));
                }
                // When pushing LessonReader, load the lesson
                if let Screen::LessonReader { lesson_idx } = &screen {
                    self.lesson_reader.load_lesson(*lesson_idx, &self.lessons);
                }
                // When pushing VizPlayer, load the viz
                if let Screen::VizPlayer { viz_idx } = &screen {
                    self.viz_player.load_viz(*viz_idx);
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
                    // Silently ignore editor errors — user will notice
                    let _ = e;
                }
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
                    Ok(TestMessage::Done) => {
                        self.problem_running.done = true;
                        self.test_receiver = None;
                        // Save progress
                        self.save_test_progress();
                        // Transition to result screen
                        if let Some(Screen::ProblemRunning { problem_idx }) =
                            self.screen_stack.last().cloned()
                        {
                            self.problem_result.scroll_offset = 0;
                            // Replace ProblemRunning with ProblemResult
                            self.screen_stack.pop();
                            self.screen_stack
                                .push(Screen::ProblemResult { problem_idx });
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

        // Advance auto-play visualization
        self.viz_player.tick();
    }

    fn save_test_progress(&mut self) {
        if let Some(Screen::ProblemRunning { problem_idx }) = self.screen_stack.last() {
            let p = &self.problems[*problem_idx];
            let passed = self.problem_running.results.iter().filter(|r| r.passed).count();
            let total = self.problem_running.results.len();
            let total_ops: usize = self.problem_running.results.iter().map(|r| r.ops).sum();
            let total_cmp: usize = self.problem_running.results.iter().map(|r| r.comparisons).sum();
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
                entry.best_comparisons =
                    Some(entry.best_comparisons.map_or(total_cmp, |b| b.min(total_cmp)));
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
            // Only handle key press events (not release/repeat)
            if key.kind != KeyEventKind::Press {
                return false;
            }

            // Global keybindings
            if key.code == KeyCode::Char('c')
                && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL)
            {
                return true; // Ctrl+C quits
            }

            // Help toggle
            if key.code == KeyCode::Char('?') {
                self.show_help = !self.show_help;
                return false;
            }

            // Dismiss help with Esc if shown
            if self.show_help && key.code == KeyCode::Esc {
                self.show_help = false;
                return false;
            }

            // If help is shown, consume all other keys
            if self.show_help {
                return false;
            }

            let action = match self.current_screen().clone() {
                Screen::Dashboard => self.dashboard.handle_key(key),
                Screen::LessonList => self.lesson_list.handle_key(key, &self.lessons),
                Screen::LessonReader { .. } => {
                    let visible_height = terminal.size().map(|s| s.height.saturating_sub(4)).unwrap_or(20);
                    self.lesson_reader
                        .handle_key(key, &self.lessons, &mut self.progress, visible_height)
                }
                Screen::ProblemList => self.problem_list.handle_key(key, &self.problems),
                Screen::ProblemDetail { problem_idx } => {
                    self.problem_detail
                        .handle_key(key, problem_idx, &self.problems)
                }
                Screen::ProblemRunning { .. } => {
                    // No interaction during test execution
                    Action::None
                }
                Screen::ProblemResult { problem_idx } => {
                    let visible_height = terminal.size().map(|s| s.height.saturating_sub(4)).unwrap_or(20);
                    let total_lines = self.problem_running.results.len() * 2 + 15;
                    self.problem_result.handle_key(
                        key,
                        problem_idx,
                        &self.problems,
                        visible_height,
                        total_lines,
                    )
                }
                Screen::VizPicker => self.viz_picker.handle_key(key),
                Screen::VizPlayer { .. } => self.viz_player.handle_key(key),
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
                self.problem_result
                    .render(f, area, problem_idx, &self.problems, &self.problem_running);
            }
            Screen::VizPicker => {
                self.viz_picker.render(f, area);
            }
            Screen::VizPlayer { .. } => {
                self.viz_player.render(f, area);
            }
        }

        // Help overlay on top
        if self.show_help {
            keybindings::render_help(f, self.current_screen());
        }
    }
}

/// Entry point: launch the TUI.
pub fn run() -> io::Result<()> {
    // Install panic hook to restore terminal on panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = terminal::disable_raw_mode();
        let _ = crossterm::execute!(io::stdout(), terminal::LeaveAlternateScreen);
        original_hook(panic_info);
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

        // Poll with timeout for tick-based updates (auto-play, test results)
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
