use rand::Rng;
use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part2_sorting::two_pointers as solutions;
use crate::tracker::{track_slice, track_string, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(TwoSumSorted),
        Box::new(RemoveDuplicates),
        Box::new(MoveZeroes),
        Box::new(ValidPalindrome),
        Box::new(SquaresSorted),
        Box::new(ThreeSum),
        Box::new(ContainerWater),
        Box::new(LongestRepeatingReplacement),
        Box::new(FruitIntoBaskets),
        Box::new(MinSizeSubarraySum),
        Box::new(TrappingRainWater),
        Box::new(MinimumWindowSubstring),
        Box::new(SubstringConcatenation),
        Box::new(SlidingWindowMax),
        Box::new(FourSum),
    ]
}

// ── Easy 1: Two Sum in Sorted Array ─────────────────────────────────────

struct TwoSumSorted;
struct TwoSumSortedTest {
    nums: Vec<i32>,
    target: i32,
}

impl Problem for TwoSumSorted {
    fn id(&self) -> &str {
        "two_pointers_two_sum_sorted"
    }
    fn name(&self) -> &str {
        "Two Sum II - Sorted Array"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a 1-indexed sorted array `nums` and an integer `target`, return the indices \
         (1-indexed) of the two numbers that add up to `target`.\n\n\
         Constraints:\n\
         - 2 <= nums.len() <= 1000\n\
         - Exactly one solution exists.\n\
         - Return (i, j) where i < j, both 1-indexed."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=20);
                let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(-500..=500)).collect();
                nums.sort();
                let idx_a = rng.random_range(0..n);
                let mut idx_b = rng.random_range(0..n);
                while idx_b == idx_a {
                    idx_b = rng.random_range(0..n);
                }
                let target = nums[idx_a] + nums[idx_b];
                TestCase {
                    data: Box::new(TwoSumSortedTest { nums, target }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TwoSumSortedTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_two_sum_sorted(&t.nums, t.target);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::two_sum_sorted(&tracked_nums, t.target);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, target={}", t.nums, t.target),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_two_sum_sorted(nums: &[i32], target: i32) -> (usize, usize) {
    let (mut l, mut r) = (0, nums.len() - 1);
    while l < r {
        let sum = nums[l] + nums[r];
        if sum == target {
            return (l + 1, r + 1);
        } else if sum < target {
            l += 1;
        } else {
            r -= 1;
        }
    }
    (0, 0)
}

// ── Easy 2: Remove Duplicates from Sorted Array ─────────────────────────

struct RemoveDuplicates;
struct RemoveDuplicatesTest {
    nums: Vec<i32>,
}

impl Problem for RemoveDuplicates {
    fn id(&self) -> &str {
        "two_pointers_remove_duplicates"
    }
    fn name(&self) -> &str {
        "Remove Duplicates from Sorted Array"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a sorted integer array `nums`, remove the duplicates in-place such that \
         each element appears only once. Return the number of unique elements `k`.\n\n\
         The first `k` elements of `nums` should hold the unique values in sorted order."
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

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RemoveDuplicatesTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let mut expected = t.nums.clone();
        expected.dedup();
        let expected_k = expected.len();

        let mut tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual_k = solutions::remove_duplicates(&mut tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        let actual_slice: Vec<i32> = tracked_nums[..actual_k].iter().map(|t| t.value).collect();

        SolutionResult {
            is_correct: actual_k == expected_k && actual_slice == expected[..],
            input_description: format!("nums={:?}", t.nums),
            expected: format!("k={expected_k}, vals={expected:?}"),
            actual: format!("k={actual_k}, vals={actual_slice:?}"),
        }
    }
}

// ── Easy 3: Move Zeroes ─────────────────────────────────────────────────

struct MoveZeroes;
struct MoveZeroesTest {
    nums: Vec<i32>,
}

impl Problem for MoveZeroes {
    fn id(&self) -> &str {
        "two_pointers_move_zeroes"
    }
    fn name(&self) -> &str {
        "Move Zeroes"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an integer array `nums`, move all 0's to the end while maintaining the \
         relative order of the non-zero elements. Do this in-place.\n\n\
         Return the resulting array."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n)
                    .map(|_| {
                        if rng.random_range(0..=3) == 0 {
                            0
                        } else {
                            rng.random_range(-50..=50)
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(MoveZeroesTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MoveZeroesTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_move_zeroes(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::move_zeroes(&tracked_nums);
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

fn ref_move_zeroes(nums: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = nums.iter().copied().filter(|&x| x != 0).collect();
    result.resize(nums.len(), 0);
    result
}

// ── Easy 4: Valid Palindrome ────────────────────────────────────────────

struct ValidPalindrome;
struct ValidPalindromeTest {
    s: String,
}

impl Problem for ValidPalindrome {
    fn id(&self) -> &str {
        "two_pointers_valid_palindrome"
    }
    fn name(&self) -> &str {
        "Valid Palindrome"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a string `s`, determine if it is a palindrome, considering only \
         alphanumeric characters and ignoring cases.\n\n\
         Return `true` if it is a palindrome, `false` otherwise."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let palindromes = [
            "A man, a plan, a canal: Panama",
            "racecar",
            "Was it a car or a cat I saw?",
            "No 'x' in Nixon",
            "ab_ba",
        ];
        let non_palindromes = [
            "hello world",
            "race a car",
            "abcdef",
            "not a palindrome!",
            "0P",
        ];
        (0..10)
            .map(|i| {
                let s = if i < 5 {
                    palindromes[i].to_string()
                } else if rng.random_range(0..=1) == 0 {
                    non_palindromes[i - 5].to_string()
                } else {
                    let len = rng.random_range(1..=15);
                    let half: String = (0..len)
                        .map(|_| {
                            let c = rng.random_range(b'a'..=b'z');
                            c as char
                        })
                        .collect();
                    let rev: String = half.chars().rev().collect();
                    format!("{half}{rev}")
                };
                TestCase {
                    data: Box::new(ValidPalindromeTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ValidPalindromeTest>().unwrap();
        let expected = ref_valid_palindrome(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::valid_palindrome(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_valid_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let lower: Vec<char> = chars.iter().map(|c| c.to_ascii_lowercase()).collect();
    let n = lower.len();
    for i in 0..n / 2 {
        if lower[i] != lower[n - 1 - i] {
            return false;
        }
    }
    true
}

// ── Easy 5: Squares of a Sorted Array ──────────────────────────────────

struct SquaresSorted;
struct SquaresSortedTest {
    nums: Vec<i32>,
}

impl Problem for SquaresSorted {
    fn id(&self) -> &str {
        "two_pointers_squares_sorted"
    }
    fn name(&self) -> &str {
        "Squares of a Sorted Array"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a sorted integer array `nums`, return an array of the squares of each \
         number, sorted in non-decreasing order.\n\n\
         Solve in O(n) time using two pointers."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                nums.sort();
                TestCase {
                    data: Box::new(SquaresSortedTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SquaresSortedTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let mut expected: Vec<i32> = t.nums.iter().map(|x| x * x).collect();
        expected.sort();
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::squares_sorted(&tracked_nums);
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

// ── Medium 1: Three Sum ────────────────────────────────────────────────

struct ThreeSum;
struct ThreeSumTest {
    nums: Vec<i32>,
}

impl Problem for ThreeSum {
    fn id(&self) -> &str {
        "two_pointers_three_sum"
    }
    fn name(&self) -> &str {
        "Three Sum"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an integer array `nums`, return all unique triplets [a, b, c] such that \
         a + b + c = 0.\n\n\
         The solution set must not contain duplicate triplets. Return each triplet sorted, \
         and the result sorted lexicographically."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=20);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-10..=10)).collect();
                TestCase {
                    data: Box::new(ThreeSumTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ThreeSumTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_three_sum(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::three_sum(&tracked_nums);
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

fn ref_three_sum(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut sorted = nums.to_vec();
    sorted.sort();
    let n = sorted.len();
    let mut result = Vec::new();
    for i in 0..n {
        if i > 0 && sorted[i] == sorted[i - 1] {
            continue;
        }
        let (mut l, mut r) = (i + 1, n - 1);
        while l < r {
            let sum = sorted[i] + sorted[l] + sorted[r];
            if sum == 0 {
                result.push(vec![sorted[i], sorted[l], sorted[r]]);
                while l < r && sorted[l] == sorted[l + 1] {
                    l += 1;
                }
                while l < r && sorted[r] == sorted[r - 1] {
                    r -= 1;
                }
                l += 1;
                r -= 1;
            } else if sum < 0 {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    result
}

// ── Medium 2: Container With Most Water ────────────────────────────────

struct ContainerWater;
struct ContainerWaterTest {
    height: Vec<i32>,
}

impl Problem for ContainerWater {
    fn id(&self) -> &str {
        "two_pointers_container_water"
    }
    fn name(&self) -> &str {
        "Container With Most Water"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given `n` non-negative integers `height` where each represents a vertical line \
         at position i, find two lines that together with the x-axis form a container that \
         holds the most water.\n\n\
         Return the maximum amount of water the container can store."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=30);
                let height: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                TestCase {
                    data: Box::new(ContainerWaterTest { height }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ContainerWaterTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_container_water(&t.height);
        let tracked_height = track_slice(&t.height, shared_log.clone());
        let actual = solutions::container_water(&tracked_height);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("height={:?}", t.height),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_container_water(height: &[i32]) -> i32 {
    let (mut l, mut r) = (0, height.len() - 1);
    let mut max_area = 0;
    while l < r {
        let area = height[l].min(height[r]) * (r - l) as i32;
        max_area = max_area.max(area);
        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    max_area
}

// ── Medium 3: Longest Repeating Character Replacement ──────────────────

struct LongestRepeatingReplacement;
struct LRRTest {
    s: String,
    k: i32,
}

impl Problem for LongestRepeatingReplacement {
    fn id(&self) -> &str {
        "two_pointers_longest_repeating_replacement"
    }
    fn name(&self) -> &str {
        "Longest Repeating Character Replacement"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a string `s` consisting of uppercase English letters and an integer `k`, \
         you can choose any character and change it to any other uppercase English letter. \
         You can perform this at most `k` times.\n\n\
         Return the length of the longest substring containing the same letter after \
         performing at most `k` replacements."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let num_chars = rng.random_range(1..=4);
                let chars: Vec<char> = (0..num_chars)
                    .map(|_| (b'A' + rng.random_range(0..=25)) as char)
                    .collect();
                let s: String = (0..n)
                    .map(|_| chars[rng.random_range(0..chars.len())])
                    .collect();
                let k = rng.random_range(0..=n as i32 / 2);
                TestCase {
                    data: Box::new(LRRTest { s, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LRRTest>().unwrap();
        let expected = ref_longest_repeating_replacement(&t.s, t.k);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::longest_repeating_replacement(&tracked, t.k);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}, k={}", t.s, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_longest_repeating_replacement(s: &str, k: i32) -> i32 {
    let bytes = s.as_bytes();
    let mut count = [0i32; 26];
    let mut max_count = 0;
    let mut max_len = 0;
    let mut left = 0;
    for right in 0..bytes.len() {
        let idx = (bytes[right] - b'A') as usize;
        count[idx] += 1;
        max_count = max_count.max(count[idx]);
        let window_len = (right - left + 1) as i32;
        if window_len - max_count > k {
            count[(bytes[left] - b'A') as usize] -= 1;
            left += 1;
        }
        max_len = max_len.max((right - left + 1) as i32);
    }
    max_len
}

// ── Medium 4: Fruit Into Baskets ───────────────────────────────────────

struct FruitIntoBaskets;
struct FruitTest {
    fruits: Vec<i32>,
}

impl Problem for FruitIntoBaskets {
    fn id(&self) -> &str {
        "two_pointers_fruit_into_baskets"
    }
    fn name(&self) -> &str {
        "Fruit Into Baskets"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "You have a row of trees, each bearing a type of fruit (given as `fruits[i]`). \
         You have two baskets and each basket can only hold one type of fruit.\n\n\
         Starting from any tree, pick exactly one fruit from each tree moving to the right. \
         Stop when you would need a third basket type.\n\n\
         Return the maximum number of fruits you can collect (longest subarray with at most \
         2 distinct values)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let num_types = rng.random_range(1..=5);
                let fruits: Vec<i32> = (0..n).map(|_| rng.random_range(0..num_types)).collect();
                TestCase {
                    data: Box::new(FruitTest { fruits }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FruitTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_fruit_into_baskets(&t.fruits);
        let tracked_fruits = track_slice(&t.fruits, shared_log.clone());
        let actual = solutions::fruit_into_baskets(&tracked_fruits);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("fruits={:?}", t.fruits),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_fruit_into_baskets(fruits: &[i32]) -> i32 {
    let mut count: HashMap<i32, i32> = HashMap::new();
    let mut left = 0;
    let mut max_len = 0;
    for right in 0..fruits.len() {
        *count.entry(fruits[right]).or_insert(0) += 1;
        while count.len() > 2 {
            let lf = fruits[left];
            *count.get_mut(&lf).unwrap() -= 1;
            if count[&lf] == 0 {
                count.remove(&lf);
            }
            left += 1;
        }
        max_len = max_len.max((right - left + 1) as i32);
    }
    max_len
}

// ── Medium 5: Minimum Size Subarray Sum ────────────────────────────────

struct MinSizeSubarraySum;
struct MinSizeTest {
    nums: Vec<i32>,
    target: i32,
}

impl Problem for MinSizeSubarraySum {
    fn id(&self) -> &str {
        "two_pointers_min_size_subarray_sum"
    }
    fn name(&self) -> &str {
        "Minimum Size Subarray Sum"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array of positive integers `nums` and a positive integer `target`, \
         return the minimal length of a contiguous subarray whose sum is greater than \
         or equal to `target`.\n\n\
         Return 0 if no such subarray exists."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(1..=50)).collect();
                let total: i32 = nums.iter().sum();
                let target = rng.random_range(1..=total + 10);
                TestCase {
                    data: Box::new(MinSizeTest { nums, target }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinSizeTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_min_size_subarray_sum(&t.nums, t.target);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::min_size_subarray_sum(&tracked_nums, t.target);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, target={}", t.nums, t.target),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_size_subarray_sum(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut sum = 0;
    let mut min_len = i32::MAX;
    for right in 0..nums.len() {
        sum += nums[right];
        while sum >= target {
            min_len = min_len.min((right - left + 1) as i32);
            sum -= nums[left];
            left += 1;
        }
    }
    if min_len == i32::MAX {
        0
    } else {
        min_len
    }
}

// ── Hard 1: Trapping Rain Water ────────────────────────────────────────

struct TrappingRainWater;
struct TrapTest {
    height: Vec<i32>,
}

impl Problem for TrappingRainWater {
    fn id(&self) -> &str {
        "two_pointers_trapping_rain_water"
    }
    fn name(&self) -> &str {
        "Trapping Rain Water"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given `n` non-negative integers representing an elevation map where the width \
         of each bar is 1, compute how much water it can trap after raining.\n\n\
         Solve using the two-pointer technique in O(n) time and O(1) space."
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

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TrapTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_trap(&t.height);
        let tracked_height = track_slice(&t.height, shared_log.clone());
        let actual = solutions::trapping_rain_water(&tracked_height);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
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

// ── Hard 2: Minimum Window Substring ───────────────────────────────────

struct MinimumWindowSubstring;
struct MWSTest {
    s: String,
    t: String,
}

impl Problem for MinimumWindowSubstring {
    fn id(&self) -> &str {
        "two_pointers_minimum_window_substring"
    }
    fn name(&self) -> &str {
        "Minimum Window Substring"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given two strings `s` and `t`, return the minimum window substring of `s` such \
         that every character in `t` (including duplicates) is included in the window.\n\n\
         If no such substring exists, return the empty string \"\".\n\
         If there are multiple answers, return any valid one."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let sn = rng.random_range(5..=40);
                let tn = rng.random_range(1..=sn.min(10));
                let charset: Vec<char> = "abcdefABCDEF".chars().collect();
                let s: String = (0..sn)
                    .map(|_| charset[rng.random_range(0..charset.len())])
                    .collect();
                // Build t from characters in s to increase chance of a valid answer.
                let s_chars: Vec<char> = s.chars().collect();
                let t: String = (0..tn)
                    .map(|_| s_chars[rng.random_range(0..s_chars.len())])
                    .collect();
                TestCase {
                    data: Box::new(MWSTest { s, t }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MWSTest>().unwrap();
        let expected = ref_min_window(&t.s, &t.t);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_s = track_string(&t.s, shared_log.clone());
        let tracked_t = track_string(&t.t, shared_log.clone());
        let actual = solutions::minimum_window_substring(&tracked_s, &tracked_t);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        // Verify actual is valid: contains all chars of t and has same length as expected.
        let valid = if expected.is_empty() {
            actual.is_empty()
        } else {
            actual.len() == expected.len() && is_valid_window(&actual, &t.t)
        };
        SolutionResult {
            is_correct: valid,
            input_description: format!("s={:?}, t={:?}", t.s, t.t),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn is_valid_window(window: &str, t: &str) -> bool {
    let mut need: HashMap<char, i32> = HashMap::new();
    for c in t.chars() {
        *need.entry(c).or_insert(0) += 1;
    }
    for c in window.chars() {
        if let Some(v) = need.get_mut(&c) {
            *v -= 1;
        }
    }
    need.values().all(|&v| v <= 0)
}

fn ref_min_window(s: &str, t: &str) -> String {
    if t.is_empty() || s.len() < t.len() {
        return String::new();
    }
    let mut need: HashMap<char, i32> = HashMap::new();
    for c in t.chars() {
        *need.entry(c).or_insert(0) += 1;
    }
    let mut window: HashMap<char, i32> = HashMap::new();
    let s_bytes: Vec<char> = s.chars().collect();
    let required = need.len();
    let mut formed = 0;
    let mut left = 0;
    let mut best = (usize::MAX, 0usize, 0usize); // (len, l, r)
    for right in 0..s_bytes.len() {
        let c = s_bytes[right];
        *window.entry(c).or_insert(0) += 1;
        if need.contains_key(&c) && window[&c] == need[&c] {
            formed += 1;
        }
        while formed == required {
            let len = right - left + 1;
            if len < best.0 {
                best = (len, left, right);
            }
            let lc = s_bytes[left];
            *window.get_mut(&lc).unwrap() -= 1;
            if need.contains_key(&lc) && window[&lc] < need[&lc] {
                formed -= 1;
            }
            left += 1;
        }
    }
    if best.0 == usize::MAX {
        String::new()
    } else {
        s_bytes[best.1..=best.2].iter().collect()
    }
}

// ── Hard 3: Substring with Concatenation of All Words ──────────────────

struct SubstringConcatenation;
struct SubConcatTest {
    s: String,
    words: Vec<String>,
}

impl Problem for SubstringConcatenation {
    fn id(&self) -> &str {
        "two_pointers_substring_concatenation"
    }
    fn name(&self) -> &str {
        "Substring with Concatenation of All Words"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a string `s` and an array of strings `words` (all of the same length), \
         find all starting indices of substring(s) in `s` that are a concatenation of \
         each word in `words` exactly once, in any order.\n\n\
         Return the indices in ascending order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let word_len = rng.random_range(2..=4);
                let num_words = rng.random_range(2..=4);
                let words: Vec<String> = (0..num_words)
                    .map(|_| {
                        (0..word_len)
                            .map(|_| (b'a' + rng.random_range(0..=3)) as char)
                            .collect()
                    })
                    .collect();
                // Build s that sometimes contains the concatenation.
                let total_len = word_len * num_words;
                let prefix_len = rng.random_range(0..=5);
                let suffix_len = rng.random_range(0..=5);
                let mut s = String::new();
                for _ in 0..prefix_len {
                    s.push((b'a' + rng.random_range(0..=3)) as char);
                }
                // Sometimes embed the words concatenation.
                if rng.random_range(0..=1) == 0 {
                    let mut perm = words.clone();
                    // Shuffle words.
                    for i in (1..perm.len()).rev() {
                        let j = rng.random_range(0..=i);
                        perm.swap(i, j);
                    }
                    for w in &perm {
                        s.push_str(w);
                    }
                } else {
                    for _ in 0..total_len {
                        s.push((b'a' + rng.random_range(0..=3)) as char);
                    }
                }
                for _ in 0..suffix_len {
                    s.push((b'a' + rng.random_range(0..=3)) as char);
                }
                TestCase {
                    data: Box::new(SubConcatTest { s, words }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SubConcatTest>().unwrap();
        let expected = ref_substring_concatenation(&t.s, &t.words);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let mut actual = solutions::substring_concatenation(&tracked, &t.words);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        actual.sort();
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}, words={:?}", t.s, t.words),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_substring_concatenation(s: &str, words: &[String]) -> Vec<usize> {
    if words.is_empty() || s.is_empty() {
        return vec![];
    }
    let word_len = words[0].len();
    let total_len = word_len * words.len();
    if s.len() < total_len {
        return vec![];
    }
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for w in words {
        *word_count.entry(w.as_str()).or_insert(0) += 1;
    }
    let mut result = Vec::new();
    for i in 0..=s.len() - total_len {
        let mut seen: HashMap<&str, i32> = HashMap::new();
        let mut valid = true;
        for j in 0..words.len() {
            let start = i + j * word_len;
            let word = &s[start..start + word_len];
            if !word_count.contains_key(word) {
                valid = false;
                break;
            }
            *seen.entry(word).or_insert(0) += 1;
            if seen[word] > word_count[word] {
                valid = false;
                break;
            }
        }
        if valid {
            result.push(i);
        }
    }
    result.sort();
    result
}

// ── Hard 4: Sliding Window Maximum ─────────────────────────────────────

struct SlidingWindowMax;
struct SWMaxTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for SlidingWindowMax {
    fn id(&self) -> &str {
        "two_pointers_sliding_window_max"
    }
    fn name(&self) -> &str {
        "Sliding Window Maximum"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array `nums` and a sliding window of size `k` that moves from the \
         very left to the very right, return the max value in each window position.\n\n\
         Use a deque-based approach for O(n) time."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let k = rng.random_range(1..=n);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(SWMaxTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SWMaxTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_sliding_window_max(&t.nums, t.k);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sliding_window_max(&tracked_nums, t.k);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_sliding_window_max(nums: &[i32], k: usize) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut result = Vec::new();
    for i in 0..nums.len() {
        while !deque.is_empty() && *deque.front().unwrap() + k <= i {
            deque.pop_front();
        }
        while !deque.is_empty() && nums[*deque.back().unwrap()] <= nums[i] {
            deque.pop_back();
        }
        deque.push_back(i);
        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }
    result
}

// ── Hard 5: Four Sum ───────────────────────────────────────────────────

struct FourSum;
struct FourSumTest {
    nums: Vec<i32>,
    target: i32,
}

impl Problem for FourSum {
    fn id(&self) -> &str {
        "two_pointers_four_sum"
    }
    fn name(&self) -> &str {
        "Four Sum"
    }
    fn topic(&self) -> &str {
        "two_pointers"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array `nums` of n integers and an integer `target`, return all unique \
         quadruplets [a, b, c, d] such that a + b + c + d = target.\n\n\
         Each quadruplet should be sorted, and the result sorted lexicographically.\n\
         The solution set must not contain duplicate quadruplets."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(4..=15);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-10..=10)).collect();
                let target = rng.random_range(-20..=20);
                TestCase {
                    data: Box::new(FourSumTest { nums, target }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FourSumTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_four_sum(&t.nums, t.target);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::four_sum(&tracked_nums, t.target);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, target={}", t.nums, t.target),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_four_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut sorted = nums.to_vec();
    sorted.sort();
    let n = sorted.len();
    let mut result = Vec::new();
    for i in 0..n {
        if i > 0 && sorted[i] == sorted[i - 1] {
            continue;
        }
        for j in i + 1..n {
            if j > i + 1 && sorted[j] == sorted[j - 1] {
                continue;
            }
            let (mut l, mut r) = (j + 1, n - 1);
            while l < r {
                let sum = sorted[i] as i64 + sorted[j] as i64 + sorted[l] as i64 + sorted[r] as i64;
                if sum == target as i64 {
                    result.push(vec![sorted[i], sorted[j], sorted[l], sorted[r]]);
                    while l < r && sorted[l] == sorted[l + 1] {
                        l += 1;
                    }
                    while l < r && sorted[r] == sorted[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                } else if sum < target as i64 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
    }
    result
}
