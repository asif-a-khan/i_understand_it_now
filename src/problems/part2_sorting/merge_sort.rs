use rand::Rng;
use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part2_sorting::merge_sort as solutions;
use crate::tracker::{track_slice, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(MergeSortBasic),
        Box::new(MergeTwoSorted),
        Box::new(CountInversions),
        Box::new(SortLinkedList),
        Box::new(MergeKSortedArrays),
        Box::new(SortArray),
        Box::new(SmallestRange),
        Box::new(CountRangeSum),
        Box::new(ReversePairs),
        Box::new(SortByFrequency),
        Box::new(ExternalSort),
        Box::new(CountSmallerAfter),
        Box::new(MaxSumAfterKOps),
        Box::new(MedianStream),
        Box::new(NthElement),
    ]
}

// ── Easy 1: Merge Sort Basic ──────────────────────────────────────────

struct MergeSortBasic;

struct MergeSortBasicTest {
    nums: Vec<i32>,
}

impl Problem for MergeSortBasic {
    fn id(&self) -> &str {
        "merge_sort_basic"
    }
    fn name(&self) -> &str {
        "Merge Sort (Tracked)"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Implement merge sort on a slice of Tracked<i32>.\n\n\
         The function signature is:\n\
         `fn merge_sort_basic(nums: &mut [Tracked<i32>])`\n\n\
         Sort the slice in ascending order using the merge sort algorithm.\n\
         Tracked<i32> supports comparison operators (<=, >=, <, >, ==) and Clone.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=50);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-1000..=1000)).collect();
                TestCase {
                    data: Box::new(MergeSortBasicTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MergeSortBasicTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();

        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let mut tracked = track_slice(&t.nums, shared_log.clone());
        solutions::merge_sort_basic(&mut tracked);
        let actual: Vec<i32> = tracked.iter().map(|t| t.value).collect();
        let inner = shared_log.borrow();
        for op in inner.operations() {
            log.record(op.clone());
        }

        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 2: Merge Two Sorted Arrays ───────────────────────────────────

struct MergeTwoSorted;

struct MergeTwoSortedTest {
    a: Vec<i32>,
    b: Vec<i32>,
}

impl Problem for MergeTwoSorted {
    fn id(&self) -> &str {
        "merge_sort_merge_two_sorted"
    }
    fn name(&self) -> &str {
        "Merge Two Sorted Arrays"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given two sorted arrays `a` and `b`, merge them into a single sorted array.\n\n\
         Return the merged array.\n\n\
         Constraints:\n\
         - 0 <= a.len(), b.len() <= 50\n\
         - Both arrays are sorted in ascending order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let na = rng.random_range(0..=20);
                let nb = rng.random_range(0..=20);
                let mut a: Vec<i32> = (0..na).map(|_| rng.random_range(-100..=100)).collect();
                let mut b: Vec<i32> = (0..nb).map(|_| rng.random_range(-100..=100)).collect();
                a.sort();
                b.sort();
                TestCase {
                    data: Box::new(MergeTwoSortedTest { a, b }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MergeTwoSortedTest>().unwrap();
        let expected = ref_merge_two_sorted(&t.a, &t.b);
        let actual = solutions::merge_two_sorted(&t.a, &t.b);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("a={:?}, b={:?}", t.a, t.b),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_merge_two_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}

// ── Easy 3: Count Inversions ──────────────────────────────────────────

struct CountInversions;

struct CountInversionsTest {
    nums: Vec<i32>,
}

impl Problem for CountInversions {
    fn id(&self) -> &str {
        "merge_sort_count_inversions"
    }
    fn name(&self) -> &str {
        "Count Inversions"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array of integers, count the number of inversions.\n\n\
         An inversion is a pair (i, j) where i < j but nums[i] > nums[j].\n\n\
         Use a merge sort based approach for O(n log n).\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=40);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(CountInversionsTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountInversionsTest>().unwrap();
        let expected = ref_count_inversions(&t.nums);
        let actual = solutions::count_inversions(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_count_inversions(nums: &[i32]) -> i64 {
    if nums.len() <= 1 {
        return 0;
    }
    let mut arr = nums.to_vec();
    ref_count_inversions_helper(&mut arr)
}

fn ref_count_inversions_helper(arr: &mut [i32]) -> i64 {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }
    let mid = n / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    let mut count =
        ref_count_inversions_helper(&mut left) + ref_count_inversions_helper(&mut right);
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
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
    count
}

// ── Easy 4: Sort Linked List (as Vec) ─────────────────────────────────

struct SortLinkedList;

struct SortLinkedListTest {
    nums: Vec<i32>,
}

impl Problem for SortLinkedList {
    fn id(&self) -> &str {
        "merge_sort_sort_linked_list"
    }
    fn name(&self) -> &str {
        "Sort a Linked List (as Vec)"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Sort a linked list using merge sort.\n\n\
         The linked list is represented as a Vec<i32>. Return a new sorted Vec<i32>.\n\n\
         Implement the merge sort algorithm: split the list in half, recursively sort \
         each half, then merge them.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=40);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-500..=500)).collect();
                TestCase {
                    data: Box::new(SortLinkedListTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortLinkedListTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();
        let actual = solutions::sort_linked_list(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 5: Merge K Sorted Arrays ─────────────────────────────────────

struct MergeKSortedArrays;

struct MergeKSortedTest {
    arrays: Vec<Vec<i32>>,
}

impl Problem for MergeKSortedArrays {
    fn id(&self) -> &str {
        "merge_sort_merge_sorted_arrays"
    }
    fn name(&self) -> &str {
        "Merge K Sorted Arrays"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given k sorted arrays, merge them into a single sorted array.\n\n\
         Return the merged result.\n\n\
         Constraints:\n\
         - 1 <= k <= 10\n\
         - 0 <= arrays[i].len() <= 20\n\
         - Each array is sorted in ascending order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let k = rng.random_range(1..=8);
                let arrays: Vec<Vec<i32>> = (0..k)
                    .map(|_| {
                        let n = rng.random_range(0..=15);
                        let mut v: Vec<i32> =
                            (0..n).map(|_| rng.random_range(-100..=100)).collect();
                        v.sort();
                        v
                    })
                    .collect();
                TestCase {
                    data: Box::new(MergeKSortedTest { arrays }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MergeKSortedTest>().unwrap();
        let mut expected: Vec<i32> = t.arrays.iter().flatten().copied().collect();
        expected.sort();
        let actual = solutions::merge_sorted_arrays(&t.arrays);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("arrays={:?}", t.arrays),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 1: Sort Array ──────────────────────────────────────────────

struct SortArray;

struct SortArrayTest {
    nums: Vec<i32>,
}

impl Problem for SortArray {
    fn id(&self) -> &str {
        "merge_sort_sort_array"
    }
    fn name(&self) -> &str {
        "Sort Array (Merge Sort)"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Sort an array using merge sort (non-tracked). Return a new sorted Vec<i32>.\n\n\
         Do not use the built-in sort. Implement merge sort from scratch.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 200\n\
         - -10000 <= nums[i] <= 10000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=100);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-10000..=10000)).collect();
                TestCase {
                    data: Box::new(SortArrayTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortArrayTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();
        let actual = solutions::sort_array(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 2: Smallest Range Covering Elements from K Lists ───────────

struct SmallestRange;

struct SmallestRangeTest {
    lists: Vec<Vec<i32>>,
}

impl Problem for SmallestRange {
    fn id(&self) -> &str {
        "merge_sort_smallest_range"
    }
    fn name(&self) -> &str {
        "Smallest Range Covering K Lists"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given k sorted lists of integers, find the smallest range [a, b] that includes \
         at least one number from each of the k lists.\n\n\
         Return (a, b) where a <= b. If multiple ranges have the same size, return the one \
         with the smallest `a`.\n\n\
         Constraints:\n\
         - 1 <= k <= 5\n\
         - 1 <= lists[i].len() <= 15\n\
         - Each list is sorted in ascending order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let k = rng.random_range(2..=5);
                let lists: Vec<Vec<i32>> = (0..k)
                    .map(|_| {
                        let n = rng.random_range(1..=10);
                        let mut v: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                        v.sort();
                        v.dedup();
                        if v.is_empty() {
                            v.push(rng.random_range(-50..=50));
                        }
                        v
                    })
                    .collect();
                TestCase {
                    data: Box::new(SmallestRangeTest { lists }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SmallestRangeTest>().unwrap();
        let expected = ref_smallest_range(&t.lists);
        let actual = solutions::smallest_range(&t.lists);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("lists={:?}", t.lists),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_smallest_range(lists: &[Vec<i32>]) -> (i32, i32) {
    use std::cmp::Reverse;
    // Min-heap: (value, list_index, element_index)
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
    let mut cur_max = i32::MIN;
    for (i, list) in lists.iter().enumerate() {
        if !list.is_empty() {
            heap.push(Reverse((list[0], i, 0)));
            cur_max = cur_max.max(list[0]);
        }
    }
    let mut best = (i32::MIN / 2, i32::MAX / 2);
    while heap.len() == lists.len() {
        let Reverse((val, li, ei)) = heap.pop().unwrap();
        if cur_max - val < best.1 - best.0 || (cur_max - val == best.1 - best.0 && val < best.0) {
            best = (val, cur_max);
        }
        if ei + 1 < lists[li].len() {
            let next_val = lists[li][ei + 1];
            cur_max = cur_max.max(next_val);
            heap.push(Reverse((next_val, li, ei + 1)));
        } else {
            break;
        }
    }
    best
}

// ── Medium 3: Count Range Sum ─────────────────────────────────────────

struct CountRangeSum;

struct CountRangeSumTest {
    nums: Vec<i32>,
    lower: i32,
    upper: i32,
}

impl Problem for CountRangeSum {
    fn id(&self) -> &str {
        "merge_sort_count_range_sum"
    }
    fn name(&self) -> &str {
        "Count Range Sum"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an integer array `nums` and two integers `lower` and `upper`, return the \
         number of range sums that lie in [lower, upper].\n\n\
         A range sum S(i, j) = nums[i] + nums[i+1] + ... + nums[j] for 0 <= i <= j < n.\n\n\
         Hint: Use prefix sums and merge sort.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 50\n\
         - -100 <= nums[i] <= 100\n\
         - lower <= upper"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                let a = rng.random_range(-200..=200);
                let b = rng.random_range(-200..=200);
                let (lower, upper) = if a <= b { (a, b) } else { (b, a) };
                TestCase {
                    data: Box::new(CountRangeSumTest { nums, lower, upper }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountRangeSumTest>().unwrap();
        let expected = ref_count_range_sum(&t.nums, t.lower, t.upper);
        let actual = solutions::count_range_sum(&t.nums, t.lower, t.upper);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}, lower={}, upper={}", t.nums, t.lower, t.upper),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_count_range_sum(nums: &[i32], lower: i32, upper: i32) -> i32 {
    // Brute force for reference
    let n = nums.len();
    let mut count = 0i32;
    for i in 0..n {
        let mut sum: i64 = 0;
        for &num in nums.iter().take(n).skip(i) {
            sum += num as i64;
            if sum >= lower as i64 && sum <= upper as i64 {
                count += 1;
            }
        }
    }
    count
}

// ── Medium 4: Reverse Pairs ───────────────────────────────────────────

struct ReversePairs;

struct ReversePairsTest {
    nums: Vec<i32>,
}

impl Problem for ReversePairs {
    fn id(&self) -> &str {
        "merge_sort_reverse_pairs"
    }
    fn name(&self) -> &str {
        "Reverse Pairs"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array `nums`, return the number of reverse pairs.\n\n\
         A reverse pair is a pair (i, j) where i < j and nums[i] > 2 * nums[j].\n\n\
         Hint: Use a modified merge sort.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=50);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-500..=500)).collect();
                TestCase {
                    data: Box::new(ReversePairsTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReversePairsTest>().unwrap();
        let expected = ref_reverse_pairs(&t.nums);
        let actual = solutions::reverse_pairs(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
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
        for j in (i + 1)..n {
            if nums[i] as i64 > 2 * nums[j] as i64 {
                count += 1;
            }
        }
    }
    count
}

// ── Medium 5: Sort by Frequency ───────────────────────────────────────

struct SortByFrequency;

struct SortByFrequencyTest {
    nums: Vec<i32>,
}

impl Problem for SortByFrequency {
    fn id(&self) -> &str {
        "merge_sort_sort_by_frequency"
    }
    fn name(&self) -> &str {
        "Sort by Frequency"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Sort array elements by frequency in descending order (most frequent first).\n\n\
         If two elements have the same frequency, the smaller value comes first.\n\n\
         Return the sorted array.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=40);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
                TestCase {
                    data: Box::new(SortByFrequencyTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortByFrequencyTest>().unwrap();
        let expected = ref_sort_by_frequency(&t.nums);
        let actual = solutions::sort_by_frequency(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_sort_by_frequency(nums: &[i32]) -> Vec<i32> {
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &n in nums {
        *freq.entry(n).or_insert(0) += 1;
    }
    let mut result = nums.to_vec();
    result.sort_by(|a, b| {
        let fa = freq[a];
        let fb = freq[b];
        fb.cmp(&fa).then(a.cmp(b))
    });
    result
}

// ── Hard 1: External Sort Simulation ──────────────────────────────────

struct ExternalSort;

struct ExternalSortTest {
    nums: Vec<i32>,
    chunk_size: usize,
}

impl Problem for ExternalSort {
    fn id(&self) -> &str {
        "merge_sort_external_sort"
    }
    fn name(&self) -> &str {
        "External Sort Simulation"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Simulate external sort: split the input into chunks of size k, sort each chunk \
         individually, then merge all sorted chunks into one sorted array.\n\n\
         Input: (nums: Vec<i32>, k: usize) where k is the chunk size.\n\
         Return the fully sorted array.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - 1 <= k <= 20"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=60);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-500..=500)).collect();
                let chunk_size = rng.random_range(1..=15);
                TestCase {
                    data: Box::new(ExternalSortTest { nums, chunk_size }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ExternalSortTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();
        let actual = solutions::external_sort(&t.nums, t.chunk_size);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}, k={}", t.nums, t.chunk_size),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 2: Count Smaller Elements to the Right ───────────────────────

struct CountSmallerAfter;

struct CountSmallerAfterTest {
    nums: Vec<i32>,
}

impl Problem for CountSmallerAfter {
    fn id(&self) -> &str {
        "merge_sort_count_smaller_after"
    }
    fn name(&self) -> &str {
        "Count Smaller After Self"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an integer array `nums`, return a new array `counts` where `counts[i]` is \
         the number of elements to the right of `nums[i]` that are strictly smaller.\n\n\
         Hint: Use merge sort on index-value pairs.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=50);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-500..=500)).collect();
                TestCase {
                    data: Box::new(CountSmallerAfterTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountSmallerAfterTest>().unwrap();
        let expected = ref_count_smaller_after(&t.nums);
        let actual = solutions::count_smaller_after(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
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

// ── Hard 3: Maximize Sum After K Negations ────────────────────────────

struct MaxSumAfterKOps;

struct MaxSumAfterKOpsTest {
    nums: Vec<i32>,
    k: i32,
}

impl Problem for MaxSumAfterKOps {
    fn id(&self) -> &str {
        "merge_sort_max_sum_after_k_ops"
    }
    fn name(&self) -> &str {
        "Maximize Sum After K Negations"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array `nums` and integer `k`, you may negate any element up to `k` times \
         (the same element can be negated multiple times). Maximize the sum of the array.\n\n\
         Hint: Sort the array, negate the smallest (most negative) elements first.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 50\n\
         - -100 <= nums[i] <= 100\n\
         - 1 <= k <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                let k = rng.random_range(1..=50);
                TestCase {
                    data: Box::new(MaxSumAfterKOpsTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxSumAfterKOpsTest>().unwrap();
        let expected = ref_max_sum_after_k_ops(&t.nums, t.k);
        let actual = solutions::max_sum_after_k_ops(&t.nums, t.k);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_sum_after_k_ops(nums: &[i32], k: i32) -> i32 {
    let mut arr = nums.to_vec();
    arr.sort();
    let mut remaining = k;
    for item in arr.iter_mut() {
        if remaining > 0 && *item < 0 {
            *item = -*item;
            remaining -= 1;
        }
    }
    if remaining % 2 == 1 {
        arr.sort();
        arr[0] = -arr[0];
    }
    arr.iter().sum()
}

// ── Hard 4: Median of a Data Stream ───────────────────────────────────

struct MedianStream;

struct MedianStreamTest {
    nums: Vec<i32>,
}

impl Problem for MedianStream {
    fn id(&self) -> &str {
        "merge_sort_median_stream"
    }
    fn name(&self) -> &str {
        "Find Median from Data Stream"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a stream of integers, find the running median after each element is added.\n\n\
         Return a Vec<f64> where result[i] is the median of nums[0..=i].\n\n\
         The median of an even-length sequence is the average of the two middle values.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 50\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(MedianStreamTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MedianStreamTest>().unwrap();
        let expected = ref_median_stream(&t.nums);
        let actual = solutions::median_stream(&t.nums);
        let is_correct = expected.len() == actual.len()
            && expected
                .iter()
                .zip(actual.iter())
                .all(|(e, a)| (e - a).abs() < 1e-5);
        SolutionResult {
            is_correct,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:.5?}"),
            actual: format!("{actual:.5?}"),
        }
    }
}

fn ref_median_stream(nums: &[i32]) -> Vec<f64> {
    let mut sorted = Vec::new();
    let mut result = Vec::new();
    for &n in nums {
        let pos = sorted.partition_point(|&x| x < n);
        sorted.insert(pos, n);
        let len = sorted.len();
        if len % 2 == 1 {
            result.push(sorted[len / 2] as f64);
        } else {
            result.push((sorted[len / 2 - 1] as f64 + sorted[len / 2] as f64) / 2.0);
        }
    }
    result
}

// ── Hard 5: Nth Smallest Element ──────────────────────────────────────

struct NthElement;

struct NthElementTest {
    nums: Vec<i32>,
    n: usize,
}

impl Problem for NthElement {
    fn id(&self) -> &str {
        "merge_sort_nth_element"
    }
    fn name(&self) -> &str {
        "Find Nth Smallest Element"
    }
    fn topic(&self) -> &str {
        "merge_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find the nth smallest element in an unsorted array using a merge-sort-like approach.\n\n\
         `n` is 1-indexed: n=1 means the smallest element.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - 1 <= n <= nums.len()\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=50);
                let nums: Vec<i32> = (0..len).map(|_| rng.random_range(-500..=500)).collect();
                let n = rng.random_range(1..=len);
                TestCase {
                    data: Box::new(NthElementTest { nums, n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NthElementTest>().unwrap();
        let mut sorted = t.nums.clone();
        sorted.sort();
        let expected = sorted[t.n - 1];
        let actual = solutions::nth_element(&t.nums, t.n);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}, n={}", t.nums, t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}
