use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::visualizer::{self, HighlightKind, VizFrame};

use super::screens::{Action, Screen};
use super::theme;

// ─── Viz Picker ───────────────────────────────────────────

pub struct VizPickerState {
    pub list_state: ListState,
    viz_names: Vec<&'static str>,
}

impl VizPickerState {
    pub fn new() -> Self {
        let viz_names = visualizer::list_references();
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            list_state,
            viz_names,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Action {
        let len = self.viz_names.len();

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
                Action::Push(Screen::VizPlayer { viz_idx: idx })
            }
            KeyCode::Esc => Action::Pop,
            _ => Action::None,
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .viz_names
            .iter()
            .map(|name| {
                let viz = visualizer::get_reference(name);
                let desc = viz
                    .as_ref()
                    .map(|v| v.description().to_string())
                    .unwrap_or_default();
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("  {:20}", name),
                        Style::new()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(desc, Style::new().fg(Color::DarkGray)),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Visualizations ")
                    .title_style(theme::title_style()),
            )
            .highlight_style(theme::selected_style())
            .highlight_symbol("> ");

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

}

// ─── Viz Player ───────────────────────────────────────────

pub struct VizPlayerState {
    pub frames: Vec<VizFrame>,
    pub current_frame: usize,
    pub auto_play: bool,
    pub delay_ms: u64,
    pub last_tick: std::time::Instant,
    pub viz_name: String,
    loaded_idx: Option<usize>,
}

impl VizPlayerState {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            current_frame: 0,
            auto_play: false,
            delay_ms: 400,
            last_tick: std::time::Instant::now(),
            viz_name: String::new(),
            loaded_idx: None,
        }
    }

    pub fn load_viz(&mut self, viz_idx: usize) {
        if self.loaded_idx == Some(viz_idx) {
            return;
        }
        self.loaded_idx = Some(viz_idx);
        self.current_frame = 0;
        self.auto_play = false;

        let names = visualizer::list_references();
        if let Some(&name) = names.get(viz_idx) {
            self.viz_name = name.to_string();
            if let Some(viz) = visualizer::get_reference(name) {
                let input = viz.default_input();
                self.frames = viz.generate_frames(&input);
            }
        }
    }

    pub fn tick(&mut self) {
        if self.auto_play && !self.frames.is_empty() {
            if self.last_tick.elapsed().as_millis() >= self.delay_ms as u128 {
                self.last_tick = std::time::Instant::now();
                if self.current_frame + 1 < self.frames.len() {
                    self.current_frame += 1;
                } else {
                    self.auto_play = false;
                }
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Action {
        let len = self.frames.len();
        if len == 0 {
            if key.code == KeyCode::Esc {
                self.loaded_idx = None;
                return Action::Pop;
            }
            return Action::None;
        }

        match key.code {
            KeyCode::Right | KeyCode::Enter | KeyCode::Char(' ') => {
                self.auto_play = false;
                if self.current_frame + 1 < len {
                    self.current_frame += 1;
                }
                Action::None
            }
            KeyCode::Left => {
                self.auto_play = false;
                self.current_frame = self.current_frame.saturating_sub(1);
                Action::None
            }
            KeyCode::Home => {
                self.auto_play = false;
                self.current_frame = 0;
                Action::None
            }
            KeyCode::End => {
                self.auto_play = false;
                self.current_frame = len - 1;
                Action::None
            }
            KeyCode::Char('a') => {
                self.auto_play = !self.auto_play;
                self.last_tick = std::time::Instant::now();
                // If at end, restart
                if self.auto_play && self.current_frame + 1 >= len {
                    self.current_frame = 0;
                }
                Action::None
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.delay_ms = self.delay_ms.saturating_sub(50).max(50);
                Action::None
            }
            KeyCode::Char('-') => {
                self.delay_ms = (self.delay_ms + 50).min(2000);
                Action::None
            }
            KeyCode::Esc => {
                self.loaded_idx = None;
                self.auto_play = false;
                Action::Pop
            }
            _ => Action::None,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        if self.frames.is_empty() {
            let block = Block::default()
                .borders(Borders::ALL)
                .title(" Visualization ")
                .title_style(theme::title_style());
            f.render_widget(
                Paragraph::new("  No frames to display.").block(block),
                area,
            );
            return;
        }

        let chunks = Layout::vertical([
            Constraint::Length(2), // Header
            Constraint::Min(6),   // Bar chart
            Constraint::Length(3), // Annotation
            Constraint::Length(2), // Legend
            Constraint::Length(2), // Controls
        ])
        .split(area);

        let frame = &self.frames[self.current_frame];
        let total = self.frames.len();

        // Header
        let mode_str = if self.auto_play {
            format!("Auto-play ({}ms)", self.delay_ms)
        } else {
            "Step-by-step".to_string()
        };
        let header = Paragraph::new(Line::from(vec![
            Span::styled(
                format!("  {} ", self.viz_name),
                theme::title_style(),
            ),
            Span::styled(
                format!(
                    "  Step {}/{}  |  {}",
                    self.current_frame + 1,
                    total,
                    mode_str
                ),
                theme::muted_style(),
            ),
        ]));
        f.render_widget(header, chunks[0]);

        // Bar chart
        render_bar_chart(f, chunks[1], frame);

        // Annotation
        let annotation = Paragraph::new(Line::from(Span::styled(
            format!("  {}", frame.annotation),
            Style::new()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        f.render_widget(annotation, chunks[2]);

        // Legend
        let legend = Paragraph::new(Line::from(vec![
            Span::raw("  "),
            Span::styled("██ Comparing ", Style::new().fg(Color::Yellow)),
            Span::styled("██ Swapping ", Style::new().fg(Color::Red)),
            Span::styled("██ Sorted ", Style::new().fg(Color::Green)),
            Span::styled("██ Active ", Style::new().fg(Color::Cyan)),
            Span::styled("██ Pivot ", Style::new().fg(Color::Magenta)),
        ]));
        f.render_widget(legend, chunks[3]);

        // Controls
        let controls = Paragraph::new(Line::from(vec![
            Span::raw("  "),
            Span::styled("[Left/Right]", Style::new().fg(theme::ACCENT)),
            Span::raw(" step  "),
            Span::styled("[A]", Style::new().fg(theme::ACCENT)),
            Span::raw(" auto-play  "),
            Span::styled("[+/-]", Style::new().fg(theme::ACCENT)),
            Span::raw(" speed  "),
            Span::styled("[Esc]", Style::new().fg(theme::ACCENT)),
            Span::raw(" back"),
        ]));
        f.render_widget(controls, chunks[4]);
    }
}

/// Render a bar chart for a VizFrame using styled text blocks.
fn render_bar_chart(f: &mut Frame, area: Rect, frame: &VizFrame) {
    let block = Block::default().borders(Borders::LEFT | Borders::BOTTOM);
    let inner = block.inner(area);
    f.render_widget(block, area);

    if frame.array.is_empty() || inner.width == 0 || inner.height == 0 {
        return;
    }

    let max_val = frame.array.iter().copied().max().unwrap_or(1).max(1);
    let bar_height = inner.height.saturating_sub(1) as usize; // leave 1 row for values
    let num_bars = frame.array.len();

    // Calculate bar width based on available space
    let available_width = inner.width as usize;
    let bar_width = ((available_width / num_bars).max(1)).min(4);
    let gap = if bar_width >= 2 { 1 } else { 0 };

    let highlight_map: std::collections::HashMap<usize, HighlightKind> =
        frame.highlights.iter().cloned().collect();

    // Render bars top to bottom
    let mut lines: Vec<Line> = Vec::new();

    for row in (0..bar_height).rev() {
        let mut spans = Vec::new();
        for (col, &val) in frame.array.iter().enumerate() {
            let normalized = (val as f64 / max_val as f64 * bar_height as f64) as usize;
            let color = highlight_color(highlight_map.get(&col));

            let cell = if normalized > row {
                "█".repeat(bar_width)
            } else {
                " ".repeat(bar_width)
            };
            spans.push(Span::styled(cell, Style::new().fg(color)));
            if gap > 0 {
                spans.push(Span::raw(" "));
            }
        }
        lines.push(Line::from(spans));
    }

    // Value labels
    let mut val_spans = Vec::new();
    for (col, &val) in frame.array.iter().enumerate() {
        let color = highlight_color(highlight_map.get(&col));
        let label = format!("{:>width$}", val, width = bar_width);
        val_spans.push(Span::styled(label, Style::new().fg(color)));
        if gap > 0 {
            val_spans.push(Span::raw(" "));
        }
    }
    lines.push(Line::from(val_spans));

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

fn highlight_color(kind: Option<&HighlightKind>) -> Color {
    match kind {
        Some(HighlightKind::Comparing) => Color::Yellow,
        Some(HighlightKind::Swapping) => Color::Red,
        Some(HighlightKind::Sorted) => Color::Green,
        Some(HighlightKind::Active) => Color::Cyan,
        Some(HighlightKind::Pivot) => Color::Magenta,
        Some(HighlightKind::Found) => Color::Green,
        None => Color::White,
    }
}
