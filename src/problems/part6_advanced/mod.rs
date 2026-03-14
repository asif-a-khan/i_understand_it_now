pub mod bit_manipulation;
pub mod math_geometry;
pub mod monotonic;
pub mod segment_fenwick;
pub mod sparse_tables;
pub mod string_algorithms;

use super::Problem;

pub fn all_problems() -> Vec<Box<dyn Problem>> {
    let mut problems: Vec<Box<dyn Problem>> = Vec::new();
    problems.extend(segment_fenwick::problems());
    problems.extend(sparse_tables::problems());
    problems.extend(monotonic::problems());
    problems.extend(bit_manipulation::problems());
    problems.extend(string_algorithms::problems());
    problems.extend(math_geometry::problems());
    problems
}
