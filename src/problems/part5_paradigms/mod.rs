pub mod backtracking;
pub mod divide_conquer;
pub mod dynamic_programming;
pub mod greedy;
pub mod intervals;

use super::Problem;

pub fn all_problems() -> Vec<Box<dyn Problem>> {
    let mut problems: Vec<Box<dyn Problem>> = Vec::new();
    problems.extend(backtracking::problems());
    problems.extend(greedy::problems());
    problems.extend(dynamic_programming::problems());
    problems.extend(divide_conquer::problems());
    problems.extend(intervals::problems());
    problems
}
