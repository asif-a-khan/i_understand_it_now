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
    VizPicker,
    VizPlayer { viz_idx: usize },
}

/// Actions returned from screen event handlers.
pub enum Action {
    None,
    Quit,
    Push(Screen),
    Pop,
    LaunchEditor(String),
}
