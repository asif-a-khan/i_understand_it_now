pub mod helpers;
pub mod part1_foundations;
pub mod part2_sorting;
pub mod part3_trees;
pub mod part4_graphs;
pub mod part5_paradigms;
pub mod part6_advanced;

use crate::tracker::OperationLog;

/// Difficulty level for a problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

/// Result of running a solution against a single test case.
pub struct SolutionResult {
    pub is_correct: bool,
    pub input_description: String,
    pub expected: String,
    pub actual: String,
}

/// A test case: opaque data that a Problem knows how to interpret.
pub struct TestCase {
    pub data: Box<dyn std::any::Any>,
}

/// Trait that every problem must implement.
pub trait Problem {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn topic(&self) -> &str;
    fn difficulty(&self) -> Difficulty;
    fn description(&self) -> &str;

    /// Generate randomized test cases.
    fn generate_tests(&self) -> Vec<TestCase>;

    /// Run the user's solution against a test case, recording operations.
    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult;
}

/// Look up a problem by its identifier.
pub fn get_problem(name: &str) -> Option<Box<dyn Problem>> {
    all_problems().into_iter().find(|p| p.id() == name)
}

/// List all available problem identifiers.
pub fn list_problems() -> Vec<String> {
    all_problems().iter().map(|p| p.id().to_string()).collect()
}

/// Get all problems grouped by topic.
pub fn all_problems() -> Vec<Box<dyn Problem>> {
    let mut problems = Vec::new();
    problems.extend(part1_foundations::all_problems());
    problems.extend(part2_sorting::all_problems());
    problems.extend(part3_trees::all_problems());
    problems.extend(part4_graphs::all_problems());
    problems.extend(part5_paradigms::all_problems());
    problems.extend(part6_advanced::all_problems());
    problems
}

/// List problems filtered by topic.
#[allow(dead_code)]
pub fn problems_by_topic(topic: &str) -> Vec<Box<dyn Problem>> {
    all_problems()
        .into_iter()
        .filter(|p| p.topic() == topic)
        .collect()
}

/// List problems filtered by difficulty.
#[allow(dead_code)]
pub fn problems_by_difficulty(difficulty: Difficulty) -> Vec<Box<dyn Problem>> {
    all_problems()
        .into_iter()
        .filter(|p| p.difficulty() == difficulty)
        .collect()
}
