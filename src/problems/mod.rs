pub mod arrays_two_sum;

use crate::tracker::OperationLog;

/// Difficulty level for a problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
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
    match name {
        "arrays_two_sum" => Some(Box::new(arrays_two_sum::ArraysTwoSum)),
        _ => None,
    }
}

/// List all available problem identifiers.
pub fn list_problems() -> Vec<&'static str> {
    vec!["arrays_two_sum"]
}
