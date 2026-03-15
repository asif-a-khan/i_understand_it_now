use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{self, Color, Stylize},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

use super::{HighlightKind, VizData, VizFrame};

/// Playback mode for the visualizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackMode {
    /// User presses Enter/Right to advance, Left to go back
    StepByStep,
    /// Auto-advance with a delay between frames
    AutoPlay,
}

/// Play a sequence of visualization frames in the terminal.
pub fn play(frames: &[VizFrame], mode: PlaybackMode, delay_ms: u64) -> io::Result<()> {
    if frames.is_empty() {
        println!("No frames to display.");
        return Ok(());
    }

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = match mode {
        PlaybackMode::StepByStep => play_step_by_step(&mut stdout, frames),
        PlaybackMode::AutoPlay => play_auto(&mut stdout, frames, delay_ms),
    };

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    result
}

fn play_step_by_step(stdout: &mut io::Stdout, frames: &[VizFrame]) -> io::Result<()> {
    let mut idx = 0;
    loop {
        render_frame(stdout, &frames[idx], idx, frames.len())?;
        render_controls(
            stdout,
            "Step-by-step: [Right/Enter] next  [Left] prev  [q] quit",
        )?;
        stdout.flush()?;

        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Right | KeyCode::Enter | KeyCode::Char(' '),
                ..
            }) => {
                if idx + 1 < frames.len() {
                    idx += 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                idx = idx.saturating_sub(1);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Home,
                ..
            }) => {
                idx = 0;
            }
            Event::Key(KeyEvent {
                code: KeyCode::End, ..
            }) => {
                idx = frames.len() - 1;
            }
            _ => {}
        }
    }
    Ok(())
}

fn play_auto(stdout: &mut io::Stdout, frames: &[VizFrame], delay_ms: u64) -> io::Result<()> {
    let delay = std::time::Duration::from_millis(delay_ms);

    for (idx, frame) in frames.iter().enumerate() {
        render_frame(stdout, frame, idx, frames.len())?;
        render_controls(stdout, "Auto-play: [q] quit  [Space] pause")?;
        stdout.flush()?;

        // Wait for delay, but check for quit key
        let start = std::time::Instant::now();
        while start.elapsed() < delay {
            if event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                    match code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char(' ') => {
                            // Pause: switch to step-by-step for remaining frames
                            render_controls(
                                stdout,
                                "Paused: [Right/Enter] next  [Left] prev  [q] quit",
                            )?;
                            stdout.flush()?;
                            return play_step_by_step_from(stdout, frames, idx);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    // Show final frame until user quits
    render_controls(stdout, "Done! Press [q] to quit")?;
    stdout.flush()?;
    loop {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter,
            ..
        }) = event::read()?
        {
            break;
        }
    }
    Ok(())
}

fn play_step_by_step_from(
    stdout: &mut io::Stdout,
    frames: &[VizFrame],
    start_idx: usize,
) -> io::Result<()> {
    let mut idx = start_idx;
    loop {
        render_frame(stdout, &frames[idx], idx, frames.len())?;
        render_controls(stdout, "Paused: [Right/Enter] next  [Left] prev  [q] quit")?;
        stdout.flush()?;

        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Right | KeyCode::Enter | KeyCode::Char(' '),
                ..
            }) => {
                if idx + 1 < frames.len() {
                    idx += 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                idx = idx.saturating_sub(1);
            }
            _ => {}
        }
    }
    Ok(())
}

fn render_frame(
    stdout: &mut io::Stdout,
    frame: &VizFrame,
    idx: usize,
    total: usize,
) -> io::Result<()> {
    queue!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    // Header
    queue!(
        stdout,
        style::PrintStyledContent(
            format!("  Step {}/{}\n\n", idx + 1, total).with(Color::DarkGrey)
        )
    )?;

    // Build highlight lookup
    let highlight_map: std::collections::HashMap<usize, HighlightKind> =
        frame.highlights.iter().cloned().collect();

    let highlight_to_color = |kind: Option<&HighlightKind>| -> Color {
        match kind {
            Some(HighlightKind::Comparing) => Color::Yellow,
            Some(HighlightKind::Swapping) => Color::Red,
            Some(HighlightKind::Sorted) => Color::DarkGreen,
            Some(HighlightKind::Active) => Color::DarkCyan,
            Some(HighlightKind::Pivot) => Color::Magenta,
            Some(HighlightKind::Found) => Color::Green,
            Some(HighlightKind::Reading) => Color::Cyan,
            Some(HighlightKind::Writing) => Color::DarkMagenta,
            Some(HighlightKind::Target) => Color::Red,
            None => Color::White,
        }
    };

    // Get array values (from viz_data or legacy array field)
    let values: Vec<String> = match frame.viz_data() {
        VizData::Array { values } => values,
        VizData::None { message } => {
            queue!(
                stdout,
                style::Print("  "),
                style::PrintStyledContent(message.with(Color::DarkGrey)),
                style::Print("\n\n"),
            )?;
            vec![]
        }
        _ => frame.array.iter().map(|v| v.to_string()).collect(),
    };

    if !values.is_empty() {
        // Render bar chart
        let max_val = frame.array.iter().copied().max().unwrap_or(1).max(1);
        let bar_height = 12usize;

        for row in (0..bar_height).rev() {
            queue!(stdout, style::Print("  "))?;
            for (col, val) in frame.array.iter().enumerate() {
                let normalized = (*val as f64 / max_val as f64 * bar_height as f64) as usize;
                let cell = if normalized > row { "██" } else { "  " };
                let color = highlight_to_color(highlight_map.get(&col));
                queue!(stdout, style::PrintStyledContent(cell.with(color)))?;
                queue!(stdout, style::Print(" "))?;
            }
            queue!(stdout, style::Print("\n"))?;
        }

        // Render values below bars
        queue!(stdout, style::Print("  "))?;
        for (col, val) in values.iter().enumerate() {
            let color = highlight_to_color(highlight_map.get(&col));
            let s = format!("{:>3} ", val);
            queue!(stdout, style::PrintStyledContent(s.with(color)))?;
        }
        queue!(stdout, style::Print("\n\n"))?;

        // Render indices
        queue!(stdout, style::Print("  "))?;
        for col in 0..values.len() {
            let s = format!("{:>3} ", col);
            queue!(stdout, style::PrintStyledContent(s.with(Color::DarkGrey)))?;
        }
        queue!(stdout, style::Print("\n\n"))?;
    }

    // Annotation
    queue!(
        stdout,
        style::Print("  "),
        style::PrintStyledContent(frame.annotation.as_str().with(Color::White).bold()),
        style::Print("\n\n"),
    )?;

    // Legend
    queue!(stdout, style::Print("  "))?;
    let legend = [
        ("██ cmp", Color::Yellow),
        ("██ swp", Color::Red),
        ("██ done", Color::DarkGreen),
        ("██ active", Color::DarkCyan),
        ("██ pivot", Color::Magenta),
        ("██ found", Color::Green),
        ("██ read", Color::Cyan),
        ("██ write", Color::DarkMagenta),
        ("██ target", Color::Red),
    ];
    for (label, color) in &legend {
        queue!(
            stdout,
            style::PrintStyledContent(label.with(*color)),
            style::Print("  "),
        )?;
    }
    queue!(stdout, style::Print("\n"))?;

    Ok(())
}

fn render_controls(stdout: &mut io::Stdout, msg: &str) -> io::Result<()> {
    queue!(
        stdout,
        style::Print("\n  "),
        style::PrintStyledContent(msg.with(Color::DarkGrey)),
        style::Print("\n"),
    )?;
    Ok(())
}
