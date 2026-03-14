use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part5_paradigms::divide_conquer as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(DcMaxSubarray),
        Box::new(DcPower),
        Box::new(DcCountInversions),
        Box::new(DcBinarySearch),
        Box::new(DcFindPeak),
        Box::new(DcMergeSort),
        Box::new(DcKthLargest),
        Box::new(DcClosestPair),
        Box::new(DcMajorityElement),
        Box::new(DcDifferentWays),
        Box::new(DcMedianTwoSorted),
        Box::new(DcSkyline),
        Box::new(DcCountRangeSum),
        Box::new(DcReversePairs),
        Box::new(DcKthSmallestSortedMatrix),
    ]
}

// ── Easy 1: Max Subarray (D&C) ──────────────────────────────────────

struct DcMaxSubarray;
struct DcMaxSubarrayTest { nums: Vec<i32> }

impl Problem for DcMaxSubarray {
    fn id(&self) -> &str { "divide_conquer_max_subarray" }
    fn name(&self) -> &str { "Max Subarray (Divide & Conquer)" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Find the contiguous subarray with the largest sum using a divide and conquer \
         approach. Split the array in half, recursively solve both halves, and handle \
         the cross-boundary case.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 10^5\n\
         - -10^4 <= nums[i] <= 10^4"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=40);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            TestCase { data: Box::new(DcMaxSubarrayTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcMaxSubarrayTest>().unwrap();
        let expected = ref_max_subarray_dc(&t.nums);
        let actual = solutions::max_subarray(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_subarray_dc(nums: &[i32]) -> i32 {
    fn helper(nums: &[i32], lo: usize, hi: usize) -> i32 {
        if lo == hi { return nums[lo]; }
        let mid = lo + (hi - lo) / 2;
        let left_max = helper(nums, lo, mid);
        let right_max = helper(nums, mid + 1, hi);
        // cross boundary
        let mut left_sum = i32::MIN;
        let mut sum = 0;
        for i in (lo..=mid).rev() {
            sum += nums[i];
            left_sum = left_sum.max(sum);
        }
        let mut right_sum = i32::MIN;
        sum = 0;
        for i in mid + 1..=hi {
            sum += nums[i];
            right_sum = right_sum.max(sum);
        }
        left_max.max(right_max).max(left_sum + right_sum)
    }
    if nums.is_empty() { return 0; }
    helper(nums, 0, nums.len() - 1)
}

// ── Easy 2: Fast Power ──────────────────────────────────────────────

struct DcPower;
struct DcPowerTest { x: f64, n: i32 }

impl Problem for DcPower {
    fn id(&self) -> &str { "divide_conquer_power" }
    fn name(&self) -> &str { "Fast Power (x^n)" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Compute x^n using fast exponentiation (divide and conquer). Handle negative \
         exponents.\n\n\
         Constraints:\n\
         - -100.0 < x < 100.0\n\
         - -30 <= n <= 30\n\
         - Result fits in f64."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let x = rng.random_range(-10.0_f64..=10.0_f64);
            let x = (x * 100.0).round() / 100.0; // 2 decimal places
            let n = rng.random_range(-15..=15);
            TestCase { data: Box::new(DcPowerTest { x, n }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcPowerTest>().unwrap();
        let expected = ref_power(t.x, t.n);
        let actual = solutions::power(t.x, t.n);
        let is_correct = if expected.is_infinite() && actual.is_infinite() {
            expected.signum() == actual.signum()
        } else if expected == 0.0 && actual == 0.0 {
            true
        } else {
            (expected - actual).abs() / expected.abs().max(1.0) < 1e-6
        };
        SolutionResult {
            is_correct,
            input_description: format!("x={}, n={}", t.x, t.n),
            expected: format!("{expected:.10}"),
            actual: format!("{actual:.10}"),
        }
    }
}

fn ref_power(x: f64, n: i32) -> f64 {
    if n == 0 { return 1.0; }
    if n < 0 { return 1.0 / ref_power(x, -n); }
    let half = ref_power(x, n / 2);
    if n % 2 == 0 { half * half } else { half * half * x }
}

// ── Easy 3: Count Inversions ─────────────────────────────────────────

struct DcCountInversions;
struct DcCountInversionsTest { nums: Vec<i32> }

impl Problem for DcCountInversions {
    fn id(&self) -> &str { "divide_conquer_count_inversions" }
    fn name(&self) -> &str { "Count Inversions" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Count the number of inversions in an array. An inversion is a pair (i, j) \
         where i < j and nums[i] > nums[j]. Use a merge-sort-based approach.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 10^5"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
            TestCase { data: Box::new(DcCountInversionsTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcCountInversionsTest>().unwrap();
        let expected = ref_count_inversions(&t.nums);
        let actual = solutions::count_inversions(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_count_inversions(nums: &[i32]) -> i64 {
    fn merge_count(arr: &mut [i32]) -> i64 {
        let n = arr.len();
        if n <= 1 { return 0; }
        let mid = n / 2;
        let mut count = 0i64;
        count += merge_count(&mut arr[..mid]);
        count += merge_count(&mut arr[mid..]);
        let left = arr[..mid].to_vec();
        let right = arr[mid..].to_vec();
        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                arr[k] = left[i];
                i += 1;
            } else {
                arr[k] = right[j];
                count += (left.len() - i) as i64;
                j += 1;
            }
            k += 1;
        }
        while i < left.len() { arr[k] = left[i]; i += 1; k += 1; }
        while j < right.len() { arr[k] = right[j]; j += 1; k += 1; }
        count
    }
    let mut arr = nums.to_vec();
    merge_count(&mut arr)
}

// ── Easy 4: Binary Search (D&C) ─────────────────────────────────────

struct DcBinarySearch;
struct DcBinarySearchTest { nums: Vec<i32>, target: i32 }

impl Problem for DcBinarySearch {
    fn id(&self) -> &str { "divide_conquer_binary_search" }
    fn name(&self) -> &str { "Binary Search (Divide & Conquer)" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a sorted array and a target value, return the index if found, or -1 \
         if not present. Implement using a recursive divide and conquer approach.\n\n\
         Constraints:\n\
         - Array is sorted in ascending order.\n\
         - -10^4 <= nums[i], target <= 10^4\n\
         - If duplicates exist, return any valid index."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
            nums.sort();
            let target = if rng.random_range(0..=2) == 0 {
                rng.random_range(-60..=60)
            } else {
                nums[rng.random_range(0..n)]
            };
            TestCase { data: Box::new(DcBinarySearchTest { nums, target }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcBinarySearchTest>().unwrap();
        let actual = solutions::binary_search(&t.nums, t.target);
        let is_correct = if actual == -1 {
            !t.nums.contains(&t.target)
        } else {
            let idx = actual as usize;
            idx < t.nums.len() && t.nums[idx] == t.target
        };
        let expected_str = if t.nums.contains(&t.target) {
            format!("valid index of {}", t.target)
        } else {
            "-1".to_string()
        };
        SolutionResult {
            is_correct,
            input_description: format!("nums={:?}, target={}", t.nums, t.target),
            expected: expected_str,
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 5: Find Peak Element ────────────────────────────────────────

struct DcFindPeak;
struct DcFindPeakTest { nums: Vec<i32> }

impl Problem for DcFindPeak {
    fn id(&self) -> &str { "divide_conquer_find_peak" }
    fn name(&self) -> &str { "Find Peak Element" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Find a peak element in the array. A peak is an element that is strictly \
         greater than its neighbors. Treat out-of-bounds as negative infinity. \
         Return the index of any peak.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 1000\n\
         - nums[i] != nums[i+1] for all valid i."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            // Generate distinct adjacent values
            let mut nums: Vec<i32> = Vec::with_capacity(n);
            for _ in 0..n {
                loop {
                    let v = rng.random_range(-100..=100);
                    if nums.is_empty() || *nums.last().unwrap() != v {
                        nums.push(v);
                        break;
                    }
                }
            }
            TestCase { data: Box::new(DcFindPeakTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcFindPeakTest>().unwrap();
        let actual = solutions::find_peak(&t.nums);
        let n = t.nums.len();
        let is_peak = actual < n
            && (actual == 0 || t.nums[actual] > t.nums[actual - 1])
            && (actual == n - 1 || t.nums[actual] > t.nums[actual + 1]);
        SolutionResult {
            is_correct: is_peak,
            input_description: format!("nums={:?}", t.nums),
            expected: "a valid peak index".to_string(),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 1: Merge Sort ────────────────────────────────────────────

struct DcMergeSort;
struct DcMergeSortTest { nums: Vec<i32> }

impl Problem for DcMergeSort {
    fn id(&self) -> &str { "divide_conquer_merge_sort" }
    fn name(&self) -> &str { "Merge Sort" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Implement merge sort. Return a new sorted vector.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 10^5"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=40);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-500..=500)).collect();
            TestCase { data: Box::new(DcMergeSortTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcMergeSortTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();
        let actual = solutions::merge_sort(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 2: Kth Largest (Quickselect) ──────────────────────────────

struct DcKthLargest;
struct DcKthLargestTest { nums: Vec<i32>, k: usize }

impl Problem for DcKthLargest {
    fn id(&self) -> &str { "divide_conquer_kth_largest" }
    fn name(&self) -> &str { "Kth Largest Element" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Find the kth largest element in an unsorted array using a quickselect \
         (divide and conquer) approach. k=1 means the largest element.\n\n\
         Constraints:\n\
         - 1 <= k <= nums.len() <= 10^4"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let k = rng.random_range(1..=n);
            TestCase { data: Box::new(DcKthLargestTest { nums, k }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcKthLargestTest>().unwrap();
        let mut sorted = t.nums.clone();
        sorted.sort_unstable();
        sorted.reverse();
        let expected = sorted[t.k - 1];
        let actual = solutions::kth_largest(&t.nums, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 3: Closest Pair of Points ─────────────────────────────────

struct DcClosestPair;
struct DcClosestPairTest { points: Vec<(i64, i64)> }

impl Problem for DcClosestPair {
    fn id(&self) -> &str { "divide_conquer_closest_pair" }
    fn name(&self) -> &str { "Closest Pair of Points" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a set of points in 2D space, find the Euclidean distance between the \
         closest pair of points. Use a divide and conquer approach for O(n log n).\n\n\
         Constraints:\n\
         - 2 <= points.len() <= 10^5\n\
         - Coordinates are integers."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=20);
            let points: Vec<(i64, i64)> = (0..n)
                .map(|_| (rng.random_range(-1000..=1000i64), rng.random_range(-1000..=1000i64)))
                .collect();
            TestCase { data: Box::new(DcClosestPairTest { points }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcClosestPairTest>().unwrap();
        let expected = ref_closest_pair(&t.points);
        let actual = solutions::closest_pair(&t.points);
        let is_correct = (expected - actual).abs() < 1e-6;
        SolutionResult {
            is_correct,
            input_description: format!("points={:?}", t.points),
            expected: format!("{expected:.6}"),
            actual: format!("{actual:.6}"),
        }
    }
}

fn ref_closest_pair(points: &[(i64, i64)]) -> f64 {
    // brute force for small test inputs
    let n = points.len();
    let mut min_dist = f64::MAX;
    for i in 0..n {
        for j in i + 1..n {
            let dx = (points[i].0 - points[j].0) as f64;
            let dy = (points[i].1 - points[j].1) as f64;
            let d = (dx * dx + dy * dy).sqrt();
            min_dist = min_dist.min(d);
        }
    }
    min_dist
}

// ── Medium 4: Majority Element ──────────────────────────────────────

struct DcMajorityElement;
struct DcMajorityTest { nums: Vec<i32> }

impl Problem for DcMajorityElement {
    fn id(&self) -> &str { "divide_conquer_majority_element" }
    fn name(&self) -> &str { "Majority Element (D&C)" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Find the majority element (appears more than n/2 times). A majority element \
         is guaranteed to exist. Solve using divide and conquer.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 5 * 10^4\n\
         - The majority element always exists."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=21) | 1; // odd for simplicity
            let majority = rng.random_range(-50..=50);
            let majority_count = n / 2 + 1;
            let mut nums: Vec<i32> = vec![majority; majority_count];
            for _ in majority_count..n {
                nums.push(rng.random_range(-50..=50));
            }
            // Shuffle
            for i in (1..nums.len()).rev() {
                let j = rng.random_range(0..=i);
                nums.swap(i, j);
            }
            TestCase { data: Box::new(DcMajorityTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcMajorityTest>().unwrap();
        let expected = ref_majority(&t.nums);
        let actual = solutions::majority_element(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_majority(nums: &[i32]) -> i32 {
    // Boyer-Moore voting
    let mut candidate = nums[0];
    let mut count = 1;
    for &n in &nums[1..] {
        if count == 0 { candidate = n; count = 1; }
        else if n == candidate { count += 1; }
        else { count -= 1; }
    }
    candidate
}

// ── Medium 5: Different Ways to Add Parentheses ──────────────────────

struct DcDifferentWays;
struct DcDifferentWaysTest { expression: String }

impl Problem for DcDifferentWays {
    fn id(&self) -> &str { "divide_conquer_different_ways" }
    fn name(&self) -> &str { "Different Ways to Add Parentheses" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a string expression of numbers and operators (+, -, *), return all \
         possible results from computing all different ways to group numbers and \
         operators. Return results sorted in ascending order.\n\n\
         Constraints:\n\
         - 1 <= expression.len() <= 20\n\
         - expression consists of digits and the operators +, -, *."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let ops = ['+', '-', '*'];
        (0..10).map(|_| {
            let num_terms = rng.random_range(2..=4);
            let mut expr = String::new();
            for i in 0..num_terms {
                if i > 0 {
                    expr.push(ops[rng.random_range(0..3)]);
                }
                expr.push_str(&rng.random_range(1..=9).to_string());
            }
            TestCase { data: Box::new(DcDifferentWaysTest { expression: expr }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcDifferentWaysTest>().unwrap();
        let mut expected = ref_different_ways(&t.expression);
        expected.sort();
        let mut actual = solutions::different_ways(&t.expression);
        actual.sort();
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("expression=\"{}\"", t.expression),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_different_ways(expr: &str) -> Vec<i32> {
    let chars: Vec<char> = expr.chars().collect();
    let mut results = Vec::new();
    for (i, &ch) in chars.iter().enumerate() {
        if ch == '+' || ch == '-' || ch == '*' {
            let left = ref_different_ways(&expr[..i]);
            let right = ref_different_ways(&expr[i + 1..]);
            for &l in &left {
                for &r in &right {
                    results.push(match ch {
                        '+' => l + r,
                        '-' => l - r,
                        '*' => l * r,
                        _ => unreachable!(),
                    });
                }
            }
        }
    }
    if results.is_empty() {
        results.push(expr.parse::<i32>().unwrap());
    }
    results
}

// ── Hard 1: Median of Two Sorted Arrays ──────────────────────────────

struct DcMedianTwoSorted;
struct DcMedianTest { nums1: Vec<i32>, nums2: Vec<i32> }

impl Problem for DcMedianTwoSorted {
    fn id(&self) -> &str { "divide_conquer_median_two_sorted" }
    fn name(&self) -> &str { "Median of Two Sorted Arrays" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given two sorted arrays, return the median of the combined sorted array. \
         Use a divide and conquer / binary search approach for O(log(min(m,n))).\n\n\
         Constraints:\n\
         - 0 <= nums1.len(), nums2.len() <= 1000\n\
         - nums1.len() + nums2.len() >= 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let m = rng.random_range(0..=15);
            let n = rng.random_range(1..=15);
            let mut nums1: Vec<i32> = (0..m).map(|_| rng.random_range(-100..=100)).collect();
            let mut nums2: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            nums1.sort();
            nums2.sort();
            TestCase { data: Box::new(DcMedianTest { nums1, nums2 }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcMedianTest>().unwrap();
        let mut merged = t.nums1.clone();
        merged.extend_from_slice(&t.nums2);
        merged.sort();
        let n = merged.len();
        let expected = if n % 2 == 1 {
            merged[n / 2] as f64
        } else {
            (merged[n / 2 - 1] as f64 + merged[n / 2] as f64) / 2.0
        };
        let actual = solutions::median_two_sorted(&t.nums1, &t.nums2);
        SolutionResult {
            is_correct: (expected - actual).abs() < 1e-5,
            input_description: format!("nums1={:?}, nums2={:?}", t.nums1, t.nums2),
            expected: format!("{expected:.5}"),
            actual: format!("{actual:.5}"),
        }
    }
}

// ── Hard 2: Skyline Problem ──────────────────────────────────────────

struct DcSkyline;
struct DcSkylineTest { buildings: Vec<(i32, i32, i32)> }

impl Problem for DcSkyline {
    fn id(&self) -> &str { "divide_conquer_skyline" }
    fn name(&self) -> &str { "The Skyline Problem" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given buildings as (left, right, height) tuples, return the skyline as \
         key points (x, height). Use a divide and conquer approach.\n\n\
         A key point is the left endpoint of a horizontal line segment in the skyline. \
         The last key point has height 0.\n\n\
         Constraints:\n\
         - 1 <= buildings.len() <= 10^4\n\
         - Buildings sorted by left coordinate."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=8);
            let mut buildings: Vec<(i32, i32, i32)> = (0..n).map(|_| {
                let l = rng.random_range(0..=20);
                let r = rng.random_range(l + 1..=l + 10);
                let h = rng.random_range(1..=20);
                (l, r, h)
            }).collect();
            buildings.sort_by_key(|b| b.0);
            TestCase { data: Box::new(DcSkylineTest { buildings }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcSkylineTest>().unwrap();
        let expected = ref_skyline(&t.buildings);
        let actual = solutions::skyline(&t.buildings);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("buildings={:?}", t.buildings),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_skyline(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
    fn divide(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
        if buildings.is_empty() { return vec![]; }
        if buildings.len() == 1 {
            let (l, r, h) = buildings[0];
            return vec![(l, h), (r, 0)];
        }
        let mid = buildings.len() / 2;
        let left = divide(&buildings[..mid]);
        let right = divide(&buildings[mid..]);
        merge_skylines(&left, &right)
    }

    fn merge_skylines(left: &[(i32, i32)], right: &[(i32, i32)]) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = Vec::new();
        let (mut i, mut j) = (0, 0);
        let (mut lh, mut rh) = (0, 0);
        while i < left.len() && j < right.len() {
            let (x, h);
            if left[i].0 < right[j].0 {
                x = left[i].0;
                lh = left[i].1;
                i += 1;
            } else if right[j].0 < left[i].0 {
                x = right[j].0;
                rh = right[j].1;
                j += 1;
            } else {
                x = left[i].0;
                lh = left[i].1;
                rh = right[j].1;
                i += 1;
                j += 1;
            }
            h = lh.max(rh);
            if result.is_empty() || result.last().unwrap().1 != h {
                result.push((x, h));
            }
        }
        while i < left.len() {
            if result.is_empty() || result.last().unwrap().1 != left[i].1 {
                result.push(left[i]);
            }
            i += 1;
        }
        while j < right.len() {
            if result.is_empty() || result.last().unwrap().1 != right[j].1 {
                result.push(right[j]);
            }
            j += 1;
        }
        result
    }

    divide(buildings)
}

// ── Hard 3: Count Range Sum ──────────────────────────────────────────

struct DcCountRangeSum;
struct DcCountRangeSumTest { nums: Vec<i32>, lower: i32, upper: i32 }

impl Problem for DcCountRangeSum {
    fn id(&self) -> &str { "divide_conquer_count_range_sum" }
    fn name(&self) -> &str { "Count of Range Sum" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an integer array and two integers lower and upper, return the number \
         of range sums that lie in [lower, upper] inclusive. A range sum S(i,j) is \
         the sum of elements from index i to j (0-indexed, i <= j).\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 10^5\n\
         - -2^31 <= nums[i] <= 2^31 - 1\n\
         - lower <= upper"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
            let a = rng.random_range(-100..=100);
            let b = rng.random_range(-100..=100);
            let lower = a.min(b);
            let upper = a.max(b);
            TestCase { data: Box::new(DcCountRangeSumTest { nums, lower, upper }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcCountRangeSumTest>().unwrap();
        let expected = ref_count_range_sum(&t.nums, t.lower, t.upper);
        let actual = solutions::count_range_sum(&t.nums, t.lower, t.upper);
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
    for i in 0..n {
        let mut sum: i64 = 0;
        for j in i..n {
            sum += nums[j] as i64;
            if sum >= lower as i64 && sum <= upper as i64 {
                count += 1;
            }
        }
    }
    count
}

// ── Hard 4: Reverse Pairs ───────────────────────────────────────────

struct DcReversePairs;
struct DcReversePairsTest { nums: Vec<i32> }

impl Problem for DcReversePairs {
    fn id(&self) -> &str { "divide_conquer_reverse_pairs" }
    fn name(&self) -> &str { "Reverse Pairs" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an array, return the number of important reverse pairs. \
         An important reverse pair is (i, j) where i < j and nums[i] > 2 * nums[j].\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 5 * 10^4"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=25);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
            TestCase { data: Box::new(DcReversePairsTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcReversePairsTest>().unwrap();
        let expected = ref_reverse_pairs(&t.nums);
        let actual = solutions::reverse_pairs(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_reverse_pairs(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut count = 0i32;
    for i in 0..n {
        for j in i + 1..n {
            if nums[i] as i64 > 2 * nums[j] as i64 {
                count += 1;
            }
        }
    }
    count
}

// ── Hard 5: Kth Smallest in Sorted Matrix ────────────────────────────

struct DcKthSmallestSortedMatrix;
struct DcKthSmallestMatrixTest { matrix: Vec<Vec<i32>>, k: usize }

impl Problem for DcKthSmallestSortedMatrix {
    fn id(&self) -> &str { "divide_conquer_kth_smallest_sorted_matrix" }
    fn name(&self) -> &str { "Kth Smallest Element in Sorted Matrix" }
    fn topic(&self) -> &str { "divide_conquer" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an n x n matrix where each row and column is sorted in ascending order, \
         find the kth smallest element. Use a binary search / divide and conquer approach.\n\n\
         Constraints:\n\
         - 1 <= n <= 300\n\
         - 1 <= k <= n^2"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=8);
            // build a sorted matrix: each row sorted, each column sorted
            let mut matrix = vec![vec![0i32; n]; n];
            matrix[0][0] = rng.random_range(-50..=0);
            for j in 1..n {
                matrix[0][j] = matrix[0][j - 1] + rng.random_range(0..=10);
            }
            for i in 1..n {
                matrix[i][0] = matrix[i - 1][0] + rng.random_range(0..=10);
                for j in 1..n {
                    matrix[i][j] = matrix[i - 1][j].max(matrix[i][j - 1]) + rng.random_range(0..=5);
                }
            }
            let k = rng.random_range(1..=n * n);
            TestCase { data: Box::new(DcKthSmallestMatrixTest { matrix, k }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DcKthSmallestMatrixTest>().unwrap();
        let mut all: Vec<i32> = t.matrix.iter().flat_map(|r| r.iter().copied()).collect();
        all.sort();
        let expected = all[t.k - 1];
        let actual = solutions::kth_smallest_sorted_matrix(&t.matrix, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("matrix={:?}, k={}", t.matrix, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}
