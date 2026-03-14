pub mod arrays;
pub mod big_o;
pub mod hash_maps;
pub mod linked_lists;
pub mod recursion;
pub mod stacks_queues;
pub mod strings;

use super::Problem;

pub fn all_problems() -> Vec<Box<dyn Problem>> {
    let mut problems: Vec<Box<dyn Problem>> = Vec::new();
    problems.extend(big_o::problems());
    problems.extend(arrays::problems());
    problems.extend(strings::problems());
    problems.extend(linked_lists::problems());
    problems.extend(stacks_queues::problems());
    problems.extend(hash_maps::problems());
    problems.extend(recursion::problems());
    problems
}
