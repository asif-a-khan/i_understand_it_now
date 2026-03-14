use ratatui::style::{Color, Modifier, Style};

pub const ACCENT: Color = Color::Cyan;
pub const SUCCESS: Color = Color::Green;
pub const WARNING: Color = Color::Yellow;
pub const ERROR: Color = Color::Red;
pub const MUTED: Color = Color::DarkGray;

pub fn title_style() -> Style {
    Style::new().fg(ACCENT).add_modifier(Modifier::BOLD)
}

pub fn heading_style() -> Style {
    Style::new().fg(Color::White).add_modifier(Modifier::BOLD)
}

pub fn selected_style() -> Style {
    Style::new().bg(Color::DarkGray).fg(Color::White)
}

pub fn muted_style() -> Style {
    Style::new().fg(MUTED)
}

pub fn success_style() -> Style {
    Style::new().fg(SUCCESS).add_modifier(Modifier::BOLD)
}

pub fn error_style() -> Style {
    Style::new().fg(ERROR).add_modifier(Modifier::BOLD)
}

pub fn warning_style() -> Style {
    Style::new().fg(WARNING)
}

pub fn difficulty_style(difficulty: &crate::problems::Difficulty) -> Style {
    match difficulty {
        crate::problems::Difficulty::Easy => Style::new().fg(SUCCESS),
        crate::problems::Difficulty::Medium => Style::new().fg(WARNING),
        crate::problems::Difficulty::Hard => Style::new().fg(ERROR),
    }
}
