use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::progress::Progress;

use super::markdown;
use super::screens::{Action, Screen};
use super::theme;

/// Metadata for a lesson file.
pub struct LessonInfo {
    pub filename: String,
    pub title: String,
    pub lesson_id: String,
}

impl LessonInfo {
    /// Scan the lessons/ directory and return sorted lesson metadata.
    pub fn load_all() -> Vec<LessonInfo> {
        let mut lessons = Vec::new();
        let dir = std::path::Path::new("lessons");
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "md") {
                    let filename = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let lesson_id = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    // Read first line for title
                    let title = std::fs::read_to_string(&path)
                        .ok()
                        .and_then(|content| {
                            content
                                .lines()
                                .next()
                                .map(|line| line.trim_start_matches('#').trim().to_string())
                        })
                        .unwrap_or_else(|| lesson_id.clone());

                    lessons.push(LessonInfo {
                        filename,
                        title,
                        lesson_id,
                    });
                }
            }
        }
        lessons.sort_by(|a, b| a.filename.cmp(&b.filename));
        lessons
    }
}

// ─── Lesson List ─────────────────────────────────────────

pub struct LessonListState {
    pub list_state: ListState,
    pub search_active: bool,
    pub search_query: String,
}

impl LessonListState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            list_state,
            search_active: false,
            search_query: String::new(),
        }
    }

    /// Return indices of lessons matching the current search query.
    fn filtered_indices(&self, lessons: &[LessonInfo]) -> Vec<usize> {
        if self.search_query.is_empty() {
            return (0..lessons.len()).collect();
        }
        let query = self.search_query.to_lowercase();
        lessons
            .iter()
            .enumerate()
            .filter(|(_, l)| {
                l.title.to_lowercase().contains(&query)
                    || l.lesson_id.to_lowercase().contains(&query)
            })
            .map(|(i, _)| i)
            .collect()
    }

    pub fn handle_key(&mut self, key: KeyEvent, lessons: &[LessonInfo]) -> Action {
        let filtered = self.filtered_indices(lessons);
        let len = filtered.len();

        // Search-mode input handling
        if self.search_active {
            match key.code {
                KeyCode::Char(c) => {
                    self.search_query.push(c);
                    // Reset selection to first filtered result
                    self.list_state.select(Some(0));
                    return Action::None;
                }
                KeyCode::Backspace => {
                    self.search_query.pop();
                    self.list_state.select(Some(0));
                    return Action::None;
                }
                KeyCode::Esc => {
                    self.search_active = false;
                    self.search_query.clear();
                    self.list_state.select(Some(0));
                    return Action::None;
                }
                KeyCode::Enter => {
                    self.search_active = false;
                    // Keep the query and current selection
                    return Action::None;
                }
                KeyCode::Down => {
                    if len > 0 {
                        let i = self.list_state.selected().unwrap_or(0);
                        self.list_state.select(Some((i + 1).min(len - 1)));
                    }
                    return Action::None;
                }
                KeyCode::Up => {
                    let i = self.list_state.selected().unwrap_or(0);
                    self.list_state.select(Some(i.saturating_sub(1)));
                    return Action::None;
                }
                _ => return Action::None,
            }
        }

        // Normal mode
        if len == 0 {
            if key.code == KeyCode::Esc {
                if !self.search_query.is_empty() {
                    self.search_query.clear();
                    self.list_state.select(Some(0));
                    return Action::None;
                }
                return Action::Pop;
            }
            return Action::None;
        }

        match key.code {
            KeyCode::Char('/') => {
                self.search_active = true;
                self.search_query.clear();
                Action::None
            }
            KeyCode::Char('j') | KeyCode::Down => {
                let i = self.list_state.selected().unwrap_or(0);
                self.list_state.select(Some((i + 1).min(len - 1)));
                Action::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let i = self.list_state.selected().unwrap_or(0);
                self.list_state.select(Some(i.saturating_sub(1)));
                Action::None
            }
            KeyCode::Enter => {
                let idx = self.list_state.selected().unwrap_or(0);
                // Map filtered index back to the original lesson index
                if let Some(&original_idx) = filtered.get(idx) {
                    Action::Push(Screen::LessonReader {
                        lesson_idx: original_idx,
                    })
                } else {
                    Action::None
                }
            }
            KeyCode::Esc => {
                if !self.search_query.is_empty() {
                    self.search_query.clear();
                    self.list_state.select(Some(0));
                    Action::None
                } else {
                    Action::Pop
                }
            }
            _ => Action::None,
        }
    }

    pub fn render(
        &mut self,
        f: &mut Frame,
        area: Rect,
        lessons: &[LessonInfo],
        progress: &Progress,
    ) {
        // Always show search bar at bottom
        let chunks = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(area);
        let list_area = chunks[0];
        let search_bar_area = chunks[1];

        let filtered = self.filtered_indices(lessons);
        let items: Vec<ListItem> = filtered
            .iter()
            .map(|&idx| {
                let lesson = &lessons[idx];
                let read = progress
                    .lessons_read
                    .get(&lesson.lesson_id)
                    .copied()
                    .unwrap_or(false);
                let check = if read { "[x]" } else { "[ ]" };
                let check_style = if read {
                    Style::new().fg(Color::Green)
                } else {
                    Style::new().fg(Color::DarkGray)
                };

                ListItem::new(Line::from(vec![
                    Span::styled(format!(" {} ", check), check_style),
                    Span::raw(&lesson.title),
                ]))
            })
            .collect();

        let read_count = progress.lessons_read.values().filter(|&&v| v).count();
        let title = if !self.search_query.is_empty() {
            format!(" Lessons — {} matching ", filtered.len())
        } else {
            format!(" Lessons ({read_count}/{}) ", lessons.len())
        };
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_style(theme::title_style()),
            )
            .highlight_style(theme::selected_style())
            .highlight_symbol("> ");

        f.render_stateful_widget(list, list_area, &mut self.list_state);

        // Persistent search bar
        let search_line = if self.search_active {
            let mut spans = vec![
                Span::styled(
                    " / ",
                    Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ),
                Span::styled(&self.search_query, Style::new().fg(Color::White)),
                Span::styled("\u{2588}", Style::new().fg(Color::White)),
            ];
            if !self.search_query.is_empty() {
                spans.push(Span::styled(
                    format!("  ({} results)", filtered.len()),
                    Style::new().fg(Color::DarkGray),
                ));
            }
            Line::from(spans)
        } else {
            Line::from(Span::styled(
                " / Press / to search...",
                Style::new()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            ))
        };
        f.render_widget(Paragraph::new(search_line), search_bar_area);
    }
}

// ─── Lesson Reader ────────────────────────────────────────

pub struct LessonReaderState {
    pub scroll_offset: u16,
    pub parsed_lines: Vec<Line<'static>>,
    pub lesson_idx: usize,
}

impl LessonReaderState {
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            parsed_lines: Vec::new(),
            lesson_idx: usize::MAX,
        }
    }

    /// Load and parse a lesson if not already loaded.
    pub fn load_lesson(&mut self, idx: usize, lessons: &[LessonInfo]) {
        if self.lesson_idx == idx {
            return; // already loaded
        }
        self.lesson_idx = idx;
        self.scroll_offset = 0;

        if let Some(lesson) = lessons.get(idx) {
            let path = std::path::Path::new("lessons").join(&lesson.filename);
            if let Ok(content) = std::fs::read_to_string(&path) {
                self.parsed_lines = markdown::render_markdown(&content);
            } else {
                self.parsed_lines = vec![Line::from(Span::styled(
                    "Failed to read lesson file",
                    theme::error_style(),
                ))];
            }
        }
    }

    pub fn handle_key(
        &mut self,
        key: KeyEvent,
        lessons: &[LessonInfo],
        progress: &mut Progress,
        visible_height: u16,
    ) -> Action {
        let max_scroll = self
            .parsed_lines
            .len()
            .saturating_sub(visible_height as usize) as u16;

        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.scroll_offset = (self.scroll_offset + 1).min(max_scroll);
                Action::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                Action::None
            }
            KeyCode::Char(' ') | KeyCode::PageDown => {
                self.scroll_offset =
                    (self.scroll_offset + visible_height.saturating_sub(2)).min(max_scroll);
                Action::None
            }
            KeyCode::PageUp => {
                self.scroll_offset = self
                    .scroll_offset
                    .saturating_sub(visible_height.saturating_sub(2));
                Action::None
            }
            KeyCode::Char('g') => {
                self.scroll_offset = 0;
                Action::None
            }
            KeyCode::Char('G') => {
                self.scroll_offset = max_scroll;
                Action::None
            }
            KeyCode::Char('m') => {
                if let Some(lesson) = lessons.get(self.lesson_idx) {
                    let current = progress
                        .lessons_read
                        .get(&lesson.lesson_id)
                        .copied()
                        .unwrap_or(false);
                    progress
                        .lessons_read
                        .insert(lesson.lesson_id.clone(), !current);
                    let _ = progress.save();
                }
                Action::None
            }
            KeyCode::Esc => {
                self.lesson_idx = usize::MAX; // force reload next time
                Action::Pop
            }
            _ => Action::None,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, lessons: &[LessonInfo], progress: &Progress) {
        let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);

        let title = lessons
            .get(self.lesson_idx)
            .map(|l| l.title.clone())
            .unwrap_or_default();

        let is_read = lessons.get(self.lesson_idx).is_some_and(|l| {
            progress
                .lessons_read
                .get(&l.lesson_id)
                .copied()
                .unwrap_or(false)
        });
        let read_indicator = if is_read { " [read] " } else { " [unread] " };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", title))
            .title_style(theme::title_style());

        let inner = block.inner(chunks[0]);

        let paragraph = Paragraph::new(self.parsed_lines.clone())
            .block(block)
            .scroll((self.scroll_offset, 0));
        f.render_widget(paragraph, chunks[0]);

        // Status bar
        let total_lines = self.parsed_lines.len();
        let pct = if total_lines > 0 {
            ((self.scroll_offset as usize + inner.height as usize) * 100 / total_lines).min(100)
        } else {
            100
        };

        let status = Line::from(vec![
            Span::styled(
                format!("  {read_indicator}"),
                if is_read {
                    theme::success_style()
                } else {
                    theme::muted_style()
                },
            ),
            Span::styled("  [m] toggle read  ", theme::muted_style()),
            Span::styled(
                format!("  {pct}%  {}/{total_lines}  ", self.scroll_offset),
                theme::muted_style(),
            ),
        ]);
        f.render_widget(Paragraph::new(status), chunks[1]);
    }
}
