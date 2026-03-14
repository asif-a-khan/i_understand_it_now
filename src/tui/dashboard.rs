use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
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
        let chunks = Layout::vertical([
            Constraint::Length(5),  // Title
            Constraint::Length(3),  // Overall progress
            Constraint::Min(10),   // Part progress
            Constraint::Length(3),  // Navigation hints
        ])
        .split(area);

        // Title
        let title = Paragraph::new(vec![
            Line::raw(""),
            Line::from(vec![
                Span::styled(
                    "  dsa-forge",
                    Style::new()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "  —  Learn DSA in Rust",
                    Style::new().fg(Color::DarkGray),
                ),
            ]),
            Line::raw(""),
        ]);
        f.render_widget(title, chunks[0]);

        // Overall progress
        let all = problems::all_problems();
        let total = all.len();
        let solved = all
            .iter()
            .filter(|p| {
                progress
                    .problems
                    .get(p.id())
                    .map_or(false, |pp| pp.solved)
            })
            .count();
        let pct = if total > 0 {
            (solved as f64 / total as f64 * 100.0) as u16
        } else {
            0
        };
        let lessons_read = progress.lessons_read.values().filter(|&&v| v).count();

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(
                        " Progress: {solved}/{total} problems solved  |  {lessons_read}/38 lessons read "
                    ))
                    .title_style(theme::heading_style()),
            )
            .gauge_style(Style::new().fg(Color::Cyan).bg(Color::DarkGray))
            .percent(pct);
        f.render_widget(gauge, chunks[1]);

        // Per-part progress
        let parts = [
            ("Part 1: Foundations", &["big_o", "arrays", "strings", "linked_lists", "stacks_queues", "hash_maps", "recursion"][..]),
            ("Part 2: Sorting & Searching", &["binary_search", "basic_sorts", "merge_sort", "quick_sort", "heap_sort", "counting_radix", "two_pointers", "prefix_sum"]),
            ("Part 3: Trees", &["binary_trees", "bst", "heaps_priority_queues", "balanced_bst", "tries"]),
            ("Part 4: Graphs", &["graph_representations", "graph_bfs_dfs", "matrix_grid", "topological_sort", "shortest_path", "mst", "union_find"]),
            ("Part 5: Algorithm Paradigms", &["backtracking", "greedy", "dynamic_programming", "divide_conquer", "intervals"]),
            ("Part 6: Advanced", &["segment_fenwick", "sparse_tables", "monotonic", "bit_manipulation", "string_algorithms", "math_geometry"]),
        ];

        let mut part_lines: Vec<Line> = vec![Line::raw("")];

        for (name, topics) in &parts {
            let part_total: usize = topics.len() * 15; // 15 problems per topic
            let part_solved = all
                .iter()
                .filter(|p| {
                    topics.contains(&p.topic())
                        && progress
                            .problems
                            .get(p.id())
                            .map_or(false, |pp| pp.solved)
                })
                .count();
            let part_pct = if part_total > 0 {
                part_solved as f64 / part_total as f64
            } else {
                0.0
            };
            let bar_width = 20;
            let filled = (part_pct * bar_width as f64) as usize;
            let bar = format!(
                "[{}{}]",
                "#".repeat(filled),
                "-".repeat(bar_width - filled)
            );
            let bar_color = if part_solved == part_total && part_total > 0 {
                Color::Green
            } else if part_solved > 0 {
                Color::Yellow
            } else {
                Color::DarkGray
            };

            part_lines.push(Line::from(vec![
                Span::raw(format!("  {:>30}  ", name)),
                Span::styled(bar, Style::new().fg(bar_color)),
                Span::styled(
                    format!("  {}/{}", part_solved, part_total),
                    Style::new().fg(Color::DarkGray),
                ),
            ]));
        }

        let parts_widget = Paragraph::new(part_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Topics ")
                .title_style(theme::heading_style()),
        );
        f.render_widget(parts_widget, chunks[2]);

        // Navigation hints
        let hints = Paragraph::new(Line::from(vec![
            Span::raw("  "),
            Span::styled("[L]", Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD)),
            Span::raw(" Lessons  "),
            Span::styled("[P]", Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD)),
            Span::raw(" Problems  "),
            Span::styled("[V]", Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD)),
            Span::raw(" Visualizations  "),
            Span::styled("[?]", Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD)),
            Span::raw(" Help  "),
            Span::styled("[Q]", Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD)),
            Span::raw(" Quit"),
        ]));
        f.render_widget(hints, chunks[3]);
    }
}
