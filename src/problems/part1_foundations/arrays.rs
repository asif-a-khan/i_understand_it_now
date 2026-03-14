use rand::Rng;
use std::collections::HashMap;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part1_foundations::arrays as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(TwoSum),
        Box::new(ContainsDuplicate),
        Box::new(RemoveDuplicatesSorted),
        Box::new(BestTimeToBuyAndSellStock),
        Box::new(MergeSortedArrays),
        Box::new(MaxSubarray),
        Box::new(RotateArray),
        Box::new(ProductExceptSelf),
        Box::new(NextPermutation),
        Box::new(SpiralMatrix),
        Box::new(TrappingRainWater),
        Box::new(FirstMissingPositive),
        Box::new(MedianTwoSortedArrays),
        Box::new(LongestConsecutiveSequence),
        Box::new(MinimumWindowSort),
    ]
}

// ── Easy 1: Two Sum ────────────────────────────────────────────────────

struct TwoSum;

struct TwoSumTest {
    nums: Vec<i32>,
    target: i32,
}

impl Problem for TwoSum {
    fn id(&self) -> &str {
        "arrays_two_sum"
    }
    fn name(&self) -> &str {
        "Two Sum"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array of integers `nums` and an integer `target`, return the indices \
         of the two numbers that add up to `target`.\n\n\
         Constraints:\n\
         - 2 <= nums.len() <= 1000\n\
         - Exactly one valid answer exists.\n\
         - Return indices in ascending order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=20);
                let idx_a = rng.random_range(0..n);
                let mut idx_b = rng.random_range(0..n);
                while idx_b == idx_a {
                    idx_b = rng.random_range(0..n);
                }
                let val_a: i32 = rng.random_range(-1000..=1000);
                let val_b: i32 = rng.random_range(-1000..=1000);
                let target = val_a + val_b;
                let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(-1000..=1000)).collect();
                nums[idx_a] = val_a;
                nums[idx_b] = val_b;
                TestCase {
                    data: Box::new(TwoSumTest { nums, target }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TwoSumTest>().unwrap();
        let expected = ref_two_sum(&t.nums, t.target);
        let actual = solutions::two_sum(&t.nums, t.target);
        let mut es = expected.clone();
        es.sort();
        let mut ac = actual.clone();
        ac.sort();
        SolutionResult {
            is_correct: es == ac,
            input_description: format!("nums={:?}, target={}", t.nums, t.target),
            expected: format!("{es:?}"),
            actual: format!("{ac:?}"),
        }
    }
}

fn ref_two_sum(nums: &[i32], target: i32) -> Vec<usize> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j, i];
        }
        map.insert(num, i);
    }
    vec![]
}

// ── Easy 2: Contains Duplicate ─────────────────────────────────────────

struct ContainsDuplicate;
struct ContainsDuplicateTest {
    nums: Vec<i32>,
}

impl Problem for ContainsDuplicate {
    fn id(&self) -> &str {
        "arrays_contains_duplicate"
    }
    fn name(&self) -> &str {
        "Contains Duplicate"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an integer array `nums`, return `true` if any value appears at least twice, \
         and `false` if every element is distinct."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                TestCase {
                    data: Box::new(ContainsDuplicateTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ContainsDuplicateTest>().unwrap();
        let mut seen = std::collections::HashSet::new();
        let expected = t.nums.iter().any(|x| !seen.insert(x));
        let actual = solutions::contains_duplicate(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 3: Remove Duplicates from Sorted Array ────────────────────────

struct RemoveDuplicatesSorted;
struct RemoveDuplicatesTest {
    nums: Vec<i32>,
}

impl Problem for RemoveDuplicatesSorted {
    fn id(&self) -> &str {
        "arrays_remove_duplicates_sorted"
    }
    fn name(&self) -> &str {
        "Remove Duplicates from Sorted Array"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a sorted integer array `nums`, remove the duplicates in-place such that \
         each element appears only once. Return the number of unique elements.\n\n\
         Modify the first k elements of `nums` to hold the unique values."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                nums.sort();
                TestCase {
                    data: Box::new(RemoveDuplicatesTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RemoveDuplicatesTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.dedup();
        let expected_k = expected.len();

        let mut input = t.nums.clone();
        let actual_k = solutions::remove_duplicates_sorted(&mut input);
        let actual_slice = &input[..actual_k];

        SolutionResult {
            is_correct: actual_k == expected_k && actual_slice == &expected[..],
            input_description: format!("nums={:?}", t.nums),
            expected: format!("k={expected_k}, vals={expected:?}"),
            actual: format!("k={actual_k}, vals={actual_slice:?}"),
        }
    }
}

// ── Easy 4: Best Time to Buy and Sell Stock ────────────────────────────

struct BestTimeToBuyAndSellStock;
struct StockTest {
    prices: Vec<i32>,
}

impl Problem for BestTimeToBuyAndSellStock {
    fn id(&self) -> &str {
        "arrays_max_profit"
    }
    fn name(&self) -> &str {
        "Best Time to Buy and Sell Stock"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array `prices` where `prices[i]` is the price on day i, find the maximum \
         profit from one buy and one sell. Return 0 if no profit is possible."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=50);
                let prices: Vec<i32> = (0..n).map(|_| rng.random_range(1..=1000)).collect();
                TestCase {
                    data: Box::new(StockTest { prices }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<StockTest>().unwrap();
        let mut min_price = i32::MAX;
        let mut max_profit = 0;
        for &p in &t.prices {
            min_price = min_price.min(p);
            max_profit = max_profit.max(p - min_price);
        }
        let actual = solutions::max_profit(&t.prices);
        SolutionResult {
            is_correct: max_profit == actual,
            input_description: format!("prices={:?}", t.prices),
            expected: format!("{max_profit}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 5: Merge Sorted Arrays ────────────────────────────────────────

struct MergeSortedArrays;
struct MergeTest {
    nums1: Vec<i32>,
    m: usize,
    nums2: Vec<i32>,
}

impl Problem for MergeSortedArrays {
    fn id(&self) -> &str {
        "arrays_merge_sorted"
    }
    fn name(&self) -> &str {
        "Merge Sorted Arrays"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Merge `nums2` into `nums1` as one sorted array. `nums1` has enough space \
         (size m+n) with the first `m` elements being the actual values."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let m = rng.random_range(0..=15);
                let n = rng.random_range(0..=15);
                let mut nums1: Vec<i32> = (0..m).map(|_| rng.random_range(-50..=50)).collect();
                nums1.sort();
                nums1.resize(m + n, 0);
                let mut nums2: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                nums2.sort();
                TestCase {
                    data: Box::new(MergeTest { nums1, m, nums2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MergeTest>().unwrap();
        let mut expected = t.nums1[..t.m].to_vec();
        expected.extend_from_slice(&t.nums2);
        expected.sort();

        let mut actual = t.nums1.clone();
        solutions::merge_sorted(&mut actual, t.m, &t.nums2);

        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums1={:?}, m={}, nums2={:?}", t.nums1, t.m, t.nums2),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 1: Maximum Subarray (Kadane's) ──────────────────────────────

struct MaxSubarray;
struct MaxSubarrayTest {
    nums: Vec<i32>,
}

impl Problem for MaxSubarray {
    fn id(&self) -> &str {
        "arrays_max_subarray"
    }
    fn name(&self) -> &str {
        "Maximum Subarray"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an integer array `nums`, find the subarray with the largest sum. \
         Return its sum.\n\nConstraints: 1 <= nums.len() <= 10^5"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=50);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(MaxSubarrayTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxSubarrayTest>().unwrap();
        let expected = ref_max_subarray(&t.nums);
        let actual = solutions::max_subarray(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_subarray(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut cur = nums[0];
    for &n in &nums[1..] {
        cur = n.max(cur + n);
        max_sum = max_sum.max(cur);
    }
    max_sum
}

// ── Medium 2: Rotate Array ─────────────────────────────────────────────

struct RotateArray;
struct RotateTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for RotateArray {
    fn id(&self) -> &str {
        "arrays_rotate"
    }
    fn name(&self) -> &str {
        "Rotate Array"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Rotate the array to the right by `k` steps in-place."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                let k = rng.random_range(0..=n * 2);
                TestCase {
                    data: Box::new(RotateTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RotateTest>().unwrap();
        let n = t.nums.len();
        let k = t.k % n;
        let mut expected = t.nums.clone();
        expected.rotate_right(k);

        let mut actual = t.nums.clone();
        solutions::rotate(&mut actual, t.k);

        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 3: Product of Array Except Self ─────────────────────────────

struct ProductExceptSelf;
struct ProductTest {
    nums: Vec<i32>,
}

impl Problem for ProductExceptSelf {
    fn id(&self) -> &str {
        "arrays_product_except_self"
    }
    fn name(&self) -> &str {
        "Product of Array Except Self"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an integer array `nums`, return an array where `answer[i]` is the product \
         of all elements except `nums[i]`. Do not use division."
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

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ProductTest>().unwrap();
        let n = t.nums.len();
        let mut expected = vec![1i32; n];
        for (i, expected_val) in expected.iter_mut().enumerate().take(n) {
            for (j, &v) in t.nums.iter().enumerate() {
                if i != j {
                    *expected_val *= v;
                }
            }
        }
        let actual = solutions::product_except_self(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 4: Next Permutation ─────────────────────────────────────────

struct NextPermutation;
struct NextPermTest {
    nums: Vec<i32>,
}

impl Problem for NextPermutation {
    fn id(&self) -> &str {
        "arrays_next_permutation"
    }
    fn name(&self) -> &str {
        "Next Permutation"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Rearrange `nums` into the lexicographically next greater permutation. \
         If not possible, rearrange to the lowest possible order (sorted ascending)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=8);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=5)).collect();
                TestCase {
                    data: Box::new(NextPermTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NextPermTest>().unwrap();
        let mut expected = t.nums.clone();
        ref_next_permutation(&mut expected);
        let mut actual = t.nums.clone();
        solutions::next_permutation(&mut actual);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_next_permutation(nums: &mut [i32]) {
    let n = nums.len();
    if n <= 1 {
        return;
    }
    let mut i = n - 2;
    while i < n && nums[i] >= nums[i + 1] {
        if i == 0 {
            nums.sort();
            return;
        }
        i -= 1;
    }
    let mut j = n - 1;
    while nums[j] <= nums[i] {
        j -= 1;
    }
    nums.swap(i, j);
    nums[i + 1..].reverse();
}

// ── Medium 5: Spiral Matrix ───────────────────────────────────────────

struct SpiralMatrix;
struct SpiralTest {
    matrix: Vec<Vec<i32>>,
}

impl Problem for SpiralMatrix {
    fn id(&self) -> &str {
        "arrays_spiral_matrix"
    }
    fn name(&self) -> &str {
        "Spiral Matrix"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an m x n matrix, return all elements in spiral order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(1..=6);
                let cols = rng.random_range(1..=6);
                let matrix: Vec<Vec<i32>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(-100..=100)).collect())
                    .collect();
                TestCase {
                    data: Box::new(SpiralTest { matrix }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SpiralTest>().unwrap();
        let expected = ref_spiral(&t.matrix);
        let actual = solutions::spiral_order(&t.matrix);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("matrix={:?}", t.matrix),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_spiral(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut result = Vec::new();
    if matrix.is_empty() {
        return result;
    }
    let (mut top, mut bottom) = (0i32, matrix.len() as i32 - 1);
    let (mut left, mut right) = (0i32, matrix[0].len() as i32 - 1);
    while top <= bottom && left <= right {
        for c in left..=right {
            result.push(matrix[top as usize][c as usize]);
        }
        top += 1;
        for r in top..=bottom {
            result.push(matrix[r as usize][right as usize]);
        }
        right -= 1;
        if top <= bottom {
            for c in (left..=right).rev() {
                result.push(matrix[bottom as usize][c as usize]);
            }
            bottom -= 1;
        }
        if left <= right {
            for r in (top..=bottom).rev() {
                result.push(matrix[r as usize][left as usize]);
            }
            left += 1;
        }
    }
    result
}

// ── Hard 1: Trapping Rain Water ────────────────────────────────────────

struct TrappingRainWater;
struct TrapTest {
    height: Vec<i32>,
}

impl Problem for TrappingRainWater {
    fn id(&self) -> &str {
        "arrays_trapping_rain_water"
    }
    fn name(&self) -> &str {
        "Trapping Rain Water"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given `n` non-negative integers representing an elevation map, compute how much \
         water it can trap after raining."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=30);
                let height: Vec<i32> = (0..n).map(|_| rng.random_range(0..=20)).collect();
                TestCase {
                    data: Box::new(TrapTest { height }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TrapTest>().unwrap();
        let expected = ref_trap(&t.height);
        let actual = solutions::trap(&t.height);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("height={:?}", t.height),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_trap(height: &[i32]) -> i32 {
    let n = height.len();
    if n < 3 {
        return 0;
    }
    let (mut l, mut r) = (0, n - 1);
    let (mut l_max, mut r_max) = (0, 0);
    let mut water = 0;
    while l < r {
        if height[l] < height[r] {
            l_max = l_max.max(height[l]);
            water += l_max - height[l];
            l += 1;
        } else {
            r_max = r_max.max(height[r]);
            water += r_max - height[r];
            r -= 1;
        }
    }
    water
}

// ── Hard 2: First Missing Positive ─────────────────────────────────────

struct FirstMissingPositive;
struct FMPTest {
    nums: Vec<i32>,
}

impl Problem for FirstMissingPositive {
    fn id(&self) -> &str {
        "arrays_first_missing_positive"
    }
    fn name(&self) -> &str {
        "First Missing Positive"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an unsorted integer array `nums`, return the smallest missing positive integer.\n\n\
         Must run in O(n) time and O(1) extra space."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n)
                    .map(|_| rng.random_range(-10..=n as i32 + 5))
                    .collect();
                TestCase {
                    data: Box::new(FMPTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FMPTest>().unwrap();
        let set: std::collections::HashSet<i32> = t.nums.iter().copied().collect();
        let mut expected = 1;
        while set.contains(&expected) {
            expected += 1;
        }
        let actual = solutions::first_missing_positive(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 3: Median of Two Sorted Arrays ────────────────────────────────

struct MedianTwoSortedArrays;
struct MedianTest {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

impl Problem for MedianTwoSortedArrays {
    fn id(&self) -> &str {
        "arrays_median_two_sorted"
    }
    fn name(&self) -> &str {
        "Median of Two Sorted Arrays"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given two sorted arrays, return the median of the two sorted arrays.\n\n\
         Return as f64. Target: O(log(m+n))."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let m = rng.random_range(0..=15);
                let n = rng.random_range(1..=15);
                let mut nums1: Vec<i32> = (0..m).map(|_| rng.random_range(-100..=100)).collect();
                let mut nums2: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                nums1.sort();
                nums2.sort();
                TestCase {
                    data: Box::new(MedianTest { nums1, nums2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MedianTest>().unwrap();
        let mut merged = t.nums1.clone();
        merged.extend_from_slice(&t.nums2);
        merged.sort();
        let n = merged.len();
        let expected = if n % 2 == 1 {
            merged[n / 2] as f64
        } else {
            (merged[n / 2 - 1] as f64 + merged[n / 2] as f64) / 2.0
        };
        let actual = solutions::find_median_sorted_arrays(&t.nums1, &t.nums2);
        SolutionResult {
            is_correct: (expected - actual).abs() < 1e-5,
            input_description: format!("nums1={:?}, nums2={:?}", t.nums1, t.nums2),
            expected: format!("{expected:.5}"),
            actual: format!("{actual:.5}"),
        }
    }
}

// ── Hard 4: Longest Consecutive Sequence ───────────────────────────────

struct LongestConsecutiveSequence;
struct LCSTest {
    nums: Vec<i32>,
}

impl Problem for LongestConsecutiveSequence {
    fn id(&self) -> &str {
        "arrays_longest_consecutive"
    }
    fn name(&self) -> &str {
        "Longest Consecutive Sequence"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an unsorted array of integers, return the length of the longest \
         consecutive elements sequence. Must run in O(n) time."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                TestCase {
                    data: Box::new(LCSTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LCSTest>().unwrap();
        let set: std::collections::HashSet<i32> = t.nums.iter().copied().collect();
        let mut expected = 0i32;
        for &n in &set {
            if !set.contains(&(n - 1)) {
                let mut len = 1;
                let mut cur = n;
                while set.contains(&(cur + 1)) {
                    cur += 1;
                    len += 1;
                }
                expected = expected.max(len);
            }
        }
        let actual = solutions::longest_consecutive(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 5: Minimum Window Sort ────────────────────────────────────────

struct MinimumWindowSort;
struct MWSTest {
    nums: Vec<i32>,
}

impl Problem for MinimumWindowSort {
    fn id(&self) -> &str {
        "arrays_minimum_window_sort"
    }
    fn name(&self) -> &str {
        "Minimum Window Sort"
    }
    fn topic(&self) -> &str {
        "arrays"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array, find the length of the smallest subarray that, if sorted, \
         would make the entire array sorted. Return 0 if already sorted."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                TestCase {
                    data: Box::new(MWSTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MWSTest>().unwrap();
        let mut sorted = t.nums.clone();
        sorted.sort();
        let mut l = 0;
        while l < t.nums.len() && t.nums[l] == sorted[l] {
            l += 1;
        }
        if l == t.nums.len() {
            let actual = solutions::minimum_window_sort(&t.nums);
            return SolutionResult {
                is_correct: actual == 0,
                input_description: format!("nums={:?}", t.nums),
                expected: "0".to_string(),
                actual: format!("{actual}"),
            };
        }
        let mut r = t.nums.len() - 1;
        while t.nums[r] == sorted[r] {
            r -= 1;
        }
        let expected = (r - l + 1) as i32;
        let actual = solutions::minimum_window_sort(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}
