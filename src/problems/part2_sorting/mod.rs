pub mod basic_sorts;
pub mod binary_search;
pub mod counting_radix;
pub mod heap_sort;
pub mod merge_sort;
pub mod prefix_sum;
pub mod quick_sort;
pub mod two_pointers;

use super::Problem;

pub fn all_problems() -> Vec<Box<dyn Problem>> {
    let mut problems: Vec<Box<dyn Problem>> = Vec::new();
    problems.extend(binary_search::problems());
    problems.extend(basic_sorts::problems());
    problems.extend(merge_sort::problems());
    problems.extend(quick_sort::problems());
    problems.extend(heap_sort::problems());
    problems.extend(counting_radix::problems());
    problems.extend(two_pointers::problems());
    problems.extend(prefix_sum::problems());
    problems
}
