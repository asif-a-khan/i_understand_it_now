pub mod graph_bfs_dfs;
pub mod graph_representations;
pub mod matrix_grid;
pub mod mst;
pub mod shortest_path;
pub mod topological_sort;
pub mod union_find;

use super::Problem;

pub fn all_problems() -> Vec<Box<dyn Problem>> {
    let mut problems: Vec<Box<dyn Problem>> = Vec::new();
    problems.extend(graph_representations::problems());
    problems.extend(graph_bfs_dfs::problems());
    problems.extend(matrix_grid::problems());
    problems.extend(topological_sort::problems());
    problems.extend(shortest_path::problems());
    problems.extend(mst::problems());
    problems.extend(union_find::problems());
    problems
}
