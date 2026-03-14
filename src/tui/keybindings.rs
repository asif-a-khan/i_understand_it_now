use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use super::screens::Screen;
use super::theme;

pub fn render_help(f: &mut Frame, screen: &Screen) {
    let area = centered_rect(60, 70, f.area());
    f.render_widget(Clear, area);

    let bindings = match screen {
        Screen::Dashboard => vec![
            ("l", "Open lesson browser"),
            ("p", "Open problem list"),
            ("v", "Open visualization picker"),
            ("?", "Toggle this help"),
            ("q", "Quit"),
        ],
        Screen::LessonList => vec![
            ("j / Down", "Move down"),
            ("k / Up", "Move up"),
            ("Enter", "Open lesson"),
            ("Esc", "Back to dashboard"),
            ("?", "Toggle this help"),
        ],
        Screen::LessonReader { .. } => vec![
            ("j / Down", "Scroll down"),
            ("k / Up", "Scroll up"),
            ("Space / PgDn", "Page down"),
            ("PgUp", "Page up"),
            ("g", "Go to top"),
            ("G", "Go to bottom"),
            ("m", "Mark lesson as read"),
            ("Esc", "Back to lesson list"),
            ("?", "Toggle this help"),
        ],
        Screen::ProblemList => vec![
            ("j / Down", "Move down"),
            ("k / Up", "Move up"),
            ("Enter", "View problem detail"),
            ("t", "Cycle topic filter"),
            ("d", "Cycle difficulty filter"),
            ("Esc", "Back to dashboard"),
            ("?", "Toggle this help"),
        ],
        Screen::ProblemDetail { .. } => vec![
            ("r", "Run tests"),
            ("e", "Edit solution in $EDITOR"),
            ("i", "Edit solution in-TUI"),
            ("Esc", "Back to problem list"),
            ("?", "Toggle this help"),
        ],
        Screen::ProblemRunning { .. } => vec![
            ("", "Tests running..."),
        ],
        Screen::ProblemResult { .. } => vec![
            ("r", "Run tests again"),
            ("e", "Edit solution in $EDITOR"),
            ("i", "Edit solution in-TUI"),
            ("w", "Watch instrumented replay"),
            ("c", "Measure complexity"),
            ("Esc", "Back to problem detail"),
            ("?", "Toggle this help"),
        ],
        Screen::ComplexityView { .. } => vec![
            ("j / Down", "Scroll down"),
            ("k / Up", "Scroll up"),
            ("Esc", "Back"),
            ("?", "Toggle this help"),
        ],
        Screen::ReplayPlayer { .. } => vec![
            ("Right / Enter", "Next frame"),
            ("Left", "Previous frame"),
            ("a", "Toggle auto-play"),
            ("+/-", "Adjust speed"),
            ("Esc", "Back"),
            ("?", "Toggle this help"),
        ],
        Screen::InTuiEditor { .. } => vec![
            ("Arrow keys", "Move cursor"),
            ("Home / End", "Start / end of line"),
            ("PgUp / PgDn", "Page up / down"),
            ("Ctrl+S", "Save file"),
            ("Esc", "Close editor"),
        ],
        Screen::VizPicker => vec![
            ("j / Down", "Move down"),
            ("k / Up", "Move up"),
            ("Enter", "Watch visualization"),
            ("Esc", "Back to dashboard"),
            ("?", "Toggle this help"),
        ],
        Screen::VizPlayer { .. } => vec![
            ("Right / Enter", "Next frame"),
            ("Left", "Previous frame"),
            ("a", "Toggle auto-play"),
            ("+/-", "Adjust speed"),
            ("Home", "First frame"),
            ("End", "Last frame"),
            ("Esc", "Back to picker"),
            ("?", "Toggle this help"),
        ],
    };

    let lines: Vec<Line> = bindings
        .iter()
        .map(|(key, desc)| {
            Line::from(vec![
                Span::styled(
                    format!("  {:>16}  ", key),
                    Style::new().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
                ),
                Span::raw(*desc),
            ])
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(theme::ACCENT))
        .title(" Keybindings ")
        .title_style(theme::title_style());

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
