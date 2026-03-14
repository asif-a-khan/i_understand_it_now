use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use ratatui::Terminal;

use super::screens::Action;
use super::theme;

/// Map a problem's topic to its solution file path.
pub fn solution_file_path(topic: &str) -> PathBuf {
    let (part_dir, file_stem) = match topic {
        "big_o" => ("part1_foundations", "big_o"),
        "arrays" => ("part1_foundations", "arrays"),
        "strings" => ("part1_foundations", "strings"),
        "linked_lists" => ("part1_foundations", "linked_lists"),
        "stacks_queues" => ("part1_foundations", "stacks_queues"),
        "hash_maps" => ("part1_foundations", "hash_maps"),
        "recursion" => ("part1_foundations", "recursion"),
        "binary_search" => ("part2_sorting", "binary_search"),
        "basic_sorts" => ("part2_sorting", "basic_sorts"),
        "merge_sort" => ("part2_sorting", "merge_sort"),
        "quick_sort" => ("part2_sorting", "quick_sort"),
        "heap_sort" => ("part2_sorting", "heap_sort"),
        "counting_radix" => ("part2_sorting", "counting_radix"),
        "two_pointers" => ("part2_sorting", "two_pointers"),
        "prefix_sum" => ("part2_sorting", "prefix_sum"),
        "binary_trees" => ("part3_trees", "binary_trees"),
        "bst" => ("part3_trees", "bst"),
        "heaps_priority_queues" => ("part3_trees", "heaps_priority_queues"),
        "balanced_bst" => ("part3_trees", "balanced_bst"),
        "tries" => ("part3_trees", "tries"),
        "graph_representations" => ("part4_graphs", "graph_representations"),
        "graph_bfs_dfs" => ("part4_graphs", "graph_bfs_dfs"),
        "matrix_grid" => ("part4_graphs", "matrix_grid"),
        "topological_sort" => ("part4_graphs", "topological_sort"),
        "shortest_path" => ("part4_graphs", "shortest_path"),
        "mst" => ("part4_graphs", "mst"),
        "union_find" => ("part4_graphs", "union_find"),
        "backtracking" => ("part5_paradigms", "backtracking"),
        "greedy" => ("part5_paradigms", "greedy"),
        "dynamic_programming" => ("part5_paradigms", "dynamic_programming"),
        "divide_conquer" => ("part5_paradigms", "divide_conquer"),
        "intervals" => ("part5_paradigms", "intervals"),
        "segment_fenwick" => ("part6_advanced", "segment_fenwick"),
        "sparse_tables" => ("part6_advanced", "sparse_tables"),
        "monotonic" => ("part6_advanced", "monotonic"),
        "bit_manipulation" => ("part6_advanced", "bit_manipulation"),
        "string_algorithms" => ("part6_advanced", "string_algorithms"),
        "math_geometry" => ("part6_advanced", "math_geometry"),
        _ => ("part1_foundations", topic),
    };

    PathBuf::from("src/solutions")
        .join(part_dir)
        .join(format!("{}.rs", file_stem))
}

/// Suspend the TUI, launch $EDITOR on the given file, then resume the TUI.
pub fn launch_external_editor(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    path: &str,
) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), terminal::LeaveAlternateScreen)?;

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let status = Command::new(&editor).arg(path).status();

    crossterm::execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    terminal.clear()?;

    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(io::Error::other(format!(
            "Editor exited with status: {}",
            s
        ))),
        Err(e) => Err(io::Error::other(format!(
            "Failed to launch editor '{}': {}",
            editor, e
        ))),
    }
}

// ─── In-TUI Editor ───────────────────────────────────────

/// Message from cargo check thread.
pub enum CheckMessage {
    Done(HashMap<usize, String>),
}

pub struct InTuiEditorState {
    /// Full file content (all functions).
    all_lines: Vec<String>,
    /// Just the function being edited (extracted slice).
    lines: Vec<String>,
    /// Original function content (for reset).
    original_lines: Vec<String>,
    /// Range in all_lines that `lines` replaces: [fn_start..fn_end).
    fn_start: usize,
    fn_end: usize,
    /// Name of the function being edited (for display).
    fn_name: String,
    cursor_row: usize,
    cursor_col: usize,
    scroll_row: usize,
    file_path: String,
    modified: bool,
    loaded_idx: Option<usize>,
    status_msg: String,
    errors: HashMap<usize, String>,
    checking: bool,
    /// Pending reset confirmation.
    confirm_reset: bool,
}

impl InTuiEditorState {
    pub fn new() -> Self {
        Self {
            all_lines: Vec::new(),
            lines: vec![String::new()],
            original_lines: Vec::new(),
            fn_start: 0,
            fn_end: 0,
            fn_name: String::new(),
            cursor_row: 0,
            cursor_col: 0,
            scroll_row: 0,
            file_path: String::new(),
            modified: false,
            loaded_idx: None,
            status_msg: String::new(),
            errors: HashMap::new(),
            checking: false,
            confirm_reset: false,
        }
    }

    pub fn load_file(&mut self, problem_idx: usize, topic: &str, problem_id: &str) {
        if self.loaded_idx == Some(problem_idx) {
            return;
        }
        self.loaded_idx = Some(problem_idx);
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.scroll_row = 0;
        self.modified = false;
        self.status_msg.clear();
        self.errors.clear();
        self.checking = false;

        let path = solution_file_path(topic);
        self.file_path = path.to_string_lossy().to_string();

        match std::fs::read_to_string(&path) {
            Ok(content) => {
                self.all_lines = content.lines().map(String::from).collect();
                self.extract_function(problem_id, topic);
            }
            Err(e) => {
                self.all_lines = vec![format!("// Error reading file: {}", e)];
                self.lines = self.all_lines.clone();
                self.fn_start = 0;
                self.fn_end = self.lines.len();
                self.fn_name = problem_id.to_string();
                self.status_msg = format!("Error: {}", e);
            }
        }
    }

    /// Extract just the function matching problem_id from the full file.
    fn extract_function(&mut self, problem_id: &str, topic: &str) {
        let func_name = problem_id
            .strip_prefix(topic)
            .and_then(|s| s.strip_prefix('_'))
            .unwrap_or(problem_id);

        self.fn_name = func_name.to_string();
        let search = format!("fn {}(", func_name);

        // Find the fn line
        let fn_line = self
            .all_lines
            .iter()
            .position(|line| line.contains(&search));

        let Some(fn_line) = fn_line else {
            // Function not found — show entire file
            self.lines = self.all_lines.clone();
            self.fn_start = 0;
            self.fn_end = self.lines.len();
            return;
        };

        // Walk backwards to find doc comments and preceding blank/comment lines
        let mut start = fn_line;
        while start > 0 {
            let prev = &self.all_lines[start - 1];
            let trimmed = prev.trim();
            if trimmed.starts_with("///")
                || trimmed.starts_with("//")
                || trimmed.starts_with("#[")
                || trimmed.is_empty()
            {
                start -= 1;
            } else {
                break;
            }
        }
        // Skip leading blank lines at the very top of the range
        while start < fn_line && self.all_lines[start].trim().is_empty() {
            start += 1;
        }

        // Walk forwards to find the closing brace (track brace depth)
        let mut end = fn_line;
        let mut depth: i32 = 0;
        let mut found_open = false;
        for (i, line) in self.all_lines.iter().enumerate().skip(fn_line) {
            for ch in line.chars() {
                if ch == '{' {
                    depth += 1;
                    found_open = true;
                } else if ch == '}' {
                    depth -= 1;
                }
            }
            if found_open && depth <= 0 {
                end = i + 1; // exclusive
                break;
            }
            end = i + 1;
        }

        self.fn_start = start;
        self.fn_end = end;
        self.lines = self.all_lines[start..end].to_vec();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.original_lines = self.lines.clone();
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Action {
        // Ctrl+S: save
        if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.confirm_reset = false;
            self.save();
            return Action::None;
        }

        // Ctrl+R: reset to original
        if key.code == KeyCode::Char('r') && key.modifiers.contains(KeyModifiers::CONTROL) {
            if self.confirm_reset {
                // Second press — do the reset
                self.lines = self.original_lines.clone();
                self.cursor_row = 0;
                self.cursor_col = 0;
                self.scroll_row = 0;
                self.modified = true;
                self.errors.clear();
                self.confirm_reset = false;
                self.status_msg = "Reset to original — Ctrl+S to save".to_string();
            } else {
                // First press — ask for confirmation
                self.confirm_reset = true;
                self.status_msg = "Press Ctrl+R again to reset to original".to_string();
            }
            return Action::None;
        }

        // Esc: close editor
        if key.code == KeyCode::Esc {
            self.loaded_idx = None;
            return Action::Pop;
        }

        self.confirm_reset = false;
        self.status_msg.clear();

        match key.code {
            KeyCode::Left => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                } else if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.cursor_col = self.lines[self.cursor_row].len();
                }
            }
            KeyCode::Right => {
                let line_len = self.lines[self.cursor_row].len();
                if self.cursor_col < line_len {
                    self.cursor_col += 1;
                } else if self.cursor_row + 1 < self.lines.len() {
                    self.cursor_row += 1;
                    self.cursor_col = 0;
                }
            }
            KeyCode::Up => {
                if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.cursor_col = self.cursor_col.min(self.lines[self.cursor_row].len());
                }
            }
            KeyCode::Down => {
                if self.cursor_row + 1 < self.lines.len() {
                    self.cursor_row += 1;
                    self.cursor_col = self.cursor_col.min(self.lines[self.cursor_row].len());
                }
            }
            KeyCode::Home => self.cursor_col = 0,
            KeyCode::End => self.cursor_col = self.lines[self.cursor_row].len(),
            KeyCode::PageUp => {
                self.cursor_row = self.cursor_row.saturating_sub(20);
                self.cursor_col = self.cursor_col.min(self.lines[self.cursor_row].len());
            }
            KeyCode::PageDown => {
                self.cursor_row = (self.cursor_row + 20).min(self.lines.len() - 1);
                self.cursor_col = self.cursor_col.min(self.lines[self.cursor_row].len());
            }
            KeyCode::Char(c) => {
                let line = &mut self.lines[self.cursor_row];
                let byte_idx = char_to_byte_idx(line, self.cursor_col);
                line.insert(byte_idx, c);
                self.cursor_col += 1;
                self.modified = true;
            }
            KeyCode::Tab => {
                let line = &mut self.lines[self.cursor_row];
                let byte_idx = char_to_byte_idx(line, self.cursor_col);
                line.insert_str(byte_idx, "    ");
                self.cursor_col += 4;
                self.modified = true;
            }
            KeyCode::Backspace => {
                if self.cursor_col > 0 {
                    let line = &mut self.lines[self.cursor_row];
                    let byte_idx = char_to_byte_idx(line, self.cursor_col - 1);
                    let next_byte = char_to_byte_idx(line, self.cursor_col);
                    line.drain(byte_idx..next_byte);
                    self.cursor_col -= 1;
                    self.modified = true;
                } else if self.cursor_row > 0 {
                    let current_line = self.lines.remove(self.cursor_row);
                    self.cursor_row -= 1;
                    self.cursor_col = self.lines[self.cursor_row].len();
                    self.lines[self.cursor_row].push_str(&current_line);
                    self.modified = true;
                }
            }
            KeyCode::Delete => {
                let line_len = self.lines[self.cursor_row].len();
                if self.cursor_col < line_len {
                    let line = &mut self.lines[self.cursor_row];
                    let byte_idx = char_to_byte_idx(line, self.cursor_col);
                    let next_byte = char_to_byte_idx(line, self.cursor_col + 1);
                    line.drain(byte_idx..next_byte);
                    self.modified = true;
                } else if self.cursor_row + 1 < self.lines.len() {
                    let next_line = self.lines.remove(self.cursor_row + 1);
                    self.lines[self.cursor_row].push_str(&next_line);
                    self.modified = true;
                }
            }
            KeyCode::Enter => {
                let line = &mut self.lines[self.cursor_row];
                let byte_idx = char_to_byte_idx(line, self.cursor_col);
                let rest = line[byte_idx..].to_string();
                line.truncate(byte_idx);
                self.cursor_row += 1;
                self.lines.insert(self.cursor_row, rest);
                self.cursor_col = 0;
                self.modified = true;
            }
            _ => {}
        }

        Action::None
    }

    fn save(&mut self) {
        // Splice edited function back into the full file
        let mut full = self.all_lines.clone();
        full.splice(self.fn_start..self.fn_end, self.lines.iter().cloned());
        self.fn_end = self.fn_start + self.lines.len();
        self.all_lines = full;

        let content = self.all_lines.join("\n") + "\n";
        match std::fs::write(&self.file_path, content) {
            Ok(()) => {
                self.modified = false;
                self.status_msg = "Saved — checking...".to_string();
                self.checking = true;
                self.errors.clear();
            }
            Err(e) => {
                self.status_msg = format!("Save failed: {}", e);
            }
        }
    }

    pub fn apply_check_result(&mut self, file_errors: HashMap<usize, String>) {
        self.checking = false;
        // Map file line numbers to editor line numbers (offset by fn_start)
        let errors: HashMap<usize, String> = file_errors
            .into_iter()
            .filter_map(|(line, msg)| {
                if line >= self.fn_start && line < self.fn_start + self.lines.len() {
                    Some((line - self.fn_start, msg))
                } else {
                    None
                }
            })
            .collect();

        if errors.is_empty() {
            self.status_msg = "Saved — no errors".to_string();
        } else {
            self.status_msg = format!("Saved — {} error(s)", errors.len());
        }
        self.errors = errors;
    }

    pub fn is_checking(&self) -> bool {
        self.checking
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);

        let gutter_width = 5u16;
        let error_gutter = 2u16;

        let title = format!(" {}  —  fn {} ", self.file_path, self.fn_name);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_style(if self.modified {
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                theme::title_style()
            });

        let inner = block.inner(chunks[0]);
        f.render_widget(block, chunks[0]);

        let visible_height = inner.height as usize;
        let visible_width = (inner.width.saturating_sub(gutter_width + error_gutter)) as usize;

        let scroll_row = if self.cursor_row < self.scroll_row {
            self.cursor_row
        } else if self.cursor_row >= self.scroll_row + visible_height {
            self.cursor_row - visible_height + 1
        } else {
            self.scroll_row
        };

        let mut display_lines: Vec<Line> = Vec::new();

        for i in scroll_row..scroll_row + visible_height {
            if i < self.lines.len() {
                let has_error = self.errors.contains_key(&i);
                let file_line_num = self.fn_start + i + 1; // 1-indexed file line

                let error_marker = if has_error {
                    Span::styled(
                        "E ",
                        Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw("  ")
                };

                let line_num = Span::styled(
                    format!("{:>4} ", file_line_num),
                    if has_error {
                        Style::new().fg(Color::Red)
                    } else if i == self.cursor_row {
                        Style::new().fg(Color::Yellow)
                    } else {
                        Style::new().fg(Color::DarkGray)
                    },
                );

                let content = &self.lines[i];
                let display_content = if content.len() > visible_width {
                    &content[..visible_width]
                } else {
                    content
                };

                let content_spans = if has_error {
                    vec![Span::styled(
                        display_content.to_string(),
                        Style::new()
                            .fg(Color::Red)
                            .add_modifier(Modifier::UNDERLINED),
                    )]
                } else {
                    highlight_rust(display_content)
                };

                let mut spans = vec![error_marker, line_num];
                spans.extend(content_spans);
                display_lines.push(Line::from(spans));

                // Show error message inline near cursor
                if has_error
                    && (i == self.cursor_row
                        || i + 1 == self.cursor_row
                        || (self.cursor_row > 0 && i == self.cursor_row - 1))
                {
                    if let Some(msg) = self.errors.get(&i) {
                        display_lines.push(Line::from(vec![
                            Span::raw("       "),
                            Span::styled(
                                format!("^ {}", msg),
                                Style::new().fg(Color::Red).add_modifier(Modifier::ITALIC),
                            ),
                        ]));
                    }
                }
            } else {
                display_lines.push(Line::from(Span::styled(
                    "~",
                    Style::new().fg(Color::DarkGray),
                )));
            }
        }

        f.render_widget(Paragraph::new(display_lines), inner);

        let cursor_screen_row = (self.cursor_row - scroll_row) as u16 + inner.y;
        let cursor_screen_col = self.cursor_col as u16 + inner.x + gutter_width + error_gutter;
        if cursor_screen_row < inner.y + inner.height && cursor_screen_col < inner.x + inner.width {
            f.set_cursor_position(Position {
                x: cursor_screen_col,
                y: cursor_screen_row,
            });
        }

        // Status bar
        let modified_indicator = if self.modified { " [+] " } else { " " };
        let checking_indicator = if self.checking { " checking... " } else { "" };
        let error_count = if !self.errors.is_empty() {
            format!(" {} error(s) ", self.errors.len())
        } else {
            String::new()
        };

        let status_style = if !self.status_msg.is_empty() {
            if self.status_msg.contains("error") {
                theme::error_style()
            } else if self.status_msg.contains("no errors") {
                theme::success_style()
            } else if self.checking {
                theme::warning_style()
            } else {
                theme::muted_style()
            }
        } else {
            theme::muted_style()
        };

        let status_text = if !self.status_msg.is_empty() {
            self.status_msg.as_str()
        } else {
            "Ctrl+S save  |  Ctrl+R reset  |  Esc close"
        };

        let status = Line::from(vec![
            Span::styled(
                format!(
                    " Ln {}, Col {}{}",
                    self.fn_start + self.cursor_row + 1,
                    self.cursor_col + 1,
                    modified_indicator
                ),
                theme::muted_style(),
            ),
            Span::styled(checking_indicator, theme::warning_style()),
            Span::styled(
                error_count,
                Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(status_text, status_style),
        ]);
        f.render_widget(Paragraph::new(status), chunks[1]);
    }
}

/// Spawn cargo check and return a receiver for the results.
pub fn spawn_cargo_check(file_path: String) -> mpsc::Receiver<CheckMessage> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let output = Command::new("cargo")
            .args(["check", "--message-format=short"])
            .output();

        let mut errors: HashMap<usize, String> = HashMap::new();

        if let Ok(output) = output {
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                if !line.contains("error") {
                    continue;
                }
                if !line.contains(&file_path) {
                    continue;
                }
                let parts: Vec<&str> = line.splitn(4, ':').collect();
                if parts.len() >= 4 {
                    if let Ok(line_num) = parts[1].trim().parse::<usize>() {
                        let msg = parts[3..].join(":").trim().to_string();
                        errors.entry(line_num.saturating_sub(1)).or_insert(msg);
                    }
                }
            }
        }

        let _ = tx.send(CheckMessage::Done(errors));
    });

    rx
}

fn char_to_byte_idx(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

// ─── Syntax Highlighting ─────────────────────────────────

const KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "if", "else", "for", "while", "loop", "match", "return", "pub", "use",
    "mod", "struct", "impl", "enum", "trait", "type", "const", "static", "where", "as", "in",
    "ref", "self", "super", "crate", "true", "false", "break", "continue", "move", "async",
    "await", "dyn", "extern", "unsafe",
];

const TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64", "bool", "char", "str", "String", "Vec", "Option", "Result", "Box", "Rc", "Arc",
    "HashMap", "HashSet", "BTreeMap", "BTreeSet", "Self",
];

fn highlight_rust(line: &str) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        // Line comment
        if ch == '/' && i + 1 < len && chars[i + 1] == '/' {
            let rest: String = chars[i..].iter().collect();
            spans.push(Span::styled(rest, Style::new().fg(Color::DarkGray)));
            return spans;
        }

        // String literal
        if ch == '"' {
            let mut s = String::new();
            s.push(ch);
            i += 1;
            while i < len {
                s.push(chars[i]);
                if chars[i] == '"' && (s.len() < 2 || chars[i - 1] != '\\') {
                    i += 1;
                    break;
                }
                i += 1;
            }
            spans.push(Span::styled(s, Style::new().fg(Color::Green)));
            continue;
        }

        // Char literal
        if ch == '\'' && i + 2 < len {
            if chars[i + 1] != ' ' && chars[i + 2] == '\'' {
                let s: String = chars[i..i + 3].iter().collect();
                spans.push(Span::styled(s, Style::new().fg(Color::Green)));
                i += 3;
                continue;
            }
            if chars[i + 1] == '\\' && i + 3 < len && chars[i + 3] == '\'' {
                let s: String = chars[i..i + 4].iter().collect();
                spans.push(Span::styled(s, Style::new().fg(Color::Green)));
                i += 4;
                continue;
            }
        }

        // Number
        if ch.is_ascii_digit() {
            let mut s = String::new();
            while i < len
                && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_')
            {
                s.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(s, Style::new().fg(Color::LightMagenta)));
            continue;
        }

        // Word
        if ch.is_alphabetic() || ch == '_' {
            let mut word = String::new();
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                word.push(chars[i]);
                i += 1;
            }

            // Macro
            if i < len && chars[i] == '!' {
                word.push('!');
                i += 1;
                spans.push(Span::styled(
                    word,
                    Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ));
                continue;
            }

            let style = if KEYWORDS.contains(&word.as_str()) {
                Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD)
            } else if TYPES.contains(&word.as_str()) {
                Style::new().fg(Color::Yellow)
            } else if word == "todo" {
                Style::new().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(Color::White)
            };
            spans.push(Span::styled(word, style));
            continue;
        }

        // Brackets/punctuation
        if "{}()[];,".contains(ch) {
            spans.push(Span::styled(
                ch.to_string(),
                Style::new().fg(Color::DarkGray),
            ));
            i += 1;
            continue;
        }

        // Operators
        if "=<>!+-*/%&|^~".contains(ch) {
            let mut op = String::new();
            while i < len && "=<>!+-*/%&|^~:".contains(chars[i]) {
                op.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(op, Style::new().fg(Color::Cyan)));
            continue;
        }

        spans.push(Span::raw(ch.to_string()));
        i += 1;
    }

    if spans.is_empty() {
        spans.push(Span::raw(""));
    }
    spans
}
