use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part2_sorting::quick_sort as solutions;
use crate::tracker::{track_slice, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(QuickSortBasic),
        Box::new(Partition),
        Box::new(KthLargest),
        Box::new(SortColors),
        Box::new(TopKFrequent),
        Box::new(ThreeWayQuickSort),
        Box::new(WiggleSortII),
        Box::new(KthSmallestMatrix),
        Box::new(FindKClosest),
        Box::new(SortByParityII),
        Box::new(MedianTwoSorted),
        Box::new(NutsAndBolts),
        Box::new(KClosestOrigin),
        Box::new(MaxGap),
        Box::new(FindDuplicate),
    ]
}

// ── Easy 1: Quick Sort Basic ──────────────────────────────────────────

struct QuickSortBasic;

struct QuickSortBasicTest {
    nums: Vec<i32>,
}

impl Problem for QuickSortBasic {
    fn id(&self) -> &str {
        "quick_sort_basic"
    }
    fn name(&self) -> &str {
        "Quick Sort (Tracked)"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Implement quick sort on a slice of Tracked<i32>.\n\n\
         The function signature is:\n\
         `fn quick_sort_basic(nums: &mut [Tracked<i32>])`\n\n\
         Sort the slice in ascending order using the quick sort algorithm.\n\
         Tracked<i32> supports comparison operators and Clone.\n\
         Use `crate::tracker::tracked_swap` for swapping elements.\n\n\
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
                    data: Box::new(QuickSortBasicTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<QuickSortBasicTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();

        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let mut tracked = track_slice(&t.nums, shared_log.clone());
        solutions::quick_sort_basic(&mut tracked);
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

// ── Easy 2: Partition ─────────────────────────────────────────────────

struct Partition;

struct PartitionTest {
    nums: Vec<i32>,
}

impl Problem for Partition {
    fn id(&self) -> &str {
        "quick_sort_partition"
    }
    fn name(&self) -> &str {
        "Partition Array"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Partition an array around a pivot (the last element).\n\n\
         After partitioning:\n\
         - All elements before the pivot index are <= pivot\n\
         - All elements after the pivot index are > pivot\n\
         - The pivot is at its correct sorted position\n\n\
         Return (partitioned_array, pivot_index).\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 50\n\
         - -100 <= nums[i] <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(PartitionTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PartitionTest>().unwrap();
        let pivot = *t.nums.last().unwrap();
        let (actual_arr, actual_idx) = solutions::partition(&t.nums);

        // Verify: pivot at correct position, left side <= pivot, right side > pivot
        let pivot_correct = actual_arr.get(actual_idx) == Some(&pivot);
        let left_ok = actual_arr[..actual_idx].iter().all(|&x| x <= pivot);
        let right_ok = if actual_idx + 1 < actual_arr.len() {
            actual_arr[actual_idx + 1..].iter().all(|&x| x > pivot)
        } else {
            true
        };
        // Same elements
        let mut orig_sorted = t.nums.clone();
        orig_sorted.sort();
        let mut actual_sorted = actual_arr.clone();
        actual_sorted.sort();
        let same_elements = orig_sorted == actual_sorted;

        let is_correct = pivot_correct && left_ok && right_ok && same_elements;

        SolutionResult {
            is_correct,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("pivot={pivot} at correct position, left<=pivot, right>pivot"),
            actual: format!("arr={actual_arr:?}, pivot_idx={actual_idx}"),
        }
    }
}

// ── Easy 3: Kth Largest Element ───────────────────────────────────────

struct KthLargest;

struct KthLargestTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for KthLargest {
    fn id(&self) -> &str {
        "quick_sort_kth_largest"
    }
    fn name(&self) -> &str {
        "Kth Largest Element"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Find the kth largest element in an unsorted array.\n\n\
         k is 1-indexed: k=1 is the largest element.\n\n\
         Use quickselect (partition-based) for average O(n).\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - 1 <= k <= nums.len()\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=50);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-500..=500)).collect();
                let k = rng.random_range(1..=n);
                TestCase {
                    data: Box::new(KthLargestTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KthLargestTest>().unwrap();
        let mut sorted = t.nums.clone();
        sorted.sort_unstable();
        let expected = sorted[sorted.len() - t.k];
        let actual = solutions::kth_largest(&t.nums, t.k);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 4: Sort Colors (Dutch National Flag) ────────────────────────

struct SortColors;

struct SortColorsTest {
    nums: Vec<i32>,
}

impl Problem for SortColors {
    fn id(&self) -> &str {
        "quick_sort_sort_colors"
    }
    fn name(&self) -> &str {
        "Sort Colors (Dutch National Flag)"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array with values 0, 1, and 2 (representing red, white, blue), \
         sort them in-place so that all 0s come first, then 1s, then 2s.\n\n\
         Return the sorted array. Use the Dutch National Flag algorithm (one pass, O(1) space).\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - nums[i] is 0, 1, or 2"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=50);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=2)).collect();
                TestCase {
                    data: Box::new(SortColorsTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortColorsTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();
        let actual = solutions::sort_colors(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 5: Top K Frequent Elements ───────────────────────────────────

struct TopKFrequent;

struct TopKFrequentTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for TopKFrequent {
    fn id(&self) -> &str {
        "quick_sort_top_k_frequent"
    }
    fn name(&self) -> &str {
        "Top K Frequent Elements"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an integer array and an integer k, return the k most frequent elements.\n\n\
         Return them sorted in ascending order.\n\n\
         If there are ties in frequency, prefer smaller values.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - 1 <= k <= number of distinct elements"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=40);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
                let distinct: std::collections::HashSet<i32> = nums.iter().copied().collect();
                let k = rng.random_range(1..=distinct.len());
                TestCase {
                    data: Box::new(TopKFrequentTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TopKFrequentTest>().unwrap();
        let expected = ref_top_k_frequent(&t.nums, t.k);
        let actual = solutions::top_k_frequent(&t.nums, t.k);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &n in nums {
        *freq.entry(n).or_insert(0) += 1;
    }
    let mut entries: Vec<(i32, usize)> = freq.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let mut result: Vec<i32> = entries.into_iter().take(k).map(|(v, _)| v).collect();
    result.sort();
    result
}

// ── Medium 1: Three-Way Quick Sort ────────────────────────────────────

struct ThreeWayQuickSort;

struct ThreeWayTest {
    nums: Vec<i32>,
}

impl Problem for ThreeWayQuickSort {
    fn id(&self) -> &str {
        "quick_sort_three_way"
    }
    fn name(&self) -> &str {
        "Three-Way Quick Sort"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Implement 3-way quicksort (Dutch National Flag partitioning) that handles \
         duplicate elements efficiently.\n\n\
         Partition into three regions: less than pivot, equal to pivot, greater than pivot.\n\n\
         Return the sorted array.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - -500 <= nums[i] <= 500\n\
         - Array may contain many duplicates."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=50);
                // Use smaller range to create duplicates
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-10..=10)).collect();
                TestCase {
                    data: Box::new(ThreeWayTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ThreeWayTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();
        let actual = solutions::three_way_quicksort(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 2: Wiggle Sort II ──────────────────────────────────────────

struct WiggleSortII;

struct WiggleSortTest {
    nums: Vec<i32>,
}

impl Problem for WiggleSortII {
    fn id(&self) -> &str {
        "quick_sort_wiggle_sort_ii"
    }
    fn name(&self) -> &str {
        "Wiggle Sort II"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Reorder the array such that nums[0] < nums[1] > nums[2] < nums[3] > ...\n\n\
         Return the wiggle-sorted array. A valid solution is guaranteed to exist.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 50\n\
         - 0 <= nums[i] <= 1000\n\
         - A valid answer always exists for the given input."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=30);
                // Ensure a valid wiggle sort exists: use enough distinct values
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                TestCase {
                    data: Box::new(WiggleSortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WiggleSortTest>().unwrap();
        let actual = solutions::wiggle_sort_ii(&t.nums);

        // Verify wiggle property: even indices < next, odd indices > next
        let mut is_correct = actual.len() == t.nums.len();
        if is_correct {
            // Check same elements
            let mut orig = t.nums.clone();
            orig.sort();
            let mut act = actual.clone();
            act.sort();
            is_correct = orig == act;
        }
        if is_correct {
            for i in 0..actual.len() {
                if i % 2 == 0 {
                    // Even index: should be less than neighbors
                    if i + 1 < actual.len() && actual[i] >= actual[i + 1] {
                        is_correct = false;
                        break;
                    }
                } else {
                    // Odd index: should be greater than neighbors
                    if i + 1 < actual.len() && actual[i] <= actual[i + 1] {
                        is_correct = false;
                        break;
                    }
                }
            }
        }

        SolutionResult {
            is_correct,
            input_description: format!("nums={:?}", t.nums),
            expected: "valid wiggle: a[0]<a[1]>a[2]<a[3]>...".to_string(),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 3: Kth Smallest in Sorted Matrix ───────────────────────────

struct KthSmallestMatrix;

struct KthSmallestMatrixTest {
    matrix: Vec<Vec<i32>>,
    k: usize,
}

impl Problem for KthSmallestMatrix {
    fn id(&self) -> &str {
        "quick_sort_kth_smallest_matrix"
    }
    fn name(&self) -> &str {
        "Kth Smallest in Sorted Matrix"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an n x n matrix where each row and column is sorted in ascending order, \
         find the kth smallest element.\n\n\
         k is 1-indexed.\n\n\
         Constraints:\n\
         - 1 <= n <= 10\n\
         - 1 <= k <= n * n\n\
         - Rows and columns are sorted ascending."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=8);
                // Build a sorted matrix: start with sorted values, distribute into rows
                let total = n * n;
                let mut vals: Vec<i32> = (0..total).map(|_| rng.random_range(-100..=100)).collect();
                vals.sort();
                let matrix: Vec<Vec<i32>> = vals.chunks(n).map(|c| c.to_vec()).collect();
                let k = rng.random_range(1..=total);
                TestCase {
                    data: Box::new(KthSmallestMatrixTest { matrix, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KthSmallestMatrixTest>().unwrap();
        let mut all: Vec<i32> = t.matrix.iter().flatten().copied().collect();
        all.sort();
        let expected = all[t.k - 1];
        let actual = solutions::kth_smallest_matrix(&t.matrix, t.k);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("matrix={:?}, k={}", t.matrix, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 4: K Closest Elements ──────────────────────────────────────

struct FindKClosest;

struct FindKClosestTest {
    arr: Vec<i32>,
    k: usize,
    target: i32,
}

impl Problem for FindKClosest {
    fn id(&self) -> &str {
        "quick_sort_find_k_closest"
    }
    fn name(&self) -> &str {
        "K Closest Elements"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a sorted array, an integer k, and a target value, find the k closest \
         elements to target. Return them sorted in ascending order.\n\n\
         If two elements are equally close, prefer the smaller one.\n\n\
         Constraints:\n\
         - 1 <= arr.len() <= 50\n\
         - 1 <= k <= arr.len()\n\
         - Array is sorted ascending."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let mut arr: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                arr.sort();
                let k = rng.random_range(1..=n);
                let target = rng.random_range(-120..=120);
                TestCase {
                    data: Box::new(FindKClosestTest { arr, k, target }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FindKClosestTest>().unwrap();
        let expected = ref_find_k_closest(&t.arr, t.k, t.target);
        let actual = solutions::find_k_closest(&t.arr, t.k, t.target);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("arr={:?}, k={}, target={}", t.arr, t.k, t.target),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_find_k_closest(arr: &[i32], k: usize, target: i32) -> Vec<i32> {
    let mut indexed: Vec<(i32, i32)> = arr.iter().map(|&x| (x, (x - target).abs())).collect();
    indexed.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let mut result: Vec<i32> = indexed.into_iter().take(k).map(|(v, _)| v).collect();
    result.sort();
    result
}

// ── Medium 5: Sort by Parity II ───────────────────────────────────────

struct SortByParityII;

struct SortByParityIITest {
    nums: Vec<i32>,
}

impl Problem for SortByParityII {
    fn id(&self) -> &str {
        "quick_sort_sort_list_by_parity_ii"
    }
    fn name(&self) -> &str {
        "Sort Array by Parity II"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array where half the elements are even and half are odd, rearrange so \
         that nums[i] is even when i is even, and nums[i] is odd when i is odd.\n\n\
         Return any valid arrangement.\n\n\
         Constraints:\n\
         - 2 <= nums.len() <= 50 (always even length)\n\
         - Exactly half the elements are even, half are odd."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let half = rng.random_range(1..=15);
                let n = half * 2;
                let mut nums = Vec::with_capacity(n);
                for _ in 0..half {
                    nums.push(rng.random_range(0..=50) * 2); // even
                }
                for _ in 0..half {
                    nums.push(rng.random_range(0..=50) * 2 + 1); // odd
                }
                // Shuffle
                for i in (1..nums.len()).rev() {
                    let j = rng.random_range(0..=i);
                    nums.swap(i, j);
                }
                TestCase {
                    data: Box::new(SortByParityIITest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortByParityIITest>().unwrap();
        let actual = solutions::sort_by_parity_ii(&t.nums);

        let same_len = actual.len() == t.nums.len();
        let parity_ok = actual
            .iter()
            .enumerate()
            .all(|(i, &v)| v % 2 == i as i32 % 2);
        let mut orig = t.nums.clone();
        orig.sort();
        let mut act = actual.clone();
        act.sort();
        let same_elements = orig == act;

        let is_correct = same_len && parity_ok && same_elements;

        SolutionResult {
            is_correct,
            input_description: format!("nums={:?}", t.nums),
            expected: "even indices have even values, odd indices have odd values".to_string(),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 1: Median of Two Sorted Arrays ───────────────────────────────

struct MedianTwoSorted;

struct MedianTwoSortedTest {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

impl Problem for MedianTwoSorted {
    fn id(&self) -> &str {
        "quick_sort_median_two_sorted"
    }
    fn name(&self) -> &str {
        "Median of Two Sorted Arrays"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given two sorted arrays nums1 and nums2, return the median of the combined \
         sorted array as f64.\n\n\
         Target time complexity: O(log(m+n)).\n\n\
         Constraints:\n\
         - 0 <= nums1.len(), nums2.len() <= 50\n\
         - nums1.len() + nums2.len() >= 1\n\
         - Both arrays are sorted ascending."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let m = rng.random_range(0..=25);
                let n = rng.random_range(1..=25);
                let mut nums1: Vec<i32> = (0..m).map(|_| rng.random_range(-200..=200)).collect();
                let mut nums2: Vec<i32> = (0..n).map(|_| rng.random_range(-200..=200)).collect();
                nums1.sort();
                nums2.sort();
                TestCase {
                    data: Box::new(MedianTwoSortedTest { nums1, nums2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MedianTwoSortedTest>().unwrap();
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

// ── Hard 2: Nuts and Bolts ────────────────────────────────────────────

struct NutsAndBolts;

struct NutsAndBoltsTest {
    nuts: Vec<i32>,
    bolts: Vec<i32>,
}

impl Problem for NutsAndBolts {
    fn id(&self) -> &str {
        "quick_sort_nuts_bolts"
    }
    fn name(&self) -> &str {
        "Nuts and Bolts Problem"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a set of nuts and a set of bolts (each nut matches exactly one bolt), \
         sort them so that nuts[i] matches bolts[i].\n\n\
         Both arrays contain the same set of unique values in different orders.\n\n\
         Return the sorted arrangement (just sort both arrays the same way).\n\n\
         Constraints:\n\
         - 1 <= n <= 50\n\
         - nuts and bolts have the same elements, different order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let mut values: Vec<i32> = (1..=n as i32).collect();
                // Shuffle for nuts
                let mut nuts = values.clone();
                for i in (1..nuts.len()).rev() {
                    let j = rng.random_range(0..=i);
                    nuts.swap(i, j);
                }
                // Shuffle for bolts
                for i in (1..values.len()).rev() {
                    let j = rng.random_range(0..=i);
                    values.swap(i, j);
                }
                TestCase {
                    data: Box::new(NutsAndBoltsTest {
                        nuts,
                        bolts: values,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NutsAndBoltsTest>().unwrap();
        let mut expected = t.nuts.clone();
        expected.sort();
        let actual = solutions::nuts_and_bolts(&t.nuts, &t.bolts);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nuts={:?}, bolts={:?}", t.nuts, t.bolts),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 3: K Closest Points to Origin ────────────────────────────────

struct KClosestOrigin;

struct KClosestOriginTest {
    points: Vec<(i32, i32)>,
    k: usize,
}

impl Problem for KClosestOrigin {
    fn id(&self) -> &str {
        "quick_sort_k_closest_origin"
    }
    fn name(&self) -> &str {
        "K Closest Points to Origin"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array of points (x, y) and an integer k, return the k closest points \
         to the origin (0, 0).\n\n\
         Distance is Euclidean (use x*x + y*y to avoid floating point).\n\
         Return results sorted by distance (ascending), then by x, then by y.\n\n\
         Constraints:\n\
         - 1 <= points.len() <= 50\n\
         - 1 <= k <= points.len()"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let points: Vec<(i32, i32)> = (0..n)
                    .map(|_| (rng.random_range(-100..=100), rng.random_range(-100..=100)))
                    .collect();
                let k = rng.random_range(1..=n);
                TestCase {
                    data: Box::new(KClosestOriginTest { points, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KClosestOriginTest>().unwrap();
        let expected = ref_k_closest_origin(&t.points, t.k);
        let actual = solutions::k_closest_origin(&t.points, t.k);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("points={:?}, k={}", t.points, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_k_closest_origin(points: &[(i32, i32)], k: usize) -> Vec<(i32, i32)> {
    let mut pts = points.to_vec();
    pts.sort_by(|a, b| {
        let da = a.0 as i64 * a.0 as i64 + a.1 as i64 * a.1 as i64;
        let db = b.0 as i64 * b.0 as i64 + b.1 as i64 * b.1 as i64;
        da.cmp(&db).then(a.0.cmp(&b.0)).then(a.1.cmp(&b.1))
    });
    pts.truncate(k);
    pts
}

// ── Hard 4: Maximum Gap ───────────────────────────────────────────────

struct MaxGap;

struct MaxGapTest {
    nums: Vec<i32>,
}

impl Problem for MaxGap {
    fn id(&self) -> &str {
        "quick_sort_max_gap"
    }
    fn name(&self) -> &str {
        "Maximum Gap"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an unsorted array, find the maximum difference between successive elements \
         in its sorted form.\n\n\
         Return 0 if the array has fewer than 2 elements.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - 0 <= nums[i] <= 10000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=50);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=1000)).collect();
                TestCase {
                    data: Box::new(MaxGapTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxGapTest>().unwrap();
        let expected = ref_max_gap(&t.nums);
        let actual = solutions::max_gap(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_gap(nums: &[i32]) -> i32 {
    if nums.len() < 2 {
        return 0;
    }
    let mut sorted = nums.to_vec();
    sorted.sort();
    sorted.windows(2).map(|w| w[1] - w[0]).max().unwrap_or(0)
}

// ── Hard 5: Find the Duplicate Number ─────────────────────────────────

struct FindDuplicate;

struct FindDuplicateTest {
    nums: Vec<i32>,
}

impl Problem for FindDuplicate {
    fn id(&self) -> &str {
        "quick_sort_find_duplicate"
    }
    fn name(&self) -> &str {
        "Find the Duplicate Number"
    }
    fn topic(&self) -> &str {
        "quick_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array of n+1 integers where each integer is in the range [1, n], \
         there is exactly one duplicated number. Find and return it.\n\n\
         You must not modify the array. Use O(1) extra space.\n\n\
         Hint: Floyd's cycle detection or binary search on value range.\n\n\
         Constraints:\n\
         - 2 <= nums.len() <= 100\n\
         - All values in [1, n] where n = nums.len() - 1\n\
         - Exactly one value is duplicated (may appear more than twice)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=50);
                let dup = rng.random_range(1..=n as i32);
                let mut nums: Vec<i32> = (1..=n as i32).collect();
                nums.push(dup);
                // Shuffle
                for i in (1..nums.len()).rev() {
                    let j = rng.random_range(0..=i);
                    nums.swap(i, j);
                }
                TestCase {
                    data: Box::new(FindDuplicateTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FindDuplicateTest>().unwrap();
        let expected = ref_find_duplicate(&t.nums);
        let actual = solutions::find_duplicate(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_find_duplicate(nums: &[i32]) -> i32 {
    // Floyd's tortoise and hare
    let mut slow = nums[0] as usize;
    let mut fast = nums[0] as usize;
    loop {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        if slow == fast {
            break;
        }
    }
    let mut slow2 = nums[0] as usize;
    while slow2 != slow {
        slow2 = nums[slow2] as usize;
        slow = nums[slow] as usize;
    }
    slow as i32
}
