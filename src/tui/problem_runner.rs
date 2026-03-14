use std::sync::mpsc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::complexity;
use crate::problems::{self, Difficulty};
use crate::progress::Progress;
use crate::tracker::{Operation, OperationLog};

use super::editor;
use super::screens::{Action, Screen};
use super::theme;

/// Lightweight problem metadata — avoids holding trait objects.
#[derive(Clone)]
pub struct ProblemInfo {
    pub id: String,
    pub name: String,
    pub topic: String,
    pub difficulty: Difficulty,
    pub description: String,
}

impl ProblemInfo {
    pub fn load_all() -> Vec<ProblemInfo> {
        problems::all_problems()
            .into_iter()
            .map(|p| ProblemInfo {
                id: p.id().to_string(),
                name: p.name().to_string(),
                topic: p.topic().to_string(),
                difficulty: p.difficulty(),
                description: p.description().to_string(),
            })
            .collect()
    }
}

/// Message sent from test execution thread.
pub enum TestMessage {
    /// A single test case completed.
    CaseResult {
        index: usize,
        total: usize,
        passed: bool,
        input_desc: String,
        expected: String,
        actual: String,
        comparisons: usize,
        swaps: usize,
        ops: usize,
    },
    /// All tests done. Includes operation log from first test for replay.
    Done { replay_ops: Vec<Operation> },
    /// A panic occurred (e.g., todo!()).
    Panicked(String),
}

/// Message from complexity measurement thread.
pub enum ComplexityMessage {
    Result {
        estimated: String,
        ascii_plot: String,
    },
    Error(String),
}

// ─── Problem List ─────────────────────────────────────────

pub struct ProblemListState {
    pub list_state: ListState,
    pub topic_filter: Option<usize>, // index into unique topics list
    pub difficulty_filter: Option<Difficulty>,
    topics: Vec<String>,
}

impl ProblemListState {
    pub fn new(problems: &[ProblemInfo]) -> Self {
        let mut topics: Vec<String> = problems.iter().map(|p| p.topic.clone()).collect();
        topics.sort();
        topics.dedup();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            list_state,
            topic_filter: None,
            difficulty_filter: None,
            topics,
        }
    }

    /// Get filtered and grouped problem indices.
    fn filtered_indices(&self, problems: &[ProblemInfo]) -> Vec<FilteredEntry> {
        let mut entries = Vec::new();
        let mut current_topic = String::new();

        for (idx, p) in problems.iter().enumerate() {
            if let Some(ti) = self.topic_filter {
                if p.topic != self.topics[ti] {
                    continue;
                }
            }
            if let Some(ref df) = self.difficulty_filter {
                if &p.difficulty != df {
                    continue;
                }
            }

            if p.topic != current_topic {
                current_topic = p.topic.clone();
                entries.push(FilteredEntry::Header(current_topic.clone()));
            }
            entries.push(FilteredEntry::Problem(idx));
        }
        entries
    }

    pub fn handle_key(&mut self, key: KeyEvent, problems: &[ProblemInfo]) -> Action {
        let entries = self.filtered_indices(problems);
        let len = entries.len();
        if len == 0 && key.code == KeyCode::Esc {
            return Action::Pop;
        }

        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                let i = self.list_state.selected().unwrap_or(0);
                let mut next = (i + 1).min(len.saturating_sub(1));
                // Skip headers
                while next < len && matches!(entries.get(next), Some(FilteredEntry::Header(_))) {
                    next += 1;
                }
                if next < len {
                    self.list_state.select(Some(next));
                }
                Action::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let i = self.list_state.selected().unwrap_or(0);
                let mut prev = i.saturating_sub(1);
                while prev > 0 && matches!(entries.get(prev), Some(FilteredEntry::Header(_))) {
                    prev = prev.saturating_sub(1);
                }
                // If we landed on a header at 0, find next problem
                if matches!(entries.get(prev), Some(FilteredEntry::Header(_))) {
                    if let Some(pos) = entries
                        .iter()
                        .position(|e| matches!(e, FilteredEntry::Problem(_)))
                    {
                        prev = pos;
                    }
                }
                self.list_state.select(Some(prev));
                Action::None
            }
            KeyCode::Enter => {
                if let Some(idx) = self.list_state.selected() {
                    if let Some(FilteredEntry::Problem(pidx)) = entries.get(idx) {
                        return Action::Push(Screen::ProblemDetail { problem_idx: *pidx });
                    }
                }
                Action::None
            }
            KeyCode::Char('t') => {
                self.topic_filter = match self.topic_filter {
                    None => Some(0),
                    Some(i) if i + 1 < self.topics.len() => Some(i + 1),
                    Some(_) => None,
                };
                self.list_state.select(Some(0));
                // Skip to first problem entry
                let entries = self.filtered_indices(problems);
                if let Some(pos) = entries
                    .iter()
                    .position(|e| matches!(e, FilteredEntry::Problem(_)))
                {
                    self.list_state.select(Some(pos));
                }
                Action::None
            }
            KeyCode::Char('d') => {
                self.difficulty_filter = match &self.difficulty_filter {
                    None => Some(Difficulty::Easy),
                    Some(Difficulty::Easy) => Some(Difficulty::Medium),
                    Some(Difficulty::Medium) => Some(Difficulty::Hard),
                    Some(Difficulty::Hard) => None,
                };
                self.list_state.select(Some(0));
                let entries = self.filtered_indices(problems);
                if let Some(pos) = entries
                    .iter()
                    .position(|e| matches!(e, FilteredEntry::Problem(_)))
                {
                    self.list_state.select(Some(pos));
                }
                Action::None
            }
            KeyCode::Esc => Action::Pop,
            _ => Action::None,
        }
    }

    pub fn render(
        &mut self,
        f: &mut Frame,
        area: Rect,
        problems: &[ProblemInfo],
        progress: &Progress,
    ) {
        let entries = self.filtered_indices(problems);

        let items: Vec<ListItem> = entries
            .iter()
            .map(|entry| match entry {
                FilteredEntry::Header(topic) => ListItem::new(Line::from(Span::styled(
                    format!("── {} ──", topic),
                    Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
                ))),
                FilteredEntry::Problem(idx) => {
                    let p = &problems[*idx];
                    let solved = progress.problems.get(&p.id).is_some_and(|pp| pp.solved);
                    let check = if solved { "[x]" } else { "[ ]" };
                    let check_style = if solved {
                        Style::new().fg(Color::Green)
                    } else {
                        Style::new().fg(Color::DarkGray)
                    };

                    ListItem::new(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(format!("{} ", check), check_style),
                        Span::styled(
                            format!("{:>6} ", p.difficulty),
                            theme::difficulty_style(&p.difficulty),
                        ),
                        Span::raw(&p.id),
                        Span::styled(format!(" — {}", p.name), Style::new().fg(Color::DarkGray)),
                    ]))
                }
            })
            .collect();

        // Build title with active filters
        let solved_count = problems
            .iter()
            .filter(|p| progress.problems.get(&p.id).is_some_and(|pp| pp.solved))
            .count();
        let mut title_parts = format!(" Problems ({solved_count}/{}) ", problems.len());
        if let Some(ti) = self.topic_filter {
            title_parts.push_str(&format!(" [topic: {}] ", self.topics[ti]));
        }
        if let Some(ref df) = self.difficulty_filter {
            title_parts.push_str(&format!(" [diff: {}] ", df));
        }

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title_parts)
                    .title_style(theme::title_style()),
            )
            .highlight_style(theme::selected_style())
            .highlight_symbol("> ");

        f.render_stateful_widget(list, area, &mut self.list_state);
    }
}

enum FilteredEntry {
    Header(String),
    Problem(usize),
}

// ─── Problem Detail ───────────────────────────────────────

pub struct ProblemDetailState;

impl ProblemDetailState {
    pub fn handle_key(
        &self,
        key: KeyEvent,
        problem_idx: usize,
        problems: &[ProblemInfo],
    ) -> Action {
        match key.code {
            KeyCode::Char('r') => Action::Push(Screen::ProblemRunning { problem_idx }),
            KeyCode::Char('e') => {
                let path = editor::solution_file_path(&problems[problem_idx].topic);
                Action::LaunchEditor(path.to_string_lossy().to_string())
            }
            KeyCode::Char('i') => Action::Push(Screen::InTuiEditor { problem_idx }),
            KeyCode::Char('v') => Action::Push(Screen::ReplayPlayer { problem_idx }),
            KeyCode::Esc => Action::Pop,
            _ => Action::None,
        }
    }

    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        problem_idx: usize,
        problems: &[ProblemInfo],
        progress: &Progress,
    ) {
        let p = &problems[problem_idx];
        let prev = progress.problems.get(&p.id);

        let mut lines = vec![
            Line::raw(""),
            Line::from(vec![
                Span::styled("  Problem: ", Style::new().fg(Color::DarkGray)),
                Span::styled(&p.name, theme::heading_style()),
            ]),
            Line::from(vec![
                Span::styled("  ID: ", Style::new().fg(Color::DarkGray)),
                Span::raw(&p.id),
            ]),
            Line::from(vec![
                Span::styled("  Difficulty: ", Style::new().fg(Color::DarkGray)),
                Span::styled(
                    format!("{}", p.difficulty),
                    theme::difficulty_style(&p.difficulty),
                ),
            ]),
            Line::from(vec![
                Span::styled("  Topic: ", Style::new().fg(Color::DarkGray)),
                Span::raw(&p.topic),
            ]),
            Line::raw(""),
            Line::styled(
                "  Description:",
                Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
            ),
            Line::raw(""),
        ];

        for desc_line in p.description.lines() {
            lines.push(Line::raw(format!("  {}", desc_line)));
        }

        lines.push(Line::raw(""));
        lines.push(Line::styled(
            "─".repeat(60),
            Style::new().fg(Color::DarkGray),
        ));

        if let Some(pp) = prev {
            let status = if pp.solved { "Solved" } else { "Attempted" };
            let status_style = if pp.solved {
                theme::success_style()
            } else {
                theme::warning_style()
            };
            lines.push(Line::from(vec![
                Span::styled("  Status: ", Style::new().fg(Color::DarkGray)),
                Span::styled(status, status_style),
            ]));
            if let Some(ops) = pp.best_total_ops {
                lines.push(Line::from(vec![
                    Span::styled("  Best ops: ", Style::new().fg(Color::DarkGray)),
                    Span::raw(format!(
                        "{} (cmp: {}, swap: {})",
                        ops,
                        pp.best_comparisons.unwrap_or(0),
                        pp.best_swaps.unwrap_or(0),
                    )),
                ]));
            }
        } else {
            lines.push(Line::from(vec![
                Span::styled("  Status: ", Style::new().fg(Color::DarkGray)),
                Span::styled("Not attempted", theme::muted_style()),
            ]));
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "[R]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Run tests  "),
            Span::styled(
                "[E]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Edit ($EDITOR)  "),
            Span::styled(
                "[I]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Edit (in-TUI)  "),
            Span::styled(
                "[V]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Visualize  "),
            Span::styled(
                "[Esc]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Back"),
        ]));

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", p.id))
            .title_style(theme::title_style());

        f.render_widget(Paragraph::new(lines).block(block), area);
    }
}

// ─── Problem Running ──────────────────────────────────────

pub struct ProblemRunningState {
    pub results: Vec<CaseResult>,
    pub done: bool,
    pub scroll_offset: u16,
}

pub struct CaseResult {
    pub index: usize,
    pub total: usize,
    pub passed: bool,
    pub input_desc: String,
    pub expected: String,
    pub actual: String,
    pub comparisons: usize,
    pub swaps: usize,
    pub ops: usize,
}

impl ProblemRunningState {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            done: false,
            scroll_offset: 0,
        }
    }

    pub fn reset(&mut self) {
        self.results.clear();
        self.done = false;
        self.scroll_offset = 0;
    }

    pub fn render(&self, f: &mut Frame, area: Rect, problem_idx: usize, problems: &[ProblemInfo]) {
        let p = &problems[problem_idx];
        let mut lines = vec![
            Line::raw(""),
            Line::from(vec![
                Span::styled("  Running: ", Style::new().fg(Color::DarkGray)),
                Span::styled(&p.name, theme::heading_style()),
            ]),
            Line::raw(""),
        ];

        for r in &self.results {
            let status = if r.passed {
                Span::styled("PASS", theme::success_style())
            } else {
                Span::styled("FAIL", theme::error_style())
            };

            let mut test_line = vec![
                Span::raw(format!("  Test {}/{}: ", r.index + 1, r.total)),
                status,
            ];

            if r.ops > 0 {
                test_line.push(Span::styled(
                    format!(
                        " | ops: {} (cmp: {}, swap: {})",
                        r.ops, r.comparisons, r.swaps
                    ),
                    Style::new().fg(Color::DarkGray),
                ));
            }

            lines.push(Line::from(test_line));

            if !r.passed {
                lines.push(Line::from(Span::styled(
                    format!("    Input:    {}", r.input_desc),
                    Style::new().fg(Color::DarkGray),
                )));
                lines.push(Line::from(Span::styled(
                    format!("    Expected: {}", r.expected),
                    Style::new().fg(Color::Green),
                )));
                lines.push(Line::from(Span::styled(
                    format!("    Got:      {}", r.actual),
                    Style::new().fg(Color::Red),
                )));
            }
        }

        if !self.done {
            lines.push(Line::raw(""));
            lines.push(Line::from(Span::styled(
                "  Running tests...",
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )));
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Testing: {} ", p.id))
            .title_style(theme::title_style());

        f.render_widget(
            Paragraph::new(lines)
                .block(block)
                .scroll((self.scroll_offset, 0)),
            area,
        );
    }
}

// ─── Problem Result ───────────────────────────────────────

pub struct ProblemResultState {
    pub scroll_offset: u16,
}

impl ProblemResultState {
    pub fn new() -> Self {
        Self { scroll_offset: 0 }
    }

    pub fn handle_key(
        &mut self,
        key: KeyEvent,
        problem_idx: usize,
        problems: &[ProblemInfo],
        visible_height: u16,
        total_lines: usize,
    ) -> Action {
        let max_scroll = total_lines.saturating_sub(visible_height as usize) as u16;

        match key.code {
            KeyCode::Char('r') => {
                self.scroll_offset = 0;
                Action::Push(Screen::ProblemRunning { problem_idx })
            }
            KeyCode::Char('e') => {
                let path = editor::solution_file_path(&problems[problem_idx].topic);
                Action::LaunchEditor(path.to_string_lossy().to_string())
            }
            KeyCode::Char('i') => Action::Push(Screen::InTuiEditor { problem_idx }),
            KeyCode::Char('v') | KeyCode::Char('w') => {
                Action::Push(Screen::ReplayPlayer { problem_idx })
            }
            KeyCode::Char('c') => Action::Push(Screen::ComplexityView { problem_idx }),
            KeyCode::Char('j') | KeyCode::Down => {
                self.scroll_offset = (self.scroll_offset + 1).min(max_scroll);
                Action::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                Action::None
            }
            KeyCode::Esc => Action::Pop,
            _ => Action::None,
        }
    }

    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        problem_idx: usize,
        problems: &[ProblemInfo],
        running_state: &ProblemRunningState,
    ) -> usize {
        let p = &problems[problem_idx];
        let mut lines = vec![
            Line::raw(""),
            Line::from(vec![
                Span::styled("  Results: ", Style::new().fg(Color::DarkGray)),
                Span::styled(&p.name, theme::heading_style()),
            ]),
            Line::raw(""),
        ];

        let passed = running_state.results.iter().filter(|r| r.passed).count();
        let total = running_state.results.len();
        let total_ops: usize = running_state.results.iter().map(|r| r.ops).sum();
        let total_cmp: usize = running_state.results.iter().map(|r| r.comparisons).sum();
        let total_swaps: usize = running_state.results.iter().map(|r| r.swaps).sum();

        for r in &running_state.results {
            let status = if r.passed {
                Span::styled("PASS", theme::success_style())
            } else {
                Span::styled("FAIL", theme::error_style())
            };

            let mut test_line = vec![
                Span::raw(format!("  Test {}/{}: ", r.index + 1, r.total)),
                status,
            ];

            if r.ops > 0 {
                test_line.push(Span::styled(
                    format!(
                        " | ops: {} (cmp: {}, swap: {})",
                        r.ops, r.comparisons, r.swaps
                    ),
                    Style::new().fg(Color::DarkGray),
                ));
            }

            lines.push(Line::from(test_line));

            if !r.passed {
                lines.push(Line::from(Span::styled(
                    format!("    Input:    {}", r.input_desc),
                    Style::new().fg(Color::DarkGray),
                )));
                lines.push(Line::from(Span::styled(
                    format!("    Expected: {}", r.expected),
                    Style::new().fg(Color::Green),
                )));
                lines.push(Line::from(Span::styled(
                    format!("    Got:      {}", r.actual),
                    Style::new().fg(Color::Red),
                )));
            }
        }

        // Summary
        lines.push(Line::raw(""));
        lines.push(Line::styled(
            "─".repeat(60),
            Style::new().fg(Color::DarkGray),
        ));

        let summary_style = if passed == total {
            theme::success_style()
        } else {
            theme::error_style()
        };
        lines.push(Line::from(Span::styled(
            format!("  Result: {passed}/{total} tests passed"),
            summary_style,
        )));

        if total_ops > 0 {
            lines.push(Line::from(Span::styled(
                format!(
                    "  Total operations: {} (comparisons: {}, swaps: {})",
                    total_ops, total_cmp, total_swaps
                ),
                Style::new().fg(Color::DarkGray),
            )));
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "[R]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Run again  "),
            Span::styled(
                "[E]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Edit  "),
            Span::styled(
                "[I]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" In-TUI edit  "),
            Span::styled(
                "[V]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Visualize  "),
            Span::styled(
                "[C]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Complexity  "),
            Span::styled(
                "[Esc]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Back"),
        ]));

        let total_lines = lines.len();
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Results: {} ", p.id))
            .title_style(theme::title_style());

        f.render_widget(
            Paragraph::new(lines)
                .block(block)
                .scroll((self.scroll_offset, 0)),
            area,
        );

        total_lines
    }
}

/// Spawn a test-execution thread for the given problem.
pub fn spawn_test_runner(problem_id: String) -> mpsc::Receiver<TestMessage> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let Some(problem) = problems::get_problem(&problem_id) else {
                let _ = tx.send(TestMessage::Panicked(format!(
                    "Unknown problem: {}",
                    problem_id
                )));
                return;
            };

            let test_cases = problem.generate_tests();
            let total = test_cases.len();
            let mut first_ops: Vec<Operation> = Vec::new();

            for (i, test) in test_cases.iter().enumerate() {
                let mut log = OperationLog::new();

                // Catch panics from individual test cases (e.g., todo!())
                let case_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    problem.run_solution(test, &mut log)
                }));

                // Capture ops from the first test for replay
                if i == 0 {
                    first_ops = log.operations().to_vec();
                }

                match case_result {
                    Ok(result) => {
                        let _ = tx.send(TestMessage::CaseResult {
                            index: i,
                            total,
                            passed: result.is_correct,
                            input_desc: result.input_description,
                            expected: result.expected,
                            actual: result.actual,
                            comparisons: log.comparisons(),
                            swaps: log.swaps(),
                            ops: log.total_operations(),
                        });
                    }
                    Err(panic_info) => {
                        let msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                            s.to_string()
                        } else if let Some(s) = panic_info.downcast_ref::<String>() {
                            s.clone()
                        } else {
                            "panicked".to_string()
                        };
                        let _ = tx.send(TestMessage::CaseResult {
                            index: i,
                            total,
                            passed: false,
                            input_desc: String::new(),
                            expected: String::new(),
                            actual: format!("PANIC: {}", msg),
                            comparisons: 0,
                            swaps: 0,
                            ops: 0,
                        });
                    }
                }
            }
            let _ = tx.send(TestMessage::Done {
                replay_ops: first_ops,
            });
        }));

        if let Err(panic_info) = result {
            let msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "panicked".to_string()
            };
            let _ = tx.send(TestMessage::Panicked(msg));
        }
    });

    rx
}

/// Spawn a complexity measurement thread.
pub fn spawn_complexity_runner(problem_id: String) -> mpsc::Receiver<ComplexityMessage> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let Some(problem) = problems::get_problem(&problem_id) else {
                let _ = tx.send(ComplexityMessage::Error("Unknown problem".to_string()));
                return;
            };
            let cr = complexity::measure_complexity(problem.as_ref());
            let _ = tx.send(ComplexityMessage::Result {
                estimated: cr.estimated_complexity,
                ascii_plot: cr.ascii_plot,
            });
        }));

        if result.is_err() {
            let _ = tx.send(ComplexityMessage::Error(
                "Panic during complexity measurement".to_string(),
            ));
        }
    });

    rx
}

// ─── Complexity View ──────────────────────────────────────

pub struct ComplexityViewState {
    pub loading: bool,
    pub estimated: String,
    pub ascii_plot: String,
    pub error: Option<String>,
    pub scroll_offset: u16,
}

impl ComplexityViewState {
    pub fn new() -> Self {
        Self {
            loading: true,
            estimated: String::new(),
            ascii_plot: String::new(),
            error: None,
            scroll_offset: 0,
        }
    }

    pub fn reset(&mut self) {
        self.loading = true;
        self.estimated.clear();
        self.ascii_plot.clear();
        self.error = None;
        self.scroll_offset = 0;
    }

    pub fn handle_key(&mut self, key: KeyEvent, visible_height: u16) -> Action {
        if self.loading {
            if key.code == KeyCode::Esc {
                return Action::Pop;
            }
            return Action::None;
        }

        let total_lines = self.ascii_plot.lines().count() + 10;
        let max_scroll = total_lines.saturating_sub(visible_height as usize) as u16;

        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.scroll_offset = (self.scroll_offset + 1).min(max_scroll);
                Action::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                Action::None
            }
            KeyCode::Esc => Action::Pop,
            _ => Action::None,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, problem_idx: usize, problems: &[ProblemInfo]) {
        let p = &problems[problem_idx];
        let mut lines = vec![
            Line::raw(""),
            Line::from(vec![
                Span::styled("  Complexity: ", Style::new().fg(Color::DarkGray)),
                Span::styled(&p.name, theme::heading_style()),
            ]),
            Line::raw(""),
        ];

        if self.loading {
            lines.push(Line::from(Span::styled(
                "  Measuring empirical complexity...",
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                "  Running solution at multiple input sizes (this may take a few seconds)",
                theme::muted_style(),
            )));
        } else if let Some(ref err) = self.error {
            lines.push(Line::from(Span::styled(
                format!("  Error: {}", err),
                theme::error_style(),
            )));
        } else {
            lines.push(Line::from(vec![
                Span::styled("  Estimated: ", Style::new().fg(Color::DarkGray)),
                Span::styled(
                    &self.estimated,
                    Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::raw(""));

            for plot_line in self.ascii_plot.lines() {
                lines.push(Line::raw(format!("  {}", plot_line)));
            }
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "[Esc]",
                Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Back"),
        ]));

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Complexity: {} ", p.id))
            .title_style(theme::title_style());

        f.render_widget(
            Paragraph::new(lines)
                .block(block)
                .scroll((self.scroll_offset, 0)),
            area,
        );
    }
}
