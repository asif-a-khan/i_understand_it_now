use rand::Rng;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part4_graphs::graph_representations as solutions;
use crate::tracker::{OperationLog, TrackedGraph};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(AdjacencyList),
        Box::new(AdjacencyMatrix),
        Box::new(DegreeCount),
        Box::new(HasEdge),
        Box::new(CountEdges),
        Box::new(IsBipartite),
        Box::new(ConnectedComponents),
        Box::new(HasCycleUndirected),
        Box::new(HasCycleDirected),
        Box::new(Transpose),
        Box::new(StronglyConnected),
        Box::new(Bridges),
        Box::new(ArticulationPoints),
        Box::new(EulerPath),
        Box::new(GraphColoring),
    ]
}

// ── Helpers ────────────────────────────────────────────────────────────

/// Generate a random undirected edge list with no self-loops.
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

/// Generate a random connected undirected edge list (spanning tree + extras).
fn random_connected_edges(rng: &mut impl Rng, n: usize, extra: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    let mut seen = HashSet::new();
    // Spanning tree
    for i in 1..n {
        let j = rng.random_range(0..i);
        let key = (j.min(i), j.max(i));
        seen.insert(key);
        edges.push((i, j));
    }
    // Extra edges
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

/// Generate a random directed edge list with no self-loops.
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

/// Generate a DAG edge list (edges only from lower to higher index).
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

/// Build adjacency list from undirected edges.
fn build_adj_undirected(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    adj
}

/// Build adjacency list from directed edges.
fn build_adj_directed(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
    }
    adj
}

/// Generate a bipartite graph edge list.
fn random_bipartite_edges(rng: &mut impl Rng, n: usize, edge_count: usize) -> Vec<(usize, usize)> {
    let half = n / 2;
    let mut edges = Vec::new();
    let mut seen = HashSet::new();
    for _ in 0..edge_count {
        let u = rng.random_range(0..half.max(1));
        let v = rng.random_range(half.max(1)..n);
        if u != v {
            let key = (u.min(v), u.max(v));
            if seen.insert(key) {
                edges.push((u, v));
            }
        }
    }
    edges
}

// ── Easy 1: Adjacency List ────────────────────────────────────────────

struct AdjacencyList;
struct AdjListTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for AdjacencyList {
    fn id(&self) -> &str {
        "graph_repr_adjacency_list"
    }
    fn name(&self) -> &str {
        "Edge List to Adjacency List"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Convert an undirected edge list to an adjacency list representation.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: Vec<Vec<usize>> where result[i] is the sorted list of neighbors of node i.\n\n\
         Constraints:\n\
         - 1 <= n <= 100\n\
         - 0 <= edges.len() <= n*(n-1)/2\n\
         - No self-loops or duplicate edges\n\
         - Each neighbor list must be sorted in ascending order"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let edge_count = rng.random_range(0..=n * 2);
                let edges = random_undirected_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(AdjListTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AdjListTest>().unwrap();
        let expected = ref_adjacency_list(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::adjacency_list(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_adjacency_list(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    for row in &mut adj {
        row.sort();
    }
    adj
}

// ── Easy 2: Adjacency Matrix ──────────────────────────────────────────

struct AdjacencyMatrix;
struct AdjMatTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for AdjacencyMatrix {
    fn id(&self) -> &str {
        "graph_repr_adjacency_matrix"
    }
    fn name(&self) -> &str {
        "Edge List to Adjacency Matrix"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Convert an undirected edge list to an adjacency matrix.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: Vec<Vec<bool>> where result[i][j] = true if edge (i,j) exists.\n\n\
         Constraints:\n\
         - 1 <= n <= 100\n\
         - No self-loops or duplicate edges"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=12);
                let edge_count = rng.random_range(0..=n * 2);
                let edges = random_undirected_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(AdjMatTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AdjMatTest>().unwrap();
        let expected = ref_adjacency_matrix(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::adjacency_matrix(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_adjacency_matrix(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let mut mat = vec![vec![false; n]; n];
    for &(u, v) in edges {
        mat[u][v] = true;
        mat[v][u] = true;
    }
    mat
}

// ── Easy 3: Degree Count ──────────────────────────────────────────────

struct DegreeCount;
struct DegreeTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for DegreeCount {
    fn id(&self) -> &str {
        "graph_repr_degree_count"
    }
    fn name(&self) -> &str {
        "Degree Count"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Count the degree of each node in an undirected graph.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: Vec<usize> where result[i] is the degree of node i.\n\n\
         Constraints:\n\
         - 1 <= n <= 100\n\
         - No self-loops"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let edge_count = rng.random_range(0..=n * 2);
                let edges = random_undirected_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(DegreeTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DegreeTest>().unwrap();
        let expected = ref_degree_count(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::degree_count(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_degree_count(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut deg = vec![0usize; n];
    for &(u, v) in edges {
        deg[u] += 1;
        deg[v] += 1;
    }
    deg
}

// ── Easy 4: Has Edge ──────────────────────────────────────────────────

struct HasEdge;
struct HasEdgeTest {
    n: usize,
    edges: Vec<(usize, usize)>,
    u: usize,
    v: usize,
}

impl Problem for HasEdge {
    fn id(&self) -> &str {
        "graph_repr_has_edge"
    }
    fn name(&self) -> &str {
        "Has Edge"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Check if an edge exists between two nodes in an undirected graph.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>, u: usize, v: usize)\n\
         Output: bool — true if edge (u, v) exists.\n\n\
         Constraints:\n\
         - 1 <= n <= 100\n\
         - 0 <= u, v < n"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let edge_count = rng.random_range(1..=n * 2);
                let edges = random_undirected_edges(&mut rng, n, edge_count);
                let u = rng.random_range(0..n);
                let v = rng.random_range(0..n);
                TestCase {
                    data: Box::new(HasEdgeTest { n, edges, u, v }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<HasEdgeTest>().unwrap();
        let expected = ref_has_edge(&t.edges, t.u, t.v);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::has_edge(&graph, t.u, t.v);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}, u={}, v={}", t.n, t.edges, t.u, t.v),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_has_edge(edges: &[(usize, usize)], u: usize, v: usize) -> bool {
    edges
        .iter()
        .any(|&(a, b)| (a == u && b == v) || (a == v && b == u))
}

// ── Easy 5: Count Edges ───────────────────────────────────────────────

struct CountEdges;
struct CountEdgesTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for CountEdges {
    fn id(&self) -> &str {
        "graph_repr_count_edges"
    }
    fn name(&self) -> &str {
        "Count Edges"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Count the total number of unique edges in an undirected graph.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: usize — the number of unique edges.\n\n\
         Constraints:\n\
         - The edge list may contain duplicates (same edge listed twice).\n\
         - (u, v) and (v, u) count as the same edge.\n\
         - No self-loops."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let base_count = rng.random_range(0..=n * 2);
                let mut edges = random_undirected_edges(&mut rng, n, base_count);
                // Add some duplicate edges
                let dup_count = rng.random_range(0..=3);
                for _ in 0..dup_count {
                    if !edges.is_empty() {
                        let idx = rng.random_range(0..edges.len());
                        let (a, b) = edges[idx];
                        // Push reverse or same direction
                        if rng.random_range(0..2) == 0 {
                            edges.push((b, a));
                        } else {
                            edges.push((a, b));
                        }
                    }
                }
                TestCase {
                    data: Box::new(CountEdgesTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountEdgesTest>().unwrap();
        let expected = ref_count_edges(&t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::count_edges(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_count_edges(edges: &[(usize, usize)]) -> usize {
    let mut seen = HashSet::new();
    for &(u, v) in edges {
        seen.insert((u.min(v), u.max(v)));
    }
    seen.len()
}

// ── Medium 6: Is Bipartite ────────────────────────────────────────────

struct IsBipartite;
struct BipartiteTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for IsBipartite {
    fn id(&self) -> &str {
        "graph_repr_is_bipartite"
    }
    fn name(&self) -> &str {
        "Is Bipartite"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Check if an undirected graph is bipartite (2-colorable).\n\n\
         A graph is bipartite if its nodes can be divided into two disjoint sets such\n\
         that every edge connects a node in one set to a node in the other.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: bool — true if the graph is bipartite.\n\n\
         Constraints:\n\
         - 1 <= n <= 100\n\
         - Graph may be disconnected."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let n = rng.random_range(2..=15);
                // Mix bipartite and non-bipartite graphs
                let edge_count = rng.random_range(1..=n * 2);
                let edges = if i % 2 == 0 {
                    random_bipartite_edges(&mut rng, n, edge_count)
                } else {
                    random_undirected_edges(&mut rng, n, edge_count)
                };
                TestCase {
                    data: Box::new(BipartiteTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BipartiteTest>().unwrap();
        let expected = ref_is_bipartite(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::is_bipartite(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_is_bipartite(n: usize, edges: &[(usize, usize)]) -> bool {
    let adj = build_adj_undirected(n, edges);
    let mut color: Vec<i32> = vec![-1; n];
    for start in 0..n {
        if color[start] != -1 {
            continue;
        }
        color[start] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                if color[v] == -1 {
                    color[v] = 1 - color[u];
                    queue.push_back(v);
                } else if color[v] == color[u] {
                    return false;
                }
            }
        }
    }
    true
}

// ── Medium 7: Connected Components ────────────────────────────────────

struct ConnectedComponents;
struct CCTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for ConnectedComponents {
    fn id(&self) -> &str {
        "graph_repr_connected_components"
    }
    fn name(&self) -> &str {
        "Connected Components"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Count the number of connected components in an undirected graph.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: usize — the number of connected components.\n\n\
         Constraints:\n\
         - 1 <= n <= 200\n\
         - Isolated nodes count as their own component."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=20);
                let edge_count = rng.random_range(0..=n);
                let edges = random_undirected_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(CCTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CCTest>().unwrap();
        let expected = ref_connected_components(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::connected_components(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_connected_components(n: usize, edges: &[(usize, usize)]) -> usize {
    let adj = build_adj_undirected(n, edges);
    let mut visited = vec![false; n];
    let mut count = 0;
    for i in 0..n {
        if !visited[i] {
            count += 1;
            let mut stack = vec![i];
            while let Some(u) = stack.pop() {
                if visited[u] {
                    continue;
                }
                visited[u] = true;
                for &v in &adj[u] {
                    if !visited[v] {
                        stack.push(v);
                    }
                }
            }
        }
    }
    count
}

// ── Medium 8: Has Cycle Undirected ────────────────────────────────────

struct HasCycleUndirected;
struct CycleUndirectedTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for HasCycleUndirected {
    fn id(&self) -> &str {
        "graph_repr_has_cycle_undirected"
    }
    fn name(&self) -> &str {
        "Detect Cycle (Undirected)"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Detect if an undirected graph contains a cycle.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: bool — true if the graph contains a cycle.\n\n\
         Constraints:\n\
         - 1 <= n <= 200\n\
         - No self-loops or duplicate edges.\n\
         - Graph may be disconnected."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=15);
                let edge_count = rng.random_range(1..=n * 2);
                let edges = random_undirected_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(CycleUndirectedTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CycleUndirectedTest>().unwrap();
        let expected = ref_has_cycle_undirected(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::has_cycle_undirected(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_has_cycle_undirected(n: usize, edges: &[(usize, usize)]) -> bool {
    // Union-Find approach
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    for &(u, v) in edges {
        let pu = find(&mut parent, u);
        let pv = find(&mut parent, v);
        if pu == pv {
            return true;
        }
        parent[pu] = pv;
    }
    false
}

// ── Medium 9: Has Cycle Directed ──────────────────────────────────────

struct HasCycleDirected;
struct CycleDirectedTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for HasCycleDirected {
    fn id(&self) -> &str {
        "graph_repr_has_cycle_directed"
    }
    fn name(&self) -> &str {
        "Detect Cycle (Directed)"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Detect if a directed graph contains a cycle.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>) where each edge is (from, to).\n\
         Output: bool — true if the graph contains a cycle.\n\n\
         Constraints:\n\
         - 1 <= n <= 200\n\
         - No self-loops.\n\
         - Graph may be disconnected."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let n = rng.random_range(3..=12);
                // Mix DAGs and graphs with cycles
                let edge_count = rng.random_range(1..=n * 2);
                let edges = if i % 2 == 0 {
                    random_dag_edges(&mut rng, n, edge_count)
                } else {
                    random_directed_edges(&mut rng, n, edge_count)
                };
                TestCase {
                    data: Box::new(CycleDirectedTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CycleDirectedTest>().unwrap();
        let expected = ref_has_cycle_directed(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, true, shared_log.clone());
        let actual = solutions::has_cycle_directed(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_has_cycle_directed(n: usize, edges: &[(usize, usize)]) -> bool {
    let adj = build_adj_directed(n, edges);
    // 0 = unvisited, 1 = in stack, 2 = done
    let mut state = vec![0u8; n];
    fn dfs(u: usize, adj: &[Vec<usize>], state: &mut Vec<u8>) -> bool {
        state[u] = 1;
        for &v in &adj[u] {
            if state[v] == 1 {
                return true;
            }
            if state[v] == 0 && dfs(v, adj, state) {
                return true;
            }
        }
        state[u] = 2;
        false
    }
    for i in 0..n {
        if state[i] == 0 && dfs(i, &adj, &mut state) {
            return true;
        }
    }
    false
}

// ── Medium 10: Transpose ──────────────────────────────────────────────

struct Transpose;
struct TransposeTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for Transpose {
    fn id(&self) -> &str {
        "graph_repr_transpose"
    }
    fn name(&self) -> &str {
        "Transpose Graph"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Compute the transpose (reverse) of a directed graph.\n\n\
         The transpose reverses the direction of every edge: if (u, v) exists\n\
         in the original, (v, u) exists in the transpose.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>) — directed edges.\n\
         Output: Vec<Vec<usize>> where result[i] is the sorted list of neighbors\n\
         of node i in the transposed graph.\n\n\
         Constraints:\n\
         - 1 <= n <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let edge_count = rng.random_range(1..=n * 2);
                let edges = random_directed_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(TransposeTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TransposeTest>().unwrap();
        let expected = ref_transpose(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, true, shared_log.clone());
        let actual = solutions::transpose(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_transpose(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[v].push(u);
    }
    for row in &mut adj {
        row.sort();
    }
    adj
}

// ── Hard 11: Strongly Connected Components ────────────────────────────

struct StronglyConnected;
struct SCCTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for StronglyConnected {
    fn id(&self) -> &str {
        "graph_repr_strongly_connected"
    }
    fn name(&self) -> &str {
        "Strongly Connected Components (Kosaraju)"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Count the number of strongly connected components in a directed graph.\n\n\
         A strongly connected component is a maximal set of nodes where every node\n\
         is reachable from every other node in the set.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>) — directed edges.\n\
         Output: usize — the number of SCCs.\n\n\
         Hint: Use Kosaraju's algorithm.\n\n\
         Constraints:\n\
         - 1 <= n <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=15);
                let edge_count = rng.random_range(n..=n * 3);
                let edges = random_directed_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(SCCTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SCCTest>().unwrap();
        let expected = ref_strongly_connected(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, true, shared_log.clone());
        let actual = solutions::strongly_connected(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_strongly_connected(n: usize, edges: &[(usize, usize)]) -> usize {
    let adj = build_adj_directed(n, edges);
    // Step 1: fill order by finish time
    let mut visited = vec![false; n];
    let mut order = Vec::new();
    fn dfs1(u: usize, adj: &[Vec<usize>], visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[u] = true;
        for &v in &adj[u] {
            if !visited[v] {
                dfs1(v, adj, visited, order);
            }
        }
        order.push(u);
    }
    for i in 0..n {
        if !visited[i] {
            dfs1(i, &adj, &mut visited, &mut order);
        }
    }
    // Step 2: transpose
    let mut radj = vec![vec![]; n];
    for &(u, v) in edges {
        radj[v].push(u);
    }
    // Step 3: DFS in reverse finish order on transposed graph
    let mut visited = vec![false; n];
    let mut count = 0;
    fn dfs2(u: usize, radj: &[Vec<usize>], visited: &mut Vec<bool>) {
        visited[u] = true;
        for &v in &radj[u] {
            if !visited[v] {
                dfs2(v, radj, visited);
            }
        }
    }
    for &u in order.iter().rev() {
        if !visited[u] {
            count += 1;
            dfs2(u, &radj, &mut visited);
        }
    }
    count
}

// ── Hard 12: Bridges ──────────────────────────────────────────────────

struct Bridges;
struct BridgesTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for Bridges {
    fn id(&self) -> &str {
        "graph_repr_bridges"
    }
    fn name(&self) -> &str {
        "Find Bridges"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find all bridges in an undirected graph.\n\n\
         A bridge is an edge whose removal disconnects the graph (or increases\n\
         the number of connected components).\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: Vec<(usize, usize)> — list of bridge edges, each with (min, max)\n\
         ordering, sorted lexicographically.\n\n\
         Constraints:\n\
         - 1 <= n <= 200\n\
         - No self-loops or duplicate edges."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=15);
                let extra = rng.random_range(0..=n);
                let edges = random_connected_edges(&mut rng, n, extra);
                TestCase {
                    data: Box::new(BridgesTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BridgesTest>().unwrap();
        let expected = ref_bridges(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::bridges(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_bridges(n: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let adj = build_adj_undirected(n, edges);
    let mut disc = vec![0i32; n];
    let mut low = vec![0i32; n];
    let mut visited = vec![false; n];
    let mut timer = 1i32;
    let mut bridges = Vec::new();

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        u: usize,
        parent: i32,
        adj: &[Vec<usize>],
        disc: &mut Vec<i32>,
        low: &mut Vec<i32>,
        visited: &mut Vec<bool>,
        timer: &mut i32,
        bridges: &mut Vec<(usize, usize)>,
    ) {
        visited[u] = true;
        disc[u] = *timer;
        low[u] = *timer;
        *timer += 1;
        for &v in &adj[u] {
            if v as i32 == parent {
                continue;
            }
            if !visited[v] {
                dfs(v, u as i32, adj, disc, low, visited, timer, bridges);
                low[u] = low[u].min(low[v]);
                if low[v] > disc[u] {
                    bridges.push((u.min(v), u.max(v)));
                }
            } else {
                low[u] = low[u].min(disc[v]);
            }
        }
    }

    for i in 0..n {
        if !visited[i] {
            dfs(
                i,
                -1,
                &adj,
                &mut disc,
                &mut low,
                &mut visited,
                &mut timer,
                &mut bridges,
            );
        }
    }
    bridges.sort();
    bridges
}

// ── Hard 13: Articulation Points ──────────────────────────────────────

struct ArticulationPoints;
struct APTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for ArticulationPoints {
    fn id(&self) -> &str {
        "graph_repr_articulation_points"
    }
    fn name(&self) -> &str {
        "Find Articulation Points"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find all articulation points (cut vertices) in an undirected graph.\n\n\
         An articulation point is a vertex whose removal disconnects the graph\n\
         (or increases the number of connected components).\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: Vec<usize> — sorted list of articulation point indices.\n\n\
         Constraints:\n\
         - 1 <= n <= 200\n\
         - No self-loops or duplicate edges."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=15);
                let extra = rng.random_range(0..=n);
                let edges = random_connected_edges(&mut rng, n, extra);
                TestCase {
                    data: Box::new(APTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<APTest>().unwrap();
        let expected = ref_articulation_points(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::articulation_points(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_articulation_points(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let adj = build_adj_undirected(n, edges);
    let mut disc = vec![0i32; n];
    let mut low = vec![0i32; n];
    let mut visited = vec![false; n];
    let mut is_ap = vec![false; n];
    let mut timer = 1i32;

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        u: usize,
        parent: i32,
        adj: &[Vec<usize>],
        disc: &mut Vec<i32>,
        low: &mut Vec<i32>,
        visited: &mut Vec<bool>,
        is_ap: &mut Vec<bool>,
        timer: &mut i32,
    ) {
        visited[u] = true;
        disc[u] = *timer;
        low[u] = *timer;
        *timer += 1;
        let mut children = 0;
        for &v in &adj[u] {
            if v as i32 == parent {
                continue;
            }
            if !visited[v] {
                children += 1;
                dfs(v, u as i32, adj, disc, low, visited, is_ap, timer);
                low[u] = low[u].min(low[v]);
                // Root with 2+ children
                if parent == -1 && children > 1 {
                    is_ap[u] = true;
                }
                // Non-root: if no back edge from subtree reaches above u
                if parent != -1 && low[v] >= disc[u] {
                    is_ap[u] = true;
                }
            } else {
                low[u] = low[u].min(disc[v]);
            }
        }
    }

    for i in 0..n {
        if !visited[i] {
            dfs(
                i,
                -1,
                &adj,
                &mut disc,
                &mut low,
                &mut visited,
                &mut is_ap,
                &mut timer,
            );
        }
    }
    let mut result: Vec<usize> = (0..n).filter(|&i| is_ap[i]).collect();
    result.sort();
    result
}

// ── Hard 14: Euler Path ───────────────────────────────────────────────

struct EulerPath;
struct EulerTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for EulerPath {
    fn id(&self) -> &str {
        "graph_repr_euler_path"
    }
    fn name(&self) -> &str {
        "Eulerian Path Exists"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Check if an Eulerian path exists in an undirected graph.\n\n\
         An Eulerian path visits every edge exactly once. It exists if and only if:\n\
         - The graph is connected (considering only nodes with degree > 0), AND\n\
         - There are exactly 0 or 2 nodes with odd degree.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: bool — true if an Eulerian path exists.\n\n\
         Constraints:\n\
         - 1 <= n <= 200\n\
         - At least one edge exists."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=15);
                let edge_count = rng.random_range(1..=n * 2);
                let edges = random_connected_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(EulerTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<EulerTest>().unwrap();
        let expected = ref_euler_path(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::euler_path(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_euler_path(n: usize, edges: &[(usize, usize)]) -> bool {
    if edges.is_empty() {
        return false;
    }
    let adj = build_adj_undirected(n, edges);

    // Check connectivity among nodes with degree > 0
    let start = (0..n).find(|&i| !adj[i].is_empty()).unwrap();
    let mut visited = vec![false; n];
    let mut stack = vec![start];
    while let Some(u) = stack.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        for &v in &adj[u] {
            if !visited[v] {
                stack.push(v);
            }
        }
    }
    // All nodes with edges must be visited
    for i in 0..n {
        if !adj[i].is_empty() && !visited[i] {
            return false;
        }
    }

    // Count nodes with odd degree
    let odd_count = (0..n).filter(|&i| !adj[i].len().is_multiple_of(2)).count();
    odd_count == 0 || odd_count == 2
}

// ── Hard 15: Graph Coloring ───────────────────────────────────────────

struct GraphColoring;
struct ColoringTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for GraphColoring {
    fn id(&self) -> &str {
        "graph_repr_graph_coloring"
    }
    fn name(&self) -> &str {
        "Graph Coloring (Greedy)"
    }
    fn topic(&self) -> &str {
        "graph_representations"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find the minimum number of colors needed to color a graph using a greedy\n\
         algorithm. Color nodes in order 0, 1, 2, ..., n-1, always assigning the\n\
         smallest color (starting from 1) not used by any neighbor.\n\n\
         This greedy approach does not always find the optimal coloring (chromatic\n\
         number), but it gives a valid upper bound.\n\n\
         Input: (n: usize, edges: Vec<(usize, usize)>)\n\
         Output: i32 — the number of distinct colors used.\n\n\
         Constraints:\n\
         - 1 <= n <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let edge_count = rng.random_range(0..=n * 2);
                let edges = random_undirected_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(ColoringTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ColoringTest>().unwrap();
        let expected = ref_graph_coloring(t.n, &t.edges);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let graph = TrackedGraph::new(t.n, &t.edges, false, shared_log.clone());
        let actual = solutions::graph_coloring(&graph);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_graph_coloring(n: usize, edges: &[(usize, usize)]) -> i32 {
    let adj = build_adj_undirected(n, edges);
    let mut colors = vec![0i32; n];
    let mut max_color = 0;
    for u in 0..n {
        let neighbor_colors: HashSet<i32> = adj[u].iter().map(|&v| colors[v]).collect();
        let mut c = 1;
        while neighbor_colors.contains(&c) {
            c += 1;
        }
        colors[u] = c;
        max_color = max_color.max(c);
    }
    max_color
}
