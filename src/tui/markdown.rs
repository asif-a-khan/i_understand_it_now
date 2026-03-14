use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

/// Parse a markdown string into styled ratatui Lines.
pub fn render_markdown(input: &str) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut in_code_block = false;
    let mut in_table = false;

    for raw_line in input.lines() {
        // Code block toggle
        if raw_line.trim_start().starts_with("```") {
            if in_code_block {
                in_code_block = false;
                lines.push(Line::raw(""));
            } else {
                in_code_block = true;
                lines.push(Line::raw(""));
            }
            continue;
        }

        // Inside code block: render as-is with dim style
        if in_code_block {
            lines.push(Line::from(Span::styled(
                format!("  {}", raw_line),
                Style::new().fg(Color::Yellow),
            )));
            continue;
        }

        let trimmed = raw_line.trim();

        // Empty line
        if trimmed.is_empty() {
            in_table = false;
            lines.push(Line::raw(""));
            continue;
        }

        // Horizontal rule
        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            in_table = false;
            lines.push(Line::from(Span::styled(
                "─".repeat(60),
                Style::new().fg(Color::DarkGray),
            )));
            continue;
        }

        // Headers
        if let Some(rest) = trimmed.strip_prefix("### ") {
            in_table = false;
            lines.push(Line::raw(""));
            lines.push(Line::from(Span::styled(
                format!("   {}", rest),
                Style::new()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("## ") {
            in_table = false;
            lines.push(Line::raw(""));
            lines.push(Line::from(Span::styled(
                format!("  {}", rest),
                Style::new()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )));
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("# ") {
            in_table = false;
            lines.push(Line::raw(""));
            lines.push(Line::from(Span::styled(
                rest.to_string(),
                Style::new()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )));
            lines.push(Line::raw(""));
            continue;
        }

        // Table rows
        if trimmed.starts_with('|') {
            // Skip separator rows like |---|---|
            if trimmed.contains("---") {
                in_table = true;
                lines.push(Line::from(Span::styled(
                    format!("  {}", trimmed),
                    Style::new().fg(Color::DarkGray),
                )));
                continue;
            }
            in_table = true;
            let spans = parse_table_row(trimmed);
            lines.push(Line::from(spans));
            continue;
        }

        if in_table && !trimmed.starts_with('|') {
            in_table = false;
        }

        // Unordered list items
        if let Some(rest) = trimmed.strip_prefix("- ") {
            let mut spans = vec![Span::styled("  * ", Style::new().fg(Color::Cyan))];
            spans.extend(parse_inline(rest));
            lines.push(Line::from(spans));
            continue;
        }

        // Numbered list items
        if let Some(dot_pos) = trimmed.find(". ") {
            let prefix = &trimmed[..dot_pos];
            if prefix.chars().all(|c| c.is_ascii_digit()) {
                let rest = &trimmed[dot_pos + 2..];
                let mut spans = vec![Span::styled(
                    format!("  {}. ", prefix),
                    Style::new().fg(Color::Cyan),
                )];
                spans.extend(parse_inline(rest));
                lines.push(Line::from(spans));
                continue;
            }
        }

        // Indented text (4+ spaces or tab) — treat as code
        if raw_line.starts_with("    ") || raw_line.starts_with('\t') {
            lines.push(Line::from(Span::styled(
                format!("  {}", raw_line),
                Style::new().fg(Color::Yellow),
            )));
            continue;
        }

        // Regular paragraph text with inline formatting
        let mut spans = vec![Span::raw("  ")];
        spans.extend(parse_inline(trimmed));
        lines.push(Line::from(spans));
    }

    lines
}

/// Parse inline formatting: **bold**, `code`, *italic*
fn parse_inline(text: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut chars = text.char_indices().peekable();
    let mut current = String::new();

    while let Some((i, ch)) = chars.next() {
        match ch {
            '`' => {
                // Inline code
                if !current.is_empty() {
                    spans.push(Span::raw(std::mem::take(&mut current)));
                }
                let mut code = String::new();
                for (_, c) in chars.by_ref() {
                    if c == '`' {
                        break;
                    }
                    code.push(c);
                }
                spans.push(Span::styled(
                    code,
                    Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ));
            }
            '*' => {
                // Check for **bold**
                if chars.peek().map_or(false, |(_, c)| *c == '*') {
                    chars.next(); // consume second *
                    if !current.is_empty() {
                        spans.push(Span::raw(std::mem::take(&mut current)));
                    }
                    let mut bold = String::new();
                    while let Some((_, c)) = chars.next() {
                        if c == '*' {
                            if chars.peek().map_or(false, |(_, c2)| *c2 == '*') {
                                chars.next(); // consume closing **
                                break;
                            }
                        }
                        bold.push(c);
                    }
                    spans.push(Span::styled(
                        bold,
                        Style::new()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    // *italic*
                    if !current.is_empty() {
                        spans.push(Span::raw(std::mem::take(&mut current)));
                    }
                    let mut italic = String::new();
                    for (_, c) in chars.by_ref() {
                        if c == '*' {
                            break;
                        }
                        italic.push(c);
                    }
                    spans.push(Span::styled(
                        italic,
                        Style::new().add_modifier(Modifier::ITALIC),
                    ));
                }
            }
            _ => {
                let _ = i; // suppress unused warning
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        spans.push(Span::raw(current));
    }

    spans
}

/// Parse a markdown table row into styled spans.
fn parse_table_row(line: &str) -> Vec<Span<'static>> {
    let mut spans = vec![Span::raw("  ")];
    let cells: Vec<&str> = line.split('|').collect();
    for (i, cell) in cells.iter().enumerate() {
        let trimmed = cell.trim();
        if i == 0 && trimmed.is_empty() {
            spans.push(Span::styled("| ", Style::new().fg(Color::DarkGray)));
            continue;
        }
        if i == cells.len() - 1 && trimmed.is_empty() {
            spans.push(Span::styled("|", Style::new().fg(Color::DarkGray)));
            continue;
        }
        if !trimmed.is_empty() {
            let mut cell_spans = parse_inline(trimmed);
            spans.append(&mut cell_spans);
        }
        if i < cells.len() - 1 {
            spans.push(Span::styled(" | ", Style::new().fg(Color::DarkGray)));
        }
    }
    spans
}
