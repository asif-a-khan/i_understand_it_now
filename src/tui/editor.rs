use std::io;
use std::path::PathBuf;
use std::process::Command;

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
        Ok(s) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Editor exited with status: {}", s),
        )),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to launch editor '{}': {}", editor, e),
        )),
    }
}

// ─── In-TUI Editor ───────────────────────────────────────

pub struct InTuiEditorState {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    scroll_row: usize,
    scroll_col: usize,
    file_path: String,
    modified: bool,
    loaded_idx: Option<usize>,
    status_msg: String,
}

impl InTuiEditorState {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_row: 0,
            cursor_col: 0,
            scroll_row: 0,
            scroll_col: 0,
            file_path: String::new(),
            modified: false,
            loaded_idx: None,
            status_msg: String::new(),
        }
    }

    pub fn load_file(&mut self, problem_idx: usize, topic: &str) {
        if self.loaded_idx == Some(problem_idx) {
            return;
        }
        self.loaded_idx = Some(problem_idx);
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.scroll_row = 0;
        self.scroll_col = 0;
        self.modified = false;
        self.status_msg.clear();

        let path = solution_file_path(topic);
        self.file_path = path.to_string_lossy().to_string();

        match std::fs::read_to_string(&path) {
            Ok(content) => {
                self.lines = content.lines().map(String::from).collect();
                if self.lines.is_empty() {
                    self.lines.push(String::new());
                }
            }
            Err(e) => {
                self.lines = vec![format!("// Error reading file: {}", e)];
                self.status_msg = format!("Error: {}", e);
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Action {
        // Ctrl+S: save
        if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.save();
            return Action::None;
        }

        // Esc: close editor
        if key.code == KeyCode::Esc {
            self.loaded_idx = None;
            return Action::Pop;
        }

        self.status_msg.clear();

        match key.code {
            // Cursor movement
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
            KeyCode::Home => {
                self.cursor_col = 0;
            }
            KeyCode::End => {
                self.cursor_col = self.lines[self.cursor_row].len();
            }
            KeyCode::PageUp => {
                self.cursor_row = self.cursor_row.saturating_sub(20);
                self.cursor_col = self.cursor_col.min(self.lines[self.cursor_row].len());
            }
            KeyCode::PageDown => {
                self.cursor_row = (self.cursor_row + 20).min(self.lines.len() - 1);
                self.cursor_col = self.cursor_col.min(self.lines[self.cursor_row].len());
            }

            // Text editing
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
        let content = self.lines.join("\n") + "\n";
        match std::fs::write(&self.file_path, content) {
            Ok(()) => {
                self.modified = false;
                self.status_msg = "Saved!".to_string();
            }
            Err(e) => {
                self.status_msg = format!("Save failed: {}", e);
            }
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Min(1),    // Editor area
            Constraint::Length(1), // Status bar
        ])
        .split(area);

        let gutter_width = 5u16; // line number width

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", self.file_path))
            .title_style(if self.modified {
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                theme::title_style()
            });

        let inner = block.inner(chunks[0]);
        f.render_widget(block, chunks[0]);

        let visible_height = inner.height as usize;
        let visible_width = (inner.width.saturating_sub(gutter_width)) as usize;

        // Adjust scroll to keep cursor visible
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
                let line_num = Span::styled(
                    format!("{:>4} ", i + 1),
                    if i == self.cursor_row {
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

                let content_style = if i == self.cursor_row {
                    Style::new().fg(Color::White)
                } else {
                    style_for_rust_line(display_content)
                };

                display_lines.push(Line::from(vec![
                    line_num,
                    Span::styled(display_content.to_string(), content_style),
                ]));
            } else {
                display_lines.push(Line::from(Span::styled(
                    "~",
                    Style::new().fg(Color::DarkGray),
                )));
            }
        }

        f.render_widget(Paragraph::new(display_lines), inner);

        // Position cursor
        let cursor_screen_row = (self.cursor_row - scroll_row) as u16 + inner.y;
        let cursor_screen_col = self.cursor_col as u16 + inner.x + gutter_width;
        if cursor_screen_row < inner.y + inner.height
            && cursor_screen_col < inner.x + inner.width
        {
            f.set_cursor_position(Position {
                x: cursor_screen_col,
                y: cursor_screen_row,
            });
        }

        // Status bar
        let modified_indicator = if self.modified { " [+] " } else { " " };
        let status_style = if !self.status_msg.is_empty() {
            if self.status_msg.starts_with("Error") || self.status_msg.starts_with("Save failed") {
                theme::error_style()
            } else {
                theme::success_style()
            }
        } else {
            theme::muted_style()
        };

        let status_text = if !self.status_msg.is_empty() {
            &self.status_msg
        } else {
            "Ctrl+S save  |  Esc close"
        };

        let status = Line::from(vec![
            Span::styled(
                format!(" Ln {}, Col {}{}", self.cursor_row + 1, self.cursor_col + 1, modified_indicator),
                theme::muted_style(),
            ),
            Span::styled(status_text, status_style),
        ]);
        f.render_widget(Paragraph::new(status), chunks[1]);
    }
}

/// Convert a char index to a byte index in a string.
fn char_to_byte_idx(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

/// Basic syntax coloring for Rust lines.
fn style_for_rust_line(line: &str) -> Style {
    let trimmed = line.trim();
    if trimmed.starts_with("//") {
        Style::new().fg(Color::DarkGray)
    } else if trimmed.starts_with("pub ")
        || trimmed.starts_with("fn ")
        || trimmed.starts_with("use ")
        || trimmed.starts_with("mod ")
        || trimmed.starts_with("let ")
        || trimmed.starts_with("struct ")
        || trimmed.starts_with("impl ")
        || trimmed.starts_with("enum ")
    {
        Style::new().fg(Color::Cyan)
    } else {
        Style::new().fg(Color::White)
    }
}
