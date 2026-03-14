use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part6_advanced::monotonic as solutions;
use crate::tracker::{track_slice, OperationLog, Tracked};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy (5)
        Box::new(MonotonicNextGreater),
        Box::new(MonotonicNextSmaller),
        Box::new(MonotonicDailyTemperatures),
        Box::new(MonotonicStockSpan),
        Box::new(MonotonicSlidingWindowMax),
        // Medium (5)
        Box::new(MonotonicLargestRectangle),
        Box::new(MonotonicMaximalRectangle),
        Box::new(MonotonicRemoveKDigits),
        Box::new(MonotonicSumOfSubarrayMinimums),
        Box::new(MonotonicSlidingWindowMin),
        // Hard (5)
        Box::new(MonotonicTrappingRainWater),
        Box::new(MonotonicMaxWidthRamp),
        Box::new(MonotonicSumSubarrayRanges),
        Box::new(MonotonicShortestSubarraySumK),
        Box::new(MonotonicMaxBinaryString),
    ]
}

// ── Reference implementations ────────────────────────────────────────

fn ref_next_greater(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![-1i32; n];
    for i in 0..n {
        for j in (i + 1)..n {
            if nums[j] > nums[i] {
                result[i] = nums[j];
                break;
            }
        }
    }
    result
}

fn ref_next_smaller(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![-1i32; n];
    for i in 0..n {
        for j in (i + 1)..n {
            if nums[j] < nums[i] {
                result[i] = nums[j];
                break;
            }
        }
    }
    result
}

fn ref_daily_temperatures(temps: &[i32]) -> Vec<i32> {
    let n = temps.len();
    let mut result = vec![0i32; n];
    for i in 0..n {
        for j in (i + 1)..n {
            if temps[j] > temps[i] {
                result[i] = (j - i) as i32;
                break;
            }
        }
    }
    result
}

fn ref_stock_span(prices: &[i32]) -> Vec<i32> {
    let n = prices.len();
    let mut result = vec![0i32; n];
    for i in 0..n {
        let mut span = 1;
        let mut j = i as i32 - 1;
        while j >= 0 && prices[j as usize] <= prices[i] {
            j -= 1;
            span += 1;
        }
        result[i] = span;
    }
    result
}

fn ref_sliding_window_max(nums: &[i32], k: usize) -> Vec<i32> {
    if nums.is_empty() || k == 0 {
        return vec![];
    }
    let n = nums.len();
    let mut result = Vec::new();
    for i in 0..=(n.saturating_sub(k)) {
        let end = (i + k).min(n);
        if let Some(&max_val) = nums[i..end].iter().max() {
            result.push(max_val);
        }
    }
    result
}

fn ref_largest_rectangle(heights: &[i32]) -> i32 {
    let n = heights.len();
    let mut max_area = 0;
    for i in 0..n {
        let mut min_h = heights[i];
        for (j, &h) in heights.iter().enumerate().take(n).skip(i) {
            min_h = min_h.min(h);
            max_area = max_area.max(min_h * (j - i + 1) as i32);
        }
    }
    max_area
}

fn ref_maximal_rectangle(matrix: &[Vec<i32>]) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut heights = vec![0i32; cols];
    let mut max_area = 0;
    for row in matrix.iter().take(rows) {
        for (c, h) in heights.iter_mut().enumerate().take(cols) {
            if row[c] == 1 {
                *h += 1;
            } else {
                *h = 0;
            }
        }
        max_area = max_area.max(ref_largest_rectangle(&heights));
    }
    max_area
}

fn ref_remove_k_digits(num: &str, k: usize) -> String {
    let mut stack: Vec<u8> = Vec::new();
    let mut remaining = k;
    for &ch in num.as_bytes() {
        while remaining > 0 && !stack.is_empty() && *stack.last().unwrap() > ch {
            stack.pop();
            remaining -= 1;
        }
        stack.push(ch);
    }
    while remaining > 0 && !stack.is_empty() {
        stack.pop();
        remaining -= 1;
    }
    let result: String = stack
        .into_iter()
        .map(|b| b as char)
        .collect::<String>()
        .trim_start_matches('0')
        .to_string();
    if result.is_empty() {
        "0".to_string()
    } else {
        result
    }
}

fn ref_sum_of_subarray_minimums(arr: &[i32]) -> i32 {
    let modulo = 1_000_000_007i64;
    let n = arr.len();
    let mut total: i64 = 0;
    for i in 0..n {
        let mut min_val = arr[i];
        for &val in arr.iter().take(n).skip(i) {
            min_val = min_val.min(val);
            total = (total + min_val as i64) % modulo;
        }
    }
    total as i32
}

fn ref_sliding_window_min(nums: &[i32], k: usize) -> Vec<i32> {
    if nums.is_empty() || k == 0 {
        return vec![];
    }
    let n = nums.len();
    let mut result = Vec::new();
    for i in 0..=(n.saturating_sub(k)) {
        let end = (i + k).min(n);
        if let Some(&min_val) = nums[i..end].iter().min() {
            result.push(min_val);
        }
    }
    result
}

fn ref_trapping_rain_water(height: &[i32]) -> i32 {
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

fn ref_max_width_ramp(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut max_width = 0i32;
    for i in 0..n {
        for j in (i + 1)..n {
            if nums[i] <= nums[j] {
                max_width = max_width.max((j - i) as i32);
            }
        }
    }
    max_width
}

fn ref_sum_subarray_ranges(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut total: i64 = 0;
    for i in 0..n {
        let mut min_val = nums[i];
        let mut max_val = nums[i];
        for &num in nums.iter().take(n).skip(i) {
            min_val = min_val.min(num);
            max_val = max_val.max(num);
            total += (max_val - min_val) as i64;
        }
    }
    total
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
                break;
            }
        }
    }
    if min_len == i32::MAX {
        -1
    } else {
        min_len
    }
}

fn ref_max_binary_string(s: &str) -> String {
    let bytes: Vec<u8> = s.bytes().collect();
    let n = bytes.len();
    if n <= 1 {
        return s.to_string();
    }
    // Find first '0'
    let first_zero = bytes.iter().position(|&b| b == b'0');
    if first_zero.is_none() {
        return s.to_string();
    }
    let first_zero = first_zero.unwrap();
    // Count zeros from first_zero onwards
    let zero_count = bytes[first_zero..].iter().filter(|&&b| b == b'0').count();
    // Result: all 1s except one 0 at position (first_zero + zero_count - 1)
    let mut result = vec![b'1'; n];
    result[first_zero + zero_count - 1] = b'0';
    String::from_utf8(result).unwrap()
}

// ── Easy 1: Next Greater Element ─────────────────────────────────────

struct MonotonicNextGreater;

struct NextGreaterTest {
    nums: Vec<i32>,
}

impl Problem for MonotonicNextGreater {
    fn id(&self) -> &str {
        "monotonic_next_greater"
    }
    fn name(&self) -> &str {
        "Next Greater Element"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "For each element, find the next greater element to its right.\n\
         Return -1 if no greater element exists.\n\n\
         Input: Vec<i32>\n\
         Output: Vec<i32>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=20);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(1..=100)).collect();
                TestCase {
                    data: Box::new(NextGreaterTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NextGreaterTest>().unwrap();
        let expected = ref_next_greater(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::next_greater(&tracked_nums);
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

// ── Easy 2: Next Smaller Element ─────────────────────────────────────

struct MonotonicNextSmaller;

struct NextSmallerTest {
    nums: Vec<i32>,
}

impl Problem for MonotonicNextSmaller {
    fn id(&self) -> &str {
        "monotonic_next_smaller"
    }
    fn name(&self) -> &str {
        "Next Smaller Element"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "For each element, find the next smaller element to its right.\n\
         Return -1 if no smaller element exists.\n\n\
         Input: Vec<i32>\n\
         Output: Vec<i32>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=20);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(1..=100)).collect();
                TestCase {
                    data: Box::new(NextSmallerTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NextSmallerTest>().unwrap();
        let expected = ref_next_smaller(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::next_smaller(&tracked_nums);
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

// ── Easy 3: Daily Temperatures ───────────────────────────────────────

struct MonotonicDailyTemperatures;

struct DailyTempsTest {
    temps: Vec<i32>,
}

impl Problem for MonotonicDailyTemperatures {
    fn id(&self) -> &str {
        "monotonic_daily_temperatures"
    }
    fn name(&self) -> &str {
        "Daily Temperatures"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given daily temperatures, for each day find how many days you have to wait \
         until a warmer temperature. Return 0 if no warmer day exists.\n\n\
         Input: Vec<i32>\n\
         Output: Vec<i32>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=20);
                let temps: Vec<i32> = (0..n).map(|_| rng.random_range(60..=100)).collect();
                TestCase {
                    data: Box::new(DailyTempsTest { temps }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DailyTempsTest>().unwrap();
        let expected = ref_daily_temperatures(&t.temps);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_temps = track_slice(&t.temps, shared_log.clone());
        let actual = solutions::daily_temperatures(&tracked_temps);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("temps={:?}", t.temps),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 4: Stock Span ───────────────────────────────────────────────

struct MonotonicStockSpan;

struct StockSpanTest {
    prices: Vec<i32>,
}

impl Problem for MonotonicStockSpan {
    fn id(&self) -> &str {
        "monotonic_stock_span"
    }
    fn name(&self) -> &str {
        "Stock Span"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "The stock span for day i is the number of consecutive days up to and including i \
         where the price was <= price[i].\n\n\
         Input: Vec<i32>\n\
         Output: Vec<i32>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=20);
                let prices: Vec<i32> = (0..n).map(|_| rng.random_range(10..=200)).collect();
                TestCase {
                    data: Box::new(StockSpanTest { prices }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<StockSpanTest>().unwrap();
        let expected = ref_stock_span(&t.prices);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_prices = track_slice(&t.prices, shared_log.clone());
        let actual = solutions::stock_span(&tracked_prices);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("prices={:?}", t.prices),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 5: Sliding Window Maximum ───────────────────────────────────

struct MonotonicSlidingWindowMax;

struct SlidingMaxTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for MonotonicSlidingWindowMax {
    fn id(&self) -> &str {
        "monotonic_sliding_window_max"
    }
    fn name(&self) -> &str {
        "Sliding Window Maximum"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Return the maximum value in each sliding window of size k.\n\n\
         Input: (nums: Vec<i32>, k: usize)\n\
         Output: Vec<i32>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=25);
                let k = rng.random_range(1..=n);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                TestCase {
                    data: Box::new(SlidingMaxTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SlidingMaxTest>().unwrap();
        let expected = ref_sliding_window_max(&t.nums, t.k);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
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

// ── Medium 1: Largest Rectangle in Histogram ─────────────────────────

struct MonotonicLargestRectangle;

struct LargestRectTest {
    heights: Vec<i32>,
}

impl Problem for MonotonicLargestRectangle {
    fn id(&self) -> &str {
        "monotonic_largest_rectangle"
    }
    fn name(&self) -> &str {
        "Largest Rectangle in Histogram"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array of bar heights, find the largest rectangular area.\n\n\
         Input: Vec<i32>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=20);
                let heights: Vec<i32> = (0..n).map(|_| rng.random_range(0..=30)).collect();
                TestCase {
                    data: Box::new(LargestRectTest { heights }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LargestRectTest>().unwrap();
        let expected = ref_largest_rectangle(&t.heights);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_heights = track_slice(&t.heights, shared_log.clone());
        let actual = solutions::largest_rectangle(&tracked_heights);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("heights={:?}", t.heights),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 2: Maximal Rectangle in Binary Matrix ─────────────────────

struct MonotonicMaximalRectangle;

struct MaxRectTest {
    matrix: Vec<Vec<i32>>,
}

impl Problem for MonotonicMaximalRectangle {
    fn id(&self) -> &str {
        "monotonic_maximal_rectangle"
    }
    fn name(&self) -> &str {
        "Maximal Rectangle in Binary Matrix"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a binary matrix (0s and 1s), find the area of the largest rectangle \
         containing only 1s.\n\n\
         Input: Vec<Vec<i32>>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(2..=8);
                let cols = rng.random_range(2..=8);
                let matrix: Vec<Vec<i32>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(0..=1)).collect())
                    .collect();
                TestCase {
                    data: Box::new(MaxRectTest { matrix }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxRectTest>().unwrap();
        let expected = ref_maximal_rectangle(&t.matrix);
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
        let actual = solutions::maximal_rectangle(&tracked);
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

// ── Medium 3: Remove K Digits ────────────────────────────────────────

struct MonotonicRemoveKDigits;

struct RemoveKTest {
    num: String,
    k: usize,
}

impl Problem for MonotonicRemoveKDigits {
    fn id(&self) -> &str {
        "monotonic_remove_k_digits"
    }
    fn name(&self) -> &str {
        "Remove K Digits"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Remove k digits from the number string to make the smallest possible number.\n\
         Return as a string with no leading zeros (return \"0\" if result is empty).\n\n\
         Input: (num: String, k: usize)\n\
         Output: String"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(2..=12);
                let num: String = (0..len)
                    .map(|i| {
                        if i == 0 {
                            (b'1' + rng.random_range(0..9u8)) as char
                        } else {
                            (b'0' + rng.random_range(0..10u8)) as char
                        }
                    })
                    .collect();
                let k = rng.random_range(1..len);
                TestCase {
                    data: Box::new(RemoveKTest { num, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RemoveKTest>().unwrap();
        let expected = ref_remove_k_digits(&t.num, t.k);
        let actual = solutions::remove_k_digits(&t.num, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("num=\"{}\", k={}", t.num, t.k),
            expected: format!("\"{expected}\""),
            actual: format!("\"{actual}\""),
        }
    }
}

// ── Medium 4: Sum of Subarray Minimums ───────────────────────────────

struct MonotonicSumOfSubarrayMinimums;

struct SubMinTest {
    arr: Vec<i32>,
}

impl Problem for MonotonicSumOfSubarrayMinimums {
    fn id(&self) -> &str {
        "monotonic_sum_of_subarray_minimums"
    }
    fn name(&self) -> &str {
        "Sum of Subarray Minimums"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Return the sum of min(subarray) for all contiguous subarrays, modulo 10^9 + 7.\n\n\
         Input: Vec<i32>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let arr: Vec<i32> = (0..n).map(|_| rng.random_range(1..=100)).collect();
                TestCase {
                    data: Box::new(SubMinTest { arr }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SubMinTest>().unwrap();
        let expected = ref_sum_of_subarray_minimums(&t.arr);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::sum_of_subarray_minimums(&tracked_arr);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}", t.arr),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 5: Sliding Window Minimum ─────────────────────────────────

struct MonotonicSlidingWindowMin;

struct SlidingMinTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for MonotonicSlidingWindowMin {
    fn id(&self) -> &str {
        "monotonic_sliding_window_min"
    }
    fn name(&self) -> &str {
        "Sliding Window Minimum"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Return the minimum value in each sliding window of size k.\n\n\
         Input: (nums: Vec<i32>, k: usize)\n\
         Output: Vec<i32>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(5..=25);
                let k = rng.random_range(1..=n);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                TestCase {
                    data: Box::new(SlidingMinTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SlidingMinTest>().unwrap();
        let expected = ref_sliding_window_min(&t.nums, t.k);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sliding_window_min(&tracked_nums, t.k);
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

// ── Hard 1: Trapping Rain Water ──────────────────────────────────────

struct MonotonicTrappingRainWater;

struct TrapTest {
    height: Vec<i32>,
}

impl Problem for MonotonicTrappingRainWater {
    fn id(&self) -> &str {
        "monotonic_trapping_rain_water"
    }
    fn name(&self) -> &str {
        "Trapping Rain Water (Monotonic Stack)"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Compute trapped rain water using a monotonic stack approach.\n\n\
         Input: Vec<i32>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=25);
                let height: Vec<i32> = (0..n).map(|_| rng.random_range(0..=20)).collect();
                TestCase {
                    data: Box::new(TrapTest { height }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TrapTest>().unwrap();
        let expected = ref_trapping_rain_water(&t.height);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
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

// ── Hard 2: Maximum Width Ramp ───────────────────────────────────────

struct MonotonicMaxWidthRamp;

struct MaxRampTest {
    nums: Vec<i32>,
}

impl Problem for MonotonicMaxWidthRamp {
    fn id(&self) -> &str {
        "monotonic_max_width_ramp"
    }
    fn name(&self) -> &str {
        "Maximum Width Ramp"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find the maximum j - i such that nums[i] <= nums[j] where i < j.\n\
         Return 0 if no such pair exists.\n\n\
         Input: Vec<i32>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=25);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=50)).collect();
                TestCase {
                    data: Box::new(MaxRampTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxRampTest>().unwrap();
        let expected = ref_max_width_ramp(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::max_width_ramp(&tracked_nums);
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

// ── Hard 3: Sum of Subarray Ranges ───────────────────────────────────

struct MonotonicSumSubarrayRanges;

struct SubRangesTest {
    nums: Vec<i32>,
}

impl Problem for MonotonicSumSubarrayRanges {
    fn id(&self) -> &str {
        "monotonic_sum_subarray_ranges"
    }
    fn name(&self) -> &str {
        "Sum of Subarray Ranges"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "The range of a subarray is max - min. Return the sum of ranges \
         over all subarrays.\n\n\
         Input: Vec<i32>\n\
         Output: i64"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                TestCase {
                    data: Box::new(SubRangesTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SubRangesTest>().unwrap();
        let expected = ref_sum_subarray_ranges(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sum_subarray_ranges(&tracked_nums);
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

// ── Hard 4: Shortest Subarray with Sum >= K ──────────────────────────

struct MonotonicShortestSubarraySumK;

struct ShortestSubSumTest {
    nums: Vec<i32>,
    k: i32,
}

impl Problem for MonotonicShortestSubarraySumK {
    fn id(&self) -> &str {
        "monotonic_shortest_subarray_sum_k"
    }
    fn name(&self) -> &str {
        "Shortest Subarray with Sum >= K"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find the length of the shortest non-empty subarray with sum >= k.\n\
         Array may contain negative numbers. Return -1 if impossible.\n\n\
         Input: (nums: Vec<i32>, k: i32)\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=15);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=50)).collect();
                let k = rng.random_range(1..=100);
                TestCase {
                    data: Box::new(ShortestSubSumTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ShortestSubSumTest>().unwrap();
        let expected = ref_shortest_subarray_sum_k(&t.nums, t.k);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
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

// ── Hard 5: Maximum Binary String ────────────────────────────────────

struct MonotonicMaxBinaryString;

struct MaxBinStringTest {
    s: String,
}

impl Problem for MonotonicMaxBinaryString {
    fn id(&self) -> &str {
        "monotonic_max_binary_string"
    }
    fn name(&self) -> &str {
        "Maximum Binary String After Operations"
    }
    fn topic(&self) -> &str {
        "monotonic"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a binary string, you can apply operations:\n\
         - \"00\" -> \"10\"\n\
         - \"10\" -> \"01\"\n\
         Return the maximum binary string achievable.\n\n\
         Input: String\n\
         Output: String"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(2..=15);
                let s: String = (0..len)
                    .map(|_| {
                        if rng.random_range(0..2) == 0 {
                            '0'
                        } else {
                            '1'
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(MaxBinStringTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxBinStringTest>().unwrap();
        let expected = ref_max_binary_string(&t.s);
        let actual = solutions::max_binary_string(&t.s);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("\"{expected}\""),
            actual: format!("\"{actual}\""),
        }
    }
}
