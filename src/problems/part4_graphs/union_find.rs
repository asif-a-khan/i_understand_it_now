use rand::Rng;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part4_graphs::union_find as solutions;
use crate::tracker::{track_slice, OperationLog, Tracked};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(UfConnectedComponents),
        Box::new(UfIsConnected),
        Box::new(UfFriendCircles),
        Box::new(UfRedundantConnection),
        Box::new(UfEarliestConnection),
        Box::new(UfAccountsMerge),
        Box::new(UfNumIslandsII),
        Box::new(UfSatisfiability),
        Box::new(UfRegionsBySlashes),
        Box::new(UfLongestConsecutive),
        Box::new(UfNumberOfIslandsRemoval),
        Box::new(UfSwimInWater),
        Box::new(UfMinCostConnectCities),
        Box::new(UfRemoveStones),
        Box::new(UfCheckingExistenceEdgeLength),
    ]
}

// ── Reference Union-Find ─────────────────────────────────────────────

struct RefUF {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl RefUF {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
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
        self.count -= 1;
        true
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn components(&self) -> usize {
        self.count
    }
}

// ── Helpers ──────────────────────────────────────────────────────────

fn random_unweighted_edges(rng: &mut impl Rng, n: usize, count: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for _ in 0..count {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v {
            edges.push((u, v));
        }
    }
    edges
}

// ── Easy 1: Connected Components ─────────────────────────────────────

struct UfConnectedComponents;

struct UfConnectedComponentsTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for UfConnectedComponents {
    fn id(&self) -> &str {
        "union_find_connected_components"
    }
    fn name(&self) -> &str {
        "Connected Components (Union-Find)"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Count the number of connected components in an undirected graph using Union-Find.\n\n\
         Input: (n, edges) where n is the number of nodes (0-indexed) and edges is a list \
         of (u, v) pairs.\n\n\
         Implement Union-Find with path compression and union by rank.\n\n\
         Constraints:\n\
         - 1 <= n <= 50\n\
         - 0 <= edges.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=30);
                let edge_count = rng.random_range(0..=(n * 2));
                let edges = random_unweighted_edges(&mut rng, n, edge_count);
                TestCase {
                    data: Box::new(UfConnectedComponentsTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<UfConnectedComponentsTest>()
            .unwrap();
        let expected = ref_uf_connected_components(t.n, &t.edges);
        let actual = solutions::connected_components(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_uf_connected_components(n: usize, edges: &[(usize, usize)]) -> usize {
    let mut uf = RefUF::new(n);
    for &(u, v) in edges {
        uf.union(u, v);
    }
    uf.components()
}

// ── Easy 2: Is Connected ─────────────────────────────────────────────

struct UfIsConnected;

struct UfIsConnectedTest {
    n: usize,
    edges: Vec<(usize, usize)>,
    u: usize,
    v: usize,
}

impl Problem for UfIsConnected {
    fn id(&self) -> &str {
        "union_find_is_connected"
    }
    fn name(&self) -> &str {
        "Check If Two Nodes Connected"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Check if two nodes u and v are connected in an undirected graph using Union-Find.\n\n\
         Input: (n, edges, u, v).\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - 0 <= edges.len() <= 100\n\
         - 0 <= u, v < n"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=30);
                let edge_count = rng.random_range(0..=(n * 2));
                let edges = random_unweighted_edges(&mut rng, n, edge_count);
                let u = rng.random_range(0..n);
                let v = rng.random_range(0..n);
                TestCase {
                    data: Box::new(UfIsConnectedTest { n, edges, u, v }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UfIsConnectedTest>().unwrap();
        let expected = ref_is_connected(t.n, &t.edges, t.u, t.v);
        let actual = solutions::is_connected(t.n, &t.edges, t.u, t.v);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}, u={}, v={}", t.n, t.edges, t.u, t.v),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_is_connected(n: usize, edges: &[(usize, usize)], u: usize, v: usize) -> bool {
    let mut uf = RefUF::new(n);
    for &(a, b) in edges {
        uf.union(a, b);
    }
    uf.connected(u, v)
}

// ── Easy 3: Friend Circles ───────────────────────────────────────────

struct UfFriendCircles;

struct UfFriendCirclesTest {
    matrix: Vec<Vec<i32>>,
}

impl Problem for UfFriendCircles {
    fn id(&self) -> &str {
        "union_find_friend_circles"
    }
    fn name(&self) -> &str {
        "Number of Friend Circles"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an n x n adjacency matrix where matrix[i][j] = 1 means person i and j are \
         friends (friendship is transitive), find the number of friend circles.\n\n\
         A friend circle is a group of people who are directly or indirectly friends.\n\n\
         Constraints:\n\
         - 1 <= n <= 30\n\
         - matrix[i][i] = 1\n\
         - matrix[i][j] = matrix[j][i]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let mut matrix = vec![vec![0i32; n]; n];
                for (i, row) in matrix.iter_mut().enumerate() {
                    row[i] = 1;
                }
                // Add random friendships
                let friendships = rng.random_range(0..=(n * n / 3));
                for _ in 0..friendships {
                    let a = rng.random_range(0..n);
                    let b = rng.random_range(0..n);
                    matrix[a][b] = 1;
                    matrix[b][a] = 1;
                }
                TestCase {
                    data: Box::new(UfFriendCirclesTest { matrix }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UfFriendCirclesTest>().unwrap();
        let expected = ref_friend_circles(&t.matrix);
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
        let actual = solutions::friend_circles(&tracked_matrix);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("matrix={:?}", t.matrix),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_friend_circles(matrix: &[Vec<i32>]) -> i32 {
    let n = matrix.len();
    let mut uf = RefUF::new(n);
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate().skip(i + 1) {
            if val == 1 {
                uf.union(i, j);
            }
        }
    }
    uf.components() as i32
}

// ── Easy 4: Redundant Connection ─────────────────────────────────────

struct UfRedundantConnection;

struct UfRedundantConnectionTest {
    edges: Vec<(usize, usize)>,
}

impl Problem for UfRedundantConnection {
    fn id(&self) -> &str {
        "union_find_redundant_connection"
    }
    fn name(&self) -> &str {
        "Redundant Connection"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a graph that started as a tree with n nodes and had one extra edge added, \
         find the redundant edge that creates a cycle.\n\n\
         Edges are given in order. Return the first edge that, when processed in order, \
         would create a cycle (both endpoints already connected).\n\n\
         Nodes are 1-indexed in the input. Return the edge as (u, v) with original indexing.\n\n\
         Constraints:\n\
         - 3 <= n <= 50\n\
         - edges.len() = n (one extra edge)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=20);
                // Build a random tree on nodes 1..=n
                let mut edges = Vec::new();
                for i in 2..=n {
                    let parent = rng.random_range(1..i);
                    edges.push((parent, i));
                }
                // Add one extra edge creating a cycle
                let u = rng.random_range(1..=n);
                let mut v = rng.random_range(1..=n);
                while v == u {
                    v = rng.random_range(1..=n);
                }
                edges.push((u, v));
                // Shuffle (but keep the order deterministic for consistent answer)
                // Actually, we should keep them in a specific order so the "last" redundant edge is well-defined
                TestCase {
                    data: Box::new(UfRedundantConnectionTest { edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<UfRedundantConnectionTest>()
            .unwrap();
        let expected = ref_redundant_connection(&t.edges);
        let actual = solutions::redundant_connection(&t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("edges={:?}", t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_redundant_connection(edges: &[(usize, usize)]) -> (usize, usize) {
    let n = edges.len();
    let mut uf = RefUF::new(n + 1); // 1-indexed
    let mut result = (0, 0);
    for &(u, v) in edges {
        if !uf.union(u, v) {
            result = (u, v);
        }
    }
    result
}

// ── Easy 5: Earliest Time When All Connected ─────────────────────────

struct UfEarliestConnection;

struct UfEarliestConnectionTest {
    n: usize,
    logs: Vec<(i32, usize, usize)>,
}

impl Problem for UfEarliestConnection {
    fn id(&self) -> &str {
        "union_find_earliest_connection"
    }
    fn name(&self) -> &str {
        "Earliest Time All Connected"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given n people and a list of (timestamp, person_a, person_b) indicating when two \
         people became friends, find the earliest time when all people are connected.\n\n\
         The logs are already sorted by timestamp.\n\n\
         Return the earliest timestamp, or -1 if they never all connect.\n\n\
         Constraints:\n\
         - 2 <= n <= 30\n\
         - 0 <= logs.len() <= 100\n\
         - People are 0-indexed"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let n = rng.random_range(2..=15);
                let log_count = if i < 7 {
                    rng.random_range(n..=(n * 3))
                } else {
                    rng.random_range(0..n) // may not connect all
                };
                let mut logs: Vec<(i32, usize, usize)> = (0..log_count)
                    .map(|_| {
                        let t = rng.random_range(0..=1000);
                        let a = rng.random_range(0..n);
                        let mut b = rng.random_range(0..n);
                        while b == a {
                            b = rng.random_range(0..n);
                        }
                        (t, a, b)
                    })
                    .collect();
                logs.sort_by_key(|l| l.0);
                TestCase {
                    data: Box::new(UfEarliestConnectionTest { n, logs }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<UfEarliestConnectionTest>()
            .unwrap();
        let expected = ref_earliest_connection(t.n, &t.logs);
        let actual = solutions::earliest_connection(t.n, &t.logs);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, logs={:?}", t.n, t.logs),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_earliest_connection(n: usize, logs: &[(i32, usize, usize)]) -> i32 {
    let mut uf = RefUF::new(n);
    for &(time, a, b) in logs {
        uf.union(a, b);
        if uf.components() == 1 {
            return time;
        }
    }
    -1
}

// ── Medium 1: Accounts Merge ─────────────────────────────────────────

struct UfAccountsMerge;

struct UfAccountsMergeTest {
    accounts: Vec<Vec<String>>,
}

impl Problem for UfAccountsMerge {
    fn id(&self) -> &str {
        "union_find_accounts_merge"
    }
    fn name(&self) -> &str {
        "Accounts Merge"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a list of accounts where each account is [name, email1, email2, ...], \
         merge accounts that share at least one email.\n\n\
         Return merged accounts: each is [name, sorted_email1, sorted_email2, ...]. \
         The outer list should be sorted by the first email in each account.\n\n\
         Constraints:\n\
         - 1 <= accounts.len() <= 20\n\
         - 1 <= emails per account <= 5"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let names = ["Alice", "Bob", "Charlie", "Diana", "Eve"];
                let n_accounts = rng.random_range(2..=10);
                let mut accounts = Vec::new();
                let mut all_emails: Vec<String> = Vec::new();

                for _ in 0..n_accounts {
                    let name = names[rng.random_range(0..names.len())].to_string();
                    let n_emails = rng.random_range(1..=3);
                    let mut emails = Vec::new();
                    for _ in 0..n_emails {
                        // Sometimes reuse existing emails to create merges
                        if !all_emails.is_empty() && rng.random_range(0..3) == 0 {
                            let idx = rng.random_range(0..all_emails.len());
                            emails.push(all_emails[idx].clone());
                        } else {
                            let email = format!(
                                "{}{}@test.com",
                                (b'a' + rng.random_range(0..26u8)) as char,
                                rng.random_range(0..100)
                            );
                            all_emails.push(email.clone());
                            emails.push(email);
                        }
                    }
                    let mut account = vec![name];
                    account.extend(emails);
                    accounts.push(account);
                }
                TestCase {
                    data: Box::new(UfAccountsMergeTest { accounts }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UfAccountsMergeTest>().unwrap();
        let expected = ref_accounts_merge(&t.accounts);
        let actual = solutions::accounts_merge(&t.accounts);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("accounts={:?}", t.accounts),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_accounts_merge(accounts: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut email_to_id: HashMap<String, usize> = HashMap::new();
    let mut email_to_name: HashMap<String, String> = HashMap::new();
    let mut uf = RefUF::new(accounts.len() * 10 + 10); // generous size
    let mut next_id = 0usize;

    for account in accounts {
        let name = &account[0];
        let first_email = &account[1];
        let first_id = *email_to_id.entry(first_email.clone()).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        email_to_name.insert(first_email.clone(), name.clone());

        for email in &account[1..] {
            let eid = *email_to_id.entry(email.clone()).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            });
            email_to_name.insert(email.clone(), name.clone());
            uf.union(first_id, eid);
        }
    }

    // Group emails by root
    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();
    for (email, &id) in &email_to_id {
        let root = uf.find(id);
        groups.entry(root).or_default().push(email.clone());
    }

    let mut result: Vec<Vec<String>> = Vec::new();
    for (_root, mut emails) in groups {
        emails.sort();
        let name = email_to_name[&emails[0]].clone();
        let mut account = vec![name];
        account.extend(emails);
        result.push(account);
    }
    result.sort_by(|a, b| a[1].cmp(&b[1]));
    result
}

// ── Medium 2: Number of Islands II ───────────────────────────────────

struct UfNumIslandsII;

struct UfNumIslandsIITest {
    rows: usize,
    cols: usize,
    positions: Vec<(usize, usize)>,
}

impl Problem for UfNumIslandsII {
    fn id(&self) -> &str {
        "union_find_num_islands_ii"
    }
    fn name(&self) -> &str {
        "Number of Islands II"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a grid of water, add land one cell at a time. After each addition, \
         return the number of islands.\n\n\
         Two land cells are connected if they are horizontally or vertically adjacent.\n\n\
         Input: (rows, cols, positions) where each position is (row, col).\n\
         Return Vec<i32> with island count after each addition.\n\n\
         If a position is added twice, the second addition has no effect (count stays same).\n\n\
         Constraints:\n\
         - 1 <= rows, cols <= 20\n\
         - 1 <= positions.len() <= 50"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(2..=10);
                let cols = rng.random_range(2..=10);
                let count = rng.random_range(3..=20);
                let positions: Vec<(usize, usize)> = (0..count)
                    .map(|_| (rng.random_range(0..rows), rng.random_range(0..cols)))
                    .collect();
                TestCase {
                    data: Box::new(UfNumIslandsIITest {
                        rows,
                        cols,
                        positions,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UfNumIslandsIITest>().unwrap();
        let expected = ref_num_islands_ii(t.rows, t.cols, &t.positions);
        let actual = solutions::num_islands_ii(t.rows, t.cols, &t.positions);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!(
                "rows={}, cols={}, positions={:?}",
                t.rows, t.cols, t.positions
            ),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_num_islands_ii(rows: usize, cols: usize, positions: &[(usize, usize)]) -> Vec<i32> {
    let total = rows * cols;
    let mut parent: Vec<i32> = vec![-1; total]; // -1 means water
    let mut rank = vec![0usize; total];
    let mut count = 0i32;
    let mut result = Vec::new();

    fn find(parent: &mut [i32], x: usize) -> usize {
        if parent[x] != x as i32 {
            parent[x] = find(parent, parent[x] as usize) as i32;
        }
        parent[x] as usize
    }

    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(r, c) in positions {
        let idx = r * cols + c;
        if parent[idx] != -1 {
            result.push(count);
            continue;
        }
        parent[idx] = idx as i32;
        count += 1;

        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nidx = nr as usize * cols + nc as usize;
                if parent[nidx] != -1 {
                    let rx = find(&mut parent, idx);
                    let ry = find(&mut parent, nidx);
                    if rx != ry {
                        match rank[rx].cmp(&rank[ry]) {
                            std::cmp::Ordering::Less => parent[rx] = ry as i32,
                            std::cmp::Ordering::Greater => parent[ry] = rx as i32,
                            std::cmp::Ordering::Equal => {
                                parent[ry] = rx as i32;
                                rank[rx] += 1;
                            }
                        }
                        count -= 1;
                    }
                }
            }
        }
        result.push(count);
    }
    result
}

// ── Medium 3: Equations Satisfiability ───────────────────────────────

struct UfSatisfiability;

struct UfSatisfiabilityTest {
    equations: Vec<String>,
}

impl Problem for UfSatisfiability {
    fn id(&self) -> &str {
        "union_find_satisfiability"
    }
    fn name(&self) -> &str {
        "Equations Satisfiability"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a list of equations like \"a==b\" or \"a!=b\", determine if all equations \
         can be satisfied simultaneously.\n\n\
         Each equation is a 4-character string: variable, op1, op2, variable.\n\
         Variables are single lowercase letters. Operators are == or !=.\n\n\
         Example: [\"a==b\", \"b!=a\"] -> false\n\n\
         Constraints:\n\
         - 1 <= equations.len() <= 30\n\
         - Variables are lowercase letters a-z"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=15);
                let vars: Vec<u8> = (b'a'..=b'a' + rng.random_range(2..=5u8)).collect();
                let equations: Vec<String> = (0..n)
                    .map(|_| {
                        let a = vars[rng.random_range(0..vars.len())] as char;
                        let b = vars[rng.random_range(0..vars.len())] as char;
                        let op = if rng.random_range(0..2) == 0 {
                            "=="
                        } else {
                            "!="
                        };
                        format!("{a}{op}{b}")
                    })
                    .collect();
                TestCase {
                    data: Box::new(UfSatisfiabilityTest { equations }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UfSatisfiabilityTest>().unwrap();
        let expected = ref_satisfiability(&t.equations);
        let actual = solutions::satisfiability(&t.equations);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("equations={:?}", t.equations),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_satisfiability(equations: &[String]) -> bool {
    let mut uf = RefUF::new(26);
    // First pass: process == equations
    for eq in equations {
        let bytes = eq.as_bytes();
        if bytes[1] == b'=' {
            let a = (bytes[0] - b'a') as usize;
            let b = (bytes[3] - b'a') as usize;
            uf.union(a, b);
        }
    }
    // Second pass: check != equations
    for eq in equations {
        let bytes = eq.as_bytes();
        if bytes[1] == b'!' {
            let a = (bytes[0] - b'a') as usize;
            let b = (bytes[3] - b'a') as usize;
            if uf.connected(a, b) {
                return false;
            }
        }
    }
    true
}

// ── Medium 4: Regions by Slashes ─────────────────────────────────────

struct UfRegionsBySlashes;

struct UfRegionsBySlashesTest {
    grid: Vec<String>,
}

impl Problem for UfRegionsBySlashes {
    fn id(&self) -> &str {
        "union_find_regions_by_slashes"
    }
    fn name(&self) -> &str {
        "Regions Cut by Slashes"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "An n x n grid is composed of 1x1 squares. Each square can be ' ', '/', or '\\\\'.\n\n\
         '/' divides the square from bottom-left to top-right.\n\
         '\\\\' divides from top-left to bottom-right.\n\
         ' ' leaves the square open.\n\n\
         Count the number of regions formed.\n\n\
         Hint: Expand each cell to a 3x3 grid, then count connected components. \
         Or use Union-Find with 4 triangles per cell.\n\n\
         Constraints:\n\
         - 1 <= n <= 10\n\
         - grid[i].len() = n\n\
         - grid[i][j] is ' ', '/', or '\\\\'"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=6);
                let grid: Vec<String> = (0..n)
                    .map(|_| {
                        (0..n)
                            .map(|_| match rng.random_range(0..3) {
                                0 => ' ',
                                1 => '/',
                                _ => '\\',
                            })
                            .collect()
                    })
                    .collect();
                TestCase {
                    data: Box::new(UfRegionsBySlashesTest { grid }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UfRegionsBySlashesTest>().unwrap();
        let expected = ref_regions_by_slashes(&t.grid);
        let actual = solutions::regions_by_slashes(&t.grid);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_regions_by_slashes(grid: &[String]) -> i32 {
    // Expand each cell to 3x3
    let n = grid.len();
    let size = n * 3;
    let mut expanded = vec![vec![0u8; size]; size];
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            let r = i * 3;
            let c = j * 3;
            match ch {
                '/' => {
                    expanded[r][c + 2] = 1;
                    expanded[r + 1][c + 1] = 1;
                    expanded[r + 2][c] = 1;
                }
                '\\' => {
                    expanded[r][c] = 1;
                    expanded[r + 1][c + 1] = 1;
                    expanded[r + 2][c + 2] = 1;
                }
                _ => {}
            }
        }
    }
    // Count connected components of 0s using BFS
    let mut visited = vec![vec![false; size]; size];
    let mut count = 0i32;
    for i in 0..size {
        for j in 0..size {
            if expanded[i][j] == 0 && !visited[i][j] {
                count += 1;
                let mut queue = std::collections::VecDeque::new();
                queue.push_back((i, j));
                visited[i][j] = true;
                while let Some((r, c)) = queue.pop_front() {
                    for (dr, dc) in &[(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0 && nr < size as i32 && nc >= 0 && nc < size as i32 {
                            let nr = nr as usize;
                            let nc = nc as usize;
                            if expanded[nr][nc] == 0 && !visited[nr][nc] {
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

// ── Medium 5: Longest Consecutive Sequence ───────────────────────────

struct UfLongestConsecutive;

struct UfLongestConsecutiveTest {
    nums: Vec<i32>,
}

impl Problem for UfLongestConsecutive {
    fn id(&self) -> &str {
        "union_find_longest_consecutive"
    }
    fn name(&self) -> &str {
        "Longest Consecutive Sequence (UF)"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an unsorted array of integers, find the length of the longest consecutive \
         elements sequence using Union-Find.\n\n\
         For each number n, if n+1 exists in the array, union them. Then find the largest \
         component.\n\n\
         Example: [100, 4, 200, 1, 3, 2] -> 4 (sequence: 1, 2, 3, 4)\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=50);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(UfLongestConsecutiveTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<UfLongestConsecutiveTest>()
            .unwrap();
        let expected = ref_longest_consecutive(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::longest_consecutive(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_longest_consecutive(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let set: HashSet<i32> = nums.iter().copied().collect();
    let mut best = 0i32;
    for &n in &set {
        if !set.contains(&(n - 1)) {
            let mut len = 1;
            let mut cur = n;
            while set.contains(&(cur + 1)) {
                cur += 1;
                len += 1;
            }
            best = best.max(len);
        }
    }
    best
}

// ── Hard 1: Components After Edge Removal ────────────────────────────

struct UfNumberOfIslandsRemoval;

struct UfNumberOfIslandsRemovalTest {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for UfNumberOfIslandsRemoval {
    fn id(&self) -> &str {
        "union_find_number_of_islands_removal"
    }
    fn name(&self) -> &str {
        "Components After Each Edge Removal"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given n nodes and a list of edges, remove edges one at a time from the END \
         of the list. After each removal, report the number of connected components.\n\n\
         Hint: Process in reverse order -- add edges from end to start using Union-Find, \
         then reverse the results.\n\n\
         Return Vec<usize> where result[i] = component count after removing edges[i..end].\n\
         result[0] = components with no edges, result[len-1] = components with all but last edge.\n\n\
         The length of the result equals edges.len().\n\n\
         Constraints:\n\
         - 2 <= n <= 30\n\
         - 1 <= edges.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let edge_count = rng.random_range(n..=(n * 3));
                let mut edges = Vec::new();
                // Build spanning tree first
                for i in 1..n {
                    let j = rng.random_range(0..i);
                    edges.push((j, i));
                }
                // Add extra edges
                for _ in 0..(edge_count - (n - 1)) {
                    let u = rng.random_range(0..n);
                    let mut v = rng.random_range(0..n);
                    while v == u {
                        v = rng.random_range(0..n);
                    }
                    edges.push((u, v));
                }
                // Shuffle
                for i in (1..edges.len()).rev() {
                    let j = rng.random_range(0..=i);
                    edges.swap(i, j);
                }
                TestCase {
                    data: Box::new(UfNumberOfIslandsRemovalTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<UfNumberOfIslandsRemovalTest>()
            .unwrap();
        let expected = ref_components_after_removal(t.n, &t.edges);
        let actual = solutions::number_of_islands_removal(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_components_after_removal(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    // result[i] = component count with only edges[0..i] present
    // (i.e., after removing edges[i..end])
    // result[0] = n (no edges), result[m-1] = count with edges[0..m-1]
    let m = edges.len();
    let mut result = vec![0usize; m];
    let mut uf = RefUF::new(n);
    for i in 0..m {
        result[i] = uf.components();
        let (u, v) = edges[i];
        uf.union(u, v);
    }
    result
}

// ── Hard 2: Swim in Rising Water ─────────────────────────────────────

struct UfSwimInWater;

struct UfSwimInWaterTest {
    grid: Vec<Vec<i32>>,
}

impl Problem for UfSwimInWater {
    fn id(&self) -> &str {
        "union_find_swim_in_water"
    }
    fn name(&self) -> &str {
        "Swim in Rising Water"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an n x n grid where grid[i][j] represents the elevation, find the minimum \
         time t such that you can swim from (0,0) to (n-1,n-1).\n\n\
         At time t, you can swim through any cell with elevation <= t. You can move to \
         adjacent cells (up/down/left/right).\n\n\
         Hint: Sort all cells by elevation. Process them in order, unioning adjacent cells \
         that are both <= current elevation. Return t when (0,0) and (n-1,n-1) are connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 15\n\
         - All values in grid are unique and in [0, n*n - 1]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=8);
                let mut vals: Vec<i32> = (0..(n * n) as i32).collect();
                // Fisher-Yates shuffle
                for i in (1..vals.len()).rev() {
                    let j = rng.random_range(0..=i);
                    vals.swap(i, j);
                }
                let grid: Vec<Vec<i32>> = vals.chunks(n).map(|c| c.to_vec()).collect();
                TestCase {
                    data: Box::new(UfSwimInWaterTest { grid }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UfSwimInWaterTest>().unwrap();
        let expected = ref_swim_in_water(&t.grid);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_grid: Vec<Vec<Tracked<i32>>> = t
            .grid
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, &v)| Tracked::new(v, r * row.len() + c, shared_log.clone()))
                    .collect()
            })
            .collect();
        let actual = solutions::swim_in_water(&tracked_grid);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_swim_in_water(grid: &[Vec<i32>]) -> i32 {
    let n = grid.len();
    let mut cells: Vec<(i32, usize, usize)> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            cells.push((val, i, j));
        }
    }
    cells.sort();

    let mut uf = RefUF::new(n * n);
    let mut processed = vec![vec![false; n]; n];
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (elev, r, c) in &cells {
        processed[*r][*c] = true;
        for (dr, dc) in &dirs {
            let nr = *r as i32 + dr;
            let nc = *c as i32 + dc;
            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                if processed[nr][nc] {
                    uf.union(r * n + c, nr * n + nc);
                }
            }
        }
        if uf.connected(0, n * n - 1) {
            return *elev;
        }
    }
    -1 // should not reach here for valid input
}

// ── Hard 3: Min Cost Connect Cities ──────────────────────────────────

struct UfMinCostConnectCities;

struct UfMinCostConnectCitiesTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl Problem for UfMinCostConnectCities {
    fn id(&self) -> &str {
        "union_find_min_cost_connect_cities"
    }
    fn name(&self) -> &str {
        "Min Cost to Connect All Cities (UF)"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given n cities and weighted edges, find the minimum cost to connect all cities \
         using Union-Find (Kruskal's algorithm).\n\n\
         Return the total weight of the MST, or -1 if not all cities can be connected.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - 0 <= edges.len() <= 200\n\
         - 1 <= weight <= 100\n\
         - Cities are 0-indexed"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let n = rng.random_range(2..=25);
                let edges = if i < 7 {
                    // Connected graph
                    let extra = rng.random_range(0..=(n * 2));
                    let mut edges = Vec::new();
                    for node in 1..n {
                        let parent = rng.random_range(0..node);
                        let w = rng.random_range(1..=100);
                        edges.push((parent, node, w));
                    }
                    for _ in 0..extra {
                        let u = rng.random_range(0..n);
                        let v = rng.random_range(0..n);
                        if u != v {
                            let w = rng.random_range(1..=100);
                            edges.push((u, v, w));
                        }
                    }
                    edges
                } else {
                    // Possibly disconnected
                    let count = rng.random_range(0..n);
                    (0..count)
                        .map(|_| {
                            let u = rng.random_range(0..n);
                            let v = rng.random_range(0..n);
                            let w = rng.random_range(1..=100);
                            (u, v, w)
                        })
                        .filter(|(u, v, _)| u != v)
                        .collect()
                };
                TestCase {
                    data: Box::new(UfMinCostConnectCitiesTest { n, edges }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<UfMinCostConnectCitiesTest>()
            .unwrap();
        let expected = ref_min_cost_connect_cities(t.n, &t.edges);
        let actual = solutions::min_cost_connect_cities(t.n, &t.edges);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}", t.n, t.edges),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_cost_connect_cities(n: usize, edges: &[(usize, usize, i32)]) -> i32 {
    let mut sorted = edges.to_vec();
    sorted.sort_by_key(|e| e.2);
    let mut uf = RefUF::new(n);
    let mut total = 0i32;
    for (u, v, w) in &sorted {
        if uf.union(*u, *v) {
            total += w;
        }
    }
    if uf.components() == 1 {
        total
    } else {
        -1
    }
}

// ── Hard 4: Most Stones Removed ──────────────────────────────────────

struct UfRemoveStones;

struct UfRemoveStonesTest {
    stones: Vec<(i32, i32)>,
}

impl Problem for UfRemoveStones {
    fn id(&self) -> &str {
        "union_find_remove_stones"
    }
    fn name(&self) -> &str {
        "Most Stones Removed"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given stones at 2D coordinates, a stone can be removed if it shares a row or \
         column with another stone that has NOT been removed.\n\n\
         Return the maximum number of stones that can be removed.\n\n\
         Key insight: stones in the same row or column form a connected component. \
         Max removable = total stones - number of connected components.\n\n\
         Use Union-Find: union stones that share a row or column.\n\n\
         Constraints:\n\
         - 1 <= stones.len() <= 50\n\
         - 0 <= row, col <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=25);
                let mut stone_set = HashSet::new();
                while stone_set.len() < n {
                    let r = rng.random_range(0..=50);
                    let c = rng.random_range(0..=50);
                    stone_set.insert((r, c));
                }
                let stones: Vec<(i32, i32)> = stone_set.into_iter().collect();
                TestCase {
                    data: Box::new(UfRemoveStonesTest { stones }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UfRemoveStonesTest>().unwrap();
        let expected = ref_remove_stones(&t.stones);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_stones: Vec<(Tracked<i32>, Tracked<i32>)> = t
            .stones
            .iter()
            .enumerate()
            .map(|(i, &(a, b))| {
                (
                    Tracked::new(a, i * 2, shared_log.clone()),
                    Tracked::new(b, i * 2 + 1, shared_log.clone()),
                )
            })
            .collect();
        let actual = solutions::remove_stones(&tracked_stones);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("stones={:?}", t.stones),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_remove_stones(stones: &[(i32, i32)]) -> i32 {
    let n = stones.len();
    if n == 0 {
        return 0;
    }
    let mut uf = RefUF::new(n);
    // Union stones that share row or column
    for i in 0..n {
        for j in (i + 1)..n {
            if stones[i].0 == stones[j].0 || stones[i].1 == stones[j].1 {
                uf.union(i, j);
            }
        }
    }
    n as i32 - uf.components() as i32
}

// ── Hard 5: Checking Existence of Edge Length Limited Paths ──────────

struct UfCheckingExistenceEdgeLength;

struct UfCheckingExistenceEdgeLengthTest {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
    queries: Vec<(usize, usize, i32)>,
}

impl Problem for UfCheckingExistenceEdgeLength {
    fn id(&self) -> &str {
        "union_find_checking_existence_edge_length"
    }
    fn name(&self) -> &str {
        "Edge-Length Limited Path Queries"
    }
    fn topic(&self) -> &str {
        "union_find"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an undirected graph and queries (u, v, limit), determine for each query \
         whether there exists a path from u to v using only edges with weight strictly \
         less than limit.\n\n\
         Hint: Sort edges by weight and queries by limit. Process queries offline: for each \
         query, add all edges with weight < limit, then check connectivity with Union-Find.\n\n\
         Return Vec<bool> with answers in original query order.\n\n\
         Constraints:\n\
         - 2 <= n <= 50\n\
         - 0 <= edges.len() <= 200\n\
         - 1 <= queries.len() <= 50\n\
         - 1 <= weight, limit <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=20);
                let edge_count = rng.random_range(0..=(n * 3));
                let mut edges = Vec::new();
                for _ in 0..edge_count {
                    let u = rng.random_range(0..n);
                    let v = rng.random_range(0..n);
                    if u != v {
                        let w = rng.random_range(1..=100);
                        edges.push((u, v, w));
                    }
                }
                let query_count = rng.random_range(1..=20);
                let queries: Vec<(usize, usize, i32)> = (0..query_count)
                    .map(|_| {
                        let u = rng.random_range(0..n);
                        let mut v = rng.random_range(0..n);
                        while v == u {
                            v = rng.random_range(0..n);
                        }
                        let limit = rng.random_range(1..=150);
                        (u, v, limit)
                    })
                    .collect();
                TestCase {
                    data: Box::new(UfCheckingExistenceEdgeLengthTest { n, edges, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<UfCheckingExistenceEdgeLengthTest>()
            .unwrap();
        let expected = ref_checking_edge_length(t.n, &t.edges, &t.queries);
        let actual = solutions::checking_existence_edge_length(t.n, &t.edges, &t.queries);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("n={}, edges={:?}, queries={:?}", t.n, t.edges, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_checking_edge_length(
    n: usize,
    edges: &[(usize, usize, i32)],
    queries: &[(usize, usize, i32)],
) -> Vec<bool> {
    // Sort edges by weight
    let mut sorted_edges = edges.to_vec();
    sorted_edges.sort_by_key(|e| e.2);

    // Sort queries by limit, keep original index
    let mut indexed_queries: Vec<(usize, usize, usize, i32)> = queries
        .iter()
        .enumerate()
        .map(|(i, &(u, v, lim))| (i, u, v, lim))
        .collect();
    indexed_queries.sort_by_key(|q| q.3);

    let mut uf = RefUF::new(n);
    let mut result = vec![false; queries.len()];
    let mut edge_idx = 0;

    for (qi, u, v, limit) in &indexed_queries {
        // Add all edges with weight < limit
        while edge_idx < sorted_edges.len() && sorted_edges[edge_idx].2 < *limit {
            let (eu, ev, _) = sorted_edges[edge_idx];
            uf.union(eu, ev);
            edge_idx += 1;
        }
        result[*qi] = uf.connected(*u, *v);
    }

    result
}
