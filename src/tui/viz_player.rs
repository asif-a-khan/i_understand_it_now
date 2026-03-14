use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;

use crate::visualizer::{HighlightKind, VizData, VizFrame};

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
    search_active: bool,
    search_query: String,
    filtered_entries: Vec<VizEntry>,
}

impl VizPickerState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            list_state,
            entries: Vec::new(),
            search_active: false,
            search_query: String::new(),
            filtered_entries: Vec::new(),
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

    pub fn handle_key(&mut self, key: KeyEvent, problems: &[ProblemInfo]) -> Action {
        let active_entries = if !self.search_query.is_empty() && !self.filtered_entries.is_empty() {
            &self.filtered_entries
        } else {
            &self.entries
        };
        let len = active_entries.len();

        if len == 0 && !self.search_active {
            if key.code == KeyCode::Esc {
                return Action::Pop;
            }
            return Action::None;
        }

        // Search mode: all chars go to query, only arrow keys navigate
        if self.search_active {
            match key.code {
                KeyCode::Char(c) => {
                    self.search_query.push(c);
                    self.rebuild_filtered(problems);
                }
                KeyCode::Backspace => {
                    self.search_query.pop();
                    if self.search_query.is_empty() {
                        self.filtered_entries.clear();
                        if self.entries.len() > 1 {
                            self.list_state.select(Some(1));
                        }
                    } else {
                        self.rebuild_filtered(problems);
                    }
                }
                KeyCode::Esc => {
                    self.search_active = false;
                    self.search_query.clear();
                    self.filtered_entries.clear();
                    if self.entries.len() > 1 {
                        self.list_state.select(Some(1));
                    }
                }
                KeyCode::Enter => {
                    self.search_active = false;
                    let entries =
                        if !self.search_query.is_empty() && !self.filtered_entries.is_empty() {
                            &self.filtered_entries
                        } else {
                            &self.entries
                        };
                    if let Some(idx) = self.list_state.selected() {
                        if let Some(VizEntry::Problem(pidx)) = entries.get(idx) {
                            return Action::Push(Screen::ReplayPlayer { problem_idx: *pidx });
                        }
                    }
                }
                KeyCode::Down => {
                    let entries =
                        if !self.search_query.is_empty() && !self.filtered_entries.is_empty() {
                            &self.filtered_entries
                        } else {
                            &self.entries
                        };
                    let flen = entries.len();
                    if flen > 0 {
                        let i = self.list_state.selected().unwrap_or(0);
                        let mut next = (i + 1).min(flen - 1);
                        while next < flen && matches!(entries.get(next), Some(VizEntry::Header(_)))
                        {
                            next += 1;
                        }
                        if next < flen {
                            self.list_state.select(Some(next));
                        }
                    }
                }
                KeyCode::Up => {
                    let entries =
                        if !self.search_query.is_empty() && !self.filtered_entries.is_empty() {
                            &self.filtered_entries
                        } else {
                            &self.entries
                        };
                    let flen = entries.len();
                    if flen > 0 {
                        let i = self.list_state.selected().unwrap_or(0);
                        let mut prev = i.saturating_sub(1);
                        while prev > 0 && matches!(entries.get(prev), Some(VizEntry::Header(_))) {
                            prev = prev.saturating_sub(1);
                        }
                        if matches!(entries.get(prev), Some(VizEntry::Header(_))) {
                            if let Some(pos) = entries
                                .iter()
                                .position(|e| matches!(e, VizEntry::Problem(_)))
                            {
                                prev = pos;
                            }
                        }
                        self.list_state.select(Some(prev));
                    }
                }
                _ => {}
            }
            return Action::None;
        }

        match key.code {
            KeyCode::Char('/') => {
                self.search_active = true;
                Action::None
            }
            KeyCode::Char('j') | KeyCode::Down => {
                let i = self.list_state.selected().unwrap_or(0);
                let mut next = (i + 1).min(len - 1);
                while next < len && matches!(active_entries.get(next), Some(VizEntry::Header(_))) {
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
                while prev > 0 && matches!(active_entries.get(prev), Some(VizEntry::Header(_))) {
                    prev = prev.saturating_sub(1);
                }
                if matches!(active_entries.get(prev), Some(VizEntry::Header(_))) {
                    if let Some(pos) = active_entries
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
                    if let Some(VizEntry::Problem(pidx)) = active_entries.get(idx) {
                        return Action::Push(Screen::ReplayPlayer { problem_idx: *pidx });
                    }
                }
                Action::None
            }
            KeyCode::Esc => {
                // If we have an active filter (but search_active is false), clear it first
                if !self.search_query.is_empty() {
                    self.search_query.clear();
                    self.filtered_entries.clear();
                    if self.entries.len() > 1 {
                        self.list_state.select(Some(1));
                    }
                    Action::None
                } else {
                    Action::Pop
                }
            }
            _ => Action::None,
        }
    }

    fn rebuild_filtered(&mut self, problems: &[ProblemInfo]) {
        let query = self.search_query.to_lowercase();
        self.filtered_entries.clear();
        let mut current_topic = String::new();
        for (idx, p) in problems.iter().enumerate() {
            if p.id.to_lowercase().contains(&query)
                || p.name.to_lowercase().contains(&query)
                || p.topic.to_lowercase().contains(&query)
            {
                if p.topic != current_topic {
                    current_topic = p.topic.clone();
                    self.filtered_entries
                        .push(VizEntry::Header(current_topic.clone()));
                }
                self.filtered_entries.push(VizEntry::Problem(idx));
            }
        }
        // Select first problem in filtered results
        if let Some(pos) = self
            .filtered_entries
            .iter()
            .position(|e| matches!(e, VizEntry::Problem(_)))
        {
            self.list_state.select(Some(pos));
        } else {
            self.list_state.select(Some(0));
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect, problems: &[ProblemInfo]) {
        self.ensure_loaded(problems);

        let active_entries = if !self.search_query.is_empty() && !self.filtered_entries.is_empty() {
            &self.filtered_entries
        } else {
            &self.entries
        };

        // Always show search bar at bottom
        let chunks = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(area);
        let list_area = chunks[0];
        let search_bar_area = chunks[1];

        let items: Vec<ListItem> = active_entries
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

        let title = if !self.search_query.is_empty() && !self.filtered_entries.is_empty() {
            let count = self
                .filtered_entries
                .iter()
                .filter(|e| matches!(e, VizEntry::Problem(_)))
                .count();
            format!(" Visualizations — {} matching ", count)
        } else {
            format!(" Visualizations — all {} problems ", problems.len())
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
            let result_count = self
                .filtered_entries
                .iter()
                .filter(|e| matches!(e, VizEntry::Problem(_)))
                .count();
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
                    format!("  ({} results)", result_count),
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

        let frame = &self.frames[self.current_frame];
        let total = self.frames.len();
        let progress_pct = if total > 1 {
            self.current_frame as f64 / (total - 1) as f64
        } else {
            1.0
        };

        // ── Responsive layout: annotation at top, viz fills middle, controls at bottom ──
        let chunks = Layout::vertical([
            Constraint::Length(2), // Annotation (centered at top)
            Constraint::Length(1), // Progress bar
            Constraint::Fill(1),   // Visualization area (all remaining space)
            Constraint::Length(1), // Metrics + legend
            Constraint::Length(1), // Controls
        ])
        .split(area);

        // ── Annotation (centered at top) ──
        let annotation = Paragraph::new(Line::from(Span::styled(
            &frame.annotation,
            Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
        f.render_widget(annotation, chunks[0]);

        // ── Progress bar (dynamic width) ──
        let mode_icon = if self.auto_play { "▶" } else { "⏸" };
        let step_info = format!(
            " {} {}/{} {}ms ",
            mode_icon,
            self.current_frame + 1,
            total,
            self.delay_ms
        );
        let step_info_len = step_info.len();
        let progress_width = (chunks[1].width as usize).saturating_sub(step_info_len + 2);
        let filled = (progress_pct * progress_width as f64) as usize;
        let filled_bar = "━".repeat(filled);
        let empty_bar = "─".repeat(progress_width.saturating_sub(filled));

        let progress = Paragraph::new(Line::from(vec![
            Span::styled(step_info, theme::muted_style()),
            Span::styled(filled_bar, Style::new().fg(Color::Cyan)),
            Span::styled(empty_bar, Style::new().fg(Color::DarkGray)),
        ]))
        .alignment(Alignment::Center);
        f.render_widget(progress, chunks[1]);

        // ── Visualization (dispatched by data type) ──
        let viz_data = frame.viz_data();
        match &viz_data {
            VizData::Array { values } => render_array(f, chunks[2], values, frame),
            VizData::Tree { nodes } => render_tree(f, chunks[2], nodes, frame),
            VizData::Graph {
                n,
                labels,
                edges,
                weighted_edges,
                directed,
            } => render_graph(
                f,
                chunks[2],
                *n,
                labels,
                edges,
                weighted_edges,
                *directed,
                frame,
            ),
            VizData::Grid { cells } => render_grid(f, chunks[2], cells, frame),
            VizData::None { message } => render_no_viz(f, chunks[2], message),
        }

        // ── Metrics + legend (single line) ──
        let (cmp_count, swap_count, read_count, write_count) = self.cumulative_metrics();
        // ── Metrics + legend (centered, only relevant items) ──
        let mut metric_spans: Vec<Span> = Vec::new();
        let metrics: Vec<(&str, usize, Color)> = vec![
            ("cmp", cmp_count, Color::Yellow),
            ("swp", swap_count, Color::Red),
            ("read", read_count, Color::Cyan),
            ("write", write_count, Color::Magenta),
        ];
        for (label, count, color) in &metrics {
            if *count > 0 {
                metric_spans.push(Span::styled(
                    format!("{} ", label),
                    Style::new().fg(Color::DarkGray),
                ));
                metric_spans.push(Span::styled(
                    format!("{}", count),
                    Style::new().fg(*color).add_modifier(Modifier::BOLD),
                ));
                metric_spans.push(Span::raw("  "));
            }
        }
        // Legend: only show kinds that appear in any frame
        let has_kind = |kind: HighlightKind| -> bool {
            self.frames
                .iter()
                .any(|f| f.highlights.iter().any(|(_, k)| *k == kind))
        };
        metric_spans.push(Span::raw("  "));
        let legend_items: Vec<(&str, Color)> = [
            ("cmp", Color::Yellow, HighlightKind::Comparing),
            ("swp", Color::Red, HighlightKind::Swapping),
            ("done", Color::Green, HighlightKind::Sorted),
            ("read", Color::Cyan, HighlightKind::Reading),
            ("write", Color::Magenta, HighlightKind::Writing),
            ("target", Color::LightRed, HighlightKind::Target),
        ]
        .iter()
        .filter(|(_, _, kind)| has_kind(*kind))
        .map(|(label, color, _)| (*label, *color))
        .collect();
        for (label, color) in &legend_items {
            metric_spans.push(Span::styled("█", Style::new().fg(*color)));
            metric_spans.push(Span::styled(
                format!(" {} ", label),
                Style::new().fg(Color::DarkGray),
            ));
        }
        f.render_widget(
            Paragraph::new(Line::from(metric_spans)).alignment(Alignment::Center),
            chunks[3],
        );

        // ── Controls (centered, adaptive to terminal width) ──
        let w = chunks[4].width as usize;
        let mut ctrl_spans: Vec<Span> = vec![
            Span::styled("[←/→]", Style::new().fg(Color::DarkGray)),
            Span::raw(" step "),
            Span::styled("[A]", Style::new().fg(Color::DarkGray)),
            Span::raw(" play "),
        ];
        if w >= 50 {
            ctrl_spans.push(Span::styled("[+/-]", Style::new().fg(Color::DarkGray)));
            ctrl_spans.push(Span::raw(" speed "));
        }
        if w >= 70 {
            ctrl_spans.push(Span::styled("[Home/End]", Style::new().fg(Color::DarkGray)));
            ctrl_spans.push(Span::raw(" jump "));
        }
        ctrl_spans.push(Span::styled("[Esc]", Style::new().fg(Color::DarkGray)));
        ctrl_spans.push(Span::raw(" back"));
        f.render_widget(
            Paragraph::new(Line::from(ctrl_spans)).alignment(Alignment::Center),
            chunks[4],
        );
    }

    /// Count cumulative metrics up to the current frame.
    fn cumulative_metrics(&self) -> (usize, usize, usize, usize) {
        let mut cmp = 0;
        let mut swp = 0;
        let mut read = 0;
        let mut write = 0;
        for i in 0..=self.current_frame {
            if let Some(frame) = self.frames.get(i) {
                for (_, k) in &frame.highlights {
                    match k {
                        HighlightKind::Comparing => cmp += 1,
                        HighlightKind::Swapping => swp += 1,
                        HighlightKind::Reading => read += 1,
                        HighlightKind::Writing => write += 1,
                        _ => {}
                    }
                }
            }
        }
        (cmp, swp, read, write)
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Renderers
// ═══════════════════════════════════════════════════════════════════════

fn highlight_color(kind: Option<&HighlightKind>) -> Color {
    match kind {
        Some(HighlightKind::Comparing) => Color::Yellow,
        Some(HighlightKind::Swapping) => Color::Red,
        Some(HighlightKind::Sorted) => Color::Green,
        Some(HighlightKind::Active) => Color::Cyan,
        Some(HighlightKind::Pivot) => Color::Magenta,
        Some(HighlightKind::Found) => Color::Green,
        Some(HighlightKind::Reading) => Color::Cyan,
        Some(HighlightKind::Writing) => Color::Magenta,
        Some(HighlightKind::Target) => Color::LightRed,
        None => Color::White,
    }
}

/// Return a full style for a highlight kind — uses bg color for Target.
fn highlight_style(kind: Option<&HighlightKind>) -> Style {
    match kind {
        Some(HighlightKind::Target) => Style::new()
            .fg(Color::White)
            .bg(Color::LightRed)
            .add_modifier(Modifier::BOLD),
        Some(HighlightKind::Found) => Style::new()
            .fg(Color::White)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD),
        other => Style::new()
            .fg(highlight_color(other))
            .add_modifier(Modifier::BOLD),
    }
}

// ─── Array Renderer ──────────────────────────────────────────────────

/// Render array/string elements as rounded-corner boxes with pointer labels.
///
/// ```text
///  ╭────╮ ╭────╮ ╭────╮ ╭────╮
///  │  7 │ │  2 │ │ 11 │ │  4 │
///  ╰────╯ ╰────╯ ╰────╯ ╰────╯
///    ▲                ▲
///   scan            match
/// ```
fn render_array(f: &mut Frame, area: Rect, values: &[String], frame: &VizFrame) {
    if values.is_empty() || area.width < 4 || area.height < 4 {
        return;
    }

    let num_cells = values.len();
    let has_pointers = !frame.pointers.is_empty();

    // Dynamic sizing: expand cells to fill available width
    let available = area.width.saturating_sub(2) as usize;
    let gap = 1usize;
    let min_val_width = values.iter().map(|v| v.len()).max().unwrap_or(1).max(1);

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

    let cell_color = |col: usize| -> Color {
        if let Some((_, c)) = pointer_map.get(&col) {
            *c
        } else {
            highlight_color(highlight_map.get(&col))
        }
    };

    let dim = Style::new().fg(Color::DarkGray);
    let mut lines: Vec<Line> = Vec::new();

    // Index row
    let mut idx_spans = vec![Span::raw(" ")];
    for col in 0..num_cells {
        let label = format!("{:^width$}", col, width = cell_w);
        idx_spans.push(Span::styled(label, dim));
        if col + 1 < num_cells {
            idx_spans.push(Span::raw(" ".repeat(gap)));
        }
    }
    lines.push(Line::from(idx_spans));

    // Top border
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

    // Value row
    let mut val_spans = vec![Span::raw(" ")];
    for (col, val) in values.iter().enumerate() {
        let color = cell_color(col);
        let val_str = format!("{:^width$}", val, width = inner_w);
        val_spans.push(Span::styled("│", Style::new().fg(color)));
        val_spans.push(Span::styled(
            val_str,
            Style::new().fg(color).add_modifier(Modifier::BOLD),
        ));
        val_spans.push(Span::styled("│", Style::new().fg(color)));
        if col + 1 < num_cells {
            val_spans.push(Span::raw(" ".repeat(gap)));
        }
    }
    lines.push(Line::from(val_spans));

    // Bottom border
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
                HighlightKind::Target => "◎",
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

    // Center vertically
    let content_height = lines.len();
    let top_pad = (area.height as usize).saturating_sub(content_height) / 2;
    let mut padded: Vec<Line> = Vec::new();
    for _ in 0..top_pad {
        padded.push(Line::raw(""));
    }
    padded.append(&mut lines);

    f.render_widget(Paragraph::new(padded), area);
}

// ─── Tree Renderer ──────────────────────────────────────────────────

/// Render a binary tree with spatial layout and connectors.
fn render_tree(f: &mut Frame, area: Rect, nodes: &[Option<String>], frame: &VizFrame) {
    if nodes.is_empty() || area.width < 6 || area.height < 3 {
        render_no_viz(f, area, "Tree is empty");
        return;
    }

    let highlight_map: std::collections::HashMap<usize, HighlightKind> =
        frame.highlights.iter().cloned().collect();

    // Compute tree depth
    let depth = (nodes.len() as f64).log2().floor() as usize + 1;
    let node_w = 5usize; // ┌───┐ width
    let avail_w = area.width as usize;
    let avail_h = area.height as usize;

    // Each level needs 3 rows (box top/val/bottom) + 2 rows connector = 5 rows per level
    // except the last level doesn't need connectors
    let rows_needed = depth * 3 + (depth.saturating_sub(1)) * 2;

    if rows_needed > avail_h || depth > 6 {
        // Fallback: compact indented view
        render_tree_indented(f, area, nodes, &highlight_map);
        return;
    }

    let mut lines: Vec<Line> = Vec::new();

    for level in 0..depth {
        let level_start = (1 << level) - 1; // first index at this level
        let level_count = 1 << level; // number of nodes at this level
        let spacing = avail_w / (level_count + 1); // even spacing

        // Box top row
        let mut top_spans: Vec<Span> = Vec::new();
        let mut val_spans: Vec<Span> = Vec::new();
        let mut bot_spans: Vec<Span> = Vec::new();
        let mut prev_end = 0usize;

        for i in 0..level_count {
            let node_idx = level_start + i;
            let center_x = spacing * (i + 1);
            let box_start = center_x.saturating_sub(node_w / 2);

            // Pad to box_start
            if box_start > prev_end {
                let pad = " ".repeat(box_start - prev_end);
                top_spans.push(Span::raw(pad.clone()));
                val_spans.push(Span::raw(pad.clone()));
                bot_spans.push(Span::raw(pad));
            }

            let color = if node_idx < nodes.len() && nodes[node_idx].is_some() {
                highlight_color(highlight_map.get(&node_idx))
            } else {
                Color::DarkGray
            };

            if node_idx < nodes.len() {
                if let Some(ref val) = nodes[node_idx] {
                    let display = if val.len() > 3 { &val[..3] } else { val };
                    top_spans.push(Span::styled("┌───┐", Style::new().fg(color)));
                    val_spans.push(Span::styled("│", Style::new().fg(color)));
                    val_spans.push(Span::styled(
                        format!("{:^3}", display),
                        Style::new().fg(color).add_modifier(Modifier::BOLD),
                    ));
                    val_spans.push(Span::styled("│", Style::new().fg(color)));
                    bot_spans.push(Span::styled("└───┘", Style::new().fg(color)));
                } else {
                    top_spans.push(Span::raw("     "));
                    val_spans.push(Span::raw("     "));
                    bot_spans.push(Span::raw("     "));
                }
            } else {
                top_spans.push(Span::raw("     "));
                val_spans.push(Span::raw("     "));
                bot_spans.push(Span::raw("     "));
            }

            prev_end = box_start + node_w;
        }

        lines.push(Line::from(top_spans));
        lines.push(Line::from(val_spans));
        lines.push(Line::from(bot_spans));

        // Connector rows (except after last level)
        if level + 1 < depth {
            let mut conn1: Vec<Span> = Vec::new();
            let mut conn2: Vec<Span> = Vec::new();
            let child_spacing = avail_w / ((level_count * 2) + 1);
            let mut c1_end = 0usize;
            let mut c2_end = 0usize;

            for i in 0..level_count {
                let node_idx = level_start + i;
                if node_idx < nodes.len() && nodes[node_idx].is_some() {
                    let parent_x = spacing * (i + 1);
                    let left_child_x = child_spacing * (2 * i + 1);
                    let right_child_x = child_spacing * (2 * i + 2);

                    // Vertical line from parent
                    if parent_x > c1_end {
                        conn1.push(Span::raw(" ".repeat(parent_x - c1_end)));
                    }
                    conn1.push(Span::styled("│", Style::new().fg(Color::DarkGray)));
                    c1_end = parent_x + 1;

                    // Horizontal branch line
                    let branch_left = left_child_x.min(parent_x);
                    let branch_right = right_child_x.max(parent_x);
                    if branch_left > c2_end {
                        conn2.push(Span::raw(" ".repeat(branch_left - c2_end)));
                    }

                    let has_left =
                        (2 * node_idx + 1) < nodes.len() && nodes[2 * node_idx + 1].is_some();
                    let has_right =
                        (2 * node_idx + 2) < nodes.len() && nodes[2 * node_idx + 2].is_some();

                    if has_left && has_right {
                        if left_child_x < parent_x {
                            conn2.push(Span::styled(
                                format!(
                                    "┌{}┴{}┐",
                                    "─".repeat(parent_x - left_child_x - 1),
                                    "─".repeat(right_child_x - parent_x - 1)
                                ),
                                Style::new().fg(Color::DarkGray),
                            ));
                        }
                    } else if has_left {
                        if left_child_x < parent_x {
                            conn2.push(Span::styled(
                                format!("┌{}┘", "─".repeat(parent_x - left_child_x - 1)),
                                Style::new().fg(Color::DarkGray),
                            ));
                        }
                    } else if has_right && parent_x < right_child_x {
                        conn2.push(Span::styled(
                            format!("└{}┐", "─".repeat(right_child_x - parent_x - 1)),
                            Style::new().fg(Color::DarkGray),
                        ));
                    }
                    c2_end = branch_right + 1;
                }
            }

            lines.push(Line::from(conn1));
            lines.push(Line::from(conn2));
        }
    }

    // Center vertically
    let content_height = lines.len();
    let top_pad = avail_h.saturating_sub(content_height) / 2;
    let mut padded: Vec<Line> = Vec::new();
    for _ in 0..top_pad {
        padded.push(Line::raw(""));
    }
    padded.append(&mut lines);

    f.render_widget(Paragraph::new(padded), area);
}

/// Compact indented tree view (fallback for large/deep trees).
fn render_tree_indented(
    f: &mut Frame,
    area: Rect,
    nodes: &[Option<String>],
    highlight_map: &std::collections::HashMap<usize, HighlightKind>,
) {
    let mut lines: Vec<Line> = Vec::new();
    render_tree_node_indented(nodes, 0, "", true, highlight_map, &mut lines);

    let content_height = lines.len();
    let top_pad = (area.height as usize).saturating_sub(content_height) / 2;
    let mut padded: Vec<Line> = Vec::new();
    for _ in 0..top_pad {
        padded.push(Line::raw(""));
    }
    padded.append(&mut lines);

    f.render_widget(Paragraph::new(padded), area);
}

fn render_tree_node_indented(
    nodes: &[Option<String>],
    idx: usize,
    prefix: &str,
    is_last: bool,
    highlight_map: &std::collections::HashMap<usize, HighlightKind>,
    lines: &mut Vec<Line<'static>>,
) {
    if idx >= nodes.len() {
        return;
    }
    if let Some(ref val) = nodes[idx] {
        let connector = if idx == 0 {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };
        let color = highlight_color(highlight_map.get(&idx));
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {}{}", prefix, connector),
                Style::new().fg(Color::DarkGray),
            ),
            Span::styled(
                val.clone(),
                Style::new().fg(color).add_modifier(Modifier::BOLD),
            ),
        ]));

        let child_prefix = if idx == 0 {
            "".to_string()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let has_left = left < nodes.len() && nodes[left].is_some();
        let has_right = right < nodes.len() && nodes[right].is_some();

        if has_left {
            render_tree_node_indented(nodes, left, &child_prefix, !has_right, highlight_map, lines);
        }
        if has_right {
            render_tree_node_indented(nodes, right, &child_prefix, true, highlight_map, lines);
        }
    }
}

// ─── Graph Renderer ──────────────────────────────────────────────────

/// Render a graph as a 2D grid of node squares with adjacency sidebar.
#[allow(clippy::too_many_arguments)]
fn render_graph(
    f: &mut Frame,
    area: Rect,
    n: usize,
    labels: &[String],
    edges: &[(usize, usize)],
    weighted_edges: &[(usize, usize, String)],
    directed: bool,
    frame: &VizFrame,
) {
    if n == 0 || area.width < 10 || area.height < 5 {
        render_no_viz(f, area, "Graph is empty");
        return;
    }

    let highlight_map: std::collections::HashMap<usize, HighlightKind> =
        frame.highlights.iter().cloned().collect();

    // Build adjacency list for sidebar
    let mut adj: Vec<Vec<String>> = vec![vec![]; n];
    for &(u, v) in edges {
        if u < n && v < n {
            adj[u].push(v.to_string());
            if !directed {
                adj[v].push(u.to_string());
            }
        }
    }
    for (u, v, w) in weighted_edges {
        if *u < n && *v < n {
            adj[*u].push(format!("{}({})", v, w));
            if !directed {
                adj[*v].push(format!("{}({})", u, w));
            }
        }
    }

    // Build edge set for quick lookup
    let mut edge_set: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    for &(u, v) in edges {
        if u < n && v < n {
            edge_set.insert((u, v));
            if !directed {
                edge_set.insert((v, u));
            }
        }
    }
    for (u, v, _) in weighted_edges {
        if *u < n && *v < n {
            edge_set.insert((*u, *v));
            if !directed {
                edge_set.insert((*v, *u));
            }
        }
    }

    // Grid dimensions: ceil(sqrt(n))
    let grid_cols = (n as f64).sqrt().ceil() as usize;
    let grid_rows = n.div_ceil(grid_cols);

    // 75% graph grid / 25% adjacency sidebar
    let chunks =
        Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]).split(area);
    let grid_area = chunks[0];
    let sidebar_area = chunks[1];

    // Compute cell width to fill available grid area dynamically
    let max_label_w = labels.iter().map(|l| l.len()).max().unwrap_or(1).max(1);
    let avail_grid_w = grid_area.width.saturating_sub(4) as usize;
    let cell_w = if grid_cols > 0 {
        (avail_grid_w / grid_cols)
            .saturating_sub(1)
            .max(max_label_w + 4) // extra space for edge direction indicators
    } else {
        max_label_w + 4
    };

    // ── Render node grid with edge indicators ──
    let mut lines: Vec<Line> = Vec::new();

    // Top border
    let mut top = String::from("  ┌");
    for c in 0..grid_cols {
        top.push_str(&"─".repeat(cell_w));
        if c + 1 < grid_cols {
            top.push('┬');
        }
    }
    top.push('┐');
    lines.push(Line::styled(top, Style::new().fg(Color::DarkGray)));

    for r in 0..grid_rows {
        // Label row (node label)
        let mut label_spans: Vec<Span> =
            vec![Span::styled("  │", Style::new().fg(Color::DarkGray))];
        for c in 0..grid_cols {
            let node_idx = r * grid_cols + c;
            if node_idx < n {
                let label = if node_idx < labels.len() {
                    &labels[node_idx]
                } else {
                    ""
                };
                let style = highlight_style(highlight_map.get(&node_idx));
                label_spans.push(Span::styled(
                    format!("{:^width$}", label, width = cell_w),
                    style,
                ));
            } else {
                label_spans.push(Span::raw(" ".repeat(cell_w)));
            }
            label_spans.push(Span::styled("│", Style::new().fg(Color::DarkGray)));
        }
        lines.push(Line::from(label_spans));

        // Edge direction row (arrows showing connections)
        let mut dir_spans: Vec<Span> = vec![Span::styled("  │", Style::new().fg(Color::DarkGray))];
        for c in 0..grid_cols {
            let node_idx = r * grid_cols + c;
            if node_idx < n {
                // Check which grid-adjacent neighbors have edges
                let right = if c + 1 < grid_cols {
                    let neighbor = r * grid_cols + c + 1;
                    neighbor < n && edge_set.contains(&(node_idx, neighbor))
                } else {
                    false
                };
                let down = if r + 1 < grid_rows {
                    let neighbor = (r + 1) * grid_cols + c;
                    neighbor < n && edge_set.contains(&(node_idx, neighbor))
                } else {
                    false
                };
                let left = if c > 0 {
                    let neighbor = r * grid_cols + c - 1;
                    edge_set.contains(&(node_idx, neighbor))
                } else {
                    false
                };
                let up = if r > 0 {
                    let neighbor = (r - 1) * grid_cols + c;
                    edge_set.contains(&(node_idx, neighbor))
                } else {
                    false
                };

                let mut dirs = String::new();
                if up {
                    dirs.push('↑');
                }
                if left {
                    dirs.push('←');
                }
                if right {
                    dirs.push('→');
                }
                if down {
                    dirs.push('↓');
                }

                // Color: if both this node and the adjacent node are highlighted, use green
                let dir_color = highlight_color(highlight_map.get(&node_idx));
                dir_spans.push(Span::styled(
                    format!("{:^width$}", dirs, width = cell_w),
                    Style::new().fg(dir_color),
                ));
            } else {
                dir_spans.push(Span::raw(" ".repeat(cell_w)));
            }
            dir_spans.push(Span::styled("│", Style::new().fg(Color::DarkGray)));
        }
        lines.push(Line::from(dir_spans));

        // Row separator or bottom border
        if r + 1 < grid_rows {
            // Separator with vertical edge indicators
            let mut sep_spans: Vec<Span> =
                vec![Span::styled("  ├", Style::new().fg(Color::DarkGray))];
            for c in 0..grid_cols {
                let node_idx = r * grid_cols + c;
                let below = (r + 1) * grid_cols + c;
                let has_edge = node_idx < n && below < n && edge_set.contains(&(node_idx, below));
                if has_edge {
                    // Highlight the separator to show vertical edge
                    let edge_color = if highlight_map.contains_key(&node_idx)
                        && highlight_map.contains_key(&below)
                    {
                        Color::Green
                    } else {
                        Color::DarkGray
                    };
                    // Show a vertical connector in the middle of the separator
                    let half = cell_w / 2;
                    sep_spans.push(Span::styled(
                        "─".repeat(half),
                        Style::new().fg(Color::DarkGray),
                    ));
                    sep_spans.push(Span::styled("┃", Style::new().fg(edge_color)));
                    sep_spans.push(Span::styled(
                        "─".repeat(cell_w.saturating_sub(half + 1)),
                        Style::new().fg(Color::DarkGray),
                    ));
                } else {
                    sep_spans.push(Span::styled(
                        "─".repeat(cell_w),
                        Style::new().fg(Color::DarkGray),
                    ));
                }
                if c + 1 < grid_cols {
                    sep_spans.push(Span::styled("┼", Style::new().fg(Color::DarkGray)));
                }
            }
            sep_spans.push(Span::styled("┤", Style::new().fg(Color::DarkGray)));
            lines.push(Line::from(sep_spans));
        }
    }

    // Bottom border
    let mut bot = String::from("  └");
    for c in 0..grid_cols {
        bot.push_str(&"─".repeat(cell_w));
        if c + 1 < grid_cols {
            bot.push('┴');
        }
    }
    bot.push('┘');
    lines.push(Line::styled(bot, Style::new().fg(Color::DarkGray)));

    // Center grid vertically
    let content_height = lines.len();
    let top_pad = (grid_area.height as usize).saturating_sub(content_height) / 2;
    let mut padded: Vec<Line> = Vec::new();
    for _ in 0..top_pad {
        padded.push(Line::raw(""));
    }
    padded.append(&mut lines);

    f.render_widget(Paragraph::new(padded), grid_area);

    // ── Render adjacency sidebar (vertically centered) ──
    let arrow = if directed { "→" } else { "─" };
    let mut adj_lines: Vec<Line> = Vec::new();

    // Compute sidebar content height for centering
    let sidebar_content_h = n
        + 2
        + if frame.pointers.is_empty() {
            0
        } else {
            frame.pointers.len() + 1
        };
    let sb_top_pad = (sidebar_area.height as usize).saturating_sub(sidebar_content_h) / 2;
    for _ in 0..sb_top_pad {
        adj_lines.push(Line::raw(""));
    }

    adj_lines.push(Line::styled(
        " Adj",
        Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
    ));
    for (i, neighbors) in adj.iter().enumerate().take(n) {
        let label = if i < labels.len() { &labels[i] } else { "" };
        let color = highlight_color(highlight_map.get(&i));
        let neighbor_str = if neighbors.is_empty() {
            "[]".to_string()
        } else {
            format!("[{}]", neighbors.join(","))
        };
        adj_lines.push(Line::from(vec![
            Span::styled(format!("{:>2}", label), Style::new().fg(color)),
            Span::styled(arrow, Style::new().fg(Color::DarkGray)),
            Span::styled(neighbor_str, Style::new().fg(Color::DarkGray)),
        ]));
    }

    // Add pointer info if present
    if !frame.pointers.is_empty() {
        adj_lines.push(Line::raw(""));
        for (idx, label) in &frame.pointers {
            let color = highlight_color(highlight_map.get(idx));
            adj_lines.push(Line::from(vec![
                Span::styled(format!(" {}:", label), Style::new().fg(color)),
                Span::styled(
                    format!("{}", idx),
                    Style::new().fg(color).add_modifier(Modifier::BOLD),
                ),
            ]));
        }
    }

    f.render_widget(Paragraph::new(adj_lines), sidebar_area);
}

// ─── Grid Renderer ──────────────────────────────────────────────────

/// Render a 2D grid with colored cells.
fn render_grid(f: &mut Frame, area: Rect, cells: &[Vec<String>], frame: &VizFrame) {
    if cells.is_empty() || area.width < 4 || area.height < 3 {
        render_no_viz(f, area, "Grid is empty");
        return;
    }

    let rows = cells.len();
    let cols = cells[0].len();

    let highlight_map: std::collections::HashMap<usize, HighlightKind> =
        frame.highlights.iter().cloned().collect();

    // Compute cell width from max content
    let max_content_w = cells
        .iter()
        .flat_map(|row| row.iter())
        .map(|c| c.len())
        .max()
        .unwrap_or(1)
        .max(1);
    let cell_w = max_content_w + 2; // padding

    let mut lines: Vec<Line> = Vec::new();

    // Top border
    let mut top = String::from("  ┌");
    for c in 0..cols {
        top.push_str(&"─".repeat(cell_w));
        if c + 1 < cols {
            top.push('┬');
        }
    }
    top.push('┐');
    lines.push(Line::styled(top, Style::new().fg(Color::DarkGray)));

    for (r, row) in cells.iter().enumerate().take(rows) {
        // Value row
        let mut val_spans: Vec<Span> = vec![Span::styled("  │", Style::new().fg(Color::DarkGray))];
        for c in 0..cols {
            let flat_idx = r * cols + c;
            let style = highlight_style(highlight_map.get(&flat_idx));
            let val = if c < row.len() { &row[c] } else { "" };
            val_spans.push(Span::styled(
                format!("{:^width$}", val, width = cell_w),
                style,
            ));
            val_spans.push(Span::styled("│", Style::new().fg(Color::DarkGray)));
        }
        lines.push(Line::from(val_spans));

        // Row separator
        if r + 1 < rows {
            let mut sep = String::from("  ├");
            for c in 0..cols {
                sep.push_str(&"─".repeat(cell_w));
                if c + 1 < cols {
                    sep.push('┼');
                }
            }
            sep.push('┤');
            lines.push(Line::styled(sep, Style::new().fg(Color::DarkGray)));
        }
    }

    // Bottom border
    let mut bot = String::from("  └");
    for c in 0..cols {
        bot.push_str(&"─".repeat(cell_w));
        if c + 1 < cols {
            bot.push('┴');
        }
    }
    bot.push('┘');
    lines.push(Line::styled(bot, Style::new().fg(Color::DarkGray)));

    // Center vertically
    let content_height = lines.len();
    let top_pad = (area.height as usize).saturating_sub(content_height) / 2;
    let mut padded: Vec<Line> = Vec::new();
    for _ in 0..top_pad {
        padded.push(Line::raw(""));
    }
    padded.append(&mut lines);

    f.render_widget(Paragraph::new(padded), area);
}

// ─── No-Viz Renderer ────────────────────────────────────────────────

/// Render a "no visualization available" message.
fn render_no_viz(f: &mut Frame, area: Rect, message: &str) {
    let msg = Paragraph::new(vec![
        Line::raw(""),
        Line::from(Span::styled(
            message,
            Style::new()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )),
    ])
    .alignment(Alignment::Center);
    f.render_widget(msg, area);
}
