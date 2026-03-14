use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part4_graphs::shortest_path as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(ShortestPathUnweighted),
        Box::new(ShortestPathBinaryGrid),
        Box::new(ShortestPathMinSteps),
        Box::new(ShortestPathNetworkDelay),
        Box::new(ShortestPathCityFewestNeighbors),
        Box::new(ShortestPathDijkstra),
        Box::new(ShortestPathBellmanFord),
        Box::new(ShortestPathCheapestFlights),
        Box::new(ShortestPathMinEffort),
        Box::new(ShortestPathMaze),
        Box::new(ShortestPathFloydWarshall),
        Box::new(ShortestPathKShortest),
        Box::new(ShortestPathMinCostConnectAll),
        Box::new(ShortestPathReconstruct),
        Box::new(ShortestPathAlternatingColors),
    ]
}

// ── Helpers ─────────────────────────────────────────────────────────────

const NO_PATH: i32 = i32::MAX;

/// Generate a random connected weighted directed graph as edge list.
fn gen_connected_weighted_directed(
    rng: &mut impl Rng,
    n: usize,
    extra: usize,
    w_lo: i32,
    w_hi: i32,
) -> Vec<(usize, usize, i32)> {
    let mut edges = Vec::new();
    // Spanning tree (directed forward)
    for i in 1..n {
        let j = rng.random_range(0..i);
        let w = rng.random_range(w_lo..=w_hi);
        edges.push((j, i, w));
    }
    // Extra edges
    for _ in 0..extra {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v {
            let w = rng.random_range(w_lo..=w_hi);
            edges.push((u, v, w));
        }
    }
    edges
}

/// Generate a random connected undirected weighted graph as edge list.
fn gen_connected_weighted_undirected(
    rng: &mut impl Rng,
    n: usize,
    extra: usize,
    w_lo: i32,
    w_hi: i32,
) -> Vec<(usize, usize, i32)> {
    let mut edges = Vec::new();
    for i in 1..n {
        let j = rng.random_range(0..i);
        let w = rng.random_range(w_lo..=w_hi);
        edges.push((j, i, w));
    }
    for _ in 0..extra {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v {
            let w = rng.random_range(w_lo..=w_hi);
            edges.push((u, v, w));
        }
    }
    edges
}

/// Generate unweighted undirected edge list.
fn gen_connected_unweighted(rng: &mut impl Rng, n: usize, extra: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for i in 1..n {
        let j = rng.random_range(0..i);
        edges.push((j, i));
    }
    for _ in 0..extra {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v {
            edges.push((u, v));
        }
    }
    edges
}

// ── Easy 1: BFS Shortest Path (Unweighted) ─────────────────────────────

struct ShortestPathUnweighted;

struct UnweightedTest {
    n: usize,
    edges: Vec<(usize, usize)>,
    src: usize,
    dst: usize,
}

impl Problem for ShortestPathUnweighted {
    fn id(&self) -> &str {
        "shortest_path_unweighted"
    }
    fn name(&self) -> &str {
        "BFS Shortest Path (Unweighted)"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an undirected unweighted graph with `n` nodes and edges, find the shortest \
         path (number of edges) from `src` to `dst`. Return -1 if no path exists.\n\n\
         Constraints:\n\
         - 1 <= n <= 1000\n\
         - 0 <= src, dst < n"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let extra = rng.random_range(0..=n);
                let edges = gen_connected_unweighted(&mut rng, n, extra);
                let src = rng.random_range(0..n);
                let dst = rng.random_range(0..n);
                TestCase {
                    data: Box::new(UnweightedTest { n, edges, src, dst }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UnweightedTest>().unwrap();
        let expected = ref_bfs_shortest(t.n, &t.edges, t.src, t.dst);
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

fn ref_bfs_shortest(n: usize, edges: &[(usize, usize)], src: usize, dst: usize) -> i32 {
    if src == dst {
        return 0;
    }
    use std::collections::VecDeque;
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut dist = vec![-1i32; n];
    dist[src] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(src);
    while let Some(node) = queue.pop_front() {
        for &next in &adj[node] {
            if dist[next] == -1 {
                dist[next] = dist[node] + 1;
                if next == dst {
                    return dist[next];
                }
                queue.push_back(next);
            }
        }
    }
    -1
}

// ── Easy 2: Shortest Path in Binary Grid ────────────────────────────────

struct ShortestPathBinaryGrid;

struct BinaryGridTest {
    grid: Vec<Vec<i32>>,
}

impl Problem for ShortestPathBinaryGrid {
    fn id(&self) -> &str {
        "shortest_path_binary_grid"
    }
    fn name(&self) -> &str {
        "Shortest Path in Binary Grid"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an n x n binary grid where 0 is passable and 1 is blocked, return the \
         length of the shortest path from (0,0) to (n-1,n-1). The path can move in 8 \
         directions (including diagonals). Return -1 if no path exists.\n\n\
         The path length includes both endpoints.\n\n\
         Constraints:\n\
         - 1 <= n <= 100\n\
         - grid[i][j] is 0 or 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=8);
                let mut grid: Vec<Vec<i32>> = (0..n)
                    .map(|_| {
                        (0..n)
                            .map(|_| if rng.random_range(0..=3) == 0 { 1 } else { 0 })
                            .collect()
                    })
                    .collect();
                grid[0][0] = 0;
                grid[n - 1][n - 1] = 0;
                TestCase {
                    data: Box::new(BinaryGridTest { grid }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BinaryGridTest>().unwrap();
        let expected = ref_shortest_binary_grid(&t.grid);
        let actual = solutions::shortest_path_binary_grid(&t.grid);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_shortest_binary_grid(grid: &[Vec<i32>]) -> i32 {
    use std::collections::VecDeque;
    let n = grid.len();
    if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
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
                let nr = nr as usize;
                let nc = nc as usize;
                if !visited[nr][nc] && grid[nr][nc] == 0 {
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

// ── Easy 3: Minimum Steps on Number Line ────────────────────────────────

struct ShortestPathMinSteps;

struct MinStepsTest {
    start: i32,
    target: i32,
}

impl Problem for ShortestPathMinSteps {
    fn id(&self) -> &str {
        "shortest_path_min_steps"
    }
    fn name(&self) -> &str {
        "Minimum Steps to Reach Target"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "On an infinite number line, you start at position `start`. Each step, you can move \
         left or right by exactly 1. Return the minimum number of steps to reach `target`.\n\n\
         Constraints:\n\
         - -10^9 <= start, target <= 10^9"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let start = rng.random_range(-100..=100);
                let target = rng.random_range(-100..=100);
                TestCase {
                    data: Box::new(MinStepsTest { start, target }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinStepsTest>().unwrap();
        let expected = (t.target - t.start).abs();
        let actual = solutions::min_steps(t.start, t.target);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("start={}, target={}", t.start, t.target),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 4: Network Delay Time ──────────────────────────────────────────

struct ShortestPathNetworkDelay;

struct NetworkDelayTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
    src: usize,
}

impl Problem for ShortestPathNetworkDelay {
    fn id(&self) -> &str {
        "shortest_path_network_delay"
    }
    fn name(&self) -> &str {
        "Network Delay Time"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "There are `n` network nodes labeled 0..n-1. Given directed weighted edges \
         (u, v, time), a signal is sent from node `src`. Return the minimum time it takes \
         for all nodes to receive the signal. Return -1 if not all nodes are reachable.\n\n\
         Constraints:\n\
         - 1 <= n <= 100\n\
         - 1 <= time <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=10);
                let extra = rng.random_range(0..=n);
                let edges = gen_connected_weighted_directed(&mut rng, n, extra, 1, 50);
                let src = rng.random_range(0..n);
                TestCase {
                    data: Box::new(NetworkDelayTest { n, edges, src }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NetworkDelayTest>().unwrap();
        let expected = ref_network_delay(t.n, &t.edges, t.src);
        let actual = solutions::network_delay(t.n, &t.edges, t.src);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}, src={}", t.n, t.edges, t.src),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_network_delay(n: usize, edges: &[(usize, usize, i32)], src: usize) -> i32 {
    let dist = ref_dijkstra(n, edges, src);
    let max_dist = *dist.iter().max().unwrap();
    if max_dist == NO_PATH {
        -1
    } else {
        max_dist
    }
}

fn ref_dijkstra(n: usize, edges: &[(usize, usize, i32)], src: usize) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
    }
    let mut dist = vec![NO_PATH; n];
    dist[src] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i32, src)));
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
}

// ── Easy 5: City with Fewest Reachable Neighbors ────────────────────────

struct ShortestPathCityFewestNeighbors;

struct CityFewestNeighborsTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
    threshold: i32,
}

impl Problem for ShortestPathCityFewestNeighbors {
    fn id(&self) -> &str {
        "shortest_path_city_fewest_neighbors"
    }
    fn name(&self) -> &str {
        "City with Fewest Reachable Neighbors"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given `n` cities connected by undirected weighted roads (edges), find the city with \
         the smallest number of other cities reachable within `threshold` distance. If there \
         are ties, return the city with the greatest number (index).\n\n\
         Constraints:\n\
         - 2 <= n <= 100\n\
         - 1 <= weight <= 10000\n\
         - 0 <= threshold <= 10000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=8);
                let extra = rng.random_range(0..=n);
                let edges = gen_connected_weighted_undirected(&mut rng, n, extra, 1, 50);
                let threshold = rng.random_range(10..=200);
                TestCase {
                    data: Box::new(CityFewestNeighborsTest {
                        n,
                        edges,
                        threshold,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CityFewestNeighborsTest>().unwrap();
        let expected = ref_city_fewest_neighbors(t.n, &t.edges, t.threshold);
        let actual = solutions::city_fewest_neighbors(t.n, &t.edges, t.threshold);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}, threshold={}", t.n, t.edges, t.threshold),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_city_fewest_neighbors(n: usize, edges: &[(usize, usize, i32)], threshold: i32) -> i32 {
    // Build undirected edge list for Dijkstra (add both directions)
    let mut all_edges = Vec::new();
    for &(u, v, w) in edges {
        all_edges.push((u, v, w));
        all_edges.push((v, u, w));
    }

    let mut best_city = 0;
    let mut best_count = usize::MAX;

    for city in 0..n {
        let dist = ref_dijkstra(n, &all_edges, city);
        let count = dist
            .iter()
            .filter(|&&d| d != NO_PATH && d <= threshold && d > 0)
            .count();
        if count <= best_count {
            best_count = count;
            best_city = city;
        }
    }
    best_city as i32
}

// ── Medium 6: Dijkstra's Algorithm ─────────────────────────────────────

struct ShortestPathDijkstra;

struct DijkstraTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
    src: usize,
}

impl Problem for ShortestPathDijkstra {
    fn id(&self) -> &str {
        "shortest_path_dijkstra"
    }
    fn name(&self) -> &str {
        "Dijkstra's Single-Source Shortest Paths"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Implement Dijkstra's algorithm. Given a directed weighted graph with `n` nodes, \
         edges (u, v, weight), and a source node `src`, return an array where result[i] is \
         the shortest distance from `src` to node `i`. Use i32::MAX for unreachable nodes.\n\n\
         Constraints:\n\
         - 1 <= n <= 1000\n\
         - 0 <= weight (non-negative weights)\n\
         - 0 <= src < n"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let extra = rng.random_range(0..=n * 2);
                let edges = gen_connected_weighted_directed(&mut rng, n, extra, 1, 100);
                let src = rng.random_range(0..n);
                TestCase {
                    data: Box::new(DijkstraTest { n, edges, src }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DijkstraTest>().unwrap();
        let expected = ref_dijkstra(t.n, &t.edges, t.src);
        let actual = solutions::dijkstra(t.n, &t.edges, t.src);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}, src={}", t.n, t.edges, t.src),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 7: Bellman-Ford ─────────────────────────────────────────────

struct ShortestPathBellmanFord;

struct BellmanFordTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
    src: usize,
}

impl Problem for ShortestPathBellmanFord {
    fn id(&self) -> &str {
        "shortest_path_bellman_ford"
    }
    fn name(&self) -> &str {
        "Bellman-Ford Algorithm"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Implement the Bellman-Ford algorithm. Given a directed weighted graph that may \
         contain negative weight edges (but no negative cycles), return the shortest \
         distances from `src` to all nodes. Use i32::MAX for unreachable nodes.\n\n\
         Constraints:\n\
         - 1 <= n <= 500\n\
         - Weights can be negative\n\
         - No negative-weight cycles"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=10);
                let extra = rng.random_range(0..=n);
                // Use DAG-like structure to avoid negative cycles
                let mut edges = Vec::new();
                for i in 1..n {
                    let j = rng.random_range(0..i);
                    let w = rng.random_range(-20..=50);
                    edges.push((j, i, w));
                }
                for _ in 0..extra {
                    let u = rng.random_range(0..n);
                    let v = rng.random_range(0..n);
                    if u < v {
                        let w = rng.random_range(-20..=50);
                        edges.push((u, v, w));
                    }
                }
                let src = rng.random_range(0..n);
                TestCase {
                    data: Box::new(BellmanFordTest { n, edges, src }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BellmanFordTest>().unwrap();
        let expected = ref_bellman_ford(t.n, &t.edges, t.src);
        let actual = solutions::bellman_ford(t.n, &t.edges, t.src);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}, src={}", t.n, t.edges, t.src),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_bellman_ford(n: usize, edges: &[(usize, usize, i32)], src: usize) -> Vec<i32> {
    let mut dist = vec![NO_PATH; n];
    dist[src] = 0;
    for _ in 0..n - 1 {
        let mut updated = false;
        for &(u, v, w) in edges {
            if dist[u] != NO_PATH && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                updated = true;
            }
        }
        if !updated {
            break;
        }
    }
    dist
}

// ── Medium 8: Cheapest Flights Within K Stops ──────────────────────────

struct ShortestPathCheapestFlights;

struct CheapestFlightsTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
    src: usize,
    dst: usize,
    k: usize,
}

impl Problem for ShortestPathCheapestFlights {
    fn id(&self) -> &str {
        "shortest_path_cheapest_flights"
    }
    fn name(&self) -> &str {
        "Cheapest Flights Within K Stops"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given `n` cities and directed flights (u, v, price), find the cheapest price from \
         `src` to `dst` with at most `k` stops (intermediate cities). Return -1 if no such \
         route exists.\n\n\
         Constraints:\n\
         - 1 <= n <= 100\n\
         - 0 <= k < n\n\
         - 1 <= price <= 10000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=10);
                let extra = rng.random_range(0..=n * 2);
                let edges = gen_connected_weighted_directed(&mut rng, n, extra, 1, 100);
                let src = rng.random_range(0..n);
                let mut dst = rng.random_range(0..n);
                while dst == src {
                    dst = rng.random_range(0..n);
                }
                let k = rng.random_range(0..n.min(5));
                TestCase {
                    data: Box::new(CheapestFlightsTest {
                        n,
                        edges,
                        src,
                        dst,
                        k,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CheapestFlightsTest>().unwrap();
        let expected = ref_cheapest_flights(t.n, &t.edges, t.src, t.dst, t.k);
        let actual = solutions::cheapest_flights(t.n, &t.edges, t.src, t.dst, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!(
                "n={}, flights={:?}, src={}, dst={}, k={}",
                t.n, t.edges, t.src, t.dst, t.k
            ),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_cheapest_flights(
    n: usize,
    edges: &[(usize, usize, i32)],
    src: usize,
    dst: usize,
    k: usize,
) -> i32 {
    // Modified Bellman-Ford: relax at most k+1 times
    let mut dist = vec![NO_PATH; n];
    dist[src] = 0;
    for _ in 0..=k {
        let prev = dist.clone();
        for &(u, v, w) in edges {
            if prev[u] != NO_PATH && prev[u] + w < dist[v] {
                dist[v] = prev[u] + w;
            }
        }
    }
    if dist[dst] == NO_PATH {
        -1
    } else {
        dist[dst]
    }
}

// ── Medium 9: Path with Minimum Effort ─────────────────────────────────

struct ShortestPathMinEffort;

struct MinEffortTest {
    heights: Vec<Vec<i32>>,
}

impl Problem for ShortestPathMinEffort {
    fn id(&self) -> &str {
        "shortest_path_path_with_min_effort"
    }
    fn name(&self) -> &str {
        "Path with Minimum Effort"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an m x n grid of `heights`, return the minimum effort to travel from the \
         top-left to the bottom-right. The effort of a path is the maximum absolute \
         difference in heights between consecutive cells. Movement is 4-directional.\n\n\
         Constraints:\n\
         - 1 <= rows, cols <= 100\n\
         - 1 <= heights[i][j] <= 10^6"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(2..=8);
                let cols = rng.random_range(2..=8);
                let heights: Vec<Vec<i32>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(1..=100)).collect())
                    .collect();
                TestCase {
                    data: Box::new(MinEffortTest { heights }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinEffortTest>().unwrap();
        let expected = ref_min_effort(&t.heights);
        let actual = solutions::path_with_min_effort(&t.heights);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("heights={:?}", t.heights),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_effort(heights: &[Vec<i32>]) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let rows = heights.len();
    let cols = heights[0].len();
    let mut dist = vec![vec![i32::MAX; cols]; rows];
    dist[0][0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i32, 0usize, 0usize)));

    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some(Reverse((effort, r, c))) = heap.pop() {
        if r == rows - 1 && c == cols - 1 {
            return effort;
        }
        if effort > dist[r][c] {
            continue;
        }
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                let new_effort = effort.max((heights[nr][nc] - heights[r][c]).abs());
                if new_effort < dist[nr][nc] {
                    dist[nr][nc] = new_effort;
                    heap.push(Reverse((new_effort, nr, nc)));
                }
            }
        }
    }
    dist[rows - 1][cols - 1]
}

// ── Medium 10: Maze (Ball Rolling) ─────────────────────────────────────

struct ShortestPathMaze;

struct MazeTest {
    maze: Vec<Vec<i32>>,
    start: (usize, usize),
    dest: (usize, usize),
}

impl Problem for ShortestPathMaze {
    fn id(&self) -> &str {
        "shortest_path_maze"
    }
    fn name(&self) -> &str {
        "The Maze II (Shortest Distance)"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "A ball is in a maze represented as a grid. 0 = empty, 1 = wall. The ball rolls \
         in a chosen direction until hitting a wall, then stops. Find the shortest distance \
         (number of cells rolled) from `start` to `dest`. Return -1 if impossible.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 100\n\
         - start and dest are empty cells"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(3..=8);
                let cols = rng.random_range(3..=8);
                let maze: Vec<Vec<i32>> = (0..rows)
                    .map(|_| {
                        (0..cols)
                            .map(|_| if rng.random_range(0..=3) == 0 { 1 } else { 0 })
                            .collect()
                    })
                    .collect();
                let start = (rng.random_range(0..rows), rng.random_range(0..cols));
                let mut dest = (rng.random_range(0..rows), rng.random_range(0..cols));
                // Ensure start and dest are empty and different
                let mut maze = maze;
                maze[start.0][start.1] = 0;
                maze[dest.0][dest.1] = 0;
                while start == dest {
                    dest = (rng.random_range(0..rows), rng.random_range(0..cols));
                    maze[dest.0][dest.1] = 0;
                }
                TestCase {
                    data: Box::new(MazeTest { maze, start, dest }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MazeTest>().unwrap();
        let expected = ref_maze_shortest(&t.maze, t.start, t.dest);
        let actual = solutions::shortest_path_maze(&t.maze, t.start, t.dest);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("maze={:?}, start={:?}, dest={:?}", t.maze, t.start, t.dest),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_maze_shortest(maze: &[Vec<i32>], start: (usize, usize), dest: (usize, usize)) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let rows = maze.len();
    let cols = maze[0].len();
    let mut dist = vec![vec![i32::MAX; cols]; rows];
    dist[start.0][start.1] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i32, start.0, start.1)));

    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some(Reverse((d, r, c))) = heap.pop() {
        if d > dist[r][c] {
            continue;
        }
        if (r, c) == dest {
            return d;
        }
        for (dr, dc) in &dirs {
            let mut nr = r as i32;
            let mut nc = c as i32;
            let mut steps = 0;
            // Roll until hitting a wall
            loop {
                let nnr = nr + dr;
                let nnc = nc + dc;
                if nnr < 0
                    || nnr >= rows as i32
                    || nnc < 0
                    || nnc >= cols as i32
                    || maze[nnr as usize][nnc as usize] == 1
                {
                    break;
                }
                nr = nnr;
                nc = nnc;
                steps += 1;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if steps > 0 && d + steps < dist[nr][nc] {
                dist[nr][nc] = d + steps;
                heap.push(Reverse((d + steps, nr, nc)));
            }
        }
    }
    if dist[dest.0][dest.1] == i32::MAX {
        -1
    } else {
        dist[dest.0][dest.1]
    }
}

// ── Hard 11: Floyd-Warshall ────────────────────────────────────────────

struct ShortestPathFloydWarshall;

struct FloydWarshallTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for ShortestPathFloydWarshall {
    fn id(&self) -> &str {
        "shortest_path_floyd_warshall"
    }
    fn name(&self) -> &str {
        "Floyd-Warshall (All-Pairs Shortest Paths)"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Implement the Floyd-Warshall algorithm. Given a directed weighted graph with `n` \
         nodes and edges (u, v, weight), return an n x n matrix where result[i][j] is the \
         shortest distance from node i to node j. Use i32::MAX for unreachable pairs. \
         Distance from a node to itself is 0.\n\n\
         Constraints:\n\
         - 1 <= n <= 200\n\
         - Weights can be negative (but no negative cycles)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=8);
                let extra = rng.random_range(0..=n);
                // DAG structure to avoid negative cycles
                let mut edges = Vec::new();
                for i in 1..n {
                    let j = rng.random_range(0..i);
                    let w = rng.random_range(-10..=50);
                    edges.push((j, i, w));
                }
                for _ in 0..extra {
                    let u = rng.random_range(0..n);
                    let v = rng.random_range(0..n);
                    if u < v {
                        let w = rng.random_range(-10..=50);
                        edges.push((u, v, w));
                    }
                }
                TestCase {
                    data: Box::new(FloydWarshallTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FloydWarshallTest>().unwrap();
        let expected = ref_floyd_warshall(t.n, &t.edges);
        let actual = solutions::floyd_warshall(t.n, &t.edges);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_floyd_warshall(n: usize, edges: &[(usize, usize, i32)]) -> Vec<Vec<i32>> {
    let mut dist = vec![vec![NO_PATH; n]; n];
    for (i, row) in dist.iter_mut().enumerate().take(n) {
        row[i] = 0;
    }
    for &(u, v, w) in edges {
        dist[u][v] = dist[u][v].min(w);
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != NO_PATH && dist[k][j] != NO_PATH {
                    let through = dist[i][k] + dist[k][j];
                    if through < dist[i][j] {
                        dist[i][j] = through;
                    }
                }
            }
        }
    }
    dist
}

// ── Hard 12: K Shortest Paths ──────────────────────────────────────────

struct ShortestPathKShortest;

struct KShortestTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
    src: usize,
    dst: usize,
    k: usize,
}

impl Problem for ShortestPathKShortest {
    fn id(&self) -> &str {
        "shortest_path_k_shortest"
    }
    fn name(&self) -> &str {
        "K Shortest Paths"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a directed weighted graph, find the K shortest path lengths from `src` to \
         `dst`. The same edge can be used multiple times. Return a sorted vector of the K \
         shortest path lengths. If fewer than K paths exist, return as many as possible.\n\n\
         Constraints:\n\
         - 1 <= n <= 100\n\
         - 1 <= weight\n\
         - 1 <= k <= 20"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=8);
                let extra = rng.random_range(1..=n * 2);
                let edges = gen_connected_weighted_directed(&mut rng, n, extra, 1, 30);
                let src = rng.random_range(0..n);
                let mut dst = rng.random_range(0..n);
                while dst == src {
                    dst = rng.random_range(0..n);
                }
                let k = rng.random_range(1..=5);
                TestCase {
                    data: Box::new(KShortestTest {
                        n,
                        edges,
                        src,
                        dst,
                        k,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KShortestTest>().unwrap();
        let expected = ref_k_shortest(t.n, &t.edges, t.src, t.dst, t.k);
        let actual = solutions::k_shortest_paths(t.n, &t.edges, t.src, t.dst, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!(
                "n={}, edges={:?}, src={}, dst={}, k={}",
                t.n, t.edges, t.src, t.dst, t.k
            ),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_k_shortest(
    n: usize,
    edges: &[(usize, usize, i32)],
    src: usize,
    dst: usize,
    k: usize,
) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
    }
    let mut count = vec![0usize; n];
    let mut result = Vec::new();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i32, src)));

    while let Some(Reverse((d, u))) = heap.pop() {
        count[u] += 1;
        if u == dst {
            result.push(d);
            if result.len() == k {
                return result;
            }
        }
        if count[u] > k {
            continue;
        }
        for &(v, w) in &adj[u] {
            heap.push(Reverse((d + w, v)));
        }
    }
    result
}

// ── Hard 13: Minimum Cost to Connect All Points ────────────────────────

struct ShortestPathMinCostConnectAll;

struct MinCostConnectTest {
    points: Vec<(i32, i32)>,
}

impl Problem for ShortestPathMinCostConnectAll {
    fn id(&self) -> &str {
        "shortest_path_minimum_cost_connect_all"
    }
    fn name(&self) -> &str {
        "Min Cost to Connect All Points"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array of `points` where points[i] = (x_i, y_i), return the minimum cost \
         to connect all points. The cost to connect two points is the Manhattan distance: \
         |x_i - x_j| + |y_i - y_j|. All points must be connected (directly or indirectly). \
         This is a minimum spanning tree problem.\n\n\
         Constraints:\n\
         - 1 <= points.len() <= 1000\n\
         - -10^6 <= x_i, y_i <= 10^6"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=10);
                let points: Vec<(i32, i32)> = (0..n)
                    .map(|_| (rng.random_range(-100..=100), rng.random_range(-100..=100)))
                    .collect();
                TestCase {
                    data: Box::new(MinCostConnectTest { points }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinCostConnectTest>().unwrap();
        let expected = ref_min_cost_connect(&t.points);
        let actual = solutions::min_cost_connect_all(&t.points);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("points={:?}", t.points),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_cost_connect(points: &[(i32, i32)]) -> i32 {
    // Prim's algorithm
    let n = points.len();
    if n <= 1 {
        return 0;
    }
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut in_mst = vec![false; n];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i32, 0usize)));
    let mut total = 0;
    let mut count = 0;

    while count < n {
        let Reverse((cost, u)) = heap.pop().unwrap();
        if in_mst[u] {
            continue;
        }
        in_mst[u] = true;
        total += cost;
        count += 1;
        for v in 0..n {
            if !in_mst[v] {
                let d = (points[u].0 - points[v].0).abs() + (points[u].1 - points[v].1).abs();
                heap.push(Reverse((d, v)));
            }
        }
    }
    total
}

// ── Hard 14: Reconstruct Shortest Path ─────────────────────────────────

struct ShortestPathReconstruct;

struct ReconstructTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
    src: usize,
    dst: usize,
}

impl Problem for ShortestPathReconstruct {
    fn id(&self) -> &str {
        "shortest_path_reconstruct"
    }
    fn name(&self) -> &str {
        "Reconstruct Shortest Path (Dijkstra)"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a directed weighted graph, find the shortest path from `src` to `dst` and \
         return the actual path as a sequence of node indices. If no path exists, return an \
         empty vector. If multiple shortest paths exist, return any one.\n\n\
         Constraints:\n\
         - 1 <= n <= 1000\n\
         - Weights are non-negative\n\
         - 0 <= src, dst < n"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=10);
                let extra = rng.random_range(0..=n * 2);
                let edges = gen_connected_weighted_directed(&mut rng, n, extra, 1, 50);
                let src = rng.random_range(0..n);
                let mut dst = rng.random_range(0..n);
                while dst == src {
                    dst = rng.random_range(0..n);
                }
                TestCase {
                    data: Box::new(ReconstructTest { n, edges, src, dst }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReconstructTest>().unwrap();
        let ref_dist = ref_dijkstra(t.n, &t.edges, t.src);
        let actual = solutions::reconstruct_shortest_path(t.n, &t.edges, t.src, t.dst);
        let valid = ref_validate_path(t.n, &t.edges, t.src, t.dst, ref_dist[t.dst], &actual);
        SolutionResult {
            is_correct: valid,
            input_description: format!(
                "n={}, edges={:?}, src={}, dst={}",
                t.n, t.edges, t.src, t.dst
            ),
            expected: if ref_dist[t.dst] == NO_PATH {
                "[] (no path)".to_string()
            } else {
                format!("any shortest path (distance={})", ref_dist[t.dst])
            },
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_validate_path(
    _n: usize,
    edges: &[(usize, usize, i32)],
    src: usize,
    dst: usize,
    expected_dist: i32,
    path: &[usize],
) -> bool {
    if expected_dist == NO_PATH {
        return path.is_empty();
    }
    if path.is_empty() || path[0] != src || path[path.len() - 1] != dst {
        return false;
    }
    // Build adjacency for quick lookup
    use std::collections::HashMap;
    let mut edge_weights: HashMap<(usize, usize), i32> = HashMap::new();
    for &(u, v, w) in edges {
        let entry = edge_weights.entry((u, v)).or_insert(w);
        *entry = (*entry).min(w);
    }
    let mut total = 0i32;
    for win in path.windows(2) {
        let u = win[0];
        let v = win[1];
        match edge_weights.get(&(u, v)) {
            Some(&w) => total += w,
            None => return false,
        }
    }
    total == expected_dist
}

// ── Hard 15: Shortest Path with Alternating Colors ─────────────────────

struct ShortestPathAlternatingColors;

struct AlternatingColorsTest {
    n: usize,
    red_edges: Vec<(usize, usize)>,
    blue_edges: Vec<(usize, usize)>,
}

impl Problem for ShortestPathAlternatingColors {
    fn id(&self) -> &str {
        "shortest_path_with_alternating_colors"
    }
    fn name(&self) -> &str {
        "Shortest Path with Alternating Colors"
    }
    fn topic(&self) -> &str {
        "shortest_path"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a directed graph with `n` nodes, some edges colored red and some blue, find \
         the shortest path from node 0 to each node that alternates between red and blue \
         edges. Return a vector of length n where result[i] is the shortest alternating-color \
         path length from 0 to i, or -1 if unreachable.\n\n\
         The first edge can be either red or blue.\n\n\
         Constraints:\n\
         - 1 <= n <= 100\n\
         - 0 <= red_edges.len(), blue_edges.len() <= 400"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=10);
                let red_count = rng.random_range(0..=n * 2);
                let blue_count = rng.random_range(0..=n * 2);
                let red_edges: Vec<(usize, usize)> = (0..red_count)
                    .map(|_| (rng.random_range(0..n), rng.random_range(0..n)))
                    .collect();
                let blue_edges: Vec<(usize, usize)> = (0..blue_count)
                    .map(|_| (rng.random_range(0..n), rng.random_range(0..n)))
                    .collect();
                TestCase {
                    data: Box::new(AlternatingColorsTest {
                        n,
                        red_edges,
                        blue_edges,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AlternatingColorsTest>().unwrap();
        let expected = ref_alternating_colors(t.n, &t.red_edges, &t.blue_edges);
        let actual = solutions::shortest_path_alternating_colors(t.n, &t.red_edges, &t.blue_edges);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!(
                "n={}, red_edges={:?}, blue_edges={:?}",
                t.n, t.red_edges, t.blue_edges
            ),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_alternating_colors(
    n: usize,
    red_edges: &[(usize, usize)],
    blue_edges: &[(usize, usize)],
) -> Vec<i32> {
    use std::collections::VecDeque;
    // Build adjacency lists: 0 = red, 1 = blue
    let mut adj = vec![vec![vec![]; 2]; n];
    for &(u, v) in red_edges {
        adj[u][0].push(v);
    }
    for &(u, v) in blue_edges {
        adj[u][1].push(v);
    }

    // BFS with state (node, last_color)
    // dist[node][color] = shortest distance arriving at node via an edge of `color`
    let mut dist = vec![vec![-1i32; 2]; n];
    let mut queue = VecDeque::new();
    // Start: can take red or blue first
    dist[0][0] = 0;
    dist[0][1] = 0;
    queue.push_back((0, 0, 0)); // node, last_color, steps
    queue.push_back((0, 1, 0));

    while let Some((node, last_color, steps)) = queue.pop_front() {
        let next_color = 1 - last_color;
        for &next_node in &adj[node][next_color] {
            if dist[next_node][next_color] == -1 {
                dist[next_node][next_color] = steps + 1;
                queue.push_back((next_node, next_color, steps + 1));
            }
        }
    }

    let mut result = vec![-1i32; n];
    for i in 0..n {
        let r = dist[i][0];
        let b = dist[i][1];
        result[i] = match (r, b) {
            (-1, -1) => -1,
            (-1, x) | (x, -1) => x,
            (a, b) => a.min(b),
        };
    }
    result
}
