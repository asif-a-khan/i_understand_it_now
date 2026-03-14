use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part2_sorting::binary_search as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(BinarySearchBasic),
        Box::new(BinarySearchInsertPosition),
        Box::new(BinarySearchFirstLast),
        Box::new(BinarySearchSqrt),
        Box::new(BinarySearchGuessNumber),
        Box::new(BinarySearchRotatedArray),
        Box::new(BinarySearchPeakElement),
        Box::new(BinarySearchFindMinRotated),
        Box::new(BinarySearch2dMatrix),
        Box::new(BinarySearchKokoBananas),
        Box::new(BinarySearchMedianSortedArrays),
        Box::new(BinarySearchSplitArrayLargest),
        Box::new(BinarySearchFindKthSmallestPair),
        Box::new(BinarySearchCountSmallerAfter),
        Box::new(BinarySearchRussianDollEnvelopes),
    ]
}

// ── Easy 1: Binary Search (basic) ──────────────────────────────────────

struct BinarySearchBasic;

struct BinarySearchBasicTest {
    nums: Vec<i32>,
    target: i32,
}

impl Problem for BinarySearchBasic {
    fn id(&self) -> &str { "binary_search_basic" }
    fn name(&self) -> &str { "Binary Search" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a sorted array of integers `nums` and an integer `target`, return the index \
         of `target` if it exists, or `None` if it does not.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 10000\n\
         - nums is sorted in ascending order\n\
         - All values in nums are unique\n\
         - Target: O(log n) time complexity"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=50);
            let mut nums = crate::problems::helpers::random_unique_vec(&mut rng, n, -500, 500);
            nums.sort();
            // 50% chance the target is in the array
            let target = if rng.random_range(0..2) == 0 {
                nums[rng.random_range(0..n)]
            } else {
                rng.random_range(-600..=600)
            };
            TestCase { data: Box::new(BinarySearchBasicTest { nums, target }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BinarySearchBasicTest>().unwrap();
        let expected = ref_binary_search(&t.nums, t.target);
        let actual = solutions::binary_search_basic(&t.nums, t.target);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, target={}", t.nums, t.target),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let (mut lo, mut hi) = (0i64, nums.len() as i64 - 1);
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let mid_u = mid as usize;
        if nums[mid_u] == target {
            return Some(mid_u);
        } else if nums[mid_u] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    None
}

// ── Easy 2: Search Insert Position ─────────────────────────────────────

struct BinarySearchInsertPosition;

struct InsertPositionTest {
    nums: Vec<i32>,
    target: i32,
}

impl Problem for BinarySearchInsertPosition {
    fn id(&self) -> &str { "binary_search_insert_position" }
    fn name(&self) -> &str { "Search Insert Position" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a sorted array of distinct integers and a `target`, return the index where \
         the target is found. If not found, return the index where it would be inserted \
         to keep the array sorted.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 10000\n\
         - nums contains distinct values in ascending order\n\
         - Target: O(log n) time complexity"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=50);
            let mut nums = crate::problems::helpers::random_unique_vec(&mut rng, n, -500, 500);
            nums.sort();
            let target = if rng.random_range(0..2) == 0 {
                nums[rng.random_range(0..n)]
            } else {
                rng.random_range(-600..=600)
            };
            TestCase { data: Box::new(InsertPositionTest { nums, target }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<InsertPositionTest>().unwrap();
        let expected = ref_search_insert(&t.nums, t.target);
        let actual = solutions::search_insert_position(&t.nums, t.target);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, target={}", t.nums, t.target),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_search_insert(nums: &[i32], target: i32) -> usize {
    let (mut lo, mut hi) = (0usize, nums.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

// ── Easy 3: Find First and Last Position ───────────────────────────────

struct BinarySearchFirstLast;

struct FirstLastTest {
    nums: Vec<i32>,
    target: i32,
}

impl Problem for BinarySearchFirstLast {
    fn id(&self) -> &str { "binary_search_first_last" }
    fn name(&self) -> &str { "Find First and Last Position of Element" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a sorted array of integers (may contain duplicates) and a `target`, return \
         the starting and ending position of `target`. Return (-1, -1) if the target is \
         not found.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 10000\n\
         - nums is sorted in non-decreasing order\n\
         - Target: O(log n) time complexity"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=40);
            let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
            nums.sort();
            let target = if n > 0 && rng.random_range(0..2) == 0 {
                nums[rng.random_range(0..n)]
            } else {
                rng.random_range(-25..=25)
            };
            TestCase { data: Box::new(FirstLastTest { nums, target }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FirstLastTest>().unwrap();
        let expected = ref_first_last(&t.nums, t.target);
        let actual = solutions::search_first_last(&t.nums, t.target);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, target={}", t.nums, t.target),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_first_last(nums: &[i32], target: i32) -> (i32, i32) {
    let first = ref_find_bound(nums, target, true);
    if first == -1 {
        return (-1, -1);
    }
    let last = ref_find_bound(nums, target, false);
    (first, last)
}

fn ref_find_bound(nums: &[i32], target: i32, find_first: bool) -> i32 {
    let (mut lo, mut hi) = (0i64, nums.len() as i64 - 1);
    let mut result = -1i32;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let mid_u = mid as usize;
        if nums[mid_u] == target {
            result = mid as i32;
            if find_first {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        } else if nums[mid_u] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    result
}

// ── Easy 4: Integer Square Root ────────────────────────────────────────

struct BinarySearchSqrt;

struct SqrtTest {
    x: u64,
}

impl Problem for BinarySearchSqrt {
    fn id(&self) -> &str { "binary_search_sqrt" }
    fn name(&self) -> &str { "Integer Square Root" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a non-negative integer `x`, return the integer square root of `x`. \
         That is, return the largest integer `r` such that `r * r <= x`.\n\n\
         Do not use built-in sqrt functions. Use binary search.\n\n\
         Constraints:\n\
         - 0 <= x <= 10^18\n\
         - Target: O(log x) time complexity"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = vec![
            TestCase { data: Box::new(SqrtTest { x: 0 }) },
            TestCase { data: Box::new(SqrtTest { x: 1 }) },
        ];
        for _ in 0..8 {
            let x: u64 = rng.random_range(0..=1_000_000_000_000u64);
            tests.push(TestCase { data: Box::new(SqrtTest { x }) });
        }
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SqrtTest>().unwrap();
        let expected = ref_isqrt(t.x);
        let actual = solutions::integer_sqrt(t.x);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("x={}", t.x),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_isqrt(x: u64) -> u64 {
    if x < 2 {
        return x;
    }
    let mut lo: u64 = 1;
    let mut hi: u64 = x.min(3_000_000_000); // sqrt(10^18) < 10^9
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if mid <= x / mid {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    lo - 1
}

// ── Easy 5: Guess Number Higher or Lower ───────────────────────────────

struct BinarySearchGuessNumber;

struct GuessNumberTest {
    n: i32,
    pick: i32,
}

impl Problem for BinarySearchGuessNumber {
    fn id(&self) -> &str { "binary_search_guess_number" }
    fn name(&self) -> &str { "Guess Number Higher or Lower" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "I pick a number from 1 to n. You guess which number I picked.\n\n\
         You are given a function `guess(num, pick) -> i32` that returns:\n\
         - -1 if pick < num (your guess is too high)\n\
         - 1 if pick > num (your guess is too low)\n\
         - 0 if pick == num\n\n\
         Return the number I picked.\n\n\
         Your function receives `(n, pick)` where pick is the secret number. \
         Use binary search to find it.\n\n\
         Constraints:\n\
         - 1 <= pick <= n <= 2^31 - 1\n\
         - Target: O(log n) time complexity"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=1_000_000);
            let pick = rng.random_range(1..=n);
            TestCase { data: Box::new(GuessNumberTest { n, pick }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<GuessNumberTest>().unwrap();
        let expected = t.pick;
        let actual = solutions::guess_number(t.n, t.pick);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, pick={}", t.n, t.pick),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 1: Search in Rotated Sorted Array ───────────────────────────

struct BinarySearchRotatedArray;

struct RotatedArrayTest {
    nums: Vec<i32>,
    target: i32,
}

impl Problem for BinarySearchRotatedArray {
    fn id(&self) -> &str { "binary_search_rotated_array" }
    fn name(&self) -> &str { "Search in Rotated Sorted Array" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "An ascending sorted array of distinct integers was rotated at some pivot. \
         For example, [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2].\n\n\
         Given the rotated array `nums` and a `target`, return its index, or -1 if \
         not found.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 5000\n\
         - All values are unique\n\
         - Target: O(log n) time complexity"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let mut sorted = crate::problems::helpers::random_unique_vec(&mut rng, n, -500, 500);
            sorted.sort();
            let rot = rng.random_range(0..n);
            let mut nums = sorted[rot..].to_vec();
            nums.extend_from_slice(&sorted[..rot]);
            let target = if rng.random_range(0..2) == 0 {
                nums[rng.random_range(0..n)]
            } else {
                rng.random_range(-600..=600)
            };
            TestCase { data: Box::new(RotatedArrayTest { nums, target }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RotatedArrayTest>().unwrap();
        let expected = ref_search_rotated(&t.nums, t.target);
        let actual = solutions::search_rotated_array(&t.nums, t.target);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, target={}", t.nums, t.target),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_search_rotated(nums: &[i32], target: i32) -> i32 {
    let (mut lo, mut hi) = (0i64, nums.len() as i64 - 1);
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let m = mid as usize;
        if nums[m] == target {
            return mid as i32;
        }
        // Left half is sorted
        if nums[lo as usize] <= nums[m] {
            if nums[lo as usize] <= target && target < nums[m] {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        } else {
            // Right half is sorted
            if nums[m] < target && target <= nums[hi as usize] {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
    }
    -1
}

// ── Medium 2: Find Peak Element ────────────────────────────────────────

struct BinarySearchPeakElement;

struct PeakElementTest {
    nums: Vec<i32>,
}

impl Problem for BinarySearchPeakElement {
    fn id(&self) -> &str { "binary_search_peak_element" }
    fn name(&self) -> &str { "Find Peak Element" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "A peak element is one that is strictly greater than its neighbors. \
         Given an array `nums`, find any peak element and return its index.\n\n\
         You may imagine `nums[-1] = nums[n] = -infinity`.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 1000\n\
         - nums[i] != nums[i+1] for all valid i (no adjacent duplicates)\n\
         - Target: O(log n) time complexity"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            // Build array with no adjacent duplicates
            let mut nums = Vec::with_capacity(n);
            nums.push(rng.random_range(-1000..=1000));
            for _ in 1..n {
                let mut v = rng.random_range(-1000..=1000);
                while v == *nums.last().unwrap() {
                    v = rng.random_range(-1000..=1000);
                }
                nums.push(v);
            }
            TestCase { data: Box::new(PeakElementTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PeakElementTest>().unwrap();
        let actual = solutions::find_peak_element(&t.nums);
        // Validate that the returned index is indeed a peak
        let is_peak = ref_is_peak(&t.nums, actual);
        SolutionResult {
            is_correct: is_peak,
            input_description: format!("nums={:?}", t.nums),
            expected: "any valid peak index".to_string(),
            actual: format!("{actual} (value={})", if actual < t.nums.len() { t.nums[actual].to_string() } else { "OUT_OF_BOUNDS".to_string() }),
        }
    }
}

fn ref_is_peak(nums: &[i32], idx: usize) -> bool {
    if idx >= nums.len() {
        return false;
    }
    let left_ok = idx == 0 || nums[idx] > nums[idx - 1];
    let right_ok = idx == nums.len() - 1 || nums[idx] > nums[idx + 1];
    left_ok && right_ok
}

// ── Medium 3: Find Minimum in Rotated Sorted Array ─────────────────────

struct BinarySearchFindMinRotated;

struct FindMinRotatedTest {
    nums: Vec<i32>,
}

impl Problem for BinarySearchFindMinRotated {
    fn id(&self) -> &str { "binary_search_find_min_rotated" }
    fn name(&self) -> &str { "Find Minimum in Rotated Sorted Array" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "A sorted array of unique elements was rotated between 1 and n times. \
         Find the minimum element.\n\n\
         For example, [3,4,5,1,2] was rotated 3 times, minimum is 1.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 5000\n\
         - All values are unique\n\
         - Target: O(log n) time complexity"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let mut sorted = crate::problems::helpers::random_unique_vec(&mut rng, n, -500, 500);
            sorted.sort();
            let rot = rng.random_range(0..n);
            let mut nums = sorted[rot..].to_vec();
            nums.extend_from_slice(&sorted[..rot]);
            TestCase { data: Box::new(FindMinRotatedTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FindMinRotatedTest>().unwrap();
        let expected = *t.nums.iter().min().unwrap();
        let actual = solutions::find_min_rotated(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 4: Search a 2D Matrix ───────────────────────────────────────

struct BinarySearch2dMatrix;

struct Matrix2dTest {
    matrix: Vec<Vec<i32>>,
    target: i32,
}

impl Problem for BinarySearch2dMatrix {
    fn id(&self) -> &str { "binary_search_2d_matrix" }
    fn name(&self) -> &str { "Search a 2D Matrix" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "You are given an m x n integer matrix with the following properties:\n\
         - Each row is sorted in non-decreasing order.\n\
         - The first integer of each row is greater than the last integer of the \
         previous row.\n\n\
         Given a `target`, return `true` if `target` is in the matrix.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 100\n\
         - Target: O(log(m * n)) time complexity"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(1..=8);
            let cols = rng.random_range(1..=8);
            let total = rows * cols;
            let mut flat = crate::problems::helpers::random_unique_vec(
                &mut rng, total, -500, 500,
            );
            flat.sort();
            let matrix: Vec<Vec<i32>> = flat.chunks(cols).map(|c| c.to_vec()).collect();
            let target = if rng.random_range(0..2) == 0 {
                flat[rng.random_range(0..total)]
            } else {
                rng.random_range(-600..=600)
            };
            TestCase { data: Box::new(Matrix2dTest { matrix, target }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<Matrix2dTest>().unwrap();
        let expected = ref_search_matrix(&t.matrix, t.target);
        let actual = solutions::search_2d_matrix(&t.matrix, t.target);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("matrix={:?}, target={}", t.matrix, t.target),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let (mut lo, mut hi) = (0usize, rows * cols);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let val = matrix[mid / cols][mid % cols];
        if val == target {
            return true;
        } else if val < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    false
}

// ── Medium 5: Koko Eating Bananas ──────────────────────────────────────

struct BinarySearchKokoBananas;

struct KokoTest {
    piles: Vec<i32>,
    h: i32,
}

impl Problem for BinarySearchKokoBananas {
    fn id(&self) -> &str { "binary_search_koko_bananas" }
    fn name(&self) -> &str { "Koko Eating Bananas" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Koko loves eating bananas. There are `n` piles of bananas, the i-th pile has \
         `piles[i]` bananas. The guards will come back in `h` hours.\n\n\
         Koko decides on a fixed eating speed `k` (bananas per hour). Each hour she picks \
         a pile and eats `k` bananas from it. If the pile has fewer than `k` bananas, she \
         finishes it and waits for the next hour.\n\n\
         Return the minimum integer `k` such that she can eat all bananas within `h` hours.\n\n\
         Constraints:\n\
         - 1 <= piles.len() <= 10000\n\
         - 1 <= piles[i] <= 10^9\n\
         - piles.len() <= h <= 10^9"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            let piles: Vec<i32> = (0..n).map(|_| rng.random_range(1..=1000)).collect();
            // h must be >= n (at minimum one hour per pile)
            let h = rng.random_range(n as i32..=n as i32 * 10);
            TestCase { data: Box::new(KokoTest { piles, h }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KokoTest>().unwrap();
        let expected = ref_min_eating_speed(&t.piles, t.h);
        let actual = solutions::min_eating_speed(&t.piles, t.h);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("piles={:?}, h={}", t.piles, t.h),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_eating_speed(piles: &[i32], h: i32) -> i32 {
    let mut lo = 1i64;
    let mut hi = *piles.iter().max().unwrap() as i64;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let hours: i64 = piles.iter().map(|&p| (p as i64 + mid - 1) / mid).sum();
        if hours <= h as i64 {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo as i32
}

// ── Hard 1: Median of Two Sorted Arrays ────────────────────────────────

struct BinarySearchMedianSortedArrays;

struct MedianSortedTest {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

impl Problem for BinarySearchMedianSortedArrays {
    fn id(&self) -> &str { "binary_search_median_sorted_arrays" }
    fn name(&self) -> &str { "Median of Two Sorted Arrays" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given two sorted arrays `nums1` and `nums2` of size m and n respectively, \
         return the median of the two sorted arrays.\n\n\
         The overall run time complexity should be O(log(min(m, n))).\n\n\
         Constraints:\n\
         - 0 <= nums1.len(), nums2.len() <= 1000\n\
         - 1 <= nums1.len() + nums2.len() <= 2000\n\
         - Return the median as f64"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let m = rng.random_range(0..=20);
            let n = rng.random_range(1..=20);
            let mut nums1 = crate::problems::helpers::random_vec(&mut rng, m, -100, 100);
            let mut nums2 = crate::problems::helpers::random_vec(&mut rng, n, -100, 100);
            nums1.sort();
            nums2.sort();
            TestCase { data: Box::new(MedianSortedTest { nums1, nums2 }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MedianSortedTest>().unwrap();
        let expected = ref_median_sorted(&t.nums1, &t.nums2);
        let actual = solutions::find_median_sorted_arrays(&t.nums1, &t.nums2);
        SolutionResult {
            is_correct: (expected - actual).abs() < 1e-5,
            input_description: format!("nums1={:?}, nums2={:?}", t.nums1, t.nums2),
            expected: format!("{expected:.5}"),
            actual: format!("{actual:.5}"),
        }
    }
}

fn ref_median_sorted(nums1: &[i32], nums2: &[i32]) -> f64 {
    let mut merged = nums1.to_vec();
    merged.extend_from_slice(nums2);
    merged.sort();
    let n = merged.len();
    if n % 2 == 1 {
        merged[n / 2] as f64
    } else {
        (merged[n / 2 - 1] as f64 + merged[n / 2] as f64) / 2.0
    }
}

// ── Hard 2: Split Array Largest Sum ────────────────────────────────────

struct BinarySearchSplitArrayLargest;

struct SplitArrayTest {
    nums: Vec<i32>,
    m: i32,
}

impl Problem for BinarySearchSplitArrayLargest {
    fn id(&self) -> &str { "binary_search_split_array_largest" }
    fn name(&self) -> &str { "Split Array Largest Sum" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an integer array `nums` and an integer `m`, split the array into `m` \
         non-empty contiguous subarrays. Minimize the largest sum among these `m` \
         subarrays. Return this minimized largest sum.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 1000\n\
         - 0 <= nums[i] <= 10^6\n\
         - 1 <= m <= min(50, nums.len())"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
            let m = rng.random_range(1..=n as i32);
            TestCase { data: Box::new(SplitArrayTest { nums, m }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SplitArrayTest>().unwrap();
        let expected = ref_split_array(&t.nums, t.m);
        let actual = solutions::split_array_largest_sum(&t.nums, t.m);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, m={}", t.nums, t.m),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_split_array(nums: &[i32], m: i32) -> i32 {
    let mut lo: i64 = *nums.iter().max().unwrap() as i64;
    let mut hi: i64 = nums.iter().map(|&x| x as i64).sum();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if ref_can_split(nums, m, mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo as i32
}

fn ref_can_split(nums: &[i32], m: i32, max_sum: i64) -> bool {
    let mut count = 1;
    let mut cur_sum: i64 = 0;
    for &n in nums {
        cur_sum += n as i64;
        if cur_sum > max_sum {
            count += 1;
            cur_sum = n as i64;
            if count > m {
                return false;
            }
        }
    }
    true
}

// ── Hard 3: Find K-th Smallest Pair Distance ───────────────────────────

struct BinarySearchFindKthSmallestPair;

struct KthPairTest {
    nums: Vec<i32>,
    k: i32,
}

impl Problem for BinarySearchFindKthSmallestPair {
    fn id(&self) -> &str { "binary_search_find_kth_smallest_pair" }
    fn name(&self) -> &str { "Find K-th Smallest Pair Distance" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "The distance of a pair (nums[i], nums[j]) is |nums[i] - nums[j]|.\n\n\
         Given an integer array `nums` and an integer `k`, return the k-th smallest \
         distance among all pairs (i, j) where 0 <= i < j < nums.len().\n\n\
         Constraints:\n\
         - 2 <= nums.len() <= 1000\n\
         - 0 <= nums[i] <= 10^6\n\
         - 1 <= k <= nums.len() * (nums.len() - 1) / 2"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=15);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
            let total_pairs = n * (n - 1) / 2;
            let k = rng.random_range(1..=total_pairs as i32);
            TestCase { data: Box::new(KthPairTest { nums, k }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KthPairTest>().unwrap();
        let expected = ref_kth_smallest_pair(&t.nums, t.k);
        let actual = solutions::kth_smallest_pair_distance(&t.nums, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_kth_smallest_pair(nums: &[i32], k: i32) -> i32 {
    let mut sorted = nums.to_vec();
    sorted.sort();
    let n = sorted.len();
    let mut lo = 0i32;
    let mut hi = sorted[n - 1] - sorted[0];
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let count = ref_count_pairs_within(&sorted, mid);
        if count >= k {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn ref_count_pairs_within(sorted: &[i32], dist: i32) -> i32 {
    let mut count = 0i32;
    let mut left = 0usize;
    for right in 1..sorted.len() {
        while sorted[right] - sorted[left] > dist {
            left += 1;
        }
        count += (right - left) as i32;
    }
    count
}

// ── Hard 4: Count of Smaller Numbers After Self ────────────────────────

struct BinarySearchCountSmallerAfter;

struct CountSmallerTest {
    nums: Vec<i32>,
}

impl Problem for BinarySearchCountSmallerAfter {
    fn id(&self) -> &str { "binary_search_count_smaller_after" }
    fn name(&self) -> &str { "Count of Smaller Numbers After Self" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an integer array `nums`, return an array `counts` where `counts[i]` is \
         the number of smaller elements to the right of `nums[i]`.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 10000\n\
         - -10^4 <= nums[i] <= 10^4\n\n\
         Hint: Use binary search with an insertion approach or a modified merge sort."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            TestCase { data: Box::new(CountSmallerTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountSmallerTest>().unwrap();
        let expected = ref_count_smaller(&t.nums);
        let actual = solutions::count_smaller_after_self(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_count_smaller(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![0i32; n];
    let mut sorted: Vec<i32> = Vec::new();
    for i in (0..n).rev() {
        // Binary search for the insertion position of nums[i] in sorted
        let pos = sorted.partition_point(|&x| x < nums[i]);
        result[i] = pos as i32;
        sorted.insert(pos, nums[i]);
    }
    result
}

// ── Hard 5: Russian Doll Envelopes ─────────────────────────────────────

struct BinarySearchRussianDollEnvelopes;

struct EnvelopesTest {
    envelopes: Vec<(i32, i32)>,
}

impl Problem for BinarySearchRussianDollEnvelopes {
    fn id(&self) -> &str { "binary_search_russian_doll_envelopes" }
    fn name(&self) -> &str { "Russian Doll Envelopes" }
    fn topic(&self) -> &str { "binary_search" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "You are given a 2D array of integers `envelopes` where \
         `envelopes[i] = (width_i, height_i)` represents the width and height of an \
         envelope.\n\n\
         One envelope can fit into another if and only if both the width and height of \
         one envelope are strictly greater than the other envelope.\n\n\
         Return the maximum number of envelopes you can Russian doll (nest inside \
         each other).\n\n\
         Constraints:\n\
         - 1 <= envelopes.len() <= 5000\n\
         - Hint: Sort by width ascending, then height descending. Then find the longest \
         increasing subsequence (LIS) on heights using binary search for O(n log n)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let envelopes: Vec<(i32, i32)> = (0..n)
                .map(|_| {
                    (rng.random_range(1..=100), rng.random_range(1..=100))
                })
                .collect();
            TestCase { data: Box::new(EnvelopesTest { envelopes }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<EnvelopesTest>().unwrap();
        let expected = ref_max_envelopes(&t.envelopes);
        let actual = solutions::max_envelopes(&t.envelopes);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("envelopes={:?}", t.envelopes),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_envelopes(envelopes: &[(i32, i32)]) -> i32 {
    if envelopes.is_empty() {
        return 0;
    }
    let mut sorted = envelopes.to_vec();
    // Sort by width ascending; if widths equal, sort by height descending
    sorted.sort_by(|a, b| {
        if a.0 == b.0 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    // LIS on heights using binary search (patience sorting)
    let mut tails: Vec<i32> = Vec::new();
    for &(_, h) in &sorted {
        let pos = tails.partition_point(|&x| x < h);
        if pos == tails.len() {
            tails.push(h);
        } else {
            tails[pos] = h;
        }
    }
    tails.len() as i32
}
