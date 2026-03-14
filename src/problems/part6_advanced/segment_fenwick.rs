use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part6_advanced::segment_fenwick as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy (5)
        Box::new(SegmentRangeSumQuery),
        Box::new(FenwickPrefixSum),
        Box::new(SegmentRangeMinQuery),
        Box::new(FenwickCountInversions),
        Box::new(SegmentPointUpdate),
        // Medium (5)
        Box::new(SegmentRangeUpdatePointQuery),
        Box::new(SegmentCountSmallerAfter),
        Box::new(Fenwick2dSum),
        Box::new(SegmentMergeSortTree),
        Box::new(SegmentLazyPropagation),
        // Hard (5)
        Box::new(SegmentPersistent),
        Box::new(SegmentIntervalScheduling),
        Box::new(FenwickRangeUpdateRangeQuery),
        Box::new(SegmentMaxSubarrayRange),
        Box::new(SegmentDynamic),
    ]
}

// ── Operation types for segment/fenwick problems ─────────────────────

// ── Reference implementations ────────────────────────────────────────

fn ref_range_sum_query(arr: &[i32], ops: &[(bool, usize, usize, i32)]) -> Vec<i32> {
    let mut a = arr.to_vec();
    let mut results = Vec::new();
    for &(is_update, x, y, val) in ops {
        if is_update {
            a[x] = val;
        } else {
            let sum: i32 = a[x..=y].iter().sum();
            results.push(sum);
        }
    }
    results
}

fn ref_prefix_sum(arr: &[i32], ops: &[(bool, usize, i32)]) -> Vec<i32> {
    let mut a = arr.to_vec();
    let mut results = Vec::new();
    for &(is_update, idx, val) in ops {
        if is_update {
            a[idx] += val;
        } else {
            let sum: i32 = a[..=idx].iter().sum();
            results.push(sum);
        }
    }
    results
}

fn ref_range_min(arr: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r)| *arr[l..=r].iter().min().unwrap())
        .collect()
}

fn ref_count_inversions(arr: &[i32]) -> i64 {
    let mut count: i64 = 0;
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] > arr[j] {
                count += 1;
            }
        }
    }
    count
}

fn ref_point_update_range_query(arr: &[i32], ops: &[(bool, usize, usize, i32)]) -> Vec<i32> {
    let mut a = arr.to_vec();
    let mut results = Vec::new();
    for &(is_update, x, y, val) in ops {
        if is_update {
            a[x] += val;
        } else {
            let sum: i32 = a[x..=y].iter().sum();
            results.push(sum);
        }
    }
    results
}

fn ref_range_update_point_query(arr: &[i32], ops: &[(bool, usize, usize, i32)]) -> Vec<i32> {
    let mut a = arr.to_vec();
    let mut results = Vec::new();
    for &(is_update, x, y, val) in ops {
        if is_update {
            for i in x..=y {
                a[i] += val;
            }
        } else {
            results.push(a[x]);
        }
    }
    results
}

fn ref_count_smaller_after(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![0i32; n];
    for i in 0..n {
        for j in (i + 1)..n {
            if nums[j] < nums[i] {
                result[i] += 1;
            }
        }
    }
    result
}

fn ref_2d_sum(
    matrix: &[Vec<i32>],
    ops: &[(bool, usize, usize, usize, usize, i32)],
) -> Vec<i32> {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };
    let mut mat: Vec<Vec<i32>> = matrix.to_vec();
    let mut results = Vec::new();
    for &(is_update, r1, c1, r2, c2, val) in ops {
        if is_update {
            mat[r1][c1] += val;
        } else {
            let mut sum = 0i32;
            for r in r1..=r2 {
                for c in c1..=c2 {
                    if r < rows && c < cols {
                        sum += mat[r][c];
                    }
                }
            }
            results.push(sum);
        }
    }
    results
}

fn ref_count_less_than_k(arr: &[i32], queries: &[(usize, usize, i32)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r, k)| arr[l..=r].iter().filter(|&&x| x < k).count() as i32)
        .collect()
}

fn ref_lazy_range_update_query(arr: &[i32], ops: &[(bool, usize, usize, i32)]) -> Vec<i32> {
    let mut a = arr.to_vec();
    let mut results = Vec::new();
    for &(is_update, x, y, val) in ops {
        if is_update {
            for i in x..=y {
                a[i] += val;
            }
        } else {
            let sum: i32 = a[x..=y].iter().sum();
            results.push(sum);
        }
    }
    results
}

fn ref_kth_smallest_in_range(arr: &[i32], queries: &[(usize, usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r, k)| {
            let mut sub: Vec<i32> = arr[l..=r].to_vec();
            sub.sort();
            sub[k.min(sub.len() - 1)]
        })
        .collect()
}

fn ref_max_non_overlapping(intervals: &[(i32, i32)]) -> i32 {
    let mut sorted: Vec<(i32, i32)> = intervals.to_vec();
    sorted.sort_by_key(|&(_, end)| end);
    let mut count = 0;
    let mut last_end = i32::MIN;
    for &(start, end) in &sorted {
        if start >= last_end {
            count += 1;
            last_end = end;
        }
    }
    count
}

fn ref_fenwick_range_update_range_query(
    arr: &[i32],
    ops: &[(bool, usize, usize, i32)],
) -> Vec<i32> {
    let mut a = arr.to_vec();
    let mut results = Vec::new();
    for &(is_update, x, y, val) in ops {
        if is_update {
            for i in x..=y {
                a[i] += val;
            }
        } else {
            let sum: i32 = a[x..=y].iter().sum();
            results.push(sum);
        }
    }
    results
}

fn ref_max_subarray_range(arr: &[i32], queries: &[(usize, usize)]) -> Vec<i32> {
    queries
        .iter()
        .map(|&(l, r)| {
            let mut max_sum = arr[l];
            for i in l..=r {
                let mut cur_sum = 0;
                for j in i..=r {
                    cur_sum += arr[j];
                    max_sum = max_sum.max(cur_sum);
                }
            }
            max_sum
        })
        .collect()
}

fn ref_dynamic_segment(
    updates: &[(i32, i32)],
    queries: &[(i32, i32)],
) -> Vec<i32> {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::new();
    for &(pos, val) in updates {
        *map.entry(pos).or_insert(0) += val;
    }
    queries
        .iter()
        .map(|&(l, r)| {
            map.range(l..=r).map(|(_, &v)| v).sum()
        })
        .collect()
}

// ── Easy 1: Range Sum Query (Mutable) ────────────────────────────────

struct SegmentRangeSumQuery;

struct RangeSumTest {
    arr: Vec<i32>,
    ops: Vec<(bool, usize, usize, i32)>, // (is_update, x, y, val)
}

impl Problem for SegmentRangeSumQuery {
    fn id(&self) -> &str { "segment_range_sum_query" }
    fn name(&self) -> &str { "Range Sum Query (Mutable)" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Implement a data structure that supports point updates and range sum queries.\n\n\
         Given an array and a list of operations:\n\
         - Update(index, value): set arr[index] = value\n\
         - Query(l, r): return sum of arr[l..=r]\n\n\
         Input: (arr: Vec<i32>, ops: Vec<(bool, usize, usize, i32)>)\n\
         - (true, idx, 0, val) = update arr[idx] to val\n\
         - (false, l, r, 0) = query sum of arr[l..=r]\n\
         Output: Vec<i32> of query results."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                let num_ops = rng.random_range(5..=15);
                let ops: Vec<(bool, usize, usize, i32)> = (0..num_ops)
                    .map(|_| {
                        if rng.random_range(0..2) == 0 {
                            let idx = rng.random_range(0..n);
                            let val = rng.random_range(-50..=50);
                            (true, idx, 0, val)
                        } else {
                            let l = rng.random_range(0..n);
                            let r = rng.random_range(l..n);
                            (false, l, r, 0)
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(RangeSumTest { arr, ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeSumTest>().unwrap();
        let expected = ref_range_sum_query(&t.arr, &t.ops);
        let actual = solutions::range_sum_query(&t.arr, &t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, ops={:?}", t.arr, t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 2: Fenwick Prefix Sum ───────────────────────────────────────

struct FenwickPrefixSum;

struct PrefixSumTest {
    arr: Vec<i32>,
    ops: Vec<(bool, usize, i32)>, // (is_update, idx, val)
}

impl Problem for FenwickPrefixSum {
    fn id(&self) -> &str { "fenwick_prefix_sum" }
    fn name(&self) -> &str { "Prefix Sum with Point Updates" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Implement prefix sum queries with point updates using a Fenwick tree (BIT).\n\n\
         Operations:\n\
         - (true, idx, delta): add delta to arr[idx]\n\
         - (false, idx, 0): return sum of arr[0..=idx]\n\n\
         Return Vec<i32> of query results."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(0..=50)).collect();
                let num_ops = rng.random_range(5..=15);
                let ops: Vec<(bool, usize, i32)> = (0..num_ops)
                    .map(|_| {
                        if rng.random_range(0..2) == 0 {
                            let idx = rng.random_range(0..n);
                            let val = rng.random_range(1..=20);
                            (true, idx, val)
                        } else {
                            let idx = rng.random_range(0..n);
                            (false, idx, 0)
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(PrefixSumTest { arr, ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PrefixSumTest>().unwrap();
        let expected = ref_prefix_sum(&t.arr, &t.ops);
        let actual = solutions::prefix_sum(&t.arr, &t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, ops={:?}", t.arr, t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 3: Range Minimum Query ──────────────────────────────────────

struct SegmentRangeMinQuery;

struct RangeMinTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SegmentRangeMinQuery {
    fn id(&self) -> &str { "segment_range_min_query" }
    fn name(&self) -> &str { "Range Minimum Query" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Build a segment tree on an array to answer range minimum queries.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Each query (l, r) asks for the minimum value in arr[l..=r].\n\
         Output: Vec<i32> of answers."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=25);
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

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeMinTest>().unwrap();
        let expected = ref_range_min(&t.arr, &t.queries);
        let actual = solutions::range_min_query(&t.arr, &t.queries);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 4: Count Inversions ─────────────────────────────────────────

struct FenwickCountInversions;

struct InversionsTest {
    arr: Vec<i32>,
}

impl Problem for FenwickCountInversions {
    fn id(&self) -> &str { "fenwick_count_inversions" }
    fn name(&self) -> &str { "Count Inversions" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Count the number of inversions in an array using a BIT (Fenwick tree).\n\n\
         An inversion is a pair (i, j) where i < j but arr[i] > arr[j].\n\n\
         Input: Vec<i32>\n\
         Output: i64 (count of inversions)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(1..=50)).collect();
                TestCase {
                    data: Box::new(InversionsTest { arr }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<InversionsTest>().unwrap();
        let expected = ref_count_inversions(&t.arr);
        let actual = solutions::count_inversions(&t.arr);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}", t.arr),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 5: Point Update Range Query ─────────────────────────────────

struct SegmentPointUpdate;

struct PointUpdateTest {
    arr: Vec<i32>,
    ops: Vec<(bool, usize, usize, i32)>,
}

impl Problem for SegmentPointUpdate {
    fn id(&self) -> &str { "segment_point_update" }
    fn name(&self) -> &str { "Point Update, Range Query" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Implement a segment tree supporting point update (add delta) and range sum query.\n\n\
         Operations:\n\
         - (true, idx, 0, delta): add delta to arr[idx]\n\
         - (false, l, r, 0): return sum of arr[l..=r]\n\n\
         Return Vec<i32> of query results."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-30..=30)).collect();
                let num_ops = rng.random_range(5..=15);
                let ops: Vec<(bool, usize, usize, i32)> = (0..num_ops)
                    .map(|_| {
                        if rng.random_range(0..2) == 0 {
                            let idx = rng.random_range(0..n);
                            let delta = rng.random_range(-20..=20);
                            (true, idx, 0, delta)
                        } else {
                            let l = rng.random_range(0..n);
                            let r = rng.random_range(l..n);
                            (false, l, r, 0)
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(PointUpdateTest { arr, ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PointUpdateTest>().unwrap();
        let expected = ref_point_update_range_query(&t.arr, &t.ops);
        let actual = solutions::point_update_range_query(&t.arr, &t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, ops={:?}", t.arr, t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 1: Range Update, Point Query ──────────────────────────────

struct SegmentRangeUpdatePointQuery;

struct RangeUpdatePointQueryTest {
    arr: Vec<i32>,
    ops: Vec<(bool, usize, usize, i32)>,
}

impl Problem for SegmentRangeUpdatePointQuery {
    fn id(&self) -> &str { "segment_range_update_point_query" }
    fn name(&self) -> &str { "Range Update, Point Query" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Support range updates (add delta to all elements in [l,r]) and point queries.\n\n\
         Operations:\n\
         - (true, l, r, delta): add delta to all arr[l..=r]\n\
         - (false, idx, 0, 0): return arr[idx]\n\n\
         Return Vec<i32> of query results."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-30..=30)).collect();
                let num_ops = rng.random_range(5..=15);
                let ops: Vec<(bool, usize, usize, i32)> = (0..num_ops)
                    .map(|_| {
                        if rng.random_range(0..2) == 0 {
                            let l = rng.random_range(0..n);
                            let r = rng.random_range(l..n);
                            let delta = rng.random_range(-10..=10);
                            (true, l, r, delta)
                        } else {
                            let idx = rng.random_range(0..n);
                            (false, idx, 0, 0)
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(RangeUpdatePointQueryTest { arr, ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeUpdatePointQueryTest>().unwrap();
        let expected = ref_range_update_point_query(&t.arr, &t.ops);
        let actual = solutions::range_update_point_query(&t.arr, &t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, ops={:?}", t.arr, t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 2: Count of Smaller Numbers After Self ────────────────────

struct SegmentCountSmallerAfter;

struct CountSmallerTest {
    nums: Vec<i32>,
}

impl Problem for SegmentCountSmallerAfter {
    fn id(&self) -> &str { "segment_count_smaller_after" }
    fn name(&self) -> &str { "Count of Smaller Numbers After Self" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an array, for each element count how many elements to its right are smaller.\n\n\
         Input: Vec<i32>\n\
         Output: Vec<i32> where result[i] = count of j > i such that nums[j] < nums[i].\n\n\
         Use a segment tree or BIT for O(n log n) solution."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=20);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
                TestCase {
                    data: Box::new(CountSmallerTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountSmallerTest>().unwrap();
        let expected = ref_count_smaller_after(&t.nums);
        let actual = solutions::count_smaller_after(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 3: 2D Prefix Sum with Updates ─────────────────────────────

struct Fenwick2dSum;

struct Fenwick2dTest {
    matrix: Vec<Vec<i32>>,
    // (is_update, r1, c1, r2, c2, val)
    // update: (true, r, c, 0, 0, val) — add val to matrix[r][c]
    // query:  (false, r1, c1, r2, c2, 0) — sum of submatrix [r1..=r2][c1..=c2]
    ops: Vec<(bool, usize, usize, usize, usize, i32)>,
}

impl Problem for Fenwick2dSum {
    fn id(&self) -> &str { "fenwick_2d_sum" }
    fn name(&self) -> &str { "2D Prefix Sum with Updates" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Implement a 2D Fenwick tree supporting point updates and rectangle sum queries.\n\n\
         Operations tuple: (is_update, r1, c1, r2, c2, val)\n\
         - Update: (true, r, c, 0, 0, val) — add val to matrix[r][c]\n\
         - Query: (false, r1, c1, r2, c2, 0) — sum of submatrix\n\n\
         Return Vec<i32> of query results."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(3..=8);
                let cols = rng.random_range(3..=8);
                let matrix: Vec<Vec<i32>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(0..=20)).collect())
                    .collect();
                let num_ops = rng.random_range(5..=12);
                let ops: Vec<(bool, usize, usize, usize, usize, i32)> = (0..num_ops)
                    .map(|_| {
                        if rng.random_range(0..2) == 0 {
                            let r = rng.random_range(0..rows);
                            let c = rng.random_range(0..cols);
                            let val = rng.random_range(1..=10);
                            (true, r, c, 0, 0, val)
                        } else {
                            let r1 = rng.random_range(0..rows);
                            let r2 = rng.random_range(r1..rows);
                            let c1 = rng.random_range(0..cols);
                            let c2 = rng.random_range(c1..cols);
                            (false, r1, c1, r2, c2, 0)
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(Fenwick2dTest { matrix, ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<Fenwick2dTest>().unwrap();
        let expected = ref_2d_sum(&t.matrix, &t.ops);
        let actual = solutions::fenwick_2d_sum(&t.matrix, &t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("matrix={:?}, ops={:?}", t.matrix, t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 4: Merge Sort Tree ────────────────────────────────────────

struct SegmentMergeSortTree;

struct MergeSortTreeTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize, i32)>, // (l, r, k) — count elements < k in arr[l..=r]
}

impl Problem for SegmentMergeSortTree {
    fn id(&self) -> &str { "segment_merge_sort_tree" }
    fn name(&self) -> &str { "Merge Sort Tree" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Build a merge sort tree (segment tree where each node stores a sorted list).\n\
         Answer queries: count elements in arr[l..=r] that are less than k.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize, i32)>)\n\
         Output: Vec<i32> of counts."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                let q = rng.random_range(5..=12);
                let queries: Vec<(usize, usize, i32)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        let k = rng.random_range(-50..=50);
                        (l, r, k)
                    })
                    .collect();
                TestCase {
                    data: Box::new(MergeSortTreeTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MergeSortTreeTest>().unwrap();
        let expected = ref_count_less_than_k(&t.arr, &t.queries);
        let actual = solutions::merge_sort_tree(&t.arr, &t.queries);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 5: Lazy Propagation ───────────────────────────────────────

struct SegmentLazyPropagation;

struct LazyPropTest {
    arr: Vec<i32>,
    ops: Vec<(bool, usize, usize, i32)>,
}

impl Problem for SegmentLazyPropagation {
    fn id(&self) -> &str { "segment_lazy_propagation" }
    fn name(&self) -> &str { "Lazy Propagation" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Implement a segment tree with lazy propagation for range update and range query.\n\n\
         Operations:\n\
         - (true, l, r, delta): add delta to all arr[l..=r]\n\
         - (false, l, r, 0): return sum of arr[l..=r]\n\n\
         Return Vec<i32> of query results."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
                let num_ops = rng.random_range(5..=15);
                let ops: Vec<(bool, usize, usize, i32)> = (0..num_ops)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        if rng.random_range(0..2) == 0 {
                            let delta = rng.random_range(-10..=10);
                            (true, l, r, delta)
                        } else {
                            (false, l, r, 0)
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(LazyPropTest { arr, ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LazyPropTest>().unwrap();
        let expected = ref_lazy_range_update_query(&t.arr, &t.ops);
        let actual = solutions::lazy_propagation(&t.arr, &t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, ops={:?}", t.arr, t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 1: Persistent Segment Tree ──────────────────────────────────

struct SegmentPersistent;

struct PersistentTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize, usize)>, // (l, r, k) — kth smallest in arr[l..=r]
}

impl Problem for SegmentPersistent {
    fn id(&self) -> &str { "segment_persistent" }
    fn name(&self) -> &str { "Persistent Segment Tree: Kth Smallest in Range" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Find the kth smallest element in a subarray using a persistent segment tree.\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize, usize)>)\n\
         Each query (l, r, k) asks for the kth smallest (0-indexed) in arr[l..=r].\n\
         Output: Vec<i32> of answers."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                let q = rng.random_range(3..=10);
                let queries: Vec<(usize, usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        let k = rng.random_range(0..=(r - l));
                        (l, r, k)
                    })
                    .collect();
                TestCase {
                    data: Box::new(PersistentTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PersistentTest>().unwrap();
        let expected = ref_kth_smallest_in_range(&t.arr, &t.queries);
        let actual = solutions::persistent_kth_smallest(&t.arr, &t.queries);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 2: Interval Scheduling with Segment Tree ────────────────────

struct SegmentIntervalScheduling;

struct IntervalSchedTest {
    intervals: Vec<(i32, i32)>,
}

impl Problem for SegmentIntervalScheduling {
    fn id(&self) -> &str { "segment_interval_scheduling" }
    fn name(&self) -> &str { "Max Non-Overlapping Intervals" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Find the maximum number of non-overlapping intervals.\n\
         Two intervals [a,b) and [c,d) are non-overlapping if b <= c or d <= a.\n\n\
         Input: Vec<(i32, i32)> — list of (start, end) intervals\n\
         Output: i32 — maximum count of non-overlapping intervals."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=15);
                let intervals: Vec<(i32, i32)> = (0..n)
                    .map(|_| {
                        let s = rng.random_range(0..=50);
                        let e = rng.random_range(s + 1..=60);
                        (s, e)
                    })
                    .collect();
                TestCase {
                    data: Box::new(IntervalSchedTest { intervals }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IntervalSchedTest>().unwrap();
        let expected = ref_max_non_overlapping(&t.intervals);
        let actual = solutions::max_non_overlapping_intervals(&t.intervals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("intervals={:?}", t.intervals),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 3: Fenwick Range Update Range Query ─────────────────────────

struct FenwickRangeUpdateRangeQuery;

struct FenwickRURQTest {
    arr: Vec<i32>,
    ops: Vec<(bool, usize, usize, i32)>,
}

impl Problem for FenwickRangeUpdateRangeQuery {
    fn id(&self) -> &str { "fenwick_range_update_range_query" }
    fn name(&self) -> &str { "Range Update + Range Query (BIT)" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Implement range update and range query using two Fenwick trees.\n\n\
         Operations:\n\
         - (true, l, r, delta): add delta to all arr[l..=r]\n\
         - (false, l, r, 0): return sum of arr[l..=r]\n\n\
         Return Vec<i32> of query results."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
                let num_ops = rng.random_range(5..=15);
                let ops: Vec<(bool, usize, usize, i32)> = (0..num_ops)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        if rng.random_range(0..2) == 0 {
                            let delta = rng.random_range(-10..=10);
                            (true, l, r, delta)
                        } else {
                            (false, l, r, 0)
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(FenwickRURQTest { arr, ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FenwickRURQTest>().unwrap();
        let expected = ref_fenwick_range_update_range_query(&t.arr, &t.ops);
        let actual = solutions::fenwick_range_update_range_query(&t.arr, &t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, ops={:?}", t.arr, t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 4: Max Subarray Sum in Range ────────────────────────────────

struct SegmentMaxSubarrayRange;

struct MaxSubarrayRangeTest {
    arr: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for SegmentMaxSubarrayRange {
    fn id(&self) -> &str { "segment_max_subarray_range" }
    fn name(&self) -> &str { "Max Subarray Sum in Range" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Build a segment tree where each node stores prefix max, suffix max, total, and max \
         subarray sum. Answer queries for max subarray sum in arr[l..=r].\n\n\
         Input: (arr: Vec<i32>, queries: Vec<(usize, usize)>)\n\
         Output: Vec<i32> of answers."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=20);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(-30..=30)).collect();
                let q = rng.random_range(5..=12);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l..n);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(MaxSubarrayRangeTest { arr, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxSubarrayRangeTest>().unwrap();
        let expected = ref_max_subarray_range(&t.arr, &t.queries);
        let actual = solutions::max_subarray_in_range(&t.arr, &t.queries);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, queries={:?}", t.arr, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 5: Dynamic Segment Tree ─────────────────────────────────────

struct SegmentDynamic;

struct DynamicSegTest {
    updates: Vec<(i32, i32)>, // (position, value) — add value at position
    queries: Vec<(i32, i32)>, // (l, r) — sum in range [l, r]
}

impl Problem for SegmentDynamic {
    fn id(&self) -> &str { "segment_dynamic" }
    fn name(&self) -> &str { "Dynamic Segment Tree with Coordinate Compression" }
    fn topic(&self) -> &str { "segment_fenwick" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Implement a dynamic segment tree (or use coordinate compression) to handle \
         sparse updates and range sum queries over a large coordinate space.\n\n\
         Input: (updates: Vec<(i32, i32)>, queries: Vec<(i32, i32)>)\n\
         - updates: (position, value) — add value at position\n\
         - queries: (l, r) — return sum of all values at positions in [l, r]\n\
         Output: Vec<i32> of query results."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n_updates = rng.random_range(3..=15);
                let updates: Vec<(i32, i32)> = (0..n_updates)
                    .map(|_| {
                        let pos = rng.random_range(0..=1_000_000);
                        let val = rng.random_range(1..=100);
                        (pos, val)
                    })
                    .collect();
                let n_queries = rng.random_range(3..=10);
                let queries: Vec<(i32, i32)> = (0..n_queries)
                    .map(|_| {
                        let l = rng.random_range(0..=1_000_000);
                        let r = rng.random_range(l..=1_000_000);
                        (l, r)
                    })
                    .collect();
                TestCase {
                    data: Box::new(DynamicSegTest { updates, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DynamicSegTest>().unwrap();
        let expected = ref_dynamic_segment(&t.updates, &t.queries);
        let actual = solutions::dynamic_segment_tree(&t.updates, &t.queries);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("updates={:?}, queries={:?}", t.updates, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}
