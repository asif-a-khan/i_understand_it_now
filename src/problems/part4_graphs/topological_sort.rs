use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part4_graphs::topological_sort as solutions;
use crate::tracker::{track_slice, OperationLog, Tracked, TrackedGraph, TrackedWeightedGraph};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(TopoSortBasic),
        Box::new(TopoSortCanFinish),
        Box::new(TopoSortFindOrder),
        Box::new(TopoSortIsDag),
        Box::new(TopoSortKahnBfs),
        Box::new(TopoSortParallelCourses),
        Box::new(TopoSortAllAncestors),
        Box::new(TopoSortLongestPathDag),
        Box::new(TopoSortSequenceReconstruction),
        Box::new(TopoSortBuildOrder),
        Box::new(TopoSortAlienDictionary),
        Box::new(TopoSortMinimumHeightTrees),
        Box::new(TopoSortLongestIncreasingPath),
        Box::new(TopoSortCriticalConnections),
        Box::new(TopoSortSortItemsByGroups),
    ]
}

// ── Helpers for test generation ─────────────────────────────────────────

/// Generate a random DAG as edge list. Nodes 0..n, edges only from smaller to larger index.
fn gen_dag_edges(rng: &mut impl Rng, n: usize, edge_count: usize) -> Vec<(usize, usize)> {
    use std::collections::HashSet;
    let mut edges = HashSet::new();
    let mut count = 0;
    let max_attempts = edge_count * 10;
    let mut attempts = 0;
    while count < edge_count && attempts < max_attempts {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u < v && edges.insert((u, v)) {
            count += 1;
        }
        attempts += 1;
    }
    edges.into_iter().collect()
}

/// Generate edges that form a tree (for MHT problem).
fn gen_tree_edges(rng: &mut impl Rng, n: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for i in 1..n {
        let parent = rng.random_range(0..i);
        edges.push((parent, i));
    }
    edges
}

// ── Easy 1: Topological Sort (basic) ────────────────────────────────────

struct TopoSortBasic;

struct TopoSortBasicTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for TopoSortBasic {
    fn id(&self) -> &str {
        "topo_sort_basic"
    }
    fn name(&self) -> &str {
        "Topological Sort"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a DAG with `n` nodes (0..n-1) and a list of directed edges (u, v) meaning \
         u must come before v, return a valid topological ordering of all nodes.\n\n\
         If multiple valid orderings exist, return any one.\n\n\
         Constraints:\n\
         - 1 <= n <= 1000\n\
         - 0 <= edges.len() <= n*(n-1)/2\n\
         - The graph is a valid DAG (no cycles)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let edge_count = rng.random_range(1..=(n * (n - 1) / 2).max(1));
                let edges = gen_dag_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(TopoSortBasicTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TopoSortBasicTest>().unwrap();
        let actual = {
            let shared_log = Rc::new(RefCell::new(OperationLog::new()));
            let graph = TrackedGraph::new(t.n, &t.edges, true, shared_log.clone());
            let result = solutions::topo_sort_basic(&graph);
            for op in shared_log.borrow().operations() {
                log.record(op.clone());
            }
            result
        };
        let valid = ref_validate_topo_order(t.n, &t.edges, &actual);
        SolutionResult {
            is_correct: valid,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: "any valid topological ordering".to_string(),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_validate_topo_order(n: usize, edges: &[(usize, usize)], order: &[usize]) -> bool {
    if order.len() != n {
        return false;
    }
    let mut pos = vec![0usize; n];
    let mut seen = vec![false; n];
    for (i, &node) in order.iter().enumerate() {
        if node >= n || seen[node] {
            return false;
        }
        seen[node] = true;
        pos[node] = i;
    }
    for &(u, v) in edges {
        if pos[u] >= pos[v] {
            return false;
        }
    }
    true
}

// ── Easy 2: Can Finish Courses ──────────────────────────────────────────

struct TopoSortCanFinish;

struct CanFinishTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for TopoSortCanFinish {
    fn id(&self) -> &str {
        "topo_sort_can_finish"
    }
    fn name(&self) -> &str {
        "Course Schedule (Can Finish)"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "There are `n` courses labeled 0 to n-1. Some courses have prerequisites given as \
         edges (a, b) meaning you must take course `a` before course `b`.\n\n\
         Return true if you can finish all courses (i.e., there is no cycle).\n\n\
         Constraints:\n\
         - 1 <= n <= 2000\n\
         - 0 <= prerequisites.len() <= 5000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=12);
                let edge_count = rng.random_range(1..=(n * 2).min(n * (n - 1) / 2).max(1));
                let edges = if rng.random_range(0..3) == 0 {
                    // Possibly introduce a cycle
                    let mut e = gen_dag_edges(&mut rng, n, edge_count.saturating_sub(1));
                    if n >= 2 {
                        let u = rng.random_range(1..n);
                        let v = rng.random_range(0..u);
                        e.push((u, v)); // back edge = cycle
                    }
                    e
                } else {
                    gen_dag_edges(&mut rng, n, edge_count)
                };
                TestCase {
                    data: Box::new(CanFinishTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CanFinishTest>().unwrap();
        let expected = ref_can_finish(t.n, &t.edges);
        let actual = {
            let shared_log = Rc::new(RefCell::new(OperationLog::new()));
            let graph = TrackedGraph::new(t.n, &t.edges, true, shared_log.clone());
            let result = solutions::can_finish(&graph);
            for op in shared_log.borrow().operations() {
                log.record(op.clone());
            }
            result
        };
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_can_finish(n: usize, edges: &[(usize, usize)]) -> bool {
    let mut adj = vec![vec![]; n];
    let mut indegree = vec![0usize; n];
    for &(u, v) in edges {
        adj[u].push(v);
        indegree[v] += 1;
    }
    let mut queue = std::collections::VecDeque::new();
    for (i, &deg) in indegree.iter().enumerate() {
        if deg == 0 {
            queue.push_back(i);
        }
    }
    let mut count = 0;
    while let Some(node) = queue.pop_front() {
        count += 1;
        for &next in &adj[node] {
            indegree[next] -= 1;
            if indegree[next] == 0 {
                queue.push_back(next);
            }
        }
    }
    count == n
}

// ── Easy 3: Course Schedule Order ───────────────────────────────────────

struct TopoSortFindOrder;

struct FindOrderTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for TopoSortFindOrder {
    fn id(&self) -> &str {
        "topo_sort_find_order"
    }
    fn name(&self) -> &str {
        "Course Schedule II (Find Order)"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "There are `n` courses (0..n-1) with prerequisite edges (a, b) meaning course `a` \
         must be taken before course `b`. Return an ordering of courses to finish all of \
         them. If impossible (cycle), return an empty vector.\n\n\
         Constraints:\n\
         - 1 <= n <= 2000\n\
         - 0 <= prerequisites.len() <= 5000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=12);
                let edge_count = rng.random_range(1..=(n * 2).min(n * (n - 1) / 2).max(1));
                let edges = if rng.random_range(0..4) == 0 {
                    let mut e = gen_dag_edges(&mut rng, n, edge_count.saturating_sub(1));
                    if n >= 2 {
                        let u = rng.random_range(1..n);
                        let v = rng.random_range(0..u);
                        e.push((u, v));
                    }
                    e
                } else {
                    gen_dag_edges(&mut rng, n, edge_count)
                };
                TestCase {
                    data: Box::new(FindOrderTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FindOrderTest>().unwrap();
        let can = ref_can_finish(t.n, &t.edges);
        let actual = {
            let shared_log = Rc::new(RefCell::new(OperationLog::new()));
            let graph = TrackedGraph::new(t.n, &t.edges, true, shared_log.clone());
            let result = solutions::find_order(&graph);
            for op in shared_log.borrow().operations() {
                log.record(op.clone());
            }
            result
        };
        let is_correct = if can {
            ref_validate_topo_order(t.n, &t.edges, &actual)
        } else {
            actual.is_empty()
        };
        SolutionResult {
            is_correct,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: if can {
                "any valid topological ordering".to_string()
            } else {
                "[] (cycle exists)".to_string()
            },
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 4: Is DAG ─────────────────────────────────────────────────────

struct TopoSortIsDag;

struct IsDagTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for TopoSortIsDag {
    fn id(&self) -> &str {
        "topo_sort_is_dag"
    }
    fn name(&self) -> &str {
        "Check if Directed Graph is DAG"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a directed graph with `n` nodes and edges (u, v), determine whether the \
         graph is a DAG (directed acyclic graph). Return true if there are no cycles.\n\n\
         Constraints:\n\
         - 1 <= n <= 2000\n\
         - 0 <= edges.len() <= 5000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=12);
                let edge_count = rng.random_range(1..=(n * 2).min(n * (n - 1) / 2).max(1));
                let edges = if rng.random_range(0..2) == 0 {
                    let mut e = gen_dag_edges(&mut rng, n, edge_count.saturating_sub(1));
                    if n >= 2 {
                        let u = rng.random_range(1..n);
                        let v = rng.random_range(0..u);
                        e.push((u, v));
                    }
                    e
                } else {
                    gen_dag_edges(&mut rng, n, edge_count)
                };
                TestCase {
                    data: Box::new(IsDagTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IsDagTest>().unwrap();
        let expected = ref_can_finish(t.n, &t.edges); // DAG = can finish
        let actual = {
            let shared_log = Rc::new(RefCell::new(OperationLog::new()));
            let graph = TrackedGraph::new(t.n, &t.edges, true, shared_log.clone());
            let result = solutions::is_dag(&graph);
            for op in shared_log.borrow().operations() {
                log.record(op.clone());
            }
            result
        };
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 5: Kahn's BFS Topological Sort ────────────────────────────────

struct TopoSortKahnBfs;

struct KahnBfsTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for TopoSortKahnBfs {
    fn id(&self) -> &str {
        "topo_sort_kahn_bfs"
    }
    fn name(&self) -> &str {
        "Topological Sort (Kahn's BFS)"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Implement topological sort using Kahn's algorithm (BFS with in-degree counting). \
         Given a DAG with `n` nodes and edges (u, v), return a valid topological ordering.\n\n\
         Constraints:\n\
         - 1 <= n <= 1000\n\
         - The graph is guaranteed to be a DAG"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let edge_count = rng.random_range(1..=(n * (n - 1) / 2).max(1));
                let edges = gen_dag_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(KahnBfsTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KahnBfsTest>().unwrap();
        let actual = {
            let shared_log = Rc::new(RefCell::new(OperationLog::new()));
            let graph = TrackedGraph::new(t.n, &t.edges, true, shared_log.clone());
            let result = solutions::kahn_bfs(&graph);
            for op in shared_log.borrow().operations() {
                log.record(op.clone());
            }
            result
        };
        let valid = ref_validate_topo_order(t.n, &t.edges, &actual);
        SolutionResult {
            is_correct: valid,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: "any valid topological ordering".to_string(),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 6: Parallel Courses ──────────────────────────────────────────

struct TopoSortParallelCourses;

struct ParallelCoursesTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for TopoSortParallelCourses {
    fn id(&self) -> &str {
        "topo_sort_parallel_courses"
    }
    fn name(&self) -> &str {
        "Parallel Courses"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "You are given `n` courses and prerequisite edges (a, b) meaning course `a` must \
         be completed before course `b`. In each semester you can take any number of courses \
         as long as all prerequisites are satisfied.\n\n\
         Return the minimum number of semesters needed to take all courses. Return -1 if \
         impossible (cycle exists).\n\n\
         Constraints:\n\
         - 1 <= n <= 5000\n\
         - 0 <= edges.len() <= 5000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=12);
                let edge_count = rng.random_range(1..=(n * 2).min(n * (n - 1) / 2).max(1));
                let edges = if rng.random_range(0..4) == 0 {
                    let mut e = gen_dag_edges(&mut rng, n, edge_count.saturating_sub(1));
                    if n >= 2 {
                        let u = rng.random_range(1..n);
                        let v = rng.random_range(0..u);
                        e.push((u, v));
                    }
                    e
                } else {
                    gen_dag_edges(&mut rng, n, edge_count)
                };
                TestCase {
                    data: Box::new(ParallelCoursesTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ParallelCoursesTest>().unwrap();
        let expected = ref_parallel_courses(t.n, &t.edges);
        let actual = {
            let shared_log = Rc::new(RefCell::new(OperationLog::new()));
            let graph = TrackedGraph::new(t.n, &t.edges, true, shared_log.clone());
            let result = solutions::parallel_courses(&graph);
            for op in shared_log.borrow().operations() {
                log.record(op.clone());
            }
            result
        };
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_parallel_courses(n: usize, edges: &[(usize, usize)]) -> i32 {
    use std::collections::VecDeque;
    let mut adj = vec![vec![]; n];
    let mut indegree = vec![0usize; n];
    for &(u, v) in edges {
        adj[u].push(v);
        indegree[v] += 1;
    }
    let mut queue = VecDeque::new();
    for (i, &deg) in indegree.iter().enumerate() {
        if deg == 0 {
            queue.push_back(i);
        }
    }
    let mut semesters = 0;
    let mut processed = 0;
    while !queue.is_empty() {
        semesters += 1;
        let level_size = queue.len();
        for _ in 0..level_size {
            let node = queue.pop_front().unwrap();
            processed += 1;
            for &next in &adj[node] {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }
    if processed == n {
        semesters
    } else {
        -1
    }
}

// ── Medium 7: All Ancestors ────────────────────────────────────────────

struct TopoSortAllAncestors;

struct AllAncestorsTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for TopoSortAllAncestors {
    fn id(&self) -> &str {
        "topo_sort_all_ancestors"
    }
    fn name(&self) -> &str {
        "All Ancestors in DAG"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a DAG with `n` nodes and directed edges (u, v), for each node find all its \
         ancestors (nodes that can reach it). Return a vector of vectors where result[i] \
         contains the sorted list of all ancestors of node i.\n\n\
         Constraints:\n\
         - 1 <= n <= 1000\n\
         - 0 <= edges.len() <= n*(n-1)/2\n\
         - The graph is a valid DAG"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=10);
                let edge_count = rng.random_range(1..=(n * (n - 1) / 2).max(1));
                let edges = gen_dag_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(AllAncestorsTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AllAncestorsTest>().unwrap();
        let expected = ref_all_ancestors(t.n, &t.edges);
        let actual = {
            let shared_log = Rc::new(RefCell::new(OperationLog::new()));
            let graph = TrackedGraph::new(t.n, &t.edges, true, shared_log.clone());
            let result = solutions::all_ancestors(&graph);
            for op in shared_log.borrow().operations() {
                log.record(op.clone());
            }
            result
        };
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_all_ancestors(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    use std::collections::BTreeSet;
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
    }
    let mut ancestors: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];

    // Process in topological order
    let mut indegree = vec![0usize; n];
    for &(_, v) in edges {
        indegree[v] += 1;
    }
    let mut queue = std::collections::VecDeque::new();
    for (i, &deg) in indegree.iter().enumerate() {
        if deg == 0 {
            queue.push_back(i);
        }
    }
    while let Some(node) = queue.pop_front() {
        for &next in &adj[node] {
            ancestors[next].insert(node);
            let parent_ancestors: Vec<usize> = ancestors[node].iter().copied().collect();
            for a in parent_ancestors {
                ancestors[next].insert(a);
            }
            indegree[next] -= 1;
            if indegree[next] == 0 {
                queue.push_back(next);
            }
        }
    }
    ancestors
        .into_iter()
        .map(|s| s.into_iter().collect())
        .collect()
}

// ── Medium 8: Longest Path in DAG ──────────────────────────────────────

struct TopoSortLongestPathDag;

struct LongestPathDagTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for TopoSortLongestPathDag {
    fn id(&self) -> &str {
        "topo_sort_longest_path_dag"
    }
    fn name(&self) -> &str {
        "Longest Path in DAG"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a weighted DAG with `n` nodes and edges (u, v, weight), find the length of \
         the longest path (by total weight) in the DAG. The path can start and end at any \
         node.\n\n\
         Constraints:\n\
         - 1 <= n <= 1000\n\
         - Weights can be positive\n\
         - The graph is a valid DAG"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=12);
                let edge_count = rng.random_range(1..=(n * (n - 1) / 2).max(1));
                let dag_edges = gen_dag_edges(&mut rng, n, edge_count);
                let edges: Vec<(usize, usize, i32)> = dag_edges
                    .into_iter()
                    .map(|(u, v)| (u, v, rng.random_range(1..=20)))
                    .collect();
                TestCase {
                    data: Box::new(LongestPathDagTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LongestPathDagTest>().unwrap();
        let expected = ref_longest_path_dag(t.n, &t.edges);
        let actual = {
            let shared_log = Rc::new(RefCell::new(OperationLog::new()));
            let graph = TrackedWeightedGraph::new(t.n, &t.edges, true, shared_log.clone());
            let result = solutions::longest_path_dag(&graph);
            for op in shared_log.borrow().operations() {
                log.record(op.clone());
            }
            result
        };
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_longest_path_dag(n: usize, edges: &[(usize, usize, i32)]) -> i32 {
    let mut adj = vec![vec![]; n];
    let mut indegree = vec![0usize; n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
        indegree[v] += 1;
    }

    let mut queue = std::collections::VecDeque::new();
    let mut dist = vec![0i32; n];
    for (i, &deg) in indegree.iter().enumerate() {
        if deg == 0 {
            queue.push_back(i);
        }
    }
    while let Some(node) = queue.pop_front() {
        for &(next, w) in &adj[node] {
            dist[next] = dist[next].max(dist[node] + w);
            indegree[next] -= 1;
            if indegree[next] == 0 {
                queue.push_back(next);
            }
        }
    }
    *dist.iter().max().unwrap_or(&0)
}

// ── Medium 9: Sequence Reconstruction ──────────────────────────────────

struct TopoSortSequenceReconstruction;

struct SequenceReconstructionTest {
    org: Vec<usize>,
    seqs: Vec<Vec<usize>>,
}

impl Problem for TopoSortSequenceReconstruction {
    fn id(&self) -> &str {
        "topo_sort_sequence_reconstruction"
    }
    fn name(&self) -> &str {
        "Sequence Reconstruction"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Check whether the original sequence `org` can be uniquely reconstructed from the \
         subsequences in `seqs`. The original sequence is a permutation of integers 1..n.\n\n\
         The sequence is uniquely reconstructable if `org` is the only topological ordering \
         derivable from the given subsequences.\n\n\
         Constraints:\n\
         - 1 <= n <= 10000\n\
         - 1 <= seqs.len() <= 10000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=8);
                let mut org: Vec<usize> = (1..=n).collect();
                // Shuffle org
                for i in (1..n).rev() {
                    let j = rng.random_range(0..=i);
                    org.swap(i, j);
                }
                // Generate subsequences from org
                let num_seqs = rng.random_range(1..=(n * 2));
                let seqs: Vec<Vec<usize>> = (0..num_seqs)
                    .map(|_| {
                        let len = rng.random_range(2..=n.min(4));
                        let mut indices: Vec<usize> = (0..n).collect();
                        // Pick random indices and keep them sorted
                        let mut picked: Vec<usize> = Vec::new();
                        for _ in 0..len.min(indices.len()) {
                            let idx = rng.random_range(0..indices.len());
                            picked.push(indices[idx]);
                            indices.swap_remove(idx);
                        }
                        picked.sort();
                        picked.iter().map(|&i| org[i]).collect()
                    })
                    .collect();
                TestCase {
                    data: Box::new(SequenceReconstructionTest { org, seqs }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<SequenceReconstructionTest>()
            .unwrap();
        let expected = ref_sequence_reconstruction(&t.org, &t.seqs);
        let actual = solutions::sequence_reconstruction(&t.org, &t.seqs, log);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("org={:?}, seqs={:?}", t.org, t.seqs),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_sequence_reconstruction(org: &[usize], seqs: &[Vec<usize>]) -> bool {
    use std::collections::{HashMap, HashSet, VecDeque};
    if org.is_empty() {
        return true;
    }
    let n = org.len();
    let mut adj: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut indegree: HashMap<usize, usize> = HashMap::new();
    let mut nodes = HashSet::new();

    for seq in seqs {
        for &v in seq {
            nodes.insert(v);
            indegree.entry(v).or_insert(0);
        }
        for w in seq.windows(2) {
            if adj.entry(w[0]).or_default().insert(w[1]) {
                *indegree.entry(w[1]).or_insert(0) += 1;
            }
        }
    }

    if nodes.len() != n {
        return false;
    }
    for &v in org {
        if !nodes.contains(&v) {
            return false;
        }
    }

    let mut queue = VecDeque::new();
    for (&node, &deg) in &indegree {
        if deg == 0 {
            queue.push_back(node);
        }
    }

    let mut result = Vec::new();
    while queue.len() == 1 {
        let node = queue.pop_front().unwrap();
        result.push(node);
        if let Some(neighbors) = adj.get(&node) {
            for &next in neighbors {
                let deg = indegree.get_mut(&next).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    result.len() == n && result == org
}

// ── Medium 10: Build Order ─────────────────────────────────────────────

struct TopoSortBuildOrder;

struct BuildOrderTest {
    projects: Vec<String>,
    deps: Vec<(String, String)>,
}

impl Problem for TopoSortBuildOrder {
    fn id(&self) -> &str {
        "topo_sort_build_order"
    }
    fn name(&self) -> &str {
        "Build Order"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a list of project names and a list of dependency pairs (a, b) meaning \
         project `a` must be built before project `b`, find a valid build order.\n\n\
         Return an empty vector if no valid order exists (circular dependency).\n\n\
         Constraints:\n\
         - 1 <= projects.len() <= 100\n\
         - All project names are unique"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=8);
                let projects: Vec<String> = (0..n).map(|i| format!("p{i}")).collect();
                let edge_count = rng.random_range(1..=(n * (n - 1) / 2).max(1));
                let dag_edges = gen_dag_edges(&mut rng, n, edge_count);
                let deps: Vec<(String, String)> = dag_edges
                    .into_iter()
                    .map(|(u, v)| (projects[u].clone(), projects[v].clone()))
                    .collect();
                TestCase {
                    data: Box::new(BuildOrderTest { projects, deps }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BuildOrderTest>().unwrap();
        let actual = solutions::build_order(&t.projects, &t.deps, log);
        let valid = ref_validate_build_order(&t.projects, &t.deps, &actual);
        SolutionResult {
            is_correct: valid,
            input_description: format!("projects={:?}, deps={:?}", t.projects, t.deps),
            expected: "any valid build order".to_string(),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_validate_build_order(
    projects: &[String],
    deps: &[(String, String)],
    order: &[String],
) -> bool {
    use std::collections::HashMap;
    if order.len() != projects.len() {
        return false;
    }
    let mut pos = HashMap::new();
    for (i, p) in order.iter().enumerate() {
        pos.insert(p.as_str(), i);
    }
    // All projects must be present
    for p in projects {
        if !pos.contains_key(p.as_str()) {
            return false;
        }
    }
    // All deps must be satisfied
    for (a, b) in deps {
        if pos[a.as_str()] >= pos[b.as_str()] {
            return false;
        }
    }
    true
}

// ── Hard 11: Alien Dictionary ──────────────────────────────────────────

struct TopoSortAlienDictionary;

struct AlienDictionaryTest {
    words: Vec<String>,
}

impl Problem for TopoSortAlienDictionary {
    fn id(&self) -> &str {
        "topo_sort_alien_dictionary"
    }
    fn name(&self) -> &str {
        "Alien Dictionary"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a sorted list of words in an alien language, derive the order of characters \
         in their alphabet. The words are sorted lexicographically by the alien language's \
         rules.\n\n\
         Return a string of characters in the correct order. If no valid ordering exists, \
         return an empty string. If multiple valid orderings exist, return any one.\n\n\
         Constraints:\n\
         - 1 <= words.len() <= 100\n\
         - 1 <= words[i].len() <= 100\n\
         - words[i] consists of lowercase English letters"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let alpha_size = rng.random_range(3..=6);
                let mut alpha: Vec<u8> = (b'a'..b'a' + alpha_size).collect();
                // Shuffle to create alien ordering
                for i in (1..alpha.len()).rev() {
                    let j = rng.random_range(0..=i);
                    alpha.swap(i, j);
                }
                // Generate words sorted by this alien ordering
                let n_words = rng.random_range(2..=8);
                let mut words: Vec<String> = (0..n_words)
                    .map(|_| {
                        let len = rng.random_range(1..=4);
                        (0..len)
                            .map(|_| alpha[rng.random_range(0..alpha_size as usize)] as char)
                            .collect()
                    })
                    .collect();
                // Sort by alien ordering
                let mut order_map = std::collections::HashMap::new();
                for (i, &ch) in alpha.iter().enumerate() {
                    order_map.insert(ch, i);
                }
                words.sort_by(|a, b| {
                    for (ca, cb) in a.bytes().zip(b.bytes()) {
                        let oa = order_map[&ca];
                        let ob = order_map[&cb];
                        if oa != ob {
                            return oa.cmp(&ob);
                        }
                    }
                    a.len().cmp(&b.len())
                });
                words.dedup();
                TestCase {
                    data: Box::new(AlienDictionaryTest { words }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AlienDictionaryTest>().unwrap();
        let expected = ref_alien_dictionary(&t.words);
        let actual = solutions::alien_dictionary(&t.words, log);
        // Validate the actual result respects the constraints
        let valid = if expected.is_empty() {
            actual.is_empty()
        } else {
            ref_validate_alien_order(&t.words, &actual)
        };
        SolutionResult {
            is_correct: valid,
            input_description: format!("words={:?}", t.words),
            expected: if expected.is_empty() {
                "\"\" (invalid)".to_string()
            } else {
                format!("{expected:?} (or any valid ordering)")
            },
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_alien_dictionary(words: &[String]) -> String {
    use std::collections::{HashMap, HashSet, VecDeque};
    let mut adj: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut indegree: HashMap<u8, usize> = HashMap::new();

    for word in words {
        for &ch in word.as_bytes() {
            indegree.entry(ch).or_insert(0);
        }
    }

    for pair in words.windows(2) {
        let w1 = pair[0].as_bytes();
        let w2 = pair[1].as_bytes();
        // Invalid: if w1 is a prefix of w2, that is fine. But if w2 is a strict prefix of w1, invalid.
        if w1.len() > w2.len() && w1.starts_with(w2) {
            return String::new();
        }
        for (&c1, &c2) in w1.iter().zip(w2.iter()) {
            if c1 != c2 {
                if adj.entry(c1).or_default().insert(c2) {
                    *indegree.entry(c2).or_insert(0) += 1;
                }
                break;
            }
        }
    }

    let mut queue = VecDeque::new();
    for (&ch, &deg) in &indegree {
        if deg == 0 {
            queue.push_back(ch);
        }
    }

    let mut result = Vec::new();
    while let Some(ch) = queue.pop_front() {
        result.push(ch as char);
        if let Some(neighbors) = adj.get(&ch) {
            for &next in neighbors {
                let deg = indegree.get_mut(&next).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    if result.len() != indegree.len() {
        String::new()
    } else {
        result.into_iter().collect()
    }
}

fn ref_validate_alien_order(words: &[String], order: &str) -> bool {
    use std::collections::HashMap;
    let order_bytes: Vec<u8> = order.bytes().collect();
    let mut pos: HashMap<u8, usize> = HashMap::new();
    for (i, &ch) in order_bytes.iter().enumerate() {
        pos.insert(ch, i);
    }

    // Check all chars in words appear in order
    for word in words {
        for &ch in word.as_bytes() {
            if !pos.contains_key(&ch) {
                return false;
            }
        }
    }

    // Check words are sorted under this ordering
    for pair in words.windows(2) {
        let w1 = pair[0].as_bytes();
        let w2 = pair[1].as_bytes();
        let mut found_diff = false;
        for (&c1, &c2) in w1.iter().zip(w2.iter()) {
            if c1 != c2 {
                if pos[&c1] >= pos[&c2] {
                    return false;
                }
                found_diff = true;
                break;
            }
        }
        if !found_diff && w1.len() > w2.len() {
            return false;
        }
    }
    true
}

// ── Hard 12: Minimum Height Trees ──────────────────────────────────────

struct TopoSortMinimumHeightTrees;

struct MinimumHeightTreesTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for TopoSortMinimumHeightTrees {
    fn id(&self) -> &str {
        "topo_sort_minimum_height_trees"
    }
    fn name(&self) -> &str {
        "Minimum Height Trees"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an undirected tree of `n` nodes labeled 0..n-1 and a list of edges, find \
         all root labels that minimize the height of the tree. Return them in any order.\n\n\
         The answer is always 1 or 2 nodes (the centroids of the tree).\n\n\
         Hint: iteratively remove leaf nodes (degree 1) until 1 or 2 nodes remain.\n\n\
         Constraints:\n\
         - 1 <= n <= 20000\n\
         - edges.len() == n - 1\n\
         - The graph is a valid tree"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=15);
                let edges = if n > 1 {
                    gen_tree_edges(&mut rng, n)
                } else {
                    vec![]
                };
                TestCase {
                    data: Box::new(MinimumHeightTreesTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinimumHeightTreesTest>().unwrap();
        let mut expected = ref_minimum_height_trees(t.n, &t.edges);
        expected.sort();
        let mut actual = {
            let shared_log = Rc::new(RefCell::new(OperationLog::new()));
            let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
            let result = solutions::minimum_height_trees(&graph);
            for op in shared_log.borrow().operations() {
                log.record(op.clone());
            }
            result
        };
        actual.sort();
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_minimum_height_trees(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    if n == 1 {
        return vec![0];
    }
    use std::collections::{HashSet, VecDeque};
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for &(u, v) in edges {
        adj[u].insert(v);
        adj[v].insert(u);
    }

    let mut leaves = VecDeque::new();
    for (i, neighbors) in adj.iter().enumerate() {
        if neighbors.len() == 1 {
            leaves.push_back(i);
        }
    }

    let mut remaining = n;
    while remaining > 2 {
        let size = leaves.len();
        remaining -= size;
        for _ in 0..size {
            let leaf = leaves.pop_front().unwrap();
            if let Some(&neighbor) = adj[leaf].iter().next() {
                adj[neighbor].remove(&leaf);
                if adj[neighbor].len() == 1 {
                    leaves.push_back(neighbor);
                }
            }
        }
    }
    leaves.into_iter().collect()
}

// ── Hard 13: Longest Increasing Path (Topo Sort Approach) ──────────────

struct TopoSortLongestIncreasingPath;

struct LongestIncreasingPathTopoTest {
    matrix: Vec<Vec<i32>>,
}

impl Problem for TopoSortLongestIncreasingPath {
    fn id(&self) -> &str {
        "topo_sort_longest_increasing_path"
    }
    fn name(&self) -> &str {
        "Longest Increasing Path (Topo Sort)"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an m x n matrix, find the length of the longest increasing path. From each \
         cell, you can move in four directions. Each cell in the path must be strictly \
         greater than the previous.\n\n\
         Solve this using a topological sort approach: build a DAG where edges go from \
         smaller to larger adjacent cells, then find the longest path.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 200\n\
         - 0 <= matrix[i][j] <= 2^31 - 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(2..=8);
                let cols = rng.random_range(2..=8);
                let matrix: Vec<Vec<i32>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(0..=20)).collect())
                    .collect();
                TestCase {
                    data: Box::new(LongestIncreasingPathTopoTest { matrix }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<LongestIncreasingPathTopoTest>()
            .unwrap();
        let expected = ref_longest_increasing_path_topo(&t.matrix);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_matrix: Vec<Vec<Tracked<i32>>> = t
            .matrix
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, &v)| Tracked::new(v, r * row.len() + c, shared_log.clone()))
                    .collect()
            })
            .collect();
        let actual = solutions::longest_increasing_path_topo(&tracked_matrix);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("matrix={:?}", t.matrix),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_longest_increasing_path_topo(matrix: &[Vec<i32>]) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Build indegree: for each cell, count how many smaller neighbors point to it
    let mut indegree = vec![vec![0usize; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            for (dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if matrix[nr][nc] < matrix[r][c] {
                        indegree[r][c] += 1;
                    }
                }
            }
        }
    }

    let mut queue = std::collections::VecDeque::new();
    for (r, indegree_row) in indegree.iter().enumerate().take(rows) {
        for (c, &deg) in indegree_row.iter().enumerate().take(cols) {
            if deg == 0 {
                queue.push_back((r, c));
            }
        }
    }

    let mut length = 0;
    while !queue.is_empty() {
        length += 1;
        let size = queue.len();
        for _ in 0..size {
            let (r, c) = queue.pop_front().unwrap();
            for (dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if matrix[nr][nc] > matrix[r][c] {
                        indegree[nr][nc] -= 1;
                        if indegree[nr][nc] == 0 {
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }
        }
    }
    length
}

// ── Hard 14: Critical Connections (Bridges) ────────────────────────────

struct TopoSortCriticalConnections;

struct CriticalConnectionsTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for TopoSortCriticalConnections {
    fn id(&self) -> &str {
        "topo_sort_critical_connections"
    }
    fn name(&self) -> &str {
        "Critical Connections in a Network"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an undirected connected graph with `n` nodes and edges, find all critical \
         connections (bridges). A bridge is an edge whose removal disconnects the graph.\n\n\
         Return the bridges as a sorted list of (u, v) pairs where u < v, sorted \
         lexicographically.\n\n\
         Hint: Use Tarjan's bridge-finding algorithm (DFS with discovery and low-link values).\n\n\
         Constraints:\n\
         - 2 <= n <= 10000\n\
         - n - 1 <= edges.len() <= n * 2"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=12);
                // Start with a tree (guaranteed connected, all edges are bridges)
                let mut edges: Vec<(usize, usize)> = Vec::new();
                for i in 1..n {
                    let parent = rng.random_range(0..i);
                    edges.push((parent, i));
                }
                // Add some extra edges (these create cycles and remove some bridges)
                let extras = rng.random_range(0..=n / 2);
                for _ in 0..extras {
                    let u = rng.random_range(0..n);
                    let v = rng.random_range(0..n);
                    if u != v {
                        edges.push((u, v));
                    }
                }
                TestCase {
                    data: Box::new(CriticalConnectionsTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CriticalConnectionsTest>().unwrap();
        let expected = ref_critical_connections(t.n, &t.edges);
        let mut actual = {
            let shared_log = Rc::new(RefCell::new(OperationLog::new()));
            let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
            let result = solutions::critical_connections(&graph);
            for op in shared_log.borrow().operations() {
                log.record(op.clone());
            }
            result
        };
        // Normalize: ensure u < v and sort
        for pair in actual.iter_mut() {
            if pair.0 > pair.1 {
                std::mem::swap(&mut pair.0, &mut pair.1);
            }
        }
        actual.sort();
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_critical_connections(n: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut disc = vec![-1i32; n];
    let mut low = vec![0i32; n];
    let mut bridges = Vec::new();
    let mut timer = 0i32;

    fn dfs(
        node: usize,
        parent: i32,
        adj: &[Vec<usize>],
        disc: &mut [i32],
        low: &mut [i32],
        timer: &mut i32,
        bridges: &mut Vec<(usize, usize)>,
    ) {
        disc[node] = *timer;
        low[node] = *timer;
        *timer += 1;

        for &next in &adj[node] {
            if next as i32 == parent {
                continue;
            }
            if disc[next] == -1 {
                dfs(next, node as i32, adj, disc, low, timer, bridges);
                low[node] = low[node].min(low[next]);
                if low[next] > disc[node] {
                    let u = node.min(next);
                    let v = node.max(next);
                    bridges.push((u, v));
                }
            } else {
                low[node] = low[node].min(disc[next]);
            }
        }
    }

    dfs(0, -1, &adj, &mut disc, &mut low, &mut timer, &mut bridges);
    bridges.sort();
    bridges
}

// ── Hard 15: Sort Items by Groups ──────────────────────────────────────

struct TopoSortSortItemsByGroups;

struct SortItemsByGroupsTest {
    n: usize,
    m: usize,
    group: Vec<i32>,
    before_items: Vec<Vec<i32>>,
}

impl Problem for TopoSortSortItemsByGroups {
    fn id(&self) -> &str {
        "topo_sort_sort_items_by_groups"
    }
    fn name(&self) -> &str {
        "Sort Items by Groups"
    }
    fn topic(&self) -> &str {
        "topological_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "There are `n` items, each belonging to one of `m` groups (or no group, denoted -1). \
         Items in the same group must appear adjacent in the final ordering. Given a list \
         `before_items[i]` of items that must appear before item `i`, find a valid ordering.\n\n\
         Return an empty vector if no valid ordering exists.\n\n\
         Constraints:\n\
         - 1 <= n, m <= 30000\n\
         - group[i] is -1 or in 0..m\n\
         - 0 <= before_items[i].len() <= n - 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=10);
                let m = rng.random_range(1..=n.max(2));
                let group: Vec<i32> = (0..n)
                    .map(|_| {
                        if rng.random_range(0..3) == 0 {
                            -1
                        } else {
                            rng.random_range(0..m as i32)
                        }
                    })
                    .collect();
                // Generate some ordering constraints
                let before_items: Vec<Vec<i32>> = (0..n)
                    .map(|i| {
                        let count = rng.random_range(0..=(n / 3).max(1));
                        let mut before = Vec::new();
                        for _ in 0..count {
                            let j = rng.random_range(0..n);
                            if j != i && j < i {
                                before.push(j as i32);
                            }
                        }
                        before.sort();
                        before.dedup();
                        before
                    })
                    .collect();
                TestCase {
                    data: Box::new(SortItemsByGroupsTest {
                        n,
                        m,
                        group,
                        before_items,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortItemsByGroupsTest>().unwrap();
        let expected = ref_sort_items_by_groups(t.n, t.m, &t.group, &t.before_items);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_group = track_slice(&t.group, shared_log.clone());
        let tracked_before: Vec<Vec<Tracked<i32>>> = t
            .before_items
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, &v)| Tracked::new(v, r * 1000 + c, shared_log.clone()))
                    .collect()
            })
            .collect();
        let actual = solutions::sort_items_by_groups(t.n, t.m, &tracked_group, &tracked_before);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        let valid = if expected.is_empty() {
            actual.is_empty()
        } else {
            ref_validate_sort_items(t.n, t.m, &t.group, &t.before_items, &actual)
        };
        SolutionResult {
            is_correct: valid,
            input_description: format!(
                "n={}, m={}, group={:?}, before_items={:?}",
                t.n, t.m, t.group, t.before_items
            ),
            expected: if expected.is_empty() {
                "[] (impossible)".to_string()
            } else {
                format!("{expected:?} (or any valid ordering)")
            },
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_sort_items_by_groups(
    n: usize,
    m: usize,
    group: &[i32],
    before_items: &[Vec<i32>],
) -> Vec<i32> {
    use std::collections::VecDeque;
    // Assign each ungrouped item its own unique group
    let mut grp = group.to_vec();
    let mut next_group = m;
    for g in grp.iter_mut() {
        if *g == -1 {
            *g = next_group as i32;
            next_group += 1;
        }
    }
    let total_groups = next_group;

    // Topo sort within each group and between groups
    let mut item_adj = vec![vec![]; n];
    let mut item_indegree = vec![0usize; n];
    let mut group_adj = vec![vec![]; total_groups];
    let mut group_indegree = vec![0usize; total_groups];

    use std::collections::HashSet;
    let mut group_edge_set: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..n {
        for &before in &before_items[i] {
            let b = before as usize;
            item_adj[b].push(i);
            item_indegree[i] += 1;
            let gb = grp[b] as usize;
            let gi = grp[i] as usize;
            if gb != gi && group_edge_set.insert((gb, gi)) {
                group_adj[gb].push(gi);
                group_indegree[gi] += 1;
            }
        }
    }

    // Topo sort groups
    let group_order = {
        let mut queue = VecDeque::new();
        for (g, &deg) in group_indegree.iter().enumerate().take(total_groups) {
            if deg == 0 {
                queue.push_back(g);
            }
        }
        let mut order = Vec::new();
        while let Some(g) = queue.pop_front() {
            order.push(g);
            for &ng in &group_adj[g] {
                group_indegree[ng] -= 1;
                if group_indegree[ng] == 0 {
                    queue.push_back(ng);
                }
            }
        }
        if order.len() != total_groups {
            return vec![];
        }
        order
    };

    // Topo sort items within each group
    let mut group_items: Vec<Vec<usize>> = vec![vec![]; total_groups];
    for i in 0..n {
        group_items[grp[i] as usize].push(i);
    }

    let mut result = Vec::new();
    for g in &group_order {
        let items = &group_items[*g];
        if items.is_empty() {
            continue;
        }
        // Topo sort these items
        let item_set: HashSet<usize> = items.iter().copied().collect();
        let mut local_indegree: std::collections::HashMap<usize, usize> = HashMap::new();
        for &item in items {
            local_indegree.insert(item, 0);
        }
        for &item in items {
            for &next in &item_adj[item] {
                if item_set.contains(&next) {
                    *local_indegree.entry(next).or_insert(0) += 1;
                }
            }
        }
        let mut queue = VecDeque::new();
        for &item in items {
            if local_indegree[&item] == 0 {
                queue.push_back(item);
            }
        }
        let mut local_order = Vec::new();
        while let Some(item) = queue.pop_front() {
            local_order.push(item as i32);
            for &next in &item_adj[item] {
                if let Some(deg) = local_indegree.get_mut(&next) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(next);
                    }
                }
            }
        }
        if local_order.len() != items.len() {
            return vec![];
        }
        result.extend(local_order);
    }
    result
}

use std::collections::HashMap;

fn ref_validate_sort_items(
    n: usize,
    _m: usize,
    group: &[i32],
    before_items: &[Vec<i32>],
    order: &[i32],
) -> bool {
    if order.len() != n {
        return false;
    }
    let mut pos = HashMap::new();
    for (i, &item) in order.iter().enumerate() {
        if item < 0 || item as usize >= n {
            return false;
        }
        pos.insert(item as usize, i);
    }
    if pos.len() != n {
        return false;
    }
    // Check before constraints
    for i in 0..n {
        for &before in &before_items[i] {
            if pos[&(before as usize)] >= pos[&i] {
                return false;
            }
        }
    }
    // Check group adjacency: items of the same group must be contiguous
    let mut group_ranges: HashMap<i32, (usize, usize)> = HashMap::new();
    for (idx, &item) in order.iter().enumerate() {
        let g = group[item as usize];
        if g == -1 {
            continue;
        }
        let entry = group_ranges.entry(g).or_insert((idx, idx));
        entry.0 = entry.0.min(idx);
        entry.1 = entry.1.max(idx);
    }
    for (&g, &(lo, hi)) in &group_ranges {
        for &item in order.iter().take(hi + 1).skip(lo) {
            let item = item as usize;
            if group[item] != g && group[item] != -1 {
                return false;
            }
        }
    }
    true
}
