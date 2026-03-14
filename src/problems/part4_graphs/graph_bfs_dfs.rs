use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part4_graphs::graph_bfs_dfs as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(BfsOrder),
        Box::new(DfsOrder),
        Box::new(IsConnected),
        Box::new(ShortestPathUnweighted),
        Box::new(FindPath),
        Box::new(CloneGraph),
        Box::new(CourseSchedule),
        Box::new(CourseScheduleII),
        Box::new(NumberOfIslands),
        Box::new(WordLadder),
        Box::new(WordLadderII),
        Box::new(SurroundedRegions),
        Box::new(PacificAtlantic),
        Box::new(AllPaths),
        Box::new(ShortestPathBinaryMatrix),
    ]
}

// ── Helpers ────────────────────────────────────────────────────────────

/// Build undirected adjacency list.
fn build_adj_undirected(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    // Sort neighbors for deterministic traversal order
    for row in &mut adj {
        row.sort();
        row.dedup();
    }
    adj
}

/// Build directed adjacency list.
#[allow(dead_code)]
fn build_adj_directed(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
    }
    for row in &mut adj {
        row.sort();
        row.dedup();
    }
    adj
}

/// Generate random undirected edges with no self-loops or duplicates.
fn random_undirected_edges(rng: &mut impl Rng, n: usize, edge_count: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    let mut seen = HashSet::new();
    for _ in 0..edge_count {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v {
            let key = (u.min(v), u.max(v));
            if seen.insert(key) {
                edges.push((u, v));
            }
        }
    }
    edges
}

/// Generate connected undirected edges.
fn random_connected_edges(rng: &mut impl Rng, n: usize, extra: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    let mut seen = HashSet::new();
    for i in 1..n {
        let j = rng.random_range(0..i);
        let key = (j.min(i), j.max(i));
        seen.insert(key);
        edges.push((i, j));
    }
    for _ in 0..extra {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v {
            let key = (u.min(v), u.max(v));
            if seen.insert(key) {
                edges.push((u, v));
            }
        }
    }
    edges
}

/// Generate DAG edges (from lower to higher index only).
fn random_dag_edges(rng: &mut impl Rng, n: usize, edge_count: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    let mut seen = HashSet::new();
    for _ in 0..edge_count {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u < v && seen.insert((u, v)) {
            edges.push((u, v));
        }
    }
    edges
}

/// Generate directed edges with possible cycles.
fn random_directed_edges(rng: &mut impl Rng, n: usize, edge_count: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    let mut seen = HashSet::new();
    for _ in 0..edge_count {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v && seen.insert((u, v)) {
            edges.push((u, v));
        }
    }
    edges
}

// ── Easy 1: BFS Order ─────────────────────────────────────────────────

struct BfsOrder;
struct BfsOrderTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for BfsOrder {
    fn id(&self) -> &str {
        "graph_bfs_dfs_bfs_order"
    }
    fn name(&self) -> &str {
        "BFS Traversal Order"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Perform a BFS traversal from node 0 on an undirected graph.\n\n\
         Return the order in which nodes are visited. When choosing which\n\
         neighbor to visit next, always pick the smallest-numbered unvisited\n\
         neighbor first.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: Vec<usize> — BFS visit order starting from node 0.\n\
         Only include nodes reachable from 0.\n\n\
         Constraints:\n\
         - 1 <= n <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let extra = rng.random_range(0..=n);
                let edges = random_connected_edges(&mut rng, n, extra);
                TestCase {
                    data: Box::new(BfsOrderTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BfsOrderTest>().unwrap();
        let expected = ref_bfs_order(t.n, &t.edges);
        let actual = solutions::bfs_order(t.n, &t.edges);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_bfs_order(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let adj = build_adj_undirected(n, edges);
    let mut visited = vec![false; n];
    let mut order = Vec::new();
    let mut queue = VecDeque::new();
    visited[0] = true;
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                queue.push_back(v);
            }
        }
    }
    order
}

// ── Easy 2: DFS Order ─────────────────────────────────────────────────

struct DfsOrder;
struct DfsOrderTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for DfsOrder {
    fn id(&self) -> &str {
        "graph_bfs_dfs_dfs_order"
    }
    fn name(&self) -> &str {
        "DFS Traversal Order"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Perform a DFS traversal from node 0 on an undirected graph.\n\n\
         Return the order in which nodes are first visited. When choosing which\n\
         neighbor to visit next, always pick the smallest-numbered unvisited\n\
         neighbor first (use recursive DFS or simulate with a stack that\n\
         processes neighbors in ascending order).\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: Vec<usize> — DFS visit order starting from node 0.\n\
         Only include nodes reachable from 0.\n\n\
         Constraints:\n\
         - 1 <= n <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let extra = rng.random_range(0..=n);
                let edges = random_connected_edges(&mut rng, n, extra);
                TestCase {
                    data: Box::new(DfsOrderTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DfsOrderTest>().unwrap();
        let expected = ref_dfs_order(t.n, &t.edges);
        let actual = solutions::dfs_order(t.n, &t.edges);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_dfs_order(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let adj = build_adj_undirected(n, edges);
    let mut visited = vec![false; n];
    let mut order = Vec::new();
    fn dfs(u: usize, adj: &[Vec<usize>], visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[u] = true;
        order.push(u);
        for &v in &adj[u] {
            if !visited[v] {
                dfs(v, adj, visited, order);
            }
        }
    }
    dfs(0, &adj, &mut visited, &mut order);
    order
}

// ── Easy 3: Is Connected ──────────────────────────────────────────────

struct IsConnected;
struct IsConnectedTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for IsConnected {
    fn id(&self) -> &str {
        "graph_bfs_dfs_is_connected"
    }
    fn name(&self) -> &str {
        "Is Graph Connected"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Check if an undirected graph is connected (all nodes reachable from node 0).\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: bool — true if the graph is connected.\n\n\
         Constraints:\n\
         - 1 <= n <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let n = rng.random_range(2..=15);
                let extra = rng.random_range(0..=n);
                let edges = if i % 2 == 0 {
                    random_connected_edges(&mut rng, n, extra)
                } else {
                    random_undirected_edges(&mut rng, n, extra)
                };
                TestCase {
                    data: Box::new(IsConnectedTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IsConnectedTest>().unwrap();
        let expected = ref_is_connected(t.n, &t.edges);
        let actual = solutions::is_connected(t.n, &t.edges);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_is_connected(n: usize, edges: &[(usize, usize)]) -> bool {
    let adj = build_adj_undirected(n, edges);
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[0] = true;
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &v in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                queue.push_back(v);
            }
        }
    }
    visited.iter().all(|&v| v)
}

// ── Easy 4: Shortest Path Unweighted ──────────────────────────────────

struct ShortestPathUnweighted;
struct SPTest {
    n: usize,
    edges: Vec<(usize, usize)>,
    src: usize,
    dst: usize,
}

impl Problem for ShortestPathUnweighted {
    fn id(&self) -> &str {
        "graph_bfs_dfs_shortest_path_unweighted"
    }
    fn name(&self) -> &str {
        "Shortest Path (Unweighted)"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Find the shortest path length between two nodes in an unweighted\n\
         undirected graph using BFS.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>, src: usize, dst: usize)\n\
         Output: i32 — the shortest path length, or -1 if unreachable.\n\n\
         Constraints:\n\
         - 1 <= n <= 200\n\
         - 0 <= src, dst < n"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let n = rng.random_range(2..=15);
                let extra = rng.random_range(0..=n);
                let edges = if i % 3 == 0 {
                    random_undirected_edges(&mut rng, n, extra)
                } else {
                    random_connected_edges(&mut rng, n, extra)
                };
                let src = rng.random_range(0..n);
                let dst = rng.random_range(0..n);
                TestCase {
                    data: Box::new(SPTest { n, edges, src, dst }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SPTest>().unwrap();
        let expected = ref_shortest_path_unweighted(t.n, &t.edges, t.src, t.dst);
        let actual = solutions::shortest_path_unweighted(t.n, &t.edges, t.src, t.dst);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!(
                "n={}, edges={:?}, src={}, dst={}",
                t.n, t.edges, t.src, t.dst
            ),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_shortest_path_unweighted(n: usize, edges: &[(usize, usize)], src: usize, dst: usize) -> i32 {
    if src == dst {
        return 0;
    }
    let adj = build_adj_undirected(n, edges);
    let mut dist = vec![-1i32; n];
    dist[src] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(src);
    while let Some(u) = queue.pop_front() {
        for &v in &adj[u] {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                if v == dst {
                    return dist[v];
                }
                queue.push_back(v);
            }
        }
    }
    -1
}

// ── Easy 5: Find Path ─────────────────────────────────────────────────

struct FindPath;
struct FindPathTest {
    n: usize,
    edges: Vec<(usize, usize)>,
    src: usize,
    dst: usize,
}

impl Problem for FindPath {
    fn id(&self) -> &str {
        "graph_bfs_dfs_find_path"
    }
    fn name(&self) -> &str {
        "Find Any Path"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Find any path from src to dst in an undirected graph.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>, src: usize, dst: usize)\n\
         Output: Vec<usize> — a path from src to dst (inclusive), or empty vec if none.\n\n\
         The path must be valid: consecutive nodes must share an edge, the first\n\
         element must be src, and the last must be dst.\n\n\
         Constraints:\n\
         - 1 <= n <= 200\n\
         - 0 <= src, dst < n"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let n = rng.random_range(2..=15);
                let extra = rng.random_range(0..=n);
                let edges = if i % 3 == 0 {
                    random_undirected_edges(&mut rng, n, extra)
                } else {
                    random_connected_edges(&mut rng, n, extra)
                };
                let src = rng.random_range(0..n);
                let dst = rng.random_range(0..n);
                TestCase {
                    data: Box::new(FindPathTest { n, edges, src, dst }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FindPathTest>().unwrap();
        let adj = build_adj_undirected(t.n, &t.edges);
        let actual = solutions::find_path(t.n, &t.edges, t.src, t.dst);

        // Check if a path exists using BFS
        let path_exists = ref_shortest_path_unweighted(t.n, &t.edges, t.src, t.dst) >= 0;

        if !path_exists {
            let is_correct = actual.is_empty();
            return SolutionResult {
                is_correct,
                input_description: format!(
                    "n={}, edges={:?}, src={}, dst={}",
                    t.n, t.edges, t.src, t.dst
                ),
                expected: "[] (no path exists)".to_string(),
                actual: format!("{actual:?}"),
            };
        }

        // Validate the path
        let valid = if actual.is_empty() || actual[0] != t.src || actual[actual.len() - 1] != t.dst
        {
            false
        } else {
            actual.windows(2).all(|w| adj[w[0]].contains(&w[1]))
        };

        SolutionResult {
            is_correct: valid,
            input_description: format!(
                "n={}, edges={:?}, src={}, dst={}",
                t.n, t.edges, t.src, t.dst
            ),
            expected: "(any valid path)".to_string(),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 6: Clone Graph ─────────────────────────────────────────────

struct CloneGraph;
struct CloneGraphTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for CloneGraph {
    fn id(&self) -> &str {
        "graph_bfs_dfs_clone_graph"
    }
    fn name(&self) -> &str {
        "Clone Graph"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Clone (deep copy) an undirected graph.\n\n\
         Given a graph as (n, edges), return (n, edges) representing the same\n\
         graph structure. The output edges should be sorted: each edge as\n\
         (min, max), and the overall list sorted lexicographically.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: (usize, Vec<(usize, usize)>) — (node count, sorted edge list).\n\n\
         Constraints:\n\
         - 1 <= n <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let extra = rng.random_range(0..=n);
                let edges = random_connected_edges(&mut rng, n, extra);
                TestCase {
                    data: Box::new(CloneGraphTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CloneGraphTest>().unwrap();
        let mut expected_edges: Vec<(usize, usize)> =
            t.edges.iter().map(|&(u, v)| (u.min(v), u.max(v))).collect();
        expected_edges.sort();
        expected_edges.dedup();

        let (actual_n, actual_edges) = solutions::clone_graph(t.n, &t.edges);
        let mut actual_normalized: Vec<(usize, usize)> = actual_edges
            .iter()
            .map(|&(u, v)| (u.min(v), u.max(v)))
            .collect();
        actual_normalized.sort();
        actual_normalized.dedup();

        SolutionResult {
            is_correct: actual_n == t.n && actual_normalized == expected_edges,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("({}, {:?})", t.n, expected_edges),
            actual: format!("({actual_n}, {actual_normalized:?})"),
        }
    }
}

// ── Medium 7: Course Schedule ─────────────────────────────────────────

struct CourseSchedule;
struct CourseScheduleTest {
    n: usize,
    prerequisites: Vec<(usize, usize)>,
}

impl Problem for CourseSchedule {
    fn id(&self) -> &str {
        "graph_bfs_dfs_course_schedule"
    }
    fn name(&self) -> &str {
        "Course Schedule"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "There are n courses labeled 0..n-1. Some have prerequisites.\n\
         prerequisites[i] = (a, b) means you must take course b before course a.\n\n\
         Return true if it is possible to finish all courses (i.e., no cycle\n\
         in the dependency graph).\n\n\
         Input: (n: usize, prerequisites: Vec<(usize, usize)>)\n\
         Output: bool — true if all courses can be finished.\n\n\
         Constraints:\n\
         - 1 <= n <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let n = rng.random_range(2..=12);
                let edge_count = rng.random_range(1..=n * 2);
                let prerequisites = if i % 2 == 0 {
                    // DAG: no cycle
                    random_dag_edges(&mut rng, n, edge_count)
                } else {
                    // Might have cycles
                    random_directed_edges(&mut rng, n, edge_count)
                };
                TestCase {
                    data: Box::new(CourseScheduleTest { n, prerequisites }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CourseScheduleTest>().unwrap();
        let expected = ref_can_finish(t.n, &t.prerequisites);
        let actual = solutions::can_finish(t.n, &t.prerequisites);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, prerequisites={:?}", t.n, t.prerequisites),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_can_finish(n: usize, prerequisites: &[(usize, usize)]) -> bool {
    // Kahn's algorithm (BFS topological sort)
    let mut adj = vec![vec![]; n];
    let mut indegree = vec![0usize; n];
    for &(a, b) in prerequisites {
        adj[b].push(a);
        indegree[a] += 1;
    }
    let mut queue: VecDeque<usize> = (0..n).filter(|&i| indegree[i] == 0).collect();
    let mut count = 0;
    while let Some(u) = queue.pop_front() {
        count += 1;
        for &v in &adj[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    count == n
}

// ── Medium 8: Course Schedule II ──────────────────────────────────────

struct CourseScheduleII;
struct CourseScheduleIITest {
    n: usize,
    prerequisites: Vec<(usize, usize)>,
}

impl Problem for CourseScheduleII {
    fn id(&self) -> &str {
        "graph_bfs_dfs_course_schedule_ii"
    }
    fn name(&self) -> &str {
        "Course Schedule II (Topological Sort)"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Return a valid course ordering (topological sort) given prerequisites.\n\
         prerequisites[i] = (a, b) means you must take course b before course a.\n\n\
         Return an empty vector if no valid ordering exists (cycle).\n\n\
         Input: (n: usize, prerequisites: Vec<(usize, usize)>)\n\
         Output: Vec<usize> — a valid topological ordering, or empty if impossible.\n\n\
         Constraints:\n\
         - 1 <= n <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let n = rng.random_range(2..=12);
                let edge_count = rng.random_range(1..=n * 2);
                let prerequisites = if i % 3 == 0 {
                    random_directed_edges(&mut rng, n, edge_count)
                } else {
                    random_dag_edges(&mut rng, n, edge_count)
                };
                TestCase {
                    data: Box::new(CourseScheduleIITest { n, prerequisites }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CourseScheduleIITest>().unwrap();
        let can = ref_can_finish(t.n, &t.prerequisites);
        let actual = solutions::course_order(t.n, &t.prerequisites);

        if !can {
            let is_correct = actual.is_empty();
            return SolutionResult {
                is_correct,
                input_description: format!("n={}, prerequisites={:?}", t.n, t.prerequisites),
                expected: "[] (cycle exists)".to_string(),
                actual: format!("{actual:?}"),
            };
        }

        // Validate: correct length, all nodes, respects prerequisites
        let valid = if actual.len() != t.n {
            false
        } else {
            let mut pos = vec![0usize; t.n];
            for (i, &node) in actual.iter().enumerate() {
                pos[node] = i;
            }
            let all_present: HashSet<usize> = actual.iter().copied().collect();
            all_present.len() == t.n && t.prerequisites.iter().all(|&(a, b)| pos[b] < pos[a])
        };

        SolutionResult {
            is_correct: valid,
            input_description: format!("n={}, prerequisites={:?}", t.n, t.prerequisites),
            expected: "(any valid topological order)".to_string(),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 9: Number of Islands ───────────────────────────────────────

struct NumberOfIslands;
struct IslandTest {
    grid: Vec<Vec<i32>>,
}

impl Problem for NumberOfIslands {
    fn id(&self) -> &str {
        "graph_bfs_dfs_number_of_islands"
    }
    fn name(&self) -> &str {
        "Number of Islands"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a 2D grid where 1 represents land and 0 represents water, count\n\
         the number of islands. An island is a group of 1s connected\n\
         horizontally or vertically (not diagonally).\n\n\
         Input: Vec<Vec<i32>> — the grid (0s and 1s).\n\
         Output: i32 — the number of islands.\n\n\
         Constraints:\n\
         - 1 <= rows, cols <= 50"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(2..=10);
                let cols = rng.random_range(2..=10);
                let grid: Vec<Vec<i32>> = (0..rows)
                    .map(|_| {
                        (0..cols)
                            .map(|_| if rng.random_range(0..3) == 0 { 0 } else { 1 })
                            .collect()
                    })
                    .collect();
                TestCase {
                    data: Box::new(IslandTest { grid }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IslandTest>().unwrap();
        let expected = ref_num_islands(&t.grid);
        let actual = solutions::num_islands(&t.grid);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_num_islands(grid: &[Vec<i32>]) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 1 && !visited[r][c] {
                count += 1;
                let mut queue = VecDeque::new();
                queue.push_back((r, c));
                visited[r][c] = true;
                while let Some((cr, cc)) = queue.pop_front() {
                    for (dr, dc) in &[(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
                        let nr = cr as i32 + dr;
                        let nc = cc as i32 + dc;
                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            let (nr, nc) = (nr as usize, nc as usize);
                            if grid[nr][nc] == 1 && !visited[nr][nc] {
                                visited[nr][nc] = true;
                                queue.push_back((nr, nc));
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

// ── Medium 10: Word Ladder ────────────────────────────────────────────

struct WordLadder;
struct WordLadderTest {
    begin: String,
    end: String,
    word_list: Vec<String>,
}

impl Problem for WordLadder {
    fn id(&self) -> &str {
        "graph_bfs_dfs_word_ladder"
    }
    fn name(&self) -> &str {
        "Word Ladder"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a begin word, an end word, and a word list, find the length of the\n\
         shortest transformation sequence from begin to end, where:\n\
         - Only one letter can be changed at a time.\n\
         - Each transformed word must exist in the word list.\n\n\
         Return the number of words in the shortest sequence (including begin\n\
         and end). Return 0 if no such sequence exists.\n\n\
         Input: (begin: String, end: String, word_list: Vec<String>)\n\
         Output: i32 — the length of the shortest transformation sequence.\n\n\
         Constraints:\n\
         - All words have the same length.\n\
         - 1 <= word.len() <= 5\n\
         - 1 <= word_list.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let word_len = rng.random_range(3..=4);
                let list_size = rng.random_range(5..=20);
                let mut word_list: Vec<String> = Vec::new();
                let mut seen = HashSet::new();
                while word_list.len() < list_size {
                    let w: String = (0..word_len)
                        .map(|_| (b'a' + rng.random_range(0..4u8)) as char)
                        .collect();
                    if seen.insert(w.clone()) {
                        word_list.push(w);
                    }
                }
                let begin: String = (0..word_len)
                    .map(|_| (b'a' + rng.random_range(0..4u8)) as char)
                    .collect();
                let end = word_list[rng.random_range(0..word_list.len())].clone();
                TestCase {
                    data: Box::new(WordLadderTest {
                        begin,
                        end,
                        word_list,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WordLadderTest>().unwrap();
        let expected = ref_word_ladder(&t.begin, &t.end, &t.word_list);
        let actual = solutions::word_ladder(&t.begin, &t.end, &t.word_list);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!(
                "begin={:?}, end={:?}, word_list={:?}",
                t.begin, t.end, t.word_list
            ),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_word_ladder(begin: &str, end: &str, word_list: &[String]) -> i32 {
    let word_set: HashSet<&str> = word_list.iter().map(|s| s.as_str()).collect();
    if !word_set.contains(end) {
        return 0;
    }
    let mut visited = HashSet::new();
    visited.insert(begin.to_string());
    let mut queue = VecDeque::new();
    queue.push_back((begin.to_string(), 1));
    while let Some((word, depth)) = queue.pop_front() {
        if word == end {
            return depth;
        }
        let chars: Vec<char> = word.chars().collect();
        for i in 0..chars.len() {
            for c in b'a'..=b'z' {
                let c = c as char;
                if c == chars[i] {
                    continue;
                }
                let mut next = chars.clone();
                next[i] = c;
                let next_word: String = next.into_iter().collect();
                if word_set.contains(next_word.as_str()) && !visited.contains(&next_word) {
                    visited.insert(next_word.clone());
                    queue.push_back((next_word, depth + 1));
                }
            }
        }
    }
    0
}

// ── Hard 11: Word Ladder II ───────────────────────────────────────────

struct WordLadderII;
struct WordLadderIITest {
    begin: String,
    end: String,
    word_list: Vec<String>,
}

impl Problem for WordLadderII {
    fn id(&self) -> &str {
        "graph_bfs_dfs_word_ladder_ii"
    }
    fn name(&self) -> &str {
        "Word Ladder II (All Shortest Paths)"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find ALL shortest transformation sequences from begin to end.\n\n\
         Each sequence changes one letter at a time and each intermediate word\n\
         must be in the word list.\n\n\
         Return all shortest sequences, sorted lexicographically.\n\
         Return an empty vector if no transformation is possible.\n\n\
         Input: (begin: String, end: String, word_list: Vec<String>)\n\
         Output: Vec<Vec<String>> — all shortest sequences, sorted.\n\n\
         Constraints:\n\
         - 1 <= word.len() <= 5\n\
         - 1 <= word_list.len() <= 50"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let word_len = rng.random_range(3..=4);
                let list_size = rng.random_range(5..=15);
                let mut word_list: Vec<String> = Vec::new();
                let mut seen = HashSet::new();
                while word_list.len() < list_size {
                    let w: String = (0..word_len)
                        .map(|_| (b'a' + rng.random_range(0..3u8)) as char)
                        .collect();
                    if seen.insert(w.clone()) {
                        word_list.push(w);
                    }
                }
                let begin: String = (0..word_len)
                    .map(|_| (b'a' + rng.random_range(0..3u8)) as char)
                    .collect();
                let end = word_list[rng.random_range(0..word_list.len())].clone();
                TestCase {
                    data: Box::new(WordLadderIITest {
                        begin,
                        end,
                        word_list,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WordLadderIITest>().unwrap();
        let expected = ref_word_ladder_ii(&t.begin, &t.end, &t.word_list);
        let actual = solutions::word_ladder_ii(&t.begin, &t.end, &t.word_list);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!(
                "begin={:?}, end={:?}, word_list={:?}",
                t.begin, t.end, t.word_list
            ),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_word_ladder_ii(begin: &str, end: &str, word_list: &[String]) -> Vec<Vec<String>> {
    let word_set: HashSet<String> = word_list.iter().cloned().collect();
    if !word_set.contains(end) {
        return vec![];
    }

    // BFS to find shortest distance from begin to each word
    let mut dist: HashMap<String, usize> = HashMap::new();
    dist.insert(begin.to_string(), 0);
    let mut queue = VecDeque::new();
    queue.push_back(begin.to_string());
    let mut found = false;

    while let Some(word) = queue.pop_front() {
        let d = dist[&word];
        if word == end {
            found = true;
        }
        let chars: Vec<char> = word.chars().collect();
        for i in 0..chars.len() {
            for c in b'a'..=b'z' {
                let c = c as char;
                if c == chars[i] {
                    continue;
                }
                let mut next = chars.clone();
                next[i] = c;
                let next_word: String = next.into_iter().collect();
                if word_set.contains(&next_word) && !dist.contains_key(&next_word) {
                    dist.insert(next_word.clone(), d + 1);
                    queue.push_back(next_word);
                }
            }
        }
    }

    if !found {
        return vec![];
    }

    // DFS backtracking to find all shortest paths
    let mut result = Vec::new();
    let mut path = vec![begin.to_string()];

    fn dfs(
        word: &str,
        end: &str,
        dist: &HashMap<String, usize>,
        _word_set: &HashSet<String>,
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if word == end {
            result.push(path.clone());
            return;
        }
        let d = dist[word];
        let chars: Vec<char> = word.chars().collect();
        for i in 0..chars.len() {
            for c in b'a'..=b'z' {
                let c = c as char;
                if c == chars[i] {
                    continue;
                }
                let mut next = chars.clone();
                next[i] = c;
                let next_word: String = next.into_iter().collect();
                if let Some(&nd) = dist.get(&next_word) {
                    if nd == d + 1 {
                        path.push(next_word.clone());
                        dfs(&next_word, end, dist, _word_set, path, result);
                        path.pop();
                    }
                }
            }
        }
    }

    dfs(begin, end, &dist, &word_set, &mut path, &mut result);
    result.sort();
    result
}

// ── Hard 12: Surrounded Regions ───────────────────────────────────────

struct SurroundedRegions;
struct SurroundedTest {
    board: Vec<Vec<char>>,
}

impl Problem for SurroundedRegions {
    fn id(&self) -> &str {
        "graph_bfs_dfs_surrounded_regions"
    }
    fn name(&self) -> &str {
        "Surrounded Regions"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a 2D board containing 'X' and 'O', capture all regions that are\n\
         completely surrounded by 'X'. A region is captured by flipping all 'O's\n\
         to 'X's. An 'O' on the border (or connected to a border 'O') is NOT\n\
         captured.\n\n\
         Input: Vec<Vec<char>> — the board.\n\
         Output: Vec<Vec<char>> — the board after capturing surrounded regions.\n\n\
         Constraints:\n\
         - 1 <= rows, cols <= 50"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(3..=8);
                let cols = rng.random_range(3..=8);
                let board: Vec<Vec<char>> = (0..rows)
                    .map(|_| {
                        (0..cols)
                            .map(|_| {
                                if rng.random_range(0..3) == 0 {
                                    'O'
                                } else {
                                    'X'
                                }
                            })
                            .collect()
                    })
                    .collect();
                TestCase {
                    data: Box::new(SurroundedTest { board }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SurroundedTest>().unwrap();
        let expected = ref_surrounded_regions(&t.board);
        let actual = solutions::solve_surrounded(&t.board);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("board={:?}", t.board),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_surrounded_regions(board: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = board.len();
    let cols = board[0].len();
    let mut result: Vec<Vec<char>> = board.to_vec();

    // Mark all 'O's connected to the border
    let mut safe = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();

    for r in 0..rows {
        for c in 0..cols {
            if (r == 0 || r == rows - 1 || c == 0 || c == cols - 1) && board[r][c] == 'O' {
                safe[r][c] = true;
                queue.push_back((r, c));
            }
        }
    }

    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in &[(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if board[nr][nc] == 'O' && !safe[nr][nc] {
                    safe[nr][nc] = true;
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    for r in 0..rows {
        for c in 0..cols {
            if board[r][c] == 'O' && !safe[r][c] {
                result[r][c] = 'X';
            }
        }
    }
    result
}

// ── Hard 13: Pacific Atlantic Water Flow ──────────────────────────────

struct PacificAtlantic;
struct PacificAtlanticTest {
    heights: Vec<Vec<i32>>,
}

impl Problem for PacificAtlantic {
    fn id(&self) -> &str {
        "graph_bfs_dfs_pacific_atlantic"
    }
    fn name(&self) -> &str {
        "Pacific Atlantic Water Flow"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an m x n grid of heights, find all cells from which water can flow\n\
         to both the Pacific ocean (top and left edges) and the Atlantic ocean\n\
         (bottom and right edges). Water flows from a cell to a neighbor with\n\
         equal or lower height.\n\n\
         Input: Vec<Vec<i32>> — the height grid.\n\
         Output: Vec<(usize, usize)> — cells that flow to both oceans, sorted\n\
         lexicographically by (row, col).\n\n\
         Constraints:\n\
         - 1 <= rows, cols <= 50\n\
         - 0 <= heights[i][j] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(2..=8);
                let cols = rng.random_range(2..=8);
                let heights: Vec<Vec<i32>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(0..=20)).collect())
                    .collect();
                TestCase {
                    data: Box::new(PacificAtlanticTest { heights }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PacificAtlanticTest>().unwrap();
        let expected = ref_pacific_atlantic(&t.heights);
        let actual = solutions::pacific_atlantic(&t.heights);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("heights={:?}", t.heights),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_pacific_atlantic(heights: &[Vec<i32>]) -> Vec<(usize, usize)> {
    let rows = heights.len();
    let cols = heights[0].len();

    fn bfs(heights: &[Vec<i32>], starts: Vec<(usize, usize)>) -> Vec<Vec<bool>> {
        let rows = heights.len();
        let cols = heights[0].len();
        let mut reachable = vec![vec![false; cols]; rows];
        let mut queue = VecDeque::new();
        for (r, c) in starts {
            reachable[r][c] = true;
            queue.push_back((r, c));
        }
        while let Some((r, c)) = queue.pop_front() {
            for (dr, dc) in &[(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if !reachable[nr][nc] && heights[nr][nc] >= heights[r][c] {
                        reachable[nr][nc] = true;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }
        reachable
    }

    // Pacific: top row + left column
    let mut pacific_starts = Vec::new();
    for c in 0..cols {
        pacific_starts.push((0, c));
    }
    for r in 1..rows {
        pacific_starts.push((r, 0));
    }
    let pacific = bfs(heights, pacific_starts);

    // Atlantic: bottom row + right column
    let mut atlantic_starts = Vec::new();
    for c in 0..cols {
        atlantic_starts.push((rows - 1, c));
    }
    for r in 0..rows - 1 {
        atlantic_starts.push((r, cols - 1));
    }
    let atlantic = bfs(heights, atlantic_starts);

    let mut result = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if pacific[r][c] && atlantic[r][c] {
                result.push((r, c));
            }
        }
    }
    result
}

// ── Hard 14: All Paths in DAG ─────────────────────────────────────────

struct AllPaths;
struct AllPathsTest {
    graph: Vec<Vec<usize>>,
}

impl Problem for AllPaths {
    fn id(&self) -> &str {
        "graph_bfs_dfs_all_paths"
    }
    fn name(&self) -> &str {
        "All Paths from Source to Target"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a DAG represented as an adjacency list, find all paths from node 0\n\
         to node n-1.\n\n\
         Input: Vec<Vec<usize>> — adjacency list (graph[i] = list of successors of i).\n\
         Output: Vec<Vec<usize>> — all paths from 0 to n-1, sorted lexicographically.\n\n\
         Constraints:\n\
         - 2 <= n <= 15\n\
         - The graph is a DAG (no cycles)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=8);
                let mut graph = vec![vec![]; n];
                // Ensure at least one path from 0 to n-1
                // Create a direct chain path
                let mut chain: Vec<usize> = (0..n).collect();
                // Shuffle internal nodes
                for i in (2..n).rev() {
                    let j = rng.random_range(1..=i);
                    chain.swap(i, j);
                }
                // Keep 0 at start
                if chain[0] != 0 {
                    let pos = chain.iter().position(|&x| x == 0).unwrap();
                    chain.swap(0, pos);
                }
                // Keep n-1 at end
                if chain[n - 1] != n - 1 {
                    let pos = chain.iter().position(|&x| x == n - 1).unwrap();
                    chain.swap(n - 1, pos);
                }
                // Add chain edges
                let mut seen = HashSet::new();
                for w in chain.windows(2) {
                    if w[0] != w[1] && seen.insert((w[0], w[1])) {
                        graph[w[0]].push(w[1]);
                    }
                }
                // Add extra DAG edges (forward only relative to topological order)
                let extra = rng.random_range(0..=n * 2);
                for _ in 0..extra {
                    let u = rng.random_range(0..n);
                    let v = rng.random_range(0..n);
                    if u < v && seen.insert((u, v)) {
                        graph[u].push(v);
                    }
                }
                for row in &mut graph {
                    row.sort();
                }
                TestCase {
                    data: Box::new(AllPathsTest { graph }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AllPathsTest>().unwrap();
        let expected = ref_all_paths(&t.graph);
        let actual = solutions::all_paths(&t.graph);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("graph={:?}", t.graph),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_all_paths(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = graph.len();
    let target = n - 1;
    let mut result = Vec::new();
    let mut path = vec![0];
    fn dfs(
        node: usize,
        target: usize,
        graph: &[Vec<usize>],
        path: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if node == target {
            result.push(path.clone());
            return;
        }
        for &next in &graph[node] {
            path.push(next);
            dfs(next, target, graph, path, result);
            path.pop();
        }
    }
    dfs(0, target, graph, &mut path, &mut result);
    result.sort();
    result
}

// ── Hard 15: Shortest Path in Binary Matrix ───────────────────────────

struct ShortestPathBinaryMatrix;
struct BinaryMatrixTest {
    grid: Vec<Vec<i32>>,
}

impl Problem for ShortestPathBinaryMatrix {
    fn id(&self) -> &str {
        "graph_bfs_dfs_shortest_path_binary_matrix"
    }
    fn name(&self) -> &str {
        "Shortest Path in Binary Matrix"
    }
    fn topic(&self) -> &str {
        "graph_bfs_dfs"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an n x n binary grid, find the length of the shortest clear path\n\
         from top-left (0,0) to bottom-right (n-1,n-1). A clear path consists\n\
         of cells with value 0, and you can move in 8 directions (horizontal,\n\
         vertical, and diagonal). The path length is the number of cells visited.\n\n\
         Return -1 if no such path exists.\n\n\
         Input: Vec<Vec<i32>> — binary grid (0 = open, 1 = blocked).\n\
         Output: i32 — length of shortest path, or -1.\n\n\
         Constraints:\n\
         - 1 <= n <= 50\n\
         - grid[0][0] and grid[n-1][n-1] may be blocked."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=10);
                let mut grid: Vec<Vec<i32>> = (0..n)
                    .map(|_| {
                        (0..n)
                            .map(|_| if rng.random_range(0..4) == 0 { 1 } else { 0 })
                            .collect()
                    })
                    .collect();
                // Ensure start and end are open sometimes
                if rng.random_range(0..3) != 0 {
                    grid[0][0] = 0;
                    grid[n - 1][n - 1] = 0;
                }
                TestCase {
                    data: Box::new(BinaryMatrixTest { grid }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BinaryMatrixTest>().unwrap();
        let expected = ref_shortest_path_binary_matrix(&t.grid);
        let actual = solutions::shortest_path_binary_matrix(&t.grid);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_shortest_path_binary_matrix(grid: &[Vec<i32>]) -> i32 {
    let n = grid.len();
    if grid[0][0] != 0 || grid[n - 1][n - 1] != 0 {
        return -1;
    }
    if n == 1 {
        return 1;
    }

    let mut visited = vec![vec![false; n]; n];
    visited[0][0] = true;
    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0usize, 1i32));

    let dirs: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    while let Some((r, c, dist)) = queue.pop_front() {
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] == 0 && !visited[nr][nc] {
                    if nr == n - 1 && nc == n - 1 {
                        return dist + 1;
                    }
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc, dist + 1));
                }
            }
        }
    }
    -1
}
