// Graph BFS & DFS — Solution Templates
// Edit these functions to solve each problem. Run with: cargo run -- solve <problem_id>

/// BFS Traversal Order: return BFS visit order from node 0.
/// Visit smallest-numbered unvisited neighbor first.
pub fn bfs_order(_n: usize, _edges: &[(usize, usize)]) -> Vec<usize> {
    todo!()
}

/// DFS Traversal Order: return DFS visit order from node 0.
/// Visit smallest-numbered unvisited neighbor first (recursive).
pub fn dfs_order(_n: usize, _edges: &[(usize, usize)]) -> Vec<usize> {
    todo!()
}

/// Is Graph Connected: return true if all nodes are reachable from node 0.
pub fn is_connected(_n: usize, _edges: &[(usize, usize)]) -> bool {
    todo!()
}

/// Shortest Path (Unweighted): return shortest path length from src to dst.
/// Return -1 if unreachable.
pub fn shortest_path_unweighted(
    _n: usize,
    _edges: &[(usize, usize)],
    _src: usize,
    _dst: usize,
) -> i32 {
    todo!()
}

/// Find Any Path: return any valid path from src to dst, or empty vec if none.
pub fn find_path(_n: usize, _edges: &[(usize, usize)], _src: usize, _dst: usize) -> Vec<usize> {
    todo!()
}

/// Clone Graph: return (n, sorted_edge_list) reproducing the same graph.
/// Each edge as (min, max), list sorted lexicographically.
pub fn clone_graph(_n: usize, _edges: &[(usize, usize)]) -> (usize, Vec<(usize, usize)>) {
    todo!()
}

/// Course Schedule: return true if all courses can be finished (no cycle).
/// prerequisites[i] = (a, b) means you must take b before a.
pub fn can_finish(_n: usize, _prerequisites: &[(usize, usize)]) -> bool {
    todo!()
}

/// Course Schedule II: return a valid topological order, or empty vec if impossible.
/// prerequisites[i] = (a, b) means you must take b before a.
pub fn course_order(_n: usize, _prerequisites: &[(usize, usize)]) -> Vec<usize> {
    todo!()
}

/// Number of Islands: count islands in a binary grid (1=land, 0=water).
/// Connected horizontally/vertically.
pub fn num_islands(_grid: &[Vec<i32>]) -> i32 {
    todo!()
}

/// Word Ladder: return the length of the shortest transformation sequence.
/// Return 0 if impossible.
pub fn word_ladder(_begin: &str, _end: &str, _word_list: &[String]) -> i32 {
    todo!()
}

/// Word Ladder II: return ALL shortest transformation sequences, sorted.
pub fn word_ladder_ii(_begin: &str, _end: &str, _word_list: &[String]) -> Vec<Vec<String>> {
    todo!()
}

/// Surrounded Regions: capture 'O' regions completely surrounded by 'X'.
/// Border-connected 'O's are NOT captured.
pub fn solve_surrounded(_board: &[Vec<char>]) -> Vec<Vec<char>> {
    todo!()
}

/// Pacific Atlantic Water Flow: return cells that can flow to both oceans.
/// Sorted by (row, col).
pub fn pacific_atlantic(_heights: &[Vec<i32>]) -> Vec<(usize, usize)> {
    todo!()
}

/// All Paths from Source to Target: find all paths from 0 to n-1 in a DAG.
/// Return sorted lexicographically.
pub fn all_paths(_graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    todo!()
}

/// Shortest Path in Binary Matrix: find shortest 8-directional path in binary grid.
/// 0=open, 1=blocked. Return path length (number of cells), or -1 if impossible.
pub fn shortest_path_binary_matrix(_grid: &[Vec<i32>]) -> i32 {
    todo!()
}
