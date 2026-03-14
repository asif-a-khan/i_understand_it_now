/// Which screen the TUI is currently showing.
#[derive(Debug, Clone)]
pub enum Screen {
    Dashboard,
    LessonList,
    LessonReader { lesson_idx: usize },
    ProblemList,
    ProblemDetail { problem_idx: usize },
    ProblemRunning { problem_idx: usize },
    ProblemResult { problem_idx: usize },
    ComplexityView { problem_idx: usize },
    ReplayPlayer { problem_idx: usize },
    InTuiEditor { problem_idx: usize },
    VizPicker,
}

/// Actions returned from screen event handlers.
pub enum Action {
    None,
    Quit,
    Push(Screen),
    Pop,
    LaunchEditor(String),
    /// Clear the screen stack and navigate directly to a screen.
    GoTo(Screen),
    /// From editor: save, run tests, then jump to replay.
    SaveRunReplay(usize),
}
