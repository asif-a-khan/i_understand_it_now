use rand::Rng;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part4_graphs::mst as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(MstMinCostConnect),
        Box::new(MstIsTree),
        Box::new(MstConnectedComponents),
        Box::new(MstMinSpanningWeight),
        Box::new(MstMaxEdgeInMst),
        Box::new(MstKruskal),
        Box::new(MstPrim),
        Box::new(MstMinCostRepairRoads),
        Box::new(MstSecondMst),
        Box::new(MstCriticalEdges),
        Box::new(MstMaxSpanningTree),
        Box::new(MstMinBottleneckPath),
        Box::new(MstOptimizeNetwork),
        Box::new(MstSteinerTree),
        Box::new(MstMinDegreeSpanningTree),
    ]
}

// ── Reference Union-Find for internal use ────────────────────────────

struct RefUF {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl RefUF {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
        true
    }
}

// ── Reference Kruskal ────────────────────────────────────────────────

fn ref_kruskal(n: usize, edges: &[(usize, usize, i32)]) -> (i32, Vec<(usize, usize, i32)>) {
    let mut sorted: Vec<(usize, usize, i32)> = edges.to_vec();
    sorted.sort_by_key(|e| e.2);
    let mut uf = RefUF::new(n);
    let mut total = 0i32;
    let mut mst_edges = Vec::new();
    for (u, v, w) in &sorted {
        if uf.union(*u, *v) {
            total += w;
            mst_edges.push((*u, *v, *w));
        }
    }
    (total, mst_edges)
}

// ── Reference Prim ───────────────────────────────────────────────────

fn ref_prim(n: usize, edges: &[(usize, usize, i32)]) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }
    let mut visited = vec![false; n];
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, 0)));
    let mut total = 0i32;
    let mut count = 0usize;
    while let Some(Reverse((w, u))) = heap.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        total += w;
        count += 1;
        for &(v, ew) in &adj[u] {
            if !visited[v] {
                heap.push(Reverse((ew, v)));
            }
        }
    }
    if count < n { -1 } else { total }
}

// ── Helper: generate random weighted edges for a connected graph ─────

fn random_connected_weighted_edges(
    rng: &mut impl Rng,
    n: usize,
    extra: usize,
) -> Vec<(usize, usize, i32)> {
    let mut edges = Vec::new();
    // Spanning tree: connect each node to a random earlier node
    for i in 1..n {
        let j = rng.random_range(0..i);
        let w = rng.random_range(1..=100);
        edges.push((j, i, w));
    }
    // Extra edges
    for _ in 0..extra {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v {
            let w = rng.random_range(1..=100);
            let (a, b) = if u < v { (u, v) } else { (v, u) };
            edges.push((a, b, w));
        }
    }
    edges
}

fn random_unweighted_edges(
    rng: &mut impl Rng,
    n: usize,
    count: usize,
) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for _ in 0..count {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v {
            let (a, b) = if u < v { (u, v) } else { (v, u) };
            edges.push((a, b));
        }
    }
    edges
}

// ── Easy 1: Min Cost to Connect All Points ───────────────────────────

struct MstMinCostConnect;

struct MstMinCostConnectTest {
    points: Vec<(i32, i32)>,
}

impl Problem for MstMinCostConnect {
    fn id(&self) -> &str { "mst_min_cost_connect" }
    fn name(&self) -> &str { "Min Cost to Connect All Points" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a list of 2D points, find the minimum cost to connect all points.\n\n\
         The cost of connecting two points (x1,y1) and (x2,y2) is |x1-x2| + |y1-y2| \
         (Manhattan distance).\n\n\
         Return the minimum total cost to make all points connected.\n\n\
         Constraints:\n\
         - 1 <= points.len() <= 50\n\
         - -100 <= x, y <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let points: Vec<(i32, i32)> = (0..n)
                .map(|_| (rng.random_range(-100..=100), rng.random_range(-100..=100)))
                .collect();
            TestCase { data: Box::new(MstMinCostConnectTest { points }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstMinCostConnectTest>().unwrap();
        let expected = ref_min_cost_connect(&t.points);
        let actual = solutions::min_cost_connect(&t.points);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("points={:?}", t.points),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_cost_connect(points: &[(i32, i32)]) -> i32 {
    let n = points.len();
    if n <= 1 {
        return 0;
    }
    // Build all edges, then Kruskal
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = (points[i].0 - points[j].0).abs() + (points[i].1 - points[j].1).abs();
            edges.push((i, j, dist));
        }
    }
    ref_kruskal(n, &edges).0
}

// ── Easy 2: Is Graph a Tree ──────────────────────────────────────────

struct MstIsTree;

struct MstIsTreeTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for MstIsTree {
    fn id(&self) -> &str { "mst_is_tree" }
    fn name(&self) -> &str { "Check if Graph is a Tree" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Check if an undirected graph is a tree.\n\n\
         A graph is a tree if it is connected, has no cycles, and has exactly n-1 edges.\n\n\
         Input: (n, edges) where n is the number of nodes (0-indexed) and edges is a list \
         of (u, v) pairs.\n\n\
         Constraints:\n\
         - 1 <= n <= 50\n\
         - 0 <= edges.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|i| {
            let n = rng.random_range(2..=20);
            let edges = if i < 5 {
                // Generate a tree (connected, n-1 edges)
                let mut e = Vec::new();
                for node in 1..n {
                    let parent = rng.random_range(0..node);
                    e.push((parent, node));
                }
                e
            } else {
                // Generate random edges (may or may not be a tree)
                let count = rng.random_range(0..=(n * 2));
                random_unweighted_edges(&mut rng, n, count)
            };
            TestCase { data: Box::new(MstIsTreeTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstIsTreeTest>().unwrap();
        let expected = ref_is_tree(t.n, &t.edges);
        let actual = solutions::is_tree(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_is_tree(n: usize, edges: &[(usize, usize)]) -> bool {
    if edges.len() != n - 1 {
        return false;
    }
    let mut uf = RefUF::new(n);
    for &(u, v) in edges {
        if !uf.union(u, v) {
            return false; // cycle detected
        }
    }
    true
}

// ── Easy 3: Connected Components ─────────────────────────────────────

struct MstConnectedComponents;

struct MstConnectedComponentsTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for MstConnectedComponents {
    fn id(&self) -> &str { "mst_connected_components" }
    fn name(&self) -> &str { "Count Connected Components" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Count the number of connected components in an undirected graph.\n\n\
         Input: (n, edges) where n is the number of nodes (0-indexed).\n\n\
         Constraints:\n\
         - 1 <= n <= 50\n\
         - 0 <= edges.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=30);
            let edge_count = rng.random_range(0..=(n * 2));
            let edges = random_unweighted_edges(&mut rng, n, edge_count);
            TestCase { data: Box::new(MstConnectedComponentsTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstConnectedComponentsTest>().unwrap();
        let expected = ref_connected_components(t.n, &t.edges);
        let actual = solutions::connected_components(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_connected_components(n: usize, edges: &[(usize, usize)]) -> usize {
    let mut uf = RefUF::new(n);
    for &(u, v) in edges {
        uf.union(u, v);
    }
    let mut count = 0;
    for i in 0..n {
        if uf.find(i) == i {
            count += 1;
        }
    }
    count
}

// ── Easy 4: Min Spanning Weight ──────────────────────────────────────

struct MstMinSpanningWeight;

struct MstMinSpanningWeightTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for MstMinSpanningWeight {
    fn id(&self) -> &str { "mst_min_spanning_weight" }
    fn name(&self) -> &str { "Total Weight of MST (Kruskal)" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Find the total weight of the Minimum Spanning Tree using Kruskal's algorithm.\n\n\
         Input: (n, edges) where edges are (u, v, weight). The graph is connected.\n\n\
         Return the total weight of the MST.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - n-1 <= edges.len() <= 200\n\
         - 1 <= weight <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=25);
            let extra = rng.random_range(0..=(n * 2));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            TestCase { data: Box::new(MstMinSpanningWeightTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstMinSpanningWeightTest>().unwrap();
        let expected = ref_kruskal(t.n, &t.edges).0;
        let actual = solutions::min_spanning_weight(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 5: Max Edge in MST ──────────────────────────────────────────

struct MstMaxEdgeInMst;

struct MstMaxEdgeInMstTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for MstMaxEdgeInMst {
    fn id(&self) -> &str { "mst_max_edge_in_mst" }
    fn name(&self) -> &str { "Maximum Edge Weight in MST" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Find the maximum edge weight in the Minimum Spanning Tree.\n\n\
         Input: (n, edges) where edges are (u, v, weight). The graph is connected.\n\n\
         This is also known as the \"bottleneck\" of the MST.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - n-1 <= edges.len() <= 200\n\
         - 1 <= weight <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=25);
            let extra = rng.random_range(0..=(n * 2));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            TestCase { data: Box::new(MstMaxEdgeInMstTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstMaxEdgeInMstTest>().unwrap();
        let expected = ref_max_edge_in_mst(t.n, &t.edges);
        let actual = solutions::max_edge_in_mst(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_edge_in_mst(n: usize, edges: &[(usize, usize, i32)]) -> i32 {
    let (_, mst_edges) = ref_kruskal(n, edges);
    mst_edges.iter().map(|e| e.2).max().unwrap_or(0)
}

// ── Medium 1: Kruskal's MST (return edges) ──────────────────────────

struct MstKruskal;

struct MstKruskalTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for MstKruskal {
    fn id(&self) -> &str { "mst_kruskal" }
    fn name(&self) -> &str { "Kruskal's MST: Return Edges" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Implement Kruskal's algorithm and return the edges in the MST.\n\n\
         Return a Vec<(usize, usize, i32)> of edges in the MST, sorted by \
         (weight, u, v) ascending. Each edge should have u < v.\n\n\
         Input: (n, edges) where edges are (u, v, weight). The graph is connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - n-1 <= edges.len() <= 200\n\
         - 1 <= weight <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=25);
            let extra = rng.random_range(0..=(n * 2));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            TestCase { data: Box::new(MstKruskalTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstKruskalTest>().unwrap();
        let expected = ref_kruskal_edges(t.n, &t.edges);
        let actual = solutions::kruskal(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_kruskal_edges(n: usize, edges: &[(usize, usize, i32)]) -> Vec<(usize, usize, i32)> {
    let (_, mst_edges) = ref_kruskal(n, edges);
    let mut result: Vec<(usize, usize, i32)> = mst_edges
        .into_iter()
        .map(|(u, v, w)| if u < v { (u, v, w) } else { (v, u, w) })
        .collect();
    result.sort_by(|a, b| a.2.cmp(&b.2).then(a.0.cmp(&b.0)).then(a.1.cmp(&b.1)));
    result
}

// ── Medium 2: Prim's MST ─────────────────────────────────────────────

struct MstPrim;

struct MstPrimTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for MstPrim {
    fn id(&self) -> &str { "mst_prim" }
    fn name(&self) -> &str { "Prim's MST: Total Weight" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Implement Prim's algorithm and return the total weight of the MST.\n\n\
         Start from node 0. Use a min-heap (priority queue).\n\n\
         Input: (n, edges) where edges are (u, v, weight). The graph is connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - n-1 <= edges.len() <= 200\n\
         - 1 <= weight <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=25);
            let extra = rng.random_range(0..=(n * 2));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            TestCase { data: Box::new(MstPrimTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstPrimTest>().unwrap();
        let expected = ref_prim(t.n, &t.edges);
        let actual = solutions::prim(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 3: Min Cost Repair Roads ──────────────────────────────────

struct MstMinCostRepairRoads;

struct MstMinCostRepairRoadsTest {
    n: usize,
    existing: Vec<(usize, usize)>,
    available: Vec<(usize, usize, i32)>,
}

impl Problem for MstMinCostRepairRoads {
    fn id(&self) -> &str { "mst_min_cost_repair_roads" }
    fn name(&self) -> &str { "Min Cost to Repair Roads" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "There are n cities. Some roads already exist (cost 0). Additional roads can be \
         built at given costs. Find the minimum cost to connect all cities.\n\n\
         Input: (n, existing_edges, available_edges) where existing_edges are (u, v) pairs \
         with cost 0, and available_edges are (u, v, cost) triples.\n\n\
         Return the minimum total cost, or -1 if impossible.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - 0 <= existing.len(), available.len() <= 200\n\
         - 1 <= cost <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=20);
            let existing_count = rng.random_range(0..n);
            let existing = random_unweighted_edges(&mut rng, n, existing_count);
            // Make sure available edges can connect everything
            let extra = rng.random_range(n..=(n * 3));
            let available = random_connected_weighted_edges(&mut rng, n, extra);
            TestCase {
                data: Box::new(MstMinCostRepairRoadsTest { n, existing, available }),
            }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstMinCostRepairRoadsTest>().unwrap();
        let expected = ref_min_cost_repair(t.n, &t.existing, &t.available);
        let actual = solutions::min_cost_repair_roads(t.n, &t.existing, &t.available);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!(
                "n={}, existing={:?}, available={:?}",
                t.n, t.existing, t.available
            ),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_cost_repair(
    n: usize,
    existing: &[(usize, usize)],
    available: &[(usize, usize, i32)],
) -> i32 {
    let mut uf = RefUF::new(n);
    for &(u, v) in existing {
        uf.union(u, v);
    }
    let mut sorted = available.to_vec();
    sorted.sort_by_key(|e| e.2);
    let mut total = 0i32;
    for (u, v, w) in &sorted {
        if uf.union(*u, *v) {
            total += w;
        }
    }
    // Check if connected
    let root = uf.find(0);
    for i in 1..n {
        if uf.find(i) != root {
            return -1;
        }
    }
    total
}

// ── Medium 4: Second-Best MST ────────────────────────────────────────

struct MstSecondMst;

struct MstSecondMstTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for MstSecondMst {
    fn id(&self) -> &str { "mst_second_mst" }
    fn name(&self) -> &str { "Second-Best MST Weight" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Find the weight of the second-best MST.\n\n\
         The second-best MST is the spanning tree with the smallest total weight that is \
         strictly greater than the MST weight. If no such tree exists (only one spanning \
         tree), return the MST weight.\n\n\
         Input: (n, edges) where edges are (u, v, weight). The graph is connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 20\n\
         - n-1 <= edges.len() <= 100\n\
         - 1 <= weight <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=12);
            let extra = rng.random_range(1..=(n * 2));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            TestCase { data: Box::new(MstSecondMstTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstSecondMstTest>().unwrap();
        let expected = ref_second_mst(t.n, &t.edges);
        let actual = solutions::second_mst(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_second_mst(n: usize, edges: &[(usize, usize, i32)]) -> i32 {
    let mst_weight = ref_kruskal(n, edges).0;
    let mut best_second = i32::MAX;

    // Try removing each edge from the MST and finding a new spanning tree
    let mut sorted: Vec<(usize, (usize, usize, i32))> = edges.iter().copied().enumerate().collect();
    sorted.sort_by_key(|e| (e.1).2);

    // Get MST edge indices
    let mut uf = RefUF::new(n);
    let mut mst_indices = Vec::new();
    for &(idx, (u, v, _w)) in &sorted {
        if uf.union(u, v) {
            mst_indices.push(idx);
        }
    }

    // For each MST edge, try building spanning tree without it
    for &skip_idx in &mst_indices {
        let mut uf2 = RefUF::new(n);
        let mut total = 0i32;
        let mut count = 0usize;
        for &(idx, (u, v, w)) in &sorted {
            if idx == skip_idx {
                continue;
            }
            if uf2.union(u, v) {
                total += w;
                count += 1;
            }
        }
        if count == n - 1 && total > mst_weight {
            best_second = best_second.min(total);
        }
    }

    if best_second == i32::MAX {
        mst_weight
    } else {
        best_second
    }
}

// ── Medium 5: Critical Edges in MST ─────────────────────────────────

struct MstCriticalEdges;

struct MstCriticalEdgesTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for MstCriticalEdges {
    fn id(&self) -> &str { "mst_critical_edges" }
    fn name(&self) -> &str { "Critical Edges in MST" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Find all critical edges -- edges that MUST be in every MST.\n\n\
         An edge is critical if removing it increases the MST weight (or disconnects the graph).\n\n\
         Return the indices of critical edges, sorted in ascending order.\n\n\
         Input: (n, edges) where edges are (u, v, weight). The graph is connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 20\n\
         - n-1 <= edges.len() <= 100\n\
         - 1 <= weight <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=12);
            let extra = rng.random_range(0..=(n * 2));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            TestCase { data: Box::new(MstCriticalEdgesTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstCriticalEdgesTest>().unwrap();
        let expected = ref_critical_edges(t.n, &t.edges);
        let actual = solutions::critical_edges(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_critical_edges(n: usize, edges: &[(usize, usize, i32)]) -> Vec<usize> {
    let mst_weight = ref_kruskal(n, edges).0;
    let mut critical = Vec::new();

    let _sorted_with_idx: Vec<(usize, (usize, usize, i32))> =
        edges.iter().copied().enumerate().collect();

    for skip in 0..edges.len() {
        let mut uf = RefUF::new(n);
        let mut total = 0i32;
        let mut count = 0usize;
        let mut remaining: Vec<(usize, usize, i32)> = edges
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != skip)
            .map(|(_, e)| *e)
            .collect();
        remaining.sort_by_key(|e| e.2);
        for (u, v, w) in &remaining {
            if uf.union(*u, *v) {
                total += w;
                count += 1;
            }
        }
        if count < n - 1 || total > mst_weight {
            critical.push(skip);
        }
    }

    critical.sort();
    critical
}

// ── Hard 1: Maximum Spanning Tree ────────────────────────────────────

struct MstMaxSpanningTree;

struct MstMaxSpanningTreeTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for MstMaxSpanningTree {
    fn id(&self) -> &str { "mst_max_spanning_tree" }
    fn name(&self) -> &str { "Maximum Spanning Tree Weight" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Find the total weight of the Maximum Spanning Tree.\n\n\
         Like Kruskal's, but pick the heaviest edges first instead of lightest.\n\n\
         Input: (n, edges) where edges are (u, v, weight). The graph is connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - n-1 <= edges.len() <= 200\n\
         - 1 <= weight <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=25);
            let extra = rng.random_range(0..=(n * 2));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            TestCase { data: Box::new(MstMaxSpanningTreeTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstMaxSpanningTreeTest>().unwrap();
        let expected = ref_max_spanning_tree(t.n, &t.edges);
        let actual = solutions::max_spanning_tree(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_spanning_tree(n: usize, edges: &[(usize, usize, i32)]) -> i32 {
    let mut sorted = edges.to_vec();
    sorted.sort_by(|a, b| b.2.cmp(&a.2)); // sort descending by weight
    let mut uf = RefUF::new(n);
    let mut total = 0i32;
    for (u, v, w) in &sorted {
        if uf.union(*u, *v) {
            total += w;
        }
    }
    total
}

// ── Hard 2: Min Bottleneck Path ──────────────────────────────────────

struct MstMinBottleneckPath;

struct MstMinBottleneckPathTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
    src: usize,
    dst: usize,
}

impl Problem for MstMinBottleneckPath {
    fn id(&self) -> &str { "mst_min_bottleneck_path" }
    fn name(&self) -> &str { "Min Bottleneck Path" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Find the min bottleneck path between src and dst.\n\n\
         The bottleneck of a path is the maximum edge weight along that path.\n\
         Find the path from src to dst that minimizes this bottleneck value.\n\n\
         Key insight: the MST contains the min bottleneck path between any two nodes.\n\n\
         Input: (n, edges, src, dst) where edges are (u, v, weight). The graph is connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - n-1 <= edges.len() <= 200\n\
         - 1 <= weight <= 100\n\
         - 0 <= src, dst < n, src != dst"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=25);
            let extra = rng.random_range(0..=(n * 2));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            let src = rng.random_range(0..n);
            let mut dst = rng.random_range(0..n);
            while dst == src {
                dst = rng.random_range(0..n);
            }
            TestCase {
                data: Box::new(MstMinBottleneckPathTest { n, edges, src, dst }),
            }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstMinBottleneckPathTest>().unwrap();
        let expected = ref_min_bottleneck_path(t.n, &t.edges, t.src, t.dst);
        let actual = solutions::min_bottleneck_path(t.n, &t.edges, t.src, t.dst);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!(
                "n={}, edges={:?}, src={}, dst={}",
                t.n, t.edges, t.src, t.dst
            ),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_bottleneck_path(n: usize, edges: &[(usize, usize, i32)], src: usize, dst: usize) -> i32 {
    // Build MST, then find path in MST from src to dst, return max edge on that path
    let (_, mst_edges) = ref_kruskal(n, edges);
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in &mst_edges {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }
    // DFS to find path and track max edge
    let mut visited = vec![false; n];
    let mut result = -1i32;
    fn dfs(
        node: usize,
        target: usize,
        max_so_far: i32,
        adj: &[Vec<(usize, i32)>],
        visited: &mut [bool],
        result: &mut i32,
    ) -> bool {
        if node == target {
            *result = max_so_far;
            return true;
        }
        visited[node] = true;
        for &(next, w) in &adj[node] {
            if !visited[next] {
                if dfs(next, target, max_so_far.max(w), adj, visited, result) {
                    return true;
                }
            }
        }
        false
    }
    dfs(src, dst, 0, &adj, &mut visited, &mut result);
    result
}

// ── Hard 3: Optimize Network ─────────────────────────────────────────

struct MstOptimizeNetwork;

struct MstOptimizeNetworkTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for MstOptimizeNetwork {
    fn id(&self) -> &str { "mst_optimize_network" }
    fn name(&self) -> &str { "Optimize Network (Remove Redundant Edges)" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Remove redundant edges from a connected weighted graph while keeping it connected \
         and minimizing total weight.\n\n\
         This is equivalent to finding the MST weight (since the MST is the minimum-weight \
         connected subgraph). Return the total weight of edges to KEEP.\n\n\
         Input: (n, edges) where edges are (u, v, weight). The graph is connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - n-1 <= edges.len() <= 200\n\
         - 1 <= weight <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=25);
            let extra = rng.random_range(2..=(n * 3));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            TestCase { data: Box::new(MstOptimizeNetworkTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstOptimizeNetworkTest>().unwrap();
        let expected = ref_kruskal(t.n, &t.edges).0;
        let actual = solutions::optimize_network(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 4: Approximate Steiner Tree ─────────────────────────────────

struct MstSteinerTree;

struct MstSteinerTreeTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
    terminals: Vec<usize>,
}

impl Problem for MstSteinerTree {
    fn id(&self) -> &str { "mst_steiner_tree" }
    fn name(&self) -> &str { "Approximate Steiner Tree" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Approximate the Steiner tree for a subset of terminal nodes.\n\n\
         A Steiner tree connects all terminal nodes with minimum total weight, possibly \
         using non-terminal nodes as intermediate points.\n\n\
         Use this approximation: compute shortest paths between all pairs of terminals, \
         then find the MST of the complete graph on terminals using those shortest-path \
         distances. Return the total weight of this MST.\n\n\
         Input: (n, edges, terminals) where edges are (u, v, weight). The graph is connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 30\n\
         - n-1 <= edges.len() <= 200\n\
         - 2 <= terminals.len() <= min(n, 10)\n\
         - 1 <= weight <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(4..=20);
            let extra = rng.random_range(0..=(n * 2));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            let t_count = rng.random_range(2..=n.min(8));
            let mut terminals: Vec<usize> = (0..n).collect();
            // Shuffle and take first t_count
            for i in (1..terminals.len()).rev() {
                let j = rng.random_range(0..=i);
                terminals.swap(i, j);
            }
            terminals.truncate(t_count);
            terminals.sort();
            TestCase {
                data: Box::new(MstSteinerTreeTest { n, edges, terminals }),
            }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstSteinerTreeTest>().unwrap();
        let expected = ref_steiner_tree(t.n, &t.edges, &t.terminals);
        let actual = solutions::steiner_tree(t.n, &t.edges, &t.terminals);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!(
                "n={}, edges={:?}, terminals={:?}",
                t.n, t.edges, t.terminals
            ),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_steiner_tree(n: usize, edges: &[(usize, usize, i32)], terminals: &[usize]) -> i32 {
    // Dijkstra from each terminal, then MST on the terminal graph
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let dijkstra = |start: usize| -> Vec<i32> {
        let mut dist = vec![i32::MAX; n];
        dist[start] = 0;
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        heap.push(Reverse((0, start)));
        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &adj[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }
        dist
    };

    let t_count = terminals.len();
    // Build complete graph on terminals
    let mut terminal_edges = Vec::new();
    let dists: Vec<Vec<i32>> = terminals.iter().map(|&t| dijkstra(t)).collect();
    for i in 0..t_count {
        for j in (i + 1)..t_count {
            let d = dists[i][terminals[j]];
            terminal_edges.push((i, j, d));
        }
    }

    ref_kruskal(t_count, &terminal_edges).0
}

// ── Hard 5: MST with Min Max Degree (Approximation) ─────────────────

struct MstMinDegreeSpanningTree;

struct MstMinDegreeSpanningTreeTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for MstMinDegreeSpanningTree {
    fn id(&self) -> &str { "mst_min_degree_spanning_tree" }
    fn name(&self) -> &str { "Min-Degree Spanning Tree (Approximation)" }
    fn topic(&self) -> &str { "mst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Find a spanning tree that minimizes the maximum degree of any node.\n\n\
         This is NP-hard in general, so use this approximation:\n\
         1. Build the MST.\n\
         2. Return the maximum degree of any node in the MST.\n\n\
         Input: (n, edges) where edges are (u, v, weight). The graph is connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - n-1 <= edges.len() <= 200\n\
         - 1 <= weight <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=25);
            let extra = rng.random_range(0..=(n * 2));
            let edges = random_connected_weighted_edges(&mut rng, n, extra);
            TestCase { data: Box::new(MstMinDegreeSpanningTreeTest { n, edges }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MstMinDegreeSpanningTreeTest>().unwrap();
        let expected = ref_min_degree_spanning_tree(t.n, &t.edges);
        let actual = solutions::min_degree_spanning_tree(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_degree_spanning_tree(n: usize, edges: &[(usize, usize, i32)]) -> i32 {
    let (_, mst_edges) = ref_kruskal(n, edges);
    let mut degree = vec![0i32; n];
    for &(u, v, _) in &mst_edges {
        degree[u] += 1;
        degree[v] += 1;
    }
    degree.into_iter().max().unwrap_or(0)
}
