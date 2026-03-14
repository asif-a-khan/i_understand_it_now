use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::visualizer::{HighlightKind, VizFrame};

use super::screens::{Action, Screen};
use super::theme;

// ─── Viz Picker ───────────────────────────────────────────

use super::problem_runner::ProblemInfo;

/// Entries in the viz picker list — either a topic header or a problem.
enum VizEntry {
    Header(String),
    Problem(usize), // index into App's problems vec
}

pub struct VizPickerState {
    pub list_state: ListState,
    entries: Vec<VizEntry>,
}

impl VizPickerState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            list_state,
            entries: Vec::new(),
        }
    }

    /// Build the entry list from the problems. Called once on first render.
    pub fn ensure_loaded(&mut self, problems: &[ProblemInfo]) {
        if !self.entries.is_empty() {
            return;
        }
        let mut current_topic = String::new();
        for (idx, p) in problems.iter().enumerate() {
            if p.topic != current_topic {
                current_topic = p.topic.clone();
                self.entries.push(VizEntry::Header(current_topic.clone()));
            }
            self.entries.push(VizEntry::Problem(idx));
        }
        // Select first problem (skip header)
        if self.entries.len() > 1 {
            self.list_state.select(Some(1));
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Action {
        let len = self.entries.len();
        if len == 0 {
            if key.code == KeyCode::Esc {
                return Action::Pop;
            }
            return Action::None;
        }

        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                let i = self.list_state.selected().unwrap_or(0);
                let mut next = (i + 1).min(len - 1);
                while next < len && matches!(self.entries.get(next), Some(VizEntry::Header(_))) {
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
                while prev > 0 && matches!(self.entries.get(prev), Some(VizEntry::Header(_))) {
                    prev = prev.saturating_sub(1);
                }
                if matches!(self.entries.get(prev), Some(VizEntry::Header(_))) {
                    if let Some(pos) = self
                        .entries
                        .iter()
                        .position(|e| matches!(e, VizEntry::Problem(_)))
                    {
                        prev = pos;
                    }
                }
                self.list_state.select(Some(prev));
                Action::None
            }
            KeyCode::Enter => {
                if let Some(idx) = self.list_state.selected() {
                    if let Some(VizEntry::Problem(pidx)) = self.entries.get(idx) {
                        return Action::Push(Screen::ReplayPlayer { problem_idx: *pidx });
                    }
                }
                Action::None
            }
            KeyCode::Esc => Action::Pop,
            _ => Action::None,
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect, problems: &[ProblemInfo]) {
        self.ensure_loaded(problems);

        let items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|entry| match entry {
                VizEntry::Header(topic) => ListItem::new(Line::from(Span::styled(
                    format!("── {} ──", topic),
                    Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
                ))),
                VizEntry::Problem(idx) => {
                    let p = &problems[*idx];
                    ListItem::new(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            format!("{:>6} ", p.difficulty),
                            theme::difficulty_style(&p.difficulty),
                        ),
                        Span::styled(&p.id, Style::new().fg(Color::Cyan)),
                        Span::styled(format!(" — {}", p.name), Style::new().fg(Color::DarkGray)),
                    ]))
                }
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Visualizations — all 570 problems ")
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

    /// Load pre-built frames (for instrumented replay / problem viz).
    pub fn load_replay_frames(&mut self, frames: Vec<VizFrame>, name: String) {
        self.frames = frames;
        self.current_frame = 0;
        self.auto_play = true;
        self.delay_ms = 500;
        self.last_tick = std::time::Instant::now();
        self.viz_name = name;
        self.loaded_idx = None;
    }

    pub fn tick(&mut self) {
        if self.auto_play
            && !self.frames.is_empty()
            && self.last_tick.elapsed().as_millis() >= self.delay_ms as u128
        {
            self.last_tick = std::time::Instant::now();
            if self.current_frame + 1 < self.frames.len() {
                self.current_frame += 1;
            } else {
                self.auto_play = false;
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
            f.render_widget(Paragraph::new("  No frames to display.").block(block), area);
            return;
        }

        let chunks = Layout::vertical([
            Constraint::Length(3), // Header + progress
            Constraint::Min(6),    // Bar chart + pointers
            Constraint::Length(2), // Annotation
            Constraint::Length(1), // Metrics
            Constraint::Length(1), // Legend
            Constraint::Length(1), // Controls
        ])
        .split(area);

        let frame = &self.frames[self.current_frame];
        let total = self.frames.len();
        let progress_pct = if total > 1 {
            self.current_frame as f64 / (total - 1) as f64
        } else {
            1.0
        };

        // Header with progress bar
        let mode_icon = if self.auto_play { "▶" } else { "⏸" };
        let progress_width = chunks[0].width.saturating_sub(4) as usize;
        let filled = (progress_pct * progress_width as f64) as usize;
        let filled_bar = "━".repeat(filled);
        let empty_bar = "─".repeat(progress_width.saturating_sub(filled));

        let header = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(format!("  {} ", self.viz_name), theme::title_style()),
                Span::styled(
                    format!(
                        " {} Step {}/{}  {}ms",
                        mode_icon,
                        self.current_frame + 1,
                        total,
                        self.delay_ms
                    ),
                    theme::muted_style(),
                ),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(filled_bar, Style::new().fg(Color::Cyan)),
                Span::styled(empty_bar, Style::new().fg(Color::DarkGray)),
            ]),
        ]);
        f.render_widget(header, chunks[0]);

        // Bar chart with pointer labels
        render_bar_chart(f, chunks[1], frame);

        // Annotation
        let annotation = Paragraph::new(vec![
            Line::raw(""),
            Line::from(Span::styled(
                format!("  {}", frame.annotation),
                Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
            )),
        ]);
        f.render_widget(annotation, chunks[2]);

        // Metrics
        let (cmp_count, swap_count) = self.cumulative_metrics();
        let metrics = Paragraph::new(Line::from(vec![
            Span::styled("  cmp ", Style::new().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", cmp_count),
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  swp ", Style::new().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", swap_count),
                Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("        ", Style::new()),
            Span::styled("█", Style::new().fg(Color::Yellow)),
            Span::styled(" cmp ", Style::new().fg(Color::DarkGray)),
            Span::styled("█", Style::new().fg(Color::Red)),
            Span::styled(" swp ", Style::new().fg(Color::DarkGray)),
            Span::styled("█", Style::new().fg(Color::Green)),
            Span::styled(" done ", Style::new().fg(Color::DarkGray)),
            Span::styled("█", Style::new().fg(Color::Cyan)),
            Span::styled(" ptr ", Style::new().fg(Color::DarkGray)),
            Span::styled("█", Style::new().fg(Color::Magenta)),
            Span::styled(" pivot", Style::new().fg(Color::DarkGray)),
        ]));
        f.render_widget(metrics, chunks[4]);

        // Controls
        let controls = Paragraph::new(Line::from(vec![
            Span::styled("  [←/→]", Style::new().fg(Color::DarkGray)),
            Span::raw(" step "),
            Span::styled("[A]", Style::new().fg(Color::DarkGray)),
            Span::raw(" play "),
            Span::styled("[+/-]", Style::new().fg(Color::DarkGray)),
            Span::raw(" speed "),
            Span::styled("[Esc]", Style::new().fg(Color::DarkGray)),
            Span::raw(" back"),
        ]));
        f.render_widget(controls, chunks[5]);
    }

    /// Count cumulative comparisons and swaps up to the current frame.
    fn cumulative_metrics(&self) -> (usize, usize) {
        let mut cmp = 0;
        let mut swp = 0;
        for i in 0..=self.current_frame {
            if let Some(frame) = self.frames.get(i) {
                let has_cmp = frame
                    .highlights
                    .iter()
                    .any(|(_, k)| *k == HighlightKind::Comparing);
                let has_swp = frame
                    .highlights
                    .iter()
                    .any(|(_, k)| *k == HighlightKind::Swapping);
                if has_cmp {
                    cmp += 1;
                }
                if has_swp {
                    swp += 1;
                }
            }
        }
        (cmp, swp)
    }
}

/// Render array elements as rounded-corner boxes with pointer labels.
///
/// Layout:
/// ```text
///  ╭────╮ ╭────╮ ╭────╮ ╭────╮
///  │  7 │ │  2 │ │ 11 │ │  4 │
///  ╰────╯ ╰────╯ ╰────╯ ╰────╯
///    ▲                ▲
///   scan            match
/// ```
fn render_bar_chart(f: &mut Frame, area: Rect, frame: &VizFrame) {
    if frame.array.is_empty() || area.width < 4 || area.height < 4 {
        return;
    }

    let num_cells = frame.array.len();
    let has_pointers = !frame.pointers.is_empty();

    // Dynamic sizing: expand cells to fill available width
    let available = area.width.saturating_sub(2) as usize; // 1 char pad each side
    let gap = 1usize;
    let min_val_width = frame
        .array
        .iter()
        .map(|v| format!("{}", v).len())
        .max()
        .unwrap_or(1)
        .max(1);

    // Each cell = ╭ + inner + ╮ = inner_w + 2, plus gap between cells
    // Total = num_cells * (inner_w + 2) + (num_cells - 1) * gap <= available
    // Solve for inner_w: inner_w = (available - (num_cells-1)*gap) / num_cells - 2
    let space_for_cells = available.saturating_sub(num_cells.saturating_sub(1) * gap);
    let cell_w = (space_for_cells / num_cells).max(min_val_width + 2);
    let inner_w = cell_w.saturating_sub(2).max(min_val_width);

    let highlight_map: std::collections::HashMap<usize, HighlightKind> =
        frame.highlights.iter().cloned().collect();

    // Build pointer lookup
    let pointer_colors = [
        Color::Cyan,
        Color::Yellow,
        Color::Magenta,
        Color::Red,
        Color::Green,
    ];
    let mut pointer_map: std::collections::HashMap<usize, (&str, Color)> =
        std::collections::HashMap::new();
    let mut label_to_color: std::collections::HashMap<&str, Color> =
        std::collections::HashMap::new();
    let mut color_idx = 0;
    for (idx, label) in &frame.pointers {
        let color = *label_to_color.entry(label.as_str()).or_insert_with(|| {
            let c = pointer_colors[color_idx % pointer_colors.len()];
            color_idx += 1;
            c
        });
        pointer_map.insert(*idx, (label.as_str(), color));
    }

    // Determine color for each cell
    let cell_color = |col: usize| -> Color {
        if let Some((_, c)) = pointer_map.get(&col) {
            *c
        } else {
            highlight_color(highlight_map.get(&col))
        }
    };

    let dim = Style::new().fg(Color::DarkGray);
    let mut lines: Vec<Line> = Vec::new();

    // Index row: [0] [1] [2] ...
    let mut idx_spans = vec![Span::raw(" ")];
    for col in 0..num_cells {
        let label = format!("{:^width$}", col, width = cell_w);
        idx_spans.push(Span::styled(label, dim));
        if col + 1 < num_cells {
            idx_spans.push(Span::raw(" ".repeat(gap)));
        }
    }
    lines.push(Line::from(idx_spans));

    // Top border: ╭────╮
    let mut top_spans = vec![Span::raw(" ")];
    for col in 0..num_cells {
        let color = cell_color(col);
        let border = format!("╭{}╮", "─".repeat(inner_w));
        top_spans.push(Span::styled(border, Style::new().fg(color)));
        if col + 1 < num_cells {
            top_spans.push(Span::raw(" ".repeat(gap)));
        }
    }
    lines.push(Line::from(top_spans));

    // Value row: │  7 │
    let mut val_spans = vec![Span::raw(" ")];
    for (col, &val) in frame.array.iter().enumerate() {
        let color = cell_color(col);
        let val_str = format!("{:^width$}", val, width = inner_w);
        let mut cell_spans = vec![
            Span::styled("│", Style::new().fg(color)),
            Span::styled(val_str, Style::new().fg(color).add_modifier(Modifier::BOLD)),
            Span::styled("│", Style::new().fg(color)),
        ];
        val_spans.append(&mut cell_spans);
        if col + 1 < num_cells {
            val_spans.push(Span::raw(" ".repeat(gap)));
        }
    }
    lines.push(Line::from(val_spans));

    // Bottom border: ╰────╯
    let mut bot_spans = vec![Span::raw(" ")];
    for col in 0..num_cells {
        let color = cell_color(col);
        let border = format!("╰{}╯", "─".repeat(inner_w));
        bot_spans.push(Span::styled(border, Style::new().fg(color)));
        if col + 1 < num_cells {
            bot_spans.push(Span::raw(" ".repeat(gap)));
        }
    }
    lines.push(Line::from(bot_spans));

    // Arrow row
    let mut arrow_spans = vec![Span::raw(" ")];
    for col in 0..num_cells {
        let content = if let Some((_, ptr_color)) = pointer_map.get(&col) {
            Span::styled(
                format!("{:^width$}", "▲", width = cell_w),
                Style::new().fg(*ptr_color).add_modifier(Modifier::BOLD),
            )
        } else if let Some(kind) = highlight_map.get(&col) {
            let sym = match kind {
                HighlightKind::Sorted => "✓",
                HighlightKind::Found => "★",
                _ => " ",
            };
            Span::styled(
                format!("{:^width$}", sym, width = cell_w),
                Style::new().fg(highlight_color(Some(kind))),
            )
        } else {
            Span::raw(" ".repeat(cell_w))
        };
        arrow_spans.push(content);
        if col + 1 < num_cells {
            arrow_spans.push(Span::raw(" ".repeat(gap)));
        }
    }
    lines.push(Line::from(arrow_spans));

    // Pointer label row
    if has_pointers {
        let mut label_spans = vec![Span::raw(" ")];
        for col in 0..num_cells {
            if let Some((label, ptr_color)) = pointer_map.get(&col) {
                label_spans.push(Span::styled(
                    format!("{:^width$}", label, width = cell_w),
                    Style::new().fg(*ptr_color).add_modifier(Modifier::BOLD),
                ));
            } else {
                label_spans.push(Span::raw(" ".repeat(cell_w)));
            }
            if col + 1 < num_cells {
                label_spans.push(Span::raw(" ".repeat(gap)));
            }
        }
        lines.push(Line::from(label_spans));
    }

    // Center vertically in the available area
    let content_height = lines.len();
    let top_pad = (area.height as usize).saturating_sub(content_height) / 2;
    let mut padded: Vec<Line> = Vec::new();
    for _ in 0..top_pad {
        padded.push(Line::raw(""));
    }
    padded.append(&mut lines);

    let paragraph = Paragraph::new(padded);
    f.render_widget(paragraph, area);
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
