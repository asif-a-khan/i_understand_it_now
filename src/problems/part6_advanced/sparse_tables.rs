use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;
use std::collections::HashMap;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part6_advanced::sparse_tables as solutions;
use crate::tracker::{track_slice, OperationLog, Tracked};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy (5)
        Box::new(SparseTableRangeMin),
        Box::new(SparseTableRangeMax),
        Box::new(SparseTableRangeGcd),
        Box::new(SparseTableBuild),
        Box::new(SparseTableStaticRmq),
        // Medium (5)
        Box::new(SparseTableLca),
        Box::new(SparseTableRangeAnd),
        Box::new(SparseTableRangeOr),
        Box::new(SparseTableSecondMinimum),
        Box::new(SparseTableIndexOfMin),
        // Hard (5)
        Box::new(SparseTable2dRmq),
        Box::new(SparseTableKthAncestor),
        Box::new(SparseTableRangeFrequency),
        Box::new(SparseTableLcpArray),
        Box::new(SparseTableDistinctInRange),
    ]
}

// ── Reference implementations ────────────────────────────────────────

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn ref_range_min(arr: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r)| *arr[l..=r].iter().min().unwrap())
        .collect()
}

fn ref_range_max(arr: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r)| *arr[l..=r].iter().max().unwrap())
        .collect()
}

fn ref_range_gcd(arr: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r)| arr[l..=r].iter().copied().reduce(gcd).unwrap())
        .collect()
}

fn ref_range_and(arr: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r)| arr[l..=r].iter().fold(!0i32, |acc, &x| acc & x))
        .collect()
}

fn ref_range_or(arr: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r)| arr[l..=r].iter().fold(0i32, |acc, &x| acc | x))
        .collect()
}

fn ref_second_minimum(arr: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r)| {
            let mut vals: Vec<i32> = arr[l..=r].to_vec();
            vals.sort();
            vals.dedup();
            if vals.len() >= 2 {
                vals[1]
            } else {
                vals[0]
            }
        })
        .collect()
}

fn ref_index_of_min(arr: &[i32], queries: &[(usize, usize)]) -> Vec<usize> {
    queries
        .iter()
        .map(|&(l, r)| {
            let mut min_idx = l;
            for i in l..=r {
                if arr[i] < arr[min_idx] {
                    min_idx = i;
                }
            }
            min_idx
        })
        .collect()
}

fn ref_2d_rmq(matrix: &[Vec<i32>], queries: &[(usize, usize, usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(r1, c1, r2, c2)| {
            let mut min_val = i32::MAX;
            for row in matrix.iter().take(r2 + 1).skip(r1) {
                for &val in row.iter().take(c2 + 1).skip(c1) {
                    min_val = min_val.min(val);
                }
            }
            min_val
        })
        .collect()
}

fn ref_kth_ancestor(parents: &[Option<usize>], queries: &[(usize, usize)]) -> Vec<Option<usize>> {
    queries
        .iter()
        .map(|&(node, k)| {
            let mut cur = Some(node);
            for _ in 0..k {
                if let Some(c) = cur {
                    if c < parents.len() {
                        cur = parents[c];
                    } else {
                        cur = None;
                    }
                } else {
                    break;
                }
            }
            cur
        })
        .collect()
}

fn ref_range_frequency(arr: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r)| {
            let mut freq: HashMap<i32, i32> = HashMap::new();
            for &v in &arr[l..=r] {
                *freq.entry(v).or_insert(0) += 1;
            }
            let max_count = *freq.values().max().unwrap();
            // Return the element with highest frequency; ties broken by smallest value
            *freq
                .iter()
                .filter(|(_, &cnt)| cnt == max_count)
                .map(|(&val, _)| val)
                .collect::<Vec<_>>()
                .iter()
                .min()
                .unwrap()
        })
        .collect()
}

fn ref_lcp_queries(s: &str, queries: &[(usize, usize)]) -> Vec<i32> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    // Build suffix array (naive)
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&a, &b| bytes[a..].cmp(&bytes[b..]));
    // Build LCP array
    let mut rank = vec![0usize; n];
    for (i, &s_idx) in sa.iter().enumerate() {
        rank[s_idx] = i;
    }
    let mut lcp_arr = vec![0i32; n.saturating_sub(1)];
    let mut h = 0usize;
    for i in 0..n {
        if rank[i] > 0 {
            let j = sa[rank[i] - 1];
            while i + h < n && j + h < n && bytes[i + h] == bytes[j + h] {
                h += 1;
            }
            lcp_arr[rank[i] - 1] = h as i32;
            h = h.saturating_sub(1);
        } else {
            h = 0;
        }
    }
    // Answer queries: min LCP in range [l, r) of the LCP array
    queries
        .iter()
        .map(|&(l, r)| {
            if l >= lcp_arr.len() || r > lcp_arr.len() || l >= r {
                return 0;
            }
            *lcp_arr[l..r].iter().min().unwrap_or(&0)
        })
        .collect()
}

fn ref_distinct_in_range(arr: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r)| {
            let mut seen = std::collections::HashSet::new();
            for &v in &arr[l..=r] {
                seen.insert(v);
            }
            seen.len() as i32
        })
        .collect()
}

// Build a tree from parents and compute LCA using brute force
fn ref_lca(tree_vals: &[Option<i32>], queries: &[(i32, i32)]) -> Vec<i32> {
    // Build adjacency from level-order tree
    use crate::problems::helpers::build_tree;
    let (arena, _root) = build_tree(tree_vals);
    if arena.is_empty() {
        return queries.iter().map(|_| -1).collect();
    }

    // Build parent map and val->node_index map
    let mut parent = vec![None::<usize>; arena.len()];
    for (i, node) in arena.iter().enumerate() {
        if let Some(l) = node.left {
            parent[l] = Some(i);
        }
        if let Some(r) = node.right {
            parent[r] = Some(i);
        }
    }

    // Map val -> first node index with that val
    let val_to_idx: HashMap<i32, usize> =
        arena.iter().enumerate().map(|(i, n)| (n.val, i)).collect();

    queries
        .iter()
        .map(|&(a, b)| {
            let idx_a = val_to_idx.get(&a);
            let idx_b = val_to_idx.get(&b);
            if idx_a.is_none() || idx_b.is_none() {
                return -1;
            }
            let mut ancestors_a = std::collections::HashSet::new();
            let mut cur = Some(*idx_a.unwrap());
            while let Some(c) = cur {
                ancestors_a.insert(c);
                cur = parent[c];
            }
            let mut cur = Some(*idx_b.unwrap());
            while let Some(c) = cur {
                if ancestors_a.contains(&c) {
                    return arena[c].val;
                }
                cur = parent[c];
            }
            -1
        })
        .collect()
}

// ── Easy 1: Range Min ────────────────────────────────────────────────

struct SparseTableRangeMin;

struct RangeMinTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableRangeMin {
    fn id(&self) -> &str {
        "sparse_table_range_min"
    }
    fn name(&self) -> &str {
        "Range Minimum Query"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Build a sparse table to answer range minimum queries in O(1) per query.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for min of arr[l..=r].\n\
         Output: Vec<i32> of answers."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=30);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                let q = rng.random_range(5..=15);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(RangeMinTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeMinTest>().unwrap();
        let expected = ref_range_min(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::range_min(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 2: Range Max ────────────────────────────────────────────────

struct SparseTableRangeMax;

struct RangeMaxTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableRangeMax {
    fn id(&self) -> &str {
        "sparse_table_range_max"
    }
    fn name(&self) -> &str {
        "Range Maximum Query"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Build a sparse table to answer range maximum queries in O(1) per query.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for max of arr[l..=r].\n\
         Output: Vec<i32> of answers."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=30);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                let q = rng.random_range(5..=15);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(RangeMaxTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeMaxTest>().unwrap();
        let expected = ref_range_max(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::range_max(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 3: Range GCD ────────────────────────────────────────────────

struct SparseTableRangeGcd;

struct RangeGcdTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableRangeGcd {
    fn id(&self) -> &str {
        "sparse_table_range_gcd"
    }
    fn name(&self) -> &str {
        "Range GCD Query"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Build a sparse table to answer range GCD queries.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for GCD of arr[l..=r].\n\
         Output: Vec<i32> of answers."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=25);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(1..=100)).collect();
                let q = rng.random_range(5..=15);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(RangeGcdTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeGcdTest>().unwrap();
        let expected = ref_range_gcd(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::range_gcd(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 4: Build Sparse Table ───────────────────────────────────────

struct SparseTableBuild;

struct BuildTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableBuild {
    fn id(&self) -> &str {
        "sparse_table_build"
    }
    fn name(&self) -> &str {
        "Build Sparse Table"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Build a sparse table and answer range minimum queries.\n\
         This is the same as sparse_table_range_min but focuses on the building step.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Output: Vec<i32> of minimum values."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=30);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                let q = rng.random_range(5..=15);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(BuildTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BuildTest>().unwrap();
        let expected = ref_range_min(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::build_and_query(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 5: Static RMQ ──────────────────────────────────────────────

struct SparseTableStaticRmq;

struct StaticRmqTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableStaticRmq {
    fn id(&self) -> &str {
        "sparse_table_static_rmq"
    }
    fn name(&self) -> &str {
        "Static RMQ (No Updates)"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Answer static range minimum queries (no updates allowed).\n\
         Build once in O(n log n), query each in O(1).\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Output: Vec<i32> of minimum values."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=30);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                let q = rng.random_range(5..=20);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(StaticRmqTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<StaticRmqTest>().unwrap();
        let expected = ref_range_min(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::static_rmq(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 1: LCA using Sparse Table ─────────────────────────────────

struct SparseTableLca;

struct LcaTest {
    tree: Vec<Option<i32>>,
    queries: Vec<(i32, i32)>,
}

impl Problem for SparseTableLca {
    fn id(&self) -> &str {
        "sparse_table_lca"
    }
    fn name(&self) -> &str {
        "LCA using Sparse Table + Euler Tour"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Find the Lowest Common Ancestor (LCA) of two nodes in a binary tree \
         using Euler tour + sparse table for RMQ.\n\n\
         Input: (tree: Vec<Option<i32>> level-order, queries: Vec<(i32, i32)>)\n\
         Each query (a, b) asks for the value of the LCA of nodes with values a and b.\n\
         Return -1 if either node is not found.\n\
         Output: Vec<i32>."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let size = rng.random_range(3..=10);
                let tree = crate::problems::helpers::random_tree(&mut rng, size, 1, 100);
                let (arena, _) = crate::problems::helpers::build_tree(&tree);
                let vals: Vec<i32> = arena.iter().map(|n| n.val).collect();
                let q = rng.random_range(2..=5);
                let queries: Vec<(i32, i32)> = (0..q)
                    .map(|_| {
                        let a = vals[rng.random_range(0..vals.len())];
                        let b = vals[rng.random_range(0..vals.len())];
                        (a, b)
                    })
                    .collect();
                TestCase {
                    data: Box::new(LcaTest { tree, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LcaTest>().unwrap();
        let expected = ref_lca(&t.tree, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_tree: Vec<Tracked<Option<i32>>> = t
            .tree
            .iter()
            .enumerate()
            .map(|(i, v)| Tracked::new(*v, i, shared_log.clone()))
            .collect();
        let tracked_queries: Vec<(Tracked<i32>, Tracked<i32>)> = t
            .queries
            .iter()
            .enumerate()
            .map(|(i, &(a, b))| {
                (
                    Tracked::new(a, t.tree.len() + i * 2, shared_log.clone()),
                    Tracked::new(b, t.tree.len() + i * 2 + 1, shared_log.clone()),
                )
            })
            .collect();
        let actual = solutions::lca_sparse_table(&tracked_tree, &tracked_queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, queries={:?}", t.tree, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 2: Range AND ──────────────────────────────────────────────

struct SparseTableRangeAnd;

struct RangeAndTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableRangeAnd {
    fn id(&self) -> &str {
        "sparse_table_range_and"
    }
    fn name(&self) -> &str {
        "Range Bitwise AND"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Build a sparse table for range bitwise AND queries.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for AND of arr[l..=r].\n\
         Output: Vec<i32>."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=25);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(0..=255)).collect();
                let q = rng.random_range(5..=15);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(RangeAndTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeAndTest>().unwrap();
        let expected = ref_range_and(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::range_and(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 3: Range OR ───────────────────────────────────────────────

struct SparseTableRangeOr;

struct RangeOrTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableRangeOr {
    fn id(&self) -> &str {
        "sparse_table_range_or"
    }
    fn name(&self) -> &str {
        "Range Bitwise OR"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Build a sparse table for range bitwise OR queries.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for OR of arr[l..=r].\n\
         Output: Vec<i32>."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=25);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(0..=255)).collect();
                let q = rng.random_range(5..=15);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(RangeOrTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeOrTest>().unwrap();
        let expected = ref_range_or(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::range_or(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 4: Second Minimum ─────────────────────────────────────────

struct SparseTableSecondMinimum;

struct SecondMinTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableSecondMinimum {
    fn id(&self) -> &str {
        "sparse_table_second_minimum"
    }
    fn name(&self) -> &str {
        "Second Minimum in Range"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Find the second smallest distinct value in a range.\n\
         If all elements in the range are equal, return that value.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for the second minimum in arr[l..=r].\n\
         Output: Vec<i32>."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(1..=30)).collect();
                let q = rng.random_range(5..=12);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(SecondMinTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SecondMinTest>().unwrap();
        let expected = ref_second_minimum(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::second_minimum(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 5: Index of Minimum ───────────────────────────────────────

struct SparseTableIndexOfMin;

struct IndexOfMinTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableIndexOfMin {
    fn id(&self) -> &str {
        "sparse_table_index_of_min"
    }
    fn name(&self) -> &str {
        "Index of Minimum in Range"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Return the index (not value) of the minimum element in a range.\n\
         If there are ties, return the leftmost index.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for the index of min in arr[l..=r].\n\
         Output: Vec<usize>."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=25);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                let q = rng.random_range(5..=15);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(IndexOfMinTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IndexOfMinTest>().unwrap();
        let expected = ref_index_of_min(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::index_of_min(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 1: 2D RMQ ──────────────────────────────────────────────────

struct SparseTable2dRmq;

struct Rmq2dTest {
    matrix: Vec<Vec<i32>>,
    queries: Vec<(usize, usize, usize, usize)>,
}

impl Problem for SparseTable2dRmq {
    fn id(&self) -> &str {
        "sparse_table_2d_rmq"
    }
    fn name(&self) -> &str {
        "2D Range Minimum Query"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Build a 2D sparse table for range minimum queries on a matrix.\n\n\
         Input: (matrix: Vec<Vec<i32>>, queries: Vec<(r1, c1, r2, c2)>)\n\
         Each query asks for the minimum in the sub-matrix [r1..=r2][c1..=c2].\n\
         Output: Vec<i32>."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(3..=8);
                let cols = rng.random_range(3..=8);
                let matrix: Vec<Vec<i32>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(-50..=50)).collect())
                    .collect();
                let q = rng.random_range(3..=10);
                let queries: Vec<(usize, usize, usize, usize)> = (0..q)
                    .map(|_| {
                        let r1 = rng.random_range(0..rows);
                        let r2 = rng.random_range(r1..rows);
                        let c1 = rng.random_range(0..cols);
                        let c2 = rng.random_range(c1..cols);
                        (r1, c1, r2, c2)
                    })
                    .collect();
                TestCase {
                    data: Box::new(Rmq2dTest { matrix, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<Rmq2dTest>().unwrap();
        let expected = ref_2d_rmq(&t.matrix, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked: Vec<Vec<Tracked<i32>>> = t
            .matrix
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, &v)| Tracked::new(v, r * row.len().max(1) + c, shared_log.clone()))
                    .collect()
            })
            .collect();
        let actual = solutions::rmq_2d(&tracked, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("matrix={:?}, queries={:?}", t.matrix, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 2: Kth Ancestor ────────────────────────────────────────────

struct SparseTableKthAncestor;

struct KthAncestorTest {
    parents: Vec<Option<usize>>,
    queries: Vec<(usize, usize)>, // (node, k)
}

impl Problem for SparseTableKthAncestor {
    fn id(&self) -> &str {
        "sparse_table_kth_ancestor"
    }
    fn name(&self) -> &str {
        "Kth Ancestor of Node"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a rooted tree (as parent array), find the kth ancestor of a node \
         using binary lifting (sparse table on ancestors).\n\n\
         Input: (parents: Vec<Option<usize>>, queries: Vec<(usize, usize)>)\n\
         parents[i] = parent of node i, None for root.\n\
         Each query (node, k) asks for the kth ancestor.\n\
         Output: Vec<Option<usize>> — None if ancestor does not exist."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let mut parents: Vec<Option<usize>> = vec![None; n];
                for (i, parent) in parents.iter_mut().enumerate().take(n).skip(1) {
                    *parent = Some(rng.random_range(0..i));
                }
                let q = rng.random_range(3..=10);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let node = rng.random_range(0..n);
                        let k = rng.random_range(0..=n);
                        (node, k)
                    })
                    .collect();
                TestCase {
                    data: Box::new(KthAncestorTest { parents, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KthAncestorTest>().unwrap();
        let expected = ref_kth_ancestor(&t.parents, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_parents: Vec<Tracked<Option<usize>>> = t
            .parents
            .iter()
            .enumerate()
            .map(|(i, v)| Tracked::new(*v, i, shared_log.clone()))
            .collect();
        let actual = solutions::kth_ancestor(&tracked_parents, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("parents={:?}, queries={:?}", t.parents, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 3: Range Frequency ──────────────────────────────────────────

struct SparseTableRangeFrequency;

struct RangeFreqTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableRangeFrequency {
    fn id(&self) -> &str {
        "sparse_table_range_frequency"
    }
    fn name(&self) -> &str {
        "Most Frequent Element in Range"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find the most frequent element in a range. Ties broken by smallest value.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for the most frequent value in arr[l..=r].\n\
         Output: Vec<i32>."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(1..=10)).collect();
                let q = rng.random_range(3..=10);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(RangeFreqTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeFreqTest>().unwrap();
        let expected = ref_range_frequency(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::range_frequency(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 4: LCP Array Queries ────────────────────────────────────────

struct SparseTableLcpArray;

struct LcpArrayTest {
    s: String,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableLcpArray {
    fn id(&self) -> &str {
        "sparse_table_longest_common_prefix_array"
    }
    fn name(&self) -> &str {
        "LCP Array Queries"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Build a suffix array and LCP array, then use a sparse table for RMQ \
         on the LCP array to answer queries.\n\n\
         Input: (s: String, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for min LCP in the LCP array at indices [l, r) \
         (i.e. the longest common prefix between the lth and rth suffixes in sorted order).\n\
         Output: Vec<i32>."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(5..=15);
                let s = crate::problems::helpers::random_string(&mut rng, len);
                let max_q_idx = if len > 1 { len - 1 } else { 1 };
                let q = rng.random_range(2..=6);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..max_q_idx);
                        let r = rng.random_range(l + 1..=max_q_idx);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(LcpArrayTest { s, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LcpArrayTest>().unwrap();
        let expected = ref_lcp_queries(&t.s, &t.queries);
        let actual = solutions::lcp_array_queries(&t.s, &t.queries);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\", queries={:?}", t.s, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 5: Distinct Elements in Range ───────────────────────────────

struct SparseTableDistinctInRange;

struct DistinctTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SparseTableDistinctInRange {
    fn id(&self) -> &str {
        "sparse_table_distinct_in_range"
    }
    fn name(&self) -> &str {
        "Count Distinct Elements in Range"
    }
    fn topic(&self) -> &str {
        "sparse_tables"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Count the number of distinct elements in a range.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for the count of distinct values in arr[l..=r].\n\
         Output: Vec<i32>."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(1..=10)).collect();
                let q = rng.random_range(3..=10);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(DistinctTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DistinctTest>().unwrap();
        let expected = ref_distinct_in_range(&t.arr, &t.queries);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::distinct_in_range(&tracked_arr, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}
