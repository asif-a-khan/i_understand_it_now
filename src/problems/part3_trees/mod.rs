pub mod balanced_bst;
pub mod binary_trees;
pub mod bst;
pub mod heaps_priority_queues;
pub mod tries;

use super::Problem;

pub fn all_problems() -> Vec<Box<dyn Problem>> {
    let mut problems: Vec<Box<dyn Problem>> = Vec::new();
    problems.extend(binary_trees::problems());
    problems.extend(bst::problems());
    problems.extend(heaps_priority_queues::problems());
    problems.extend(balanced_bst::problems());
    problems.extend(tries::problems());
    problems
}
