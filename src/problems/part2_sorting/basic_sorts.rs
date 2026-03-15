use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part2_sorting::basic_sorts as solutions;
use crate::tracker::{track_slice, track_string, OperationLog, Tracked};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy
        Box::new(BubbleSort),
        Box::new(SelectionSort),
        Box::new(InsertionSort),
        Box::new(IsSorted),
        Box::new(SortColors),
        // Medium
        Box::new(SortByParity),
        Box::new(RelativeSort),
        Box::new(LargestNumber),
        Box::new(WiggleSort),
        Box::new(PancakeSort),
        // Hard
        Box::new(CountInversions),
        Box::new(HIndex),
        Box::new(CustomSortString),
        Box::new(MinimumSwaps),
        Box::new(SortNearlySorted),
    ]
}

// ── Shared test data types ─────────────────────────────────────────────

struct SortTest {
    nums: Vec<i32>,
}

struct TwoVecTest {
    arr1: Vec<i32>,
    arr2: Vec<i32>,
}

struct TwoStringTest {
    order: String,
    s: String,
}

struct VecWithKTest {
    nums: Vec<i32>,
    k: usize,
}

// ── Helper: run a tracked sort problem ─────────────────────────────────

/// Run a user's tracked sort function, compare result to std sort, and copy
/// the operation log out for visualization.
fn run_tracked_sort(
    test: &TestCase,
    log: &mut OperationLog,
    sort_fn: fn(&mut [Tracked<i32>]),
) -> SolutionResult {
    let t = test.data.downcast_ref::<SortTest>().unwrap();
    let mut expected = t.nums.clone();
    expected.sort();

    let shared_log = Rc::new(RefCell::new(OperationLog::new()));
    let mut tracked = track_slice(&t.nums, shared_log.clone());

    sort_fn(&mut tracked);

    let actual: Vec<i32> = tracked.iter().map(|t| t.value).collect();

    let inner = shared_log.borrow();
    for op in inner.operations() {
        log.record(op.clone());
    }

    SolutionResult {
        is_correct: actual == expected,
        input_description: format!("{:?}", t.nums),
        expected: format!("{expected:?}"),
        actual: format!("{actual:?}"),
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 1: Bubble Sort
// ═══════════════════════════════════════════════════════════════════════

struct BubbleSort;

impl Problem for BubbleSort {
    fn id(&self) -> &str {
        "basic_sorts_bubble"
    }
    fn name(&self) -> &str {
        "Bubble Sort"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Implement bubble sort on a tracked slice of integers.\n\n\
         Use `tracked_swap(arr, i, j)` to swap elements.\n\
         Comparisons via `<`, `>`, `<=`, `>=`, `==` are recorded automatically.\n\n\
         Constraints:\n\
         - 0 <= arr.len() <= 200\n\
         - -1000 <= arr[i] <= 1000\n\n\
         Expected time complexity: O(n^2)\n\
         Expected space complexity: O(1)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-1000..=1000)).collect();
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        run_tracked_sort(test, log, solutions::bubble_sort)
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 2: Selection Sort
// ═══════════════════════════════════════════════════════════════════════

struct SelectionSort;

impl Problem for SelectionSort {
    fn id(&self) -> &str {
        "basic_sorts_selection"
    }
    fn name(&self) -> &str {
        "Selection Sort"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Implement selection sort on a tracked slice of integers.\n\n\
         Use `tracked_swap(arr, i, j)` to swap elements.\n\
         Comparisons via `<`, `>`, `<=`, `>=`, `==` are recorded automatically.\n\n\
         Constraints:\n\
         - 0 <= arr.len() <= 200\n\
         - -1000 <= arr[i] <= 1000\n\n\
         Expected time complexity: O(n^2)\n\
         Expected space complexity: O(1)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-1000..=1000)).collect();
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        run_tracked_sort(test, log, solutions::selection_sort)
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 3: Insertion Sort
// ═══════════════════════════════════════════════════════════════════════

struct InsertionSort;

impl Problem for InsertionSort {
    fn id(&self) -> &str {
        "basic_sorts_insertion"
    }
    fn name(&self) -> &str {
        "Insertion Sort"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Implement insertion sort on a tracked slice of integers.\n\n\
         Use `tracked_swap(arr, i, j)` to swap elements.\n\
         Comparisons via `<`, `>`, `<=`, `>=`, `==` are recorded automatically.\n\n\
         Constraints:\n\
         - 0 <= arr.len() <= 200\n\
         - -1000 <= arr[i] <= 1000\n\n\
         Expected time complexity: O(n^2)\n\
         Expected space complexity: O(1)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-1000..=1000)).collect();
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        run_tracked_sort(test, log, solutions::insertion_sort)
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 4: Is Sorted
// ═══════════════════════════════════════════════════════════════════════

struct IsSorted;

impl Problem for IsSorted {
    fn id(&self) -> &str {
        "basic_sorts_is_sorted"
    }
    fn name(&self) -> &str {
        "Is Sorted"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Check if an array is sorted in non-decreasing order.\n\n\
         Return `true` if sorted, `false` otherwise.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 1000\n\
         - -10000 <= nums[i] <= 10000\n\n\
         An empty array or single-element array is considered sorted."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let n = rng.random_range(0..=30);
                let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                // Make ~half the tests already sorted
                if i % 2 == 0 {
                    nums.sort();
                }
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = t.nums.windows(2).all(|w| w[0] <= w[1]);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::is_sorted(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("{:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 5: Sort Colors (Dutch National Flag)
// ═══════════════════════════════════════════════════════════════════════

struct SortColors;

impl Problem for SortColors {
    fn id(&self) -> &str {
        "basic_sorts_sort_colors"
    }
    fn name(&self) -> &str {
        "Sort Colors (Dutch National Flag)"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array containing only 0s, 1s, and 2s, sort it in-place.\n\n\
         This is the Dutch National Flag problem. Try to solve it in one pass \
         with O(1) extra space.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 300\n\
         - nums[i] is 0, 1, or 2"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=2)).collect();
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let mut expected = t.nums.clone();
        expected.sort();
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sort_colors(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("{:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 1: Sort by Parity
// ═══════════════════════════════════════════════════════════════════════

struct SortByParity;

impl Problem for SortByParity {
    fn id(&self) -> &str {
        "basic_sorts_sort_by_parity"
    }
    fn name(&self) -> &str {
        "Sort by Parity"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an integer array `nums`, rearrange it so that all even numbers come \
         before all odd numbers.\n\n\
         The relative order among even numbers and among odd numbers does not matter.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 500\n\
         - 0 <= nums[i] <= 5000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sort_by_parity(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }

        // Validate: all evens come before all odds, and same multiset
        let mut sorted_input = t.nums.clone();
        sorted_input.sort();
        let mut sorted_actual = actual.clone();
        sorted_actual.sort();
        let same_elements = sorted_input == sorted_actual;

        let first_odd = actual
            .iter()
            .position(|x| x % 2 != 0)
            .unwrap_or(actual.len());
        let last_even = actual
            .iter()
            .rposition(|x| x % 2 == 0)
            .map(|i| i + 1)
            .unwrap_or(0);
        let valid_order = last_even <= first_odd || first_odd == actual.len();

        SolutionResult {
            is_correct: same_elements && valid_order,
            input_description: format!("{:?}", t.nums),
            expected: "evens before odds".to_string(),
            actual: format!("{actual:?}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 2: Relative Sort Array
// ═══════════════════════════════════════════════════════════════════════

struct RelativeSort;

impl Problem for RelativeSort {
    fn id(&self) -> &str {
        "basic_sorts_relative_sort"
    }
    fn name(&self) -> &str {
        "Relative Sort Array"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Sort `arr1` such that the relative ordering of items in `arr1` follows \
         the order defined by `arr2`.\n\n\
         Elements that don't appear in `arr2` should be placed at the end of `arr1` \
         in ascending order.\n\n\
         Constraints:\n\
         - arr2 is a subset of unique elements from arr1\n\
         - 1 <= arr1.len() <= 1000\n\
         - 0 <= arr2.len() <= arr1.len()\n\
         - 0 <= arr1[i], arr2[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n1 = rng.random_range(1..=20);
                let arr1: Vec<i32> = (0..n1).map(|_| rng.random_range(0..=20)).collect();
                // arr2 is a subset of unique values from arr1
                let mut unique: Vec<i32> = arr1
                    .iter()
                    .copied()
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();
                // Shuffle unique values
                for i in (1..unique.len()).rev() {
                    let j = rng.random_range(0..=i);
                    unique.swap(i, j);
                }
                let n2 = rng.random_range(0..=unique.len());
                let arr2: Vec<i32> = unique[..n2].to_vec();
                TestCase {
                    data: Box::new(TwoVecTest { arr1, arr2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TwoVecTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_relative_sort(&t.arr1, &t.arr2);
        let tracked_arr1 = track_slice(&t.arr1, shared_log.clone());
        let tracked_arr2 = track_slice(&t.arr2, shared_log.clone());
        let actual = solutions::relative_sort(&tracked_arr1, &tracked_arr2);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("arr1={:?}, arr2={:?}", t.arr1, t.arr2),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_relative_sort(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let order_map: HashMap<i32, usize> = arr2.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let mut result = arr1.to_vec();
    result.sort_by(|a, b| match (order_map.get(a), order_map.get(b)) {
        (Some(ia), Some(ib)) => ia.cmp(ib),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.cmp(b),
    });
    result
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 3: Largest Number
// ═══════════════════════════════════════════════════════════════════════

struct LargestNumber;

impl Problem for LargestNumber {
    fn id(&self) -> &str {
        "basic_sorts_largest_number"
    }
    fn name(&self) -> &str {
        "Largest Number"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a list of non-negative integers `nums`, arrange them such that they \
         form the largest number and return it as a string.\n\n\
         The result may be very large, so return a string instead of an integer.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - 0 <= nums[i] <= 10^9\n\n\
         Example: [10, 2] -> \"210\"\n\
         Example: [3, 30, 34, 5, 9] -> \"9534330\""
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=15);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=999)).collect();
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_largest_number(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::largest_number(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("{:?}", t.nums),
            expected: expected.clone(),
            actual: actual.clone(),
        }
    }
}

fn ref_largest_number(nums: &[i32]) -> String {
    let mut strs: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
    strs.sort_by(|a, b| {
        let ab = format!("{a}{b}");
        let ba = format!("{b}{a}");
        ba.cmp(&ab)
    });
    let result = strs.join("");
    // Handle all-zeros edge case
    if result.starts_with('0') {
        "0".to_string()
    } else {
        result
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 4: Wiggle Sort
// ═══════════════════════════════════════════════════════════════════════

struct WiggleSort;

impl Problem for WiggleSort {
    fn id(&self) -> &str {
        "basic_sorts_wiggle_sort"
    }
    fn name(&self) -> &str {
        "Wiggle Sort"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Rearrange `nums` into wiggle order: nums[0] <= nums[1] >= nums[2] <= nums[3] ...\n\n\
         Multiple valid answers may exist; any valid wiggle arrangement is accepted.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 500\n\
         - 0 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::wiggle_sort(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }

        // Validate wiggle property
        let valid_wiggle = actual.len() == t.nums.len()
            && is_valid_wiggle(&actual)
            && same_multiset(&t.nums, &actual);

        SolutionResult {
            is_correct: valid_wiggle,
            input_description: format!("{:?}", t.nums),
            expected: "valid wiggle: a[0]<=a[1]>=a[2]<=a[3]...".to_string(),
            actual: format!("{actual:?}"),
        }
    }
}

fn is_valid_wiggle(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if i % 2 == 1 {
            // Odd index: should be >= previous
            if arr[i] < arr[i - 1] {
                return false;
            }
        } else {
            // Even index: should be <= previous
            if arr[i] > arr[i - 1] {
                return false;
            }
        }
    }
    true
}

fn same_multiset(a: &[i32], b: &[i32]) -> bool {
    let mut sa = a.to_vec();
    let mut sb = b.to_vec();
    sa.sort();
    sb.sort();
    sa == sb
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 5: Pancake Sort
// ═══════════════════════════════════════════════════════════════════════

struct PancakeSort;

impl Problem for PancakeSort {
    fn id(&self) -> &str {
        "basic_sorts_pancake_sort"
    }
    fn name(&self) -> &str {
        "Pancake Sort"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Sort the array using only \"pancake flips\".\n\n\
         A pancake flip of length `k` reverses the first `k` elements of the array.\n\
         Repeatedly apply flips until the array is sorted.\n\n\
         Return the sorted array. (The flip sequence is not checked, only the result.)\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - All values in nums are unique\n\
         - nums is a permutation of 1..=n"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let mut nums: Vec<i32> = (1..=n as i32).collect();
                // Fisher-Yates shuffle
                for i in (1..nums.len()).rev() {
                    let j = rng.random_range(0..=i);
                    nums.swap(i, j);
                }
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let mut expected = t.nums.clone();
        expected.sort();
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::pancake_sort(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("{:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 1: Count Inversions
// ═══════════════════════════════════════════════════════════════════════

struct CountInversions;

impl Problem for CountInversions {
    fn id(&self) -> &str {
        "basic_sorts_count_inversions"
    }
    fn name(&self) -> &str {
        "Count Inversions"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Count the number of inversions in an array.\n\n\
         An inversion is a pair (i, j) where i < j but nums[i] > nums[j].\n\n\
         The number of inversions measures how far the array is from being sorted. \
         A sorted array has 0 inversions; a reverse-sorted array has n*(n-1)/2.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 5000\n\
         - -10^9 <= nums[i] <= 10^9\n\n\
         Hint: A brute-force O(n^2) solution works but a modified merge sort gives O(n log n)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=50);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-1000..=1000)).collect();
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_count_inversions(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::count_inversions(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("{:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_count_inversions(nums: &[i32]) -> i64 {
    let mut count: i64 = 0;
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] > nums[j] {
                count += 1;
            }
        }
    }
    count
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 2: H-Index
// ═══════════════════════════════════════════════════════════════════════

struct HIndex;

impl Problem for HIndex {
    fn id(&self) -> &str {
        "basic_sorts_h_index"
    }
    fn name(&self) -> &str {
        "H-Index"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array of integers `citations` where citations[i] is the number of \
         citations a researcher received for their i-th paper, return the researcher's \
         h-index.\n\n\
         The h-index is the largest value `h` such that the researcher has at least `h` \
         papers with >= `h` citations each.\n\n\
         Constraints:\n\
         - 0 <= citations.len() <= 5000\n\
         - 0 <= citations[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=20)).collect();
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_h_index(&t.nums);
        let tracked_citations = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::h_index(&tracked_citations);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("{:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_h_index(citations: &[i32]) -> i32 {
    let mut sorted = citations.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let mut h = 0;
    for (i, &c) in sorted.iter().enumerate() {
        if c >= (i as i32 + 1) {
            h = i as i32 + 1;
        } else {
            break;
        }
    }
    h
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 3: Custom Sort String
// ═══════════════════════════════════════════════════════════════════════

struct CustomSortString;

impl Problem for CustomSortString {
    fn id(&self) -> &str {
        "basic_sorts_custom_sort_string"
    }
    fn name(&self) -> &str {
        "Custom Sort String"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "You are given two strings `order` and `s`.\n\n\
         All characters of `order` are unique and represent a custom ordering of \
         lowercase English letters.\n\n\
         Permute the characters of `s` so that they match the order that `order` was \
         sorted. Characters not in `order` can be placed at any position but should \
         maintain relative order among themselves.\n\n\
         Constraints:\n\
         - 1 <= order.len() <= 26\n\
         - 1 <= s.len() <= 200\n\
         - order and s consist of lowercase English letters\n\
         - All characters of order are unique"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                // Generate random order (subset of a-z, shuffled)
                let order_len = rng.random_range(1..=26);
                let mut chars: Vec<u8> = (b'a'..=b'z').collect();
                for i in (1..chars.len()).rev() {
                    let j = rng.random_range(0..=i);
                    chars.swap(i, j);
                }
                let order: String = chars[..order_len].iter().map(|&c| c as char).collect();

                // Generate random string s
                let s_len = rng.random_range(1..=20);
                let s: String = (0..s_len)
                    .map(|_| (b'a' + rng.random_range(0..26u8)) as char)
                    .collect();

                TestCase {
                    data: Box::new(TwoStringTest { order, s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TwoStringTest>().unwrap();
        let expected = ref_custom_sort_string(&t.order, &t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_order = track_string(&t.order, shared_log.clone());
        let tracked_s = track_string(&t.s, shared_log.clone());
        let actual = solutions::custom_sort_string(&tracked_order, &tracked_s);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }

        // Validate: same character multiset and respects order
        let valid = same_char_multiset(&expected, &actual) && respects_order(&t.order, &actual);

        SolutionResult {
            is_correct: valid,
            input_description: format!("order=\"{}\", s=\"{}\"", t.order, t.s),
            expected: expected.clone(),
            actual: actual.clone(),
        }
    }
}

fn ref_custom_sort_string(order: &str, s: &str) -> String {
    let priority: HashMap<char, usize> = order.chars().enumerate().map(|(i, c)| (c, i)).collect();
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by_key(|c| priority.get(c).copied().unwrap_or(order.len()));
    chars.into_iter().collect()
}

fn same_char_multiset(a: &str, b: &str) -> bool {
    let mut ac: Vec<char> = a.chars().collect();
    let mut bc: Vec<char> = b.chars().collect();
    ac.sort();
    bc.sort();
    ac == bc
}

fn respects_order(order: &str, s: &str) -> bool {
    let priority: HashMap<char, usize> = order.chars().enumerate().map(|(i, c)| (c, i)).collect();
    let chars: Vec<char> = s.chars().collect();
    for i in 1..chars.len() {
        let p_prev = priority.get(&chars[i - 1]).copied();
        let p_curr = priority.get(&chars[i]).copied();
        match (p_prev, p_curr) {
            (Some(a), Some(b)) if a > b => return false,
            _ => {}
        }
    }
    true
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 4: Minimum Swaps to Sort
// ═══════════════════════════════════════════════════════════════════════

struct MinimumSwaps;

impl Problem for MinimumSwaps {
    fn id(&self) -> &str {
        "basic_sorts_minimum_swaps"
    }
    fn name(&self) -> &str {
        "Minimum Swaps to Sort"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a permutation of integers from 1 to n, find the minimum number of \
         swaps required to sort the array.\n\n\
         A swap exchanges any two elements in the array.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 1000\n\
         - nums is a permutation of 1..=n\n\n\
         Hint: Think about cycles in the permutation. Each cycle of length k requires \
         (k - 1) swaps."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let mut nums: Vec<i32> = (1..=n as i32).collect();
                // Fisher-Yates shuffle
                for i in (1..nums.len()).rev() {
                    let j = rng.random_range(0..=i);
                    nums.swap(i, j);
                }
                TestCase {
                    data: Box::new(SortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_minimum_swaps(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::minimum_swaps(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("{:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_minimum_swaps(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut visited = vec![false; n];
    let mut swaps = 0;
    for i in 0..n {
        if visited[i] || nums[i] == (i as i32 + 1) {
            continue;
        }
        let mut cycle_len = 0;
        let mut j = i;
        while !visited[j] {
            visited[j] = true;
            j = (nums[j] - 1) as usize;
            cycle_len += 1;
        }
        swaps += cycle_len - 1;
    }
    swaps
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 5: Sort Nearly Sorted (K-Sorted) Array
// ═══════════════════════════════════════════════════════════════════════

struct SortNearlySorted;

impl Problem for SortNearlySorted {
    fn id(&self) -> &str {
        "basic_sorts_sort_nearly_sorted"
    }
    fn name(&self) -> &str {
        "Sort Nearly Sorted (K-Sorted) Array"
    }
    fn topic(&self) -> &str {
        "basic_sorts"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a k-sorted array where each element is at most `k` positions away \
         from its sorted position, sort the array efficiently.\n\n\
         A k-sorted array means for every element at index i, its correct sorted \
         position is in the range [i-k, i+k].\n\n\
         Return the fully sorted array.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 1000\n\
         - 0 <= k <= nums.len()\n\
         - -10000 <= nums[i] <= 10000\n\n\
         Hint: A min-heap of size (k+1) can sort this in O(n log k) time, \
         which is better than O(n log n) for small k."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n: usize = rng.random_range(0..=40);
                let k: usize = if n == 0 {
                    0
                } else {
                    rng.random_range(1..=n.min(10))
                };
                // Generate a k-sorted array
                let mut sorted: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                sorted.sort();
                // Perturb positions by at most k
                let mut nums = sorted.clone();
                for i in 0..n {
                    let lo = i.saturating_sub(k);
                    let hi = (i + k).min(n - 1);
                    let j = rng.random_range(lo..=hi);
                    nums.swap(i, j);
                }
                TestCase {
                    data: Box::new(VecWithKTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<VecWithKTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let mut expected = t.nums.clone();
        expected.sort();
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sort_nearly_sorted(&tracked_nums, t.k);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}
