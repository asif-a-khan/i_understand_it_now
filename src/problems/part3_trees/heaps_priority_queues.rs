use rand::Rng;
use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part3_trees::heaps_priority_queues as solutions;
use crate::tracker::{track_slice, OperationLog, Tracked};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(KthLargestElement),
        Box::new(LastStoneWeight),
        Box::new(KClosestPoints),
        Box::new(SortByIncreasingFrequency),
        Box::new(IsMinHeap),
        Box::new(TopKFrequent),
        Box::new(KClosestToValue),
        Box::new(ReorganizeString),
        Box::new(MergeKSortedLists),
        Box::new(TaskScheduler),
        Box::new(FindMedianStream),
        Box::new(SlidingWindowMedian),
        Box::new(SmallestRange),
        Box::new(Ipo),
        Box::new(Skyline),
    ]
}

// ── Easy 1: Kth Largest Element ──────────────────────────────────────────

struct KthLargestElement;
struct KthLargestTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for KthLargestElement {
    fn id(&self) -> &str {
        "heaps_kth_largest_element"
    }
    fn name(&self) -> &str {
        "Kth Largest Element"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an integer array `nums` and an integer `k`, return the kth largest element.\n\n\
         Constraints:\n\
         - 1 <= k <= nums.len() <= 1000\n\
         - -10000 <= nums[i] <= 10000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                let k = rng.random_range(1..=n);
                TestCase {
                    data: Box::new(KthLargestTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KthLargestTest>().unwrap();
        let expected = ref_kth_largest(&t.nums, t.k);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::kth_largest(&tracked_nums, t.k);
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

fn ref_kth_largest(nums: &[i32], k: usize) -> i32 {
    let mut sorted = nums.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    sorted[k - 1]
}

// ── Easy 2: Last Stone Weight ────────────────────────────────────────────

struct LastStoneWeight;
struct LastStoneWeightTest {
    stones: Vec<i32>,
}

impl Problem for LastStoneWeight {
    fn id(&self) -> &str {
        "heaps_last_stone_weight"
    }
    fn name(&self) -> &str {
        "Last Stone Weight"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "You have a collection of stones, each with a positive integer weight. Each turn, \
         smash the two heaviest stones together. If they are equal, both are destroyed. \
         Otherwise, the lighter one is destroyed and the heavier one loses weight equal to \
         the lighter one. Return the weight of the last remaining stone, or 0 if none remain.\n\n\
         Constraints:\n\
         - 1 <= stones.len() <= 30\n\
         - 1 <= stones[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let stones: Vec<i32> = (0..n).map(|_| rng.random_range(1..=100)).collect();
                TestCase {
                    data: Box::new(LastStoneWeightTest { stones }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LastStoneWeightTest>().unwrap();
        let expected = ref_last_stone_weight(&t.stones);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_stones = track_slice(&t.stones, shared_log.clone());
        let actual = solutions::last_stone_weight(&tracked_stones);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("stones={:?}", t.stones),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_last_stone_weight(stones: &[i32]) -> i32 {
    let mut heap = BinaryHeap::from(stones.to_vec());
    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        if a != b {
            heap.push(a - b);
        }
    }
    heap.pop().unwrap_or(0)
}

// ── Easy 3: K Closest Points to Origin ──────────────────────────────────

struct KClosestPoints;
struct KClosestTest {
    points: Vec<(i32, i32)>,
    k: usize,
}

impl Problem for KClosestPoints {
    fn id(&self) -> &str {
        "heaps_k_closest_points"
    }
    fn name(&self) -> &str {
        "K Closest Points to Origin"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array of points where points[i] = (x, y) represents a point on the X-Y plane, \
         return the `k` closest points to the origin (0, 0).\n\n\
         Return the result sorted by distance (ascending). If two points have the same distance, \
         order by x then y.\n\n\
         Constraints:\n\
         - 1 <= k <= points.len() <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let points: Vec<(i32, i32)> = (0..n)
                    .map(|_| (rng.random_range(-50..=50), rng.random_range(-50..=50)))
                    .collect();
                let k = rng.random_range(1..=n);
                TestCase {
                    data: Box::new(KClosestTest { points, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KClosestTest>().unwrap();
        let expected = ref_k_closest(&t.points, t.k);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_points: Vec<(Tracked<i32>, Tracked<i32>)> = t
            .points
            .iter()
            .enumerate()
            .map(|(i, &(a, b))| {
                (
                    Tracked::new(a, i * 2, shared_log.clone()),
                    Tracked::new(b, i * 2 + 1, shared_log.clone()),
                )
            })
            .collect();
        let actual = solutions::k_closest_points(&tracked_points, t.k);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("points={:?}, k={}", t.points, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_k_closest(points: &[(i32, i32)], k: usize) -> Vec<(i32, i32)> {
    let mut pts = points.to_vec();
    pts.sort_by(|a, b| {
        let da = a.0 as i64 * a.0 as i64 + a.1 as i64 * a.1 as i64;
        let db = b.0 as i64 * b.0 as i64 + b.1 as i64 * b.1 as i64;
        da.cmp(&db).then(a.0.cmp(&b.0)).then(a.1.cmp(&b.1))
    });
    pts.truncate(k);
    pts
}

// ── Easy 4: Sort Array by Increasing Frequency ──────────────────────────

struct SortByIncreasingFrequency;
struct FreqSortTest {
    nums: Vec<i32>,
}

impl Problem for SortByIncreasingFrequency {
    fn id(&self) -> &str {
        "heaps_sort_array_by_increasing_frequency"
    }
    fn name(&self) -> &str {
        "Sort Array by Increasing Frequency"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Sort the array in increasing order based on the frequency of the values. \
         If multiple values have the same frequency, sort them in decreasing order.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - -100 <= nums[i] <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
                TestCase {
                    data: Box::new(FreqSortTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FreqSortTest>().unwrap();
        let expected = ref_sort_by_freq(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sort_by_increasing_frequency(&tracked_nums);
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

fn ref_sort_by_freq(nums: &[i32]) -> Vec<i32> {
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &n in nums {
        *freq.entry(n).or_insert(0) += 1;
    }
    let mut result = nums.to_vec();
    result.sort_by(|a, b| freq[a].cmp(&freq[b]).then(b.cmp(a)));
    result
}

// ── Easy 5: Is Min Heap ─────────────────────────────────────────────────

struct IsMinHeap;
struct IsMinHeapTest {
    arr: Vec<i32>,
}

impl Problem for IsMinHeap {
    fn id(&self) -> &str {
        "heaps_is_min_heap"
    }
    fn name(&self) -> &str {
        "Is Min Heap"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Check if the given array represents a valid min-heap. In a min-heap, every parent \
         node has a value less than or equal to its children.\n\n\
         Constraints:\n\
         - 0 <= arr.len() <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=20);
                let arr = if rng.random_range(0..2) == 0 {
                    // Generate a valid min-heap by heapifying
                    let mut v: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                    for i in (0..v.len() / 2).rev() {
                        min_heapify(&mut v, i);
                    }
                    v
                } else {
                    // Generate random array (may or may not be a heap)
                    (0..n).map(|_| rng.random_range(-50..=50)).collect()
                };
                TestCase {
                    data: Box::new(IsMinHeapTest { arr }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IsMinHeapTest>().unwrap();
        let expected = ref_is_min_heap(&t.arr);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::is_min_heap(&tracked_arr);
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

fn min_heapify(arr: &mut [i32], i: usize) {
    let n = arr.len();
    let mut smallest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    if left < n && arr[left] < arr[smallest] {
        smallest = left;
    }
    if right < n && arr[right] < arr[smallest] {
        smallest = right;
    }
    if smallest != i {
        arr.swap(i, smallest);
        min_heapify(arr, smallest);
    }
}

fn ref_is_min_heap(arr: &[i32]) -> bool {
    for i in 0..arr.len() {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        if left < arr.len() && arr[i] > arr[left] {
            return false;
        }
        if right < arr.len() && arr[i] > arr[right] {
            return false;
        }
    }
    true
}

// ── Medium 1: Top K Frequent Elements ───────────────────────────────────

struct TopKFrequent;
struct TopKFreqTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for TopKFrequent {
    fn id(&self) -> &str {
        "heaps_top_k_frequent"
    }
    fn name(&self) -> &str {
        "Top K Frequent Elements"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an integer array `nums` and an integer `k`, return the `k` most frequent elements. \
         Return them sorted in descending order of frequency. If two elements have the same \
         frequency, the larger value comes first.\n\n\
         Constraints:\n\
         - 1 <= k <= number of distinct elements <= nums.len() <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
                let distinct: std::collections::HashSet<i32> = nums.iter().copied().collect();
                let k = rng.random_range(1..=distinct.len());
                TestCase {
                    data: Box::new(TopKFreqTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TopKFreqTest>().unwrap();
        let expected = ref_top_k_frequent(&t.nums, t.k);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::top_k_frequent(&tracked_nums, t.k);
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

fn ref_top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &n in nums {
        *freq.entry(n).or_insert(0) += 1;
    }
    let mut entries: Vec<(i32, usize)> = freq.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
    entries.into_iter().take(k).map(|(v, _)| v).collect()
}

// ── Medium 2: K Closest Elements to Value ───────────────────────────────

struct KClosestToValue;
struct KClosestValTest {
    arr: Vec<i32>,
    k: usize,
    target: i32,
}

impl Problem for KClosestToValue {
    fn id(&self) -> &str {
        "heaps_k_closest_to_value"
    }
    fn name(&self) -> &str {
        "K Closest Elements to Value"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a sorted integer array `arr`, an integer `k`, and an integer `target`, \
         return the `k` closest elements to `target`. Return the result sorted in ascending order.\n\n\
         If two elements are equally close, prefer the smaller one.\n\n\
         Constraints:\n\
         - 1 <= k <= arr.len() <= 1000\n\
         - arr is sorted in ascending order"
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
                    data: Box::new(KClosestValTest { arr, k, target }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KClosestValTest>().unwrap();
        let expected = ref_k_closest_to_value(&t.arr, t.k, t.target);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_arr = track_slice(&t.arr, shared_log.clone());
        let actual = solutions::k_closest_to_value(&tracked_arr, t.k, t.target);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("arr={:?}, k={}, target={}", t.arr, t.k, t.target),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_k_closest_to_value(arr: &[i32], k: usize, target: i32) -> Vec<i32> {
    let mut indexed: Vec<(i32, i32)> = arr.iter().map(|&v| ((v - target).abs(), v)).collect();
    indexed.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut result: Vec<i32> = indexed.into_iter().take(k).map(|(_, v)| v).collect();
    result.sort();
    result
}

// ── Medium 3: Reorganize String ─────────────────────────────────────────

struct ReorganizeString;
struct ReorgStringTest {
    s: String,
}

impl Problem for ReorganizeString {
    fn id(&self) -> &str {
        "heaps_reorganize_string"
    }
    fn name(&self) -> &str {
        "Reorganize String"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a string `s`, rearrange the characters so that no two adjacent characters are \
         the same. Return any valid rearrangement, or an empty string if it is not possible.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 500\n\
         - s consists of lowercase English letters only"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let charset_size = rng.random_range(1..=5u8);
                let s: String = (0..n)
                    .map(|_| (b'a' + rng.random_range(0..charset_size)) as char)
                    .collect();
                TestCase {
                    data: Box::new(ReorgStringTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReorgStringTest>().unwrap();
        let can_do = ref_can_reorganize(&t.s);
        let actual = solutions::reorganize_string(&t.s);

        if !can_do {
            SolutionResult {
                is_correct: actual.is_empty(),
                input_description: format!("s=\"{}\"", t.s),
                expected: "\"\" (impossible)".to_string(),
                actual: format!("\"{}\"", actual),
            }
        } else {
            let valid = validate_reorganized(&t.s, &actual);
            SolutionResult {
                is_correct: valid,
                input_description: format!("s=\"{}\"", t.s),
                expected: "any valid reorganization".to_string(),
                actual: format!("\"{}\"", actual),
            }
        }
    }
}

fn ref_can_reorganize(s: &str) -> bool {
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    let max_freq = freq.values().max().copied().unwrap_or(0);
    max_freq <= s.len().div_ceil(2)
}

fn validate_reorganized(original: &str, result: &str) -> bool {
    if result.len() != original.len() {
        return false;
    }
    // Check same character frequencies
    let mut orig_freq: HashMap<char, usize> = HashMap::new();
    for c in original.chars() {
        *orig_freq.entry(c).or_insert(0) += 1;
    }
    let mut res_freq: HashMap<char, usize> = HashMap::new();
    for c in result.chars() {
        *res_freq.entry(c).or_insert(0) += 1;
    }
    if orig_freq != res_freq {
        return false;
    }
    // Check no adjacent duplicates
    let chars: Vec<char> = result.chars().collect();
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            return false;
        }
    }
    true
}

// ── Medium 4: Merge K Sorted Lists ──────────────────────────────────────

struct MergeKSortedLists;
struct MergeKTest {
    lists: Vec<Vec<i32>>,
}

impl Problem for MergeKSortedLists {
    fn id(&self) -> &str {
        "heaps_merge_k_sorted_lists"
    }
    fn name(&self) -> &str {
        "Merge K Sorted Lists"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "You are given an array of `k` sorted (ascending) integer lists. Merge all lists \
         into one sorted list and return it.\n\n\
         Constraints:\n\
         - 0 <= k <= 100\n\
         - 0 <= list[i].len() <= 500\n\
         - Each list is sorted in ascending order"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let k = rng.random_range(0..=8);
                let lists: Vec<Vec<i32>> = (0..k)
                    .map(|_| {
                        let n = rng.random_range(0..=10);
                        let mut v: Vec<i32> =
                            (0..n).map(|_| rng.random_range(-100..=100)).collect();
                        v.sort();
                        v
                    })
                    .collect();
                TestCase {
                    data: Box::new(MergeKTest { lists }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MergeKTest>().unwrap();
        let mut expected: Vec<i32> = t.lists.iter().flatten().copied().collect();
        expected.sort();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_lists: Vec<Vec<Tracked<i32>>> = t
            .lists
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, &v)| Tracked::new(v, r * 1000 + c, shared_log.clone()))
                    .collect()
            })
            .collect();
        let actual = solutions::merge_k_sorted_lists(&tracked_lists);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("lists={:?}", t.lists),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 5: Task Scheduler ────────────────────────────────────────────

struct TaskScheduler;
struct TaskSchedulerTest {
    tasks: Vec<char>,
    n: i32,
}

impl Problem for TaskScheduler {
    fn id(&self) -> &str {
        "heaps_task_scheduler"
    }
    fn name(&self) -> &str {
        "Task Scheduler"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a list of tasks (represented as characters A-Z) and a cooldown integer `n`, \
         return the minimum number of intervals the CPU needs to finish all tasks.\n\n\
         There must be at least `n` intervals between two executions of the same task. \
         The CPU can be idle during an interval.\n\n\
         Constraints:\n\
         - 1 <= tasks.len() <= 10000\n\
         - tasks[i] is uppercase English letter\n\
         - 0 <= n <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=20);
                let charset = rng.random_range(1..=6u8);
                let tasks: Vec<char> = (0..len)
                    .map(|_| (b'A' + rng.random_range(0..charset)) as char)
                    .collect();
                let n = rng.random_range(0..=4);
                TestCase {
                    data: Box::new(TaskSchedulerTest { tasks, n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TaskSchedulerTest>().unwrap();
        let expected = ref_task_scheduler(&t.tasks, t.n);
        let actual = solutions::task_scheduler(&t.tasks, t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tasks={:?}, n={}", t.tasks, t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_task_scheduler(tasks: &[char], n: i32) -> i32 {
    let mut freq = [0i32; 26];
    for &t in tasks {
        freq[(t as u8 - b'A') as usize] += 1;
    }
    let max_freq = *freq.iter().max().unwrap();
    let max_count = freq.iter().filter(|&&f| f == max_freq).count() as i32;
    let result = (max_freq - 1) * (n + 1) + max_count;
    result.max(tasks.len() as i32)
}

// ── Hard 1: Find Median from Data Stream ────────────────────────────────

struct FindMedianStream;
struct MedianStreamTest {
    nums: Vec<i32>,
}

impl Problem for FindMedianStream {
    fn id(&self) -> &str {
        "heaps_find_median_stream"
    }
    fn name(&self) -> &str {
        "Find Median from Data Stream"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a stream of integers, compute the running median after each element is added.\n\n\
         Return a Vec<f64> where the i-th element is the median after processing the first \
         i+1 numbers.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 1000\n\
         - -100000 <= nums[i] <= 100000"
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

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MedianStreamTest>().unwrap();
        let expected = ref_median_stream(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::find_median_stream(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        let is_correct = expected.len() == actual.len()
            && expected
                .iter()
                .zip(actual.iter())
                .all(|(e, a)| (e - a).abs() < 1e-5);
        SolutionResult {
            is_correct,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_median_stream(nums: &[i32]) -> Vec<f64> {
    let mut seen = Vec::new();
    let mut result = Vec::new();
    for &n in nums {
        seen.push(n);
        seen.sort();
        let len = seen.len();
        if len % 2 == 1 {
            result.push(seen[len / 2] as f64);
        } else {
            result.push((seen[len / 2 - 1] as f64 + seen[len / 2] as f64) / 2.0);
        }
    }
    result
}

// ── Hard 2: Sliding Window Median ───────────────────────────────────────

struct SlidingWindowMedian;
struct SlidingMedianTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for SlidingWindowMedian {
    fn id(&self) -> &str {
        "heaps_sliding_window_median"
    }
    fn name(&self) -> &str {
        "Sliding Window Median"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array of integers `nums` and an integer `k`, compute the median of each \
         sliding window of size `k`.\n\n\
         Return a Vec<f64> of medians.\n\n\
         Constraints:\n\
         - 1 <= k <= nums.len() <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                let k = rng.random_range(1..=n);
                TestCase {
                    data: Box::new(SlidingMedianTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SlidingMedianTest>().unwrap();
        let expected = ref_sliding_window_median(&t.nums, t.k);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sliding_window_median(&tracked_nums, t.k);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        let is_correct = expected.len() == actual.len()
            && expected
                .iter()
                .zip(actual.iter())
                .all(|(e, a)| (e - a).abs() < 1e-5);
        SolutionResult {
            is_correct,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_sliding_window_median(nums: &[i32], k: usize) -> Vec<f64> {
    let mut result = Vec::new();
    for i in 0..=nums.len() - k {
        let mut window: Vec<i32> = nums[i..i + k].to_vec();
        window.sort();
        if k % 2 == 1 {
            result.push(window[k / 2] as f64);
        } else {
            result.push((window[k / 2 - 1] as f64 + window[k / 2] as f64) / 2.0);
        }
    }
    result
}

// ── Hard 3: Smallest Range Covering Elements from Each List ─────────────

struct SmallestRange;
struct SmallestRangeTest {
    lists: Vec<Vec<i32>>,
}

impl Problem for SmallestRange {
    fn id(&self) -> &str {
        "heaps_smallest_range"
    }
    fn name(&self) -> &str {
        "Smallest Range Covering Elements from K Lists"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "You have `k` sorted integer lists. Find the smallest range [a, b] such that at \
         least one number from each list is included in the range.\n\n\
         Return (a, b). If there are multiple ranges of the same size, return the one with \
         the smallest `a`.\n\n\
         Constraints:\n\
         - 1 <= k <= 20\n\
         - 1 <= list[i].len() <= 50\n\
         - Each list is sorted in ascending order"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let k = rng.random_range(1..=5);
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

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SmallestRangeTest>().unwrap();
        let expected = ref_smallest_range(&t.lists);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_lists: Vec<Vec<Tracked<i32>>> = t
            .lists
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, &v)| Tracked::new(v, r * 1000 + c, shared_log.clone()))
                    .collect()
            })
            .collect();
        let actual = solutions::smallest_range(&tracked_lists);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("lists={:?}", t.lists),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_smallest_range(lists: &[Vec<i32>]) -> (i32, i32) {
    // Merge all elements with their list index, then use sliding window
    let mut all: Vec<(i32, usize)> = Vec::new();
    for (i, list) in lists.iter().enumerate() {
        for &v in list {
            all.push((v, i));
        }
    }
    all.sort();

    let k = lists.len();
    let mut count: HashMap<usize, usize> = HashMap::new();
    let mut covered = 0;
    let mut best = (all[0].0, *all.last().map(|(v, _)| v).unwrap_or(&0));
    let mut left = 0;

    for right in 0..all.len() {
        let entry = count.entry(all[right].1).or_insert(0);
        if *entry == 0 {
            covered += 1;
        }
        *entry += 1;

        while covered == k {
            let range = all[right].0 - all[left].0;
            let best_range = best.1 - best.0;
            if range < best_range || (range == best_range && all[left].0 < best.0) {
                best = (all[left].0, all[right].0);
            }
            let entry = count.get_mut(&all[left].1).unwrap();
            *entry -= 1;
            if *entry == 0 {
                covered -= 1;
            }
            left += 1;
        }
    }
    best
}

// ── Hard 4: IPO ─────────────────────────────────────────────────────────

struct Ipo;
struct IpoTest {
    k: usize,
    w: i32,
    profits: Vec<i32>,
    capital: Vec<i32>,
}

impl Problem for Ipo {
    fn id(&self) -> &str {
        "heaps_ipo"
    }
    fn name(&self) -> &str {
        "IPO"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "You are given `k` projects to complete, starting with initial capital `w`. \
         Each project `i` requires `capital[i]` to start and gives `profits[i]` pure profit \
         upon completion. You can only start a project if your current capital >= capital[i]. \
         After completing a project, your capital increases by its profit.\n\n\
         Return the maximum capital achievable after selecting at most `k` projects.\n\n\
         Constraints:\n\
         - 1 <= k <= 1000\n\
         - 0 <= w <= 10^9\n\
         - profits.len() == capital.len() <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=15);
                let k = rng.random_range(1..=n);
                let w = rng.random_range(0..=50);
                let profits: Vec<i32> = (0..n).map(|_| rng.random_range(1..=100)).collect();
                let capital: Vec<i32> = (0..n).map(|_| rng.random_range(0..=50)).collect();
                TestCase {
                    data: Box::new(IpoTest {
                        k,
                        w,
                        profits,
                        capital,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IpoTest>().unwrap();
        let expected = ref_ipo(t.k, t.w, &t.profits, &t.capital);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_profits = track_slice(&t.profits, shared_log.clone());
        let tracked_capital = track_slice(&t.capital, shared_log.clone());
        let actual =
            solutions::find_maximized_capital(t.k, t.w, &tracked_profits, &tracked_capital);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!(
                "k={}, w={}, profits={:?}, capital={:?}",
                t.k, t.w, t.profits, t.capital
            ),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_ipo(k: usize, w: i32, profits: &[i32], capital: &[i32]) -> i32 {
    let n = profits.len();
    let mut projects: Vec<(i32, i32)> = capital
        .iter()
        .copied()
        .zip(profits.iter().copied())
        .collect();
    projects.sort();

    let mut current_capital = w;
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut idx = 0;

    for _ in 0..k {
        while idx < n && projects[idx].0 <= current_capital {
            max_heap.push(projects[idx].1);
            idx += 1;
        }
        if let Some(profit) = max_heap.pop() {
            current_capital += profit;
        } else {
            break;
        }
    }
    current_capital
}

// ── Hard 5: The Skyline Problem ─────────────────────────────────────────

struct Skyline;
struct SkylineTest {
    buildings: Vec<(i32, i32, i32)>,
}

impl Problem for Skyline {
    fn id(&self) -> &str {
        "heaps_skyline"
    }
    fn name(&self) -> &str {
        "The Skyline Problem"
    }
    fn topic(&self) -> &str {
        "heaps_priority_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "A city's skyline is the outer contour of the silhouette formed by all the buildings \
         when viewed from a distance. Given an array of buildings where buildings[i] = \
         (left, right, height), return the skyline as a list of key points (x, y).\n\n\
         A key point is the left endpoint of a horizontal segment. The last key point always \
         has y = 0.\n\n\
         Constraints:\n\
         - 1 <= buildings.len() <= 200\n\
         - 0 <= left < right <= 10000\n\
         - 1 <= height <= 10000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=10);
                let mut buildings: Vec<(i32, i32, i32)> = (0..n)
                    .map(|_| {
                        let left = rng.random_range(0..=50);
                        let right = rng.random_range(left + 1..=left + 20);
                        let height = rng.random_range(1..=50);
                        (left, right, height)
                    })
                    .collect();
                buildings.sort();
                TestCase {
                    data: Box::new(SkylineTest { buildings }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SkylineTest>().unwrap();
        let expected = ref_skyline(&t.buildings);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_buildings: Vec<(Tracked<i32>, Tracked<i32>, Tracked<i32>)> = t
            .buildings
            .iter()
            .enumerate()
            .map(|(i, &(l, r, h))| {
                (
                    Tracked::new(l, i * 3, shared_log.clone()),
                    Tracked::new(r, i * 3 + 1, shared_log.clone()),
                    Tracked::new(h, i * 3 + 2, shared_log.clone()),
                )
            })
            .collect();
        let actual = solutions::get_skyline(&tracked_buildings);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("buildings={:?}", t.buildings),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_skyline(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
    // Sweep line approach using sorted events
    let mut events: Vec<(i32, i32)> = Vec::new();
    for &(l, r, h) in buildings {
        events.push((l, -h)); // building start (negative height for sorting)
        events.push((r, h)); // building end
    }
    events.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut result = Vec::new();
    let mut heights: BinaryHeap<i32> = BinaryHeap::new();
    let mut removed: HashMap<i32, usize> = HashMap::new();
    heights.push(0);
    let mut prev_max = 0;

    for (x, h) in events {
        if h < 0 {
            heights.push(-h);
        } else {
            *removed.entry(h).or_insert(0) += 1;
        }
        // Lazy deletion
        while let Some(&top) = heights.peek() {
            if let Some(count) = removed.get_mut(&top) {
                if *count > 0 {
                    *count -= 1;
                    heights.pop();
                    continue;
                }
            }
            break;
        }
        let cur_max = *heights.peek().unwrap_or(&0);
        if cur_max != prev_max {
            result.push((x, cur_max));
            prev_max = cur_max;
        }
    }
    result
}
