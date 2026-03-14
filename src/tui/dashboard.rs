use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::problems;
use crate::progress::Progress;

use super::screens::{Action, Screen};
use super::theme;

pub struct DashboardState;

impl DashboardState {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('l') => Action::Push(Screen::LessonList),
            KeyCode::Char('p') => Action::Push(Screen::ProblemList),
            KeyCode::Char('v') => Action::Push(Screen::VizPicker),
            KeyCode::Char('q') => Action::Quit,
            _ => Action::None,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, progress: &Progress) {
        let all = problems::all_problems();
        let total = all.len();
        let solved = all
            .iter()
            .filter(|p| progress.problems.get(p.id()).is_some_and(|pp| pp.solved))
            .count();
        let lessons_read = progress.lessons_read.values().filter(|&&v| v).count();
        let total_lessons = 38usize;

        // Layout: title (3 lines) + 2x2 card grid (fills)
        let main_chunks = Layout::vertical([
            Constraint::Length(3), // Title + summary
            Constraint::Fill(1),   // 2x2 grid
        ])
        .split(area);

        // ── Title bar ──
        let pct = if total > 0 { solved * 100 / total } else { 0 };
        let title = Paragraph::new(vec![
            Line::raw(""),
            Line::from(vec![
                Span::styled(
                    " I Understand It Now",
                    Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(
                        "  —  {solved}/{total} solved ({pct}%)  |  {lessons_read}/{total_lessons} lessons"
                    ),
                    Style::new().fg(Color::DarkGray),
                ),
            ]),
        ]);
        f.render_widget(title, main_chunks[0]);

        // ── 2x2 card grid ──
        let rows = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[1]);

        let top_cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rows[0]);

        let bot_cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rows[1]);

        // ── Card: Progress (top-left, FIRST) ──
        render_progress_card(f, top_cols[0], progress, &all);

        // ── Card: Lessons (top-right) ──
        render_card(
            f,
            top_cols[1],
            " Lessons ",
            &[
                ("Browse and study DSA concepts", Color::DarkGray),
                ("with real-world analogies", Color::DarkGray),
            ],
            &format!("{lessons_read}/{total_lessons} completed"),
            lessons_read as f64 / total_lessons as f64,
            Color::Cyan,
            "[L]",
        );

        // ── Card: Problems (bottom-left) ──
        render_card(
            f,
            bot_cols[0],
            " Problems ",
            &[
                ("Practice with randomized tests", Color::DarkGray),
                ("15 problems per topic", Color::DarkGray),
            ],
            &format!("{solved}/{total} solved"),
            solved as f64 / total.max(1) as f64,
            Color::Yellow,
            "[P]",
        );

        // ── Card: Visualizations (bottom-right) ──
        render_card(
            f,
            bot_cols[1],
            " Visualizations ",
            &[
                ("Watch algorithms execute", Color::DarkGray),
                ("step by step with animation", Color::DarkGray),
            ],
            &format!("{total} available"),
            1.0,
            Color::Green,
            "[V]",
        );
    }
}

/// Render a Material-inspired card with title, description, progress bar, and key hint.
#[allow(clippy::too_many_arguments)]
fn render_card(
    f: &mut Frame,
    area: Rect,
    title: &str,
    description: &[(&str, Color)],
    progress_label: &str,
    progress_pct: f64,
    bar_color: Color,
    key_hint: &str,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let h = inner.height as usize;
    let w = inner.width as usize;
    let mut lines: Vec<Line> = Vec::new();

    // Center content vertically
    let content_lines = description.len() + 4;
    let top_pad = h.saturating_sub(content_lines) / 2;
    for _ in 0..top_pad {
        lines.push(Line::raw(""));
    }

    // Description
    for (text, color) in description {
        lines.push(Line::from(Span::styled(*text, Style::new().fg(*color))));
    }

    lines.push(Line::raw(""));

    // Progress bar — fill all available horizontal space
    let label_len = progress_label.len() + 3; // "  " + label
    let bar_w = w.saturating_sub(label_len).saturating_sub(2);
    let filled = (progress_pct * bar_w as f64) as usize;
    let active_color = if progress_pct >= 1.0 {
        Color::Green
    } else {
        bar_color
    };
    lines.push(Line::from(vec![
        Span::styled("━".repeat(filled), Style::new().fg(active_color)),
        Span::styled(
            "─".repeat(bar_w.saturating_sub(filled)),
            Style::new().fg(Color::DarkGray),
        ),
        Span::styled(
            format!("  {}", progress_label),
            Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
    ]));

    // Key hint
    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        format!("Press {}", key_hint),
        Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
    )));

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(paragraph, inner);
}

/// Render the Progress card — categories spread evenly, full-width bars, distinct colors.
fn render_progress_card(
    f: &mut Frame,
    area: Rect,
    progress: &Progress,
    all: &[Box<dyn crate::problems::Problem>],
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Progress ")
        .title_style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let parts: &[(&str, &[&str], Color)] = &[
        (
            "Foundations",
            &[
                "big_o",
                "arrays",
                "strings",
                "linked_lists",
                "stacks_queues",
                "hash_maps",
                "recursion",
            ],
            Color::Cyan,
        ),
        (
            "Sorting",
            &[
                "binary_search",
                "basic_sorts",
                "merge_sort",
                "quick_sort",
                "heap_sort",
                "counting_radix",
                "two_pointers",
                "prefix_sum",
            ],
            Color::Yellow,
        ),
        (
            "Trees",
            &[
                "binary_trees",
                "bst",
                "heaps_priority_queues",
                "balanced_bst",
                "tries",
            ],
            Color::Green,
        ),
        (
            "Graphs",
            &[
                "graph_representations",
                "graph_bfs_dfs",
                "matrix_grid",
                "topological_sort",
                "shortest_path",
                "mst",
                "union_find",
            ],
            Color::Blue,
        ),
        (
            "Paradigms",
            &[
                "backtracking",
                "greedy",
                "dynamic_programming",
                "divide_conquer",
                "intervals",
            ],
            Color::Magenta,
        ),
        (
            "Advanced",
            &[
                "segment_fenwick",
                "sparse_tables",
                "monotonic",
                "bit_manipulation",
                "string_algorithms",
                "math_geometry",
            ],
            Color::Red,
        ),
    ];

    let h = inner.height as usize;
    let w = inner.width as usize;
    let num_parts = parts.len();

    // Spread categories evenly across available height
    let spacing = if num_parts > 1 {
        h.saturating_sub(num_parts) / (num_parts + 1)
    } else {
        h / 2
    };

    // Dynamic label width based on longest name
    let label_w = parts.iter().map(|(n, _, _)| n.len()).max().unwrap_or(10);
    // Bar fills remaining width after label and count
    let count_w = 8; // " XX/XXX"
    let bar_w = w.saturating_sub(label_w + count_w + 4);

    let mut lines: Vec<Line> = Vec::new();
    let mut row = 0;

    for (i, (name, topics, color)) in parts.iter().enumerate() {
        // Add spacing before each category
        let target_row = spacing * (i + 1) + i;
        while row < target_row && row < h {
            lines.push(Line::raw(""));
            row += 1;
        }

        let part_total = topics.len() * 15;
        let part_solved = all
            .iter()
            .filter(|p| {
                topics.contains(&p.topic())
                    && progress.problems.get(p.id()).is_some_and(|pp| pp.solved)
            })
            .count();
        let pct = if part_total > 0 {
            part_solved as f64 / part_total as f64
        } else {
            0.0
        };
        let filled = (pct * bar_w as f64) as usize;
        let active_color = if part_solved == part_total && part_total > 0 {
            Color::Green
        } else {
            *color
        };

        lines.push(Line::from(vec![
            Span::styled(
                format!(" {:>width$} ", name, width = label_w),
                Style::new().fg(*color).add_modifier(Modifier::BOLD),
            ),
            Span::styled("━".repeat(filled), Style::new().fg(active_color)),
            Span::styled(
                "─".repeat(bar_w.saturating_sub(filled)),
                Style::new().fg(Color::DarkGray),
            ),
            Span::styled(
                format!(" {}/{}", part_solved, part_total),
                Style::new().fg(Color::DarkGray),
            ),
        ]));
        row += 1;
    }

    f.render_widget(Paragraph::new(lines), inner);
}
