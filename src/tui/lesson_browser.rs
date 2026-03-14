use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
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
                if path.extension().map_or(false, |e| e == "md") {
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
                            content.lines().next().map(|line| {
                                line.trim_start_matches('#').trim().to_string()
                            })
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
}

impl LessonListState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self { list_state }
    }

    pub fn handle_key(
        &mut self,
        key: KeyEvent,
        lessons: &[LessonInfo],
    ) -> Action {
        let len = lessons.len();
        if len == 0 {
            if key.code == KeyCode::Esc {
                return Action::Pop;
            }
            return Action::None;
        }

        match key.code {
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
                Action::Push(Screen::LessonReader { lesson_idx: idx })
            }
            KeyCode::Esc => Action::Pop,
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
        let items: Vec<ListItem> = lessons
            .iter()
            .map(|lesson| {
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
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Lessons ({read_count}/{}) ", lessons.len()))
                    .title_style(theme::title_style()),
            )
            .highlight_style(theme::selected_style())
            .highlight_symbol("> ");

        f.render_stateful_widget(list, area, &mut self.list_state);
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
                self.parsed_lines =
                    vec![Line::from(Span::styled("Failed to read lesson file", theme::error_style()))];
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
        let max_scroll = self.parsed_lines.len().saturating_sub(visible_height as usize) as u16;

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

    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        lessons: &[LessonInfo],
        progress: &Progress,
    ) {
        let chunks = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

        let title = lessons
            .get(self.lesson_idx)
            .map(|l| l.title.clone())
            .unwrap_or_default();

        let is_read = lessons.get(self.lesson_idx).map_or(false, |l| {
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
