use rand::Rng;
use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part2_sorting::prefix_sum as solutions;
use crate::tracker::{track_slice, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(RangeSum),
        Box::new(RunningSum),
        Box::new(PivotIndex),
        Box::new(SumOfAbsoluteDifferences),
        Box::new(CountNegatives),
        Box::new(SubarraySumK),
        Box::new(ProductExceptSelf),
        Box::new(ContiguousArray),
        Box::new(RangeSum2D),
        Box::new(MaxSubarrayLength),
        Box::new(CountRangeSum),
        Box::new(MaxSumRectangle),
        Box::new(ShortestSubarraySumK),
        Box::new(NumberOfSubarraysOddSum),
        Box::new(XorQueries),
    ]
}

// ── Easy 1: Range Sum Query ─────────────────────────────────────────────

struct RangeSum;
struct RangeSumTest {
    nums: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for RangeSum {
    fn id(&self) -> &str {
        "prefix_sum_range_sum"
    }
    fn name(&self) -> &str {
        "Range Sum Query - Immutable"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an integer array `nums` and a list of queries `(left, right)`, return an \
         array of sums where each sum is the total of `nums[left..=right]`.\n\n\
         Pre-compute a prefix sum array for O(1) per query."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                let q = rng.random_range(1..=10);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let a = rng.random_range(0..n);
                        let b = rng.random_range(a..n);
                        (a, b)
                    })
                    .collect();
                TestCase {
                    data: Box::new(RangeSumTest { nums, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeSumTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected: Vec<i32> = t
            .queries
            .iter()
            .map(|&(l, r)| t.nums[l..=r].iter().sum())
            .collect();
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::range_sum(&tracked_nums, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, queries={:?}", t.nums, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 2: Running Sum ────────────────────────────────────────────────

struct RunningSum;
struct RunningSumTest {
    nums: Vec<i32>,
}

impl Problem for RunningSum {
    fn id(&self) -> &str {
        "prefix_sum_running_sum"
    }
    fn name(&self) -> &str {
        "Running Sum of 1D Array"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array `nums`, return the running sum where `running_sum[i] = sum(nums[0..=i])`."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(RunningSumTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RunningSumTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let mut expected = Vec::with_capacity(t.nums.len());
        let mut acc = 0;
        for &v in &t.nums {
            acc += v;
            expected.push(acc);
        }
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::running_sum(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 3: Pivot Index ────────────────────────────────────────────────

struct PivotIndex;
struct PivotIndexTest {
    nums: Vec<i32>,
}

impl Problem for PivotIndex {
    fn id(&self) -> &str {
        "prefix_sum_pivot_index"
    }
    fn name(&self) -> &str {
        "Find Pivot Index"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array `nums`, return the leftmost pivot index where the sum of elements \
         strictly to the left equals the sum of elements strictly to the right.\n\n\
         Return -1 if no such index exists. The element at the pivot is not included in \
         either side's sum."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                TestCase {
                    data: Box::new(PivotIndexTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PivotIndexTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_pivot_index(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::pivot_index(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_pivot_index(nums: &[i32]) -> i32 {
    let total: i32 = nums.iter().sum();
    let mut left_sum = 0;
    for (i, &v) in nums.iter().enumerate() {
        if left_sum == total - left_sum - v {
            return i as i32;
        }
        left_sum += v;
    }
    -1
}

// ── Easy 4: Sum of Absolute Differences ────────────────────────────────

struct SumOfAbsoluteDifferences;
struct SADTest {
    nums: Vec<i32>,
}

impl Problem for SumOfAbsoluteDifferences {
    fn id(&self) -> &str {
        "prefix_sum_sum_of_absolute_differences"
    }
    fn name(&self) -> &str {
        "Sum of Absolute Differences in Sorted Array"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a sorted integer array `nums`, return an array `result` where \
         `result[i] = sum of |nums[i] - nums[j]|` for all j != i.\n\n\
         Use prefix sums for an O(n) solution."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=20);
                let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(1..=100)).collect();
                nums.sort();
                TestCase {
                    data: Box::new(SADTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SADTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_sum_abs_diff(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sum_of_absolute_differences(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_sum_abs_diff(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + nums[i] as i64;
    }
    let total = prefix[n];
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let left_sum = prefix[i];
        let right_sum = total - prefix[i + 1];
        let val = nums[i] as i64;
        // For elements to the left: all are <= nums[i], so sum of |nums[i]-nums[j]| = i*val - left_sum
        // For elements to the right: all are >= nums[i], so sum = right_sum - (n-i-1)*val
        let ans = (i as i64 * val - left_sum) + (right_sum - (n - i - 1) as i64 * val);
        result.push(ans as i32);
    }
    result
}

// ── Easy 5: Count Negatives in Sorted Matrix ───────────────────────────

struct CountNegatives;
struct CountNegTest {
    matrix: Vec<Vec<i32>>,
}

impl Problem for CountNegatives {
    fn id(&self) -> &str {
        "prefix_sum_count_negatives"
    }
    fn name(&self) -> &str {
        "Count Negatives in a Sorted Matrix"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an m x n matrix `grid` where each row and column is sorted in non-increasing \
         order, return the number of negative numbers in `grid`."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(1..=8);
                let cols = rng.random_range(1..=8);
                let matrix: Vec<Vec<i32>> = (0..rows)
                    .map(|_| {
                        let mut row: Vec<i32> =
                            (0..cols).map(|_| rng.random_range(-50..=50)).collect();
                        row.sort_by(|a, b| b.cmp(a)); // non-increasing
                        row
                    })
                    .collect();
                TestCase {
                    data: Box::new(CountNegTest { matrix }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountNegTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected: i32 = t
            .matrix
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&v| v < 0)
            .count() as i32;
        let tracked_matrix: Vec<Vec<_>> = t
            .matrix
            .iter()
            .map(|v| track_slice(v, shared_log.clone()))
            .collect();
        let actual = solutions::count_negatives(&tracked_matrix);
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

// ── Medium 1: Subarray Sum Equals K ────────────────────────────────────

struct SubarraySumK;
struct SubSumKTest {
    nums: Vec<i32>,
    k: i32,
}

impl Problem for SubarraySumK {
    fn id(&self) -> &str {
        "prefix_sum_subarray_sum_k"
    }
    fn name(&self) -> &str {
        "Subarray Sum Equals K"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array `nums` and an integer `k`, return the total number of contiguous \
         subarrays whose sum equals `k`.\n\n\
         Use a prefix sum + hash map approach for O(n) time."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
                let k = rng.random_range(-30..=30);
                TestCase {
                    data: Box::new(SubSumKTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SubSumKTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_subarray_sum_k(&t.nums, t.k);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::subarray_sum_k(&tracked_nums, t.k);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_subarray_sum_k(nums: &[i32], k: i32) -> i32 {
    let mut count = 0;
    let mut prefix_sum = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, 1);
    for &v in nums {
        prefix_sum += v;
        if let Some(&c) = map.get(&(prefix_sum - k)) {
            count += c;
        }
        *map.entry(prefix_sum).or_insert(0) += 1;
    }
    count
}

// ── Medium 2: Product of Array Except Self ─────────────────────────────

struct ProductExceptSelf;
struct ProductTest {
    nums: Vec<i32>,
}

impl Problem for ProductExceptSelf {
    fn id(&self) -> &str {
        "prefix_sum_product_except_self"
    }
    fn name(&self) -> &str {
        "Product of Array Except Self"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an integer array `nums`, return an array `answer` such that `answer[i]` \
         is equal to the product of all elements of `nums` except `nums[i]`.\n\n\
         Do not use division. Solve in O(n) using prefix and suffix products."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-10..=10)).collect();
                TestCase {
                    data: Box::new(ProductTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ProductTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let n = t.nums.len();
        let mut expected = vec![1i32; n];
        for (i, expected_val) in expected.iter_mut().enumerate().take(n) {
            for (j, &v) in t.nums.iter().enumerate() {
                if i != j {
                    *expected_val *= v;
                }
            }
        }
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::product_except_self(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 3: Contiguous Array ─────────────────────────────────────────

struct ContiguousArray;
struct ContiguousArrayTest {
    nums: Vec<i32>,
}

impl Problem for ContiguousArray {
    fn id(&self) -> &str {
        "prefix_sum_contiguous_array"
    }
    fn name(&self) -> &str {
        "Contiguous Array"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a binary array `nums` (containing only 0s and 1s), find the maximum length \
         of a contiguous subarray with an equal number of 0 and 1.\n\n\
         Hint: treat 0 as -1 and use prefix sums."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=1)).collect();
                TestCase {
                    data: Box::new(ContiguousArrayTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ContiguousArrayTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_contiguous_array(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::contiguous_array(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_contiguous_array(nums: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, -1);
    let mut max_len = 0;
    let mut count = 0;
    for (i, &v) in nums.iter().enumerate() {
        count += if v == 1 { 1 } else { -1 };
        if let Some(&prev_i) = map.get(&count) {
            max_len = max_len.max(i as i32 - prev_i);
        } else {
            map.insert(count, i as i32);
        }
    }
    max_len
}

// ── Medium 4: 2D Range Sum Query ───────────────────────────────────────

struct RangeSum2D;
struct RangeSum2DTest {
    matrix: Vec<Vec<i32>>,
    queries: Vec<(usize, usize, usize, usize)>,
}

impl Problem for RangeSum2D {
    fn id(&self) -> &str {
        "prefix_sum_2d_range_sum"
    }
    fn name(&self) -> &str {
        "Range Sum Query 2D - Immutable"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a 2D matrix and a list of queries (row1, col1, row2, col2), return the sum \
         of elements in the rectangle defined by (row1, col1) as the upper-left corner and \
         (row2, col2) as the lower-right corner (inclusive).\n\n\
         Pre-compute a 2D prefix sum for O(1) per query."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(1..=8);
                let cols = rng.random_range(1..=8);
                let matrix: Vec<Vec<i32>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(-50..=50)).collect())
                    .collect();
                let q = rng.random_range(1..=8);
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
                    data: Box::new(RangeSum2DTest { matrix, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RangeSum2DTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected: Vec<i32> = t
            .queries
            .iter()
            .map(|&(r1, c1, r2, c2)| {
                let mut sum = 0;
                for r in r1..=r2 {
                    for c in c1..=c2 {
                        sum += t.matrix[r][c];
                    }
                }
                sum
            })
            .collect();
        let tracked_matrix: Vec<Vec<_>> = t
            .matrix
            .iter()
            .map(|v| track_slice(v, shared_log.clone()))
            .collect();
        let actual = solutions::range_sum_2d(&tracked_matrix, &t.queries);
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

// ── Medium 5: Maximum Length Subarray with Sum <= k ─────────────────────

struct MaxSubarrayLength;
struct MaxSubLenTest {
    nums: Vec<i32>,
    k: i32,
}

impl Problem for MaxSubarrayLength {
    fn id(&self) -> &str {
        "prefix_sum_max_subarray_length"
    }
    fn name(&self) -> &str {
        "Maximum Length Subarray with Sum at Most K"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array of positive integers `nums` and a positive integer `k`, return the \
         maximum length of a subarray whose sum is less than or equal to `k`.\n\n\
         Return 0 if no such subarray exists."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(1..=20)).collect();
                let k = rng.random_range(1..=100);
                TestCase {
                    data: Box::new(MaxSubLenTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxSubLenTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_max_subarray_length(&t.nums, t.k);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::max_subarray_length(&tracked_nums, t.k);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_subarray_length(nums: &[i32], k: i32) -> i32 {
    let mut max_len = 0;
    let mut left = 0;
    let mut sum = 0;
    for right in 0..nums.len() {
        sum += nums[right];
        while sum > k && left <= right {
            sum -= nums[left];
            left += 1;
        }
        if sum <= k {
            max_len = max_len.max((right - left + 1) as i32);
        }
    }
    max_len
}

// ── Hard 1: Count of Range Sum ─────────────────────────────────────────

struct CountRangeSum;
struct CountRangeSumTest {
    nums: Vec<i32>,
    lower: i32,
    upper: i32,
}

impl Problem for CountRangeSum {
    fn id(&self) -> &str {
        "prefix_sum_count_range_sum"
    }
    fn name(&self) -> &str {
        "Count of Range Sum"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an integer array `nums` and two integers `lower` and `upper`, return the \
         number of range sums that lie in [lower, upper] inclusive.\n\n\
         Range sum S(i, j) = nums[i] + nums[i+1] + ... + nums[j] for 0 <= i <= j < n."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
                let a = rng.random_range(-50..=50);
                let b = rng.random_range(a..=a + 50);
                TestCase {
                    data: Box::new(CountRangeSumTest {
                        nums,
                        lower: a,
                        upper: b,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountRangeSumTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_count_range_sum(&t.nums, t.lower, t.upper);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::count_range_sum(&tracked_nums, t.lower, t.upper);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, lower={}, upper={}", t.nums, t.lower, t.upper),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_count_range_sum(nums: &[i32], lower: i32, upper: i32) -> i32 {
    let n = nums.len();
    let mut count = 0;
    let lower = lower as i64;
    let upper = upper as i64;
    for i in 0..n {
        let mut sum: i64 = 0;
        for &num in nums.iter().take(n).skip(i) {
            sum += num as i64;
            if sum >= lower && sum <= upper {
                count += 1;
            }
        }
    }
    count
}

// ── Hard 2: Maximum Sum Rectangle ──────────────────────────────────────

struct MaxSumRectangle;
struct MaxSumRectTest {
    matrix: Vec<Vec<i32>>,
}

impl Problem for MaxSumRectangle {
    fn id(&self) -> &str {
        "prefix_sum_max_sum_rectangle"
    }
    fn name(&self) -> &str {
        "Maximum Sum Rectangle in 2D Matrix"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a 2D matrix of integers, find the rectangle (submatrix) with the \
         maximum sum. Return that sum.\n\n\
         Use Kadane's algorithm combined with column prefix sums for an efficient solution."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(1..=8);
                let cols = rng.random_range(1..=8);
                let matrix: Vec<Vec<i32>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(-20..=20)).collect())
                    .collect();
                TestCase {
                    data: Box::new(MaxSumRectTest { matrix }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxSumRectTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_max_sum_rectangle(&t.matrix);
        let tracked_matrix: Vec<Vec<_>> = t
            .matrix
            .iter()
            .map(|v| track_slice(v, shared_log.clone()))
            .collect();
        let actual = solutions::max_sum_rectangle(&tracked_matrix);
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

fn ref_max_sum_rectangle(matrix: &[Vec<i32>]) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut max_sum = i32::MIN;
    #[allow(clippy::needless_range_loop)]
    for left in 0..cols {
        let mut temp = vec![0i32; rows];
        for right in left..cols {
            for (r, t) in temp.iter_mut().enumerate().take(rows) {
                *t += matrix[r][right];
            }
            // Kadane's on temp
            let mut cur = temp[0];
            let mut best = temp[0];
            for &v in &temp[1..] {
                cur = v.max(cur + v);
                best = best.max(cur);
            }
            max_sum = max_sum.max(best);
        }
    }
    max_sum
}

// ── Hard 3: Shortest Subarray with Sum >= K ────────────────────────────

struct ShortestSubarraySumK;
struct ShortestSubKTest {
    nums: Vec<i32>,
    k: i32,
}

impl Problem for ShortestSubarraySumK {
    fn id(&self) -> &str {
        "prefix_sum_shortest_subarray_sum_k"
    }
    fn name(&self) -> &str {
        "Shortest Subarray with Sum at Least K"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an integer array `nums` (may contain negative values) and an integer `k`, \
         return the length of the shortest non-empty subarray whose sum is at least `k`.\n\n\
         Return -1 if no such subarray exists.\n\n\
         Use prefix sums with a monotonic deque for O(n) time."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=30)).collect();
                let k = rng.random_range(1..=50);
                TestCase {
                    data: Box::new(ShortestSubKTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ShortestSubKTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_shortest_subarray_sum_k(&t.nums, t.k);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::shortest_subarray_sum_k(&tracked_nums, t.k);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_shortest_subarray_sum_k(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let mut min_len = i32::MAX;
    for i in 0..n {
        let mut sum: i64 = 0;
        for (j, &num) in nums.iter().enumerate().take(n).skip(i) {
            sum += num as i64;
            if sum >= k as i64 {
                min_len = min_len.min((j - i + 1) as i32);
                break; // No need to extend further from this start for brute force min
            }
        }
    }
    if min_len == i32::MAX {
        -1
    } else {
        min_len
    }
}

// ── Hard 4: Number of Subarrays with Odd Sum ───────────────────────────

struct NumberOfSubarraysOddSum;
struct OddSumTest {
    nums: Vec<i32>,
}

impl Problem for NumberOfSubarraysOddSum {
    fn id(&self) -> &str {
        "prefix_sum_number_of_subarrays_odd_sum"
    }
    fn name(&self) -> &str {
        "Number of Sub-arrays With Odd Sum"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array of integers `nums`, return the number of subarrays that have an \
         odd sum. Return the answer modulo 10^9 + 7.\n\n\
         Use prefix sum parity tracking for an efficient solution."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(1..=100)).collect();
                TestCase {
                    data: Box::new(OddSumTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<OddSumTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_subarrays_odd_sum(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::number_of_subarrays_odd_sum(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_subarrays_odd_sum(nums: &[i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    // Count prefix sums with even/odd parity.
    // A subarray sum is odd when prefix[j] and prefix[i] have different parities.
    let mut count = 0i64;
    let mut even_count = 1i64; // prefix[0] = 0 is even
    let mut odd_count = 0i64;
    let mut prefix_sum = 0i64;
    for &v in nums {
        prefix_sum += v as i64;
        if prefix_sum % 2 == 0 {
            // This prefix is even; odd subarrays end here when paired with odd prefixes
            count = (count + odd_count) % MOD;
            even_count += 1;
        } else {
            // This prefix is odd; odd subarrays end here when paired with even prefixes
            count = (count + even_count) % MOD;
            odd_count += 1;
        }
    }
    count as i32
}

// ── Hard 5: XOR Queries of a Subarray ──────────────────────────────────

struct XorQueries;
struct XorQueriesTest {
    nums: Vec<i32>,
    queries: Vec<(usize, usize)>,
}

impl Problem for XorQueries {
    fn id(&self) -> &str {
        "prefix_sum_xor_queries"
    }
    fn name(&self) -> &str {
        "XOR Queries of a Subarray"
    }
    fn topic(&self) -> &str {
        "prefix_sum"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array `nums` and a list of queries `(left, right)`, return an array of \
         answers where each answer is the XOR of elements from `nums[left]` to `nums[right]`.\n\n\
         Pre-compute a prefix XOR array for O(1) per query."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=1000)).collect();
                let q = rng.random_range(1..=10);
                let queries: Vec<(usize, usize)> = (0..q)
                    .map(|_| {
                        let a = rng.random_range(0..n);
                        let b = rng.random_range(a..n);
                        (a, b)
                    })
                    .collect();
                TestCase {
                    data: Box::new(XorQueriesTest { nums, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<XorQueriesTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected: Vec<i32> = t
            .queries
            .iter()
            .map(|&(l, r)| t.nums[l..=r].iter().fold(0, |acc, &x| acc ^ x))
            .collect();
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::xor_queries(&tracked_nums, &t.queries);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, queries={:?}", t.nums, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}
