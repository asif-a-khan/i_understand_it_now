use rand::Rng;
use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part2_sorting::heap_sort as solutions;
use crate::tracker::{track_slice, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(HeapSortBasic),
        Box::new(KthLargest),
        Box::new(LastStoneWeight),
        Box::new(KWeakestRows),
        Box::new(RelativeRanks),
        Box::new(KClosestPoints),
        Box::new(TopKFrequent),
        Box::new(SortNearlySorted),
        Box::new(MergeKSorted),
        Box::new(TaskScheduler),
        Box::new(FindMedianStream),
        Box::new(SlidingWindowMedian),
        Box::new(TrappingRainWaterII),
        Box::new(SmallestRange),
        Box::new(Ipo),
    ]
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 1: Heap Sort Basic
// ═══════════════════════════════════════════════════════════════════════

struct HeapSortBasic;
struct HeapSortBasicTest {
    nums: Vec<i32>,
}

impl Problem for HeapSortBasic {
    fn id(&self) -> &str {
        "heap_sort_basic"
    }
    fn name(&self) -> &str {
        "Heap Sort"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Implement heap sort to sort an array of integers in ascending order \
         using Tracked<i32>.\n\n\
         Build a max-heap, then repeatedly extract the max element.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(HeapSortBasicTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<HeapSortBasicTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();

        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let mut tracked = track_slice(&t.nums, shared_log.clone());
        solutions::heap_sort(&mut tracked);
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

// ═══════════════════════════════════════════════════════════════════════
// Easy 2: Kth Largest Element
// ═══════════════════════════════════════════════════════════════════════

struct KthLargest;
struct KthLargestTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for KthLargest {
    fn id(&self) -> &str {
        "heap_sort_kth_largest"
    }
    fn name(&self) -> &str {
        "Kth Largest Element"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an integer array `nums` and an integer `k`, return the kth largest \
         element in the array (1-indexed: k=1 means the largest).\n\n\
         Use a heap-based approach.\n\n\
         Constraints:\n\
         - 1 <= k <= nums.len() <= 100\n\
         - -1000 <= nums[i] <= 1000"
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
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_kth_largest(&t.nums, t.k);
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

// ═══════════════════════════════════════════════════════════════════════
// Easy 3: Last Stone Weight
// ═══════════════════════════════════════════════════════════════════════

struct LastStoneWeight;
struct LastStoneWeightTest {
    stones: Vec<i32>,
}

impl Problem for LastStoneWeight {
    fn id(&self) -> &str {
        "heap_sort_last_stone_weight"
    }
    fn name(&self) -> &str {
        "Last Stone Weight"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "You have a collection of stones, each with a positive integer weight.\n\n\
         Each turn, pick the two heaviest stones and smash them. If they have equal weight, \
         both are destroyed. Otherwise, the lighter stone is destroyed and the heavier stone \
         loses weight equal to the lighter stone.\n\n\
         Return the weight of the last remaining stone (or 0 if none remain).\n\n\
         Constraints:\n\
         - 1 <= stones.len() <= 30\n\
         - 1 <= stones[i] <= 100"
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
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_last_stone_weight(&t.stones);
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

// ═══════════════════════════════════════════════════════════════════════
// Easy 4: K Weakest Rows in a Binary Matrix
// ═══════════════════════════════════════════════════════════════════════

struct KWeakestRows;
struct KWeakestRowsTest {
    matrix: Vec<Vec<i32>>,
    k: usize,
}

impl Problem for KWeakestRows {
    fn id(&self) -> &str {
        "heap_sort_k_weakest_rows"
    }
    fn name(&self) -> &str {
        "K Weakest Rows in a Binary Matrix"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an m x n binary matrix `mat` where each row has 1s followed by 0s, \
         and an integer `k`, return the indices of the k weakest rows.\n\n\
         A row is weaker if it has fewer 1s. Ties are broken by row index.\n\n\
         Constraints:\n\
         - 2 <= m, n <= 20\n\
         - 1 <= k <= m\n\
         - mat[i][j] is 0 or 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let m = rng.random_range(2..=10);
                let n = rng.random_range(2..=10);
                let matrix: Vec<Vec<i32>> = (0..m)
                    .map(|_| {
                        let ones = rng.random_range(0..=n);
                        let mut row = vec![1; ones];
                        row.extend(vec![0; n - ones]);
                        row
                    })
                    .collect();
                let k = rng.random_range(1..=m);
                TestCase {
                    data: Box::new(KWeakestRowsTest { matrix, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KWeakestRowsTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_k_weakest_rows(&t.matrix, t.k);
        let tracked_matrix: Vec<Vec<_>> = t
            .matrix
            .iter()
            .map(|v| track_slice(v, shared_log.clone()))
            .collect();
        let actual = solutions::k_weakest_rows(&tracked_matrix, t.k);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("matrix={:?}, k={}", t.matrix, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_k_weakest_rows(matrix: &[Vec<i32>], k: usize) -> Vec<usize> {
    let mut strengths: Vec<(i32, usize)> = matrix
        .iter()
        .enumerate()
        .map(|(i, row)| (row.iter().sum::<i32>(), i))
        .collect();
    strengths.sort();
    strengths.iter().take(k).map(|&(_, idx)| idx).collect()
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 5: Relative Ranks
// ═══════════════════════════════════════════════════════════════════════

struct RelativeRanks;
struct RelativeRanksTest {
    scores: Vec<i32>,
}

impl Problem for RelativeRanks {
    fn id(&self) -> &str {
        "heap_sort_relative_ranks"
    }
    fn name(&self) -> &str {
        "Relative Ranks"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array of unique scores, return an array of ranks:\n\
         - 1st place: \"Gold Medal\"\n\
         - 2nd place: \"Silver Medal\"\n\
         - 3rd place: \"Bronze Medal\"\n\
         - 4th place onwards: \"4\", \"5\", ...\n\n\
         Constraints:\n\
         - 1 <= scores.len() <= 100\n\
         - 0 <= scores[i] <= 10000\n\
         - All scores are unique"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let scores = crate::problems::helpers::random_unique_vec(&mut rng, n, 0, 10000);
                TestCase {
                    data: Box::new(RelativeRanksTest { scores }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RelativeRanksTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_relative_ranks(&t.scores);
        let tracked_scores = track_slice(&t.scores, shared_log.clone());
        let actual = solutions::relative_ranks(&tracked_scores);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("scores={:?}", t.scores),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_relative_ranks(scores: &[i32]) -> Vec<String> {
    let mut indexed: Vec<(i32, usize)> = scores
        .iter()
        .copied()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();
    indexed.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    let mut result = vec![String::new(); scores.len()];
    for (rank, &(_, idx)) in indexed.iter().enumerate() {
        result[idx] = match rank {
            0 => "Gold Medal".to_string(),
            1 => "Silver Medal".to_string(),
            2 => "Bronze Medal".to_string(),
            r => (r + 1).to_string(),
        };
    }
    result
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 1: K Closest Points to Origin
// ═══════════════════════════════════════════════════════════════════════

struct KClosestPoints;
struct KClosestPointsTest {
    points: Vec<(i32, i32)>,
    k: usize,
}

impl Problem for KClosestPoints {
    fn id(&self) -> &str {
        "heap_sort_k_closest_points"
    }
    fn name(&self) -> &str {
        "K Closest Points to Origin"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array of points (x, y) and an integer k, return the k closest \
         points to the origin (0, 0).\n\n\
         Return the result sorted by distance (ascending). Ties broken by x then y.\n\n\
         Distance = x*x + y*y (no need for sqrt).\n\n\
         Constraints:\n\
         - 1 <= k <= points.len() <= 100\n\
         - -100 <= x, y <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let points: Vec<(i32, i32)> = (0..n)
                    .map(|_| (rng.random_range(-50..=50), rng.random_range(-50..=50)))
                    .collect();
                let k = rng.random_range(1..=n);
                TestCase {
                    data: Box::new(KClosestPointsTest { points, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KClosestPointsTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_k_closest(&t.points, t.k);
        let tracked_points: Vec<(crate::tracker::Tracked<i32>, crate::tracker::Tracked<i32>)> = t
            .points
            .iter()
            .enumerate()
            .map(|(i, &(a, b))| {
                (
                    crate::tracker::Tracked::new(a, i * 2, shared_log.clone()),
                    crate::tracker::Tracked::new(b, i * 2 + 1, shared_log.clone()),
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
    pts.sort_by_key(|&(x, y)| (x * x + y * y, x, y));
    pts.truncate(k);
    pts
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 2: Top K Frequent Elements
// ═══════════════════════════════════════════════════════════════════════

struct TopKFrequent;
struct TopKFrequentTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for TopKFrequent {
    fn id(&self) -> &str {
        "heap_sort_top_k_frequent"
    }
    fn name(&self) -> &str {
        "Top K Frequent Elements"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an integer array `nums` and an integer `k`, return the k most frequent \
         elements. Return them sorted by frequency (descending). Ties broken by value \
         (ascending).\n\n\
         Constraints:\n\
         - 1 <= k <= number of unique elements\n\
         - 1 <= nums.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
                let unique_count = nums.iter().collect::<std::collections::HashSet<_>>().len();
                let k = rng.random_range(1..=unique_count);
                TestCase {
                    data: Box::new(TopKFrequentTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TopKFrequentTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_top_k_frequent(&t.nums, t.k);
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
    let mut items: Vec<(i32, usize)> = freq.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    items.iter().take(k).map(|&(v, _)| v).collect()
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 3: Sort a Nearly Sorted (K-Sorted) Array
// ═══════════════════════════════════════════════════════════════════════

struct SortNearlySorted;
struct SortNearlySortedTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for SortNearlySorted {
    fn id(&self) -> &str {
        "heap_sort_sort_nearly_sorted"
    }
    fn name(&self) -> &str {
        "Sort a Nearly Sorted Array"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array where each element is at most `k` positions away from its \
         sorted position, sort the array efficiently using a min-heap of size k+1.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - 0 <= k <= nums.len() - 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                nums.sort();
                let k = rng.random_range(0..=std::cmp::min(5, n - 1));
                // Perturb: swap elements within k distance
                for i in 0..n {
                    let j_max = std::cmp::min(i + k, n - 1);
                    if j_max > i {
                        let j = rng.random_range(i..=j_max);
                        nums.swap(i, j);
                    }
                }
                TestCase {
                    data: Box::new(SortNearlySortedTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortNearlySortedTest>().unwrap();
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

// ═══════════════════════════════════════════════════════════════════════
// Medium 4: Merge K Sorted Lists
// ═══════════════════════════════════════════════════════════════════════

struct MergeKSorted;
struct MergeKSortedTest {
    lists: Vec<Vec<i32>>,
}

impl Problem for MergeKSorted {
    fn id(&self) -> &str {
        "heap_sort_merge_k_sorted"
    }
    fn name(&self) -> &str {
        "Merge K Sorted Lists"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given k sorted integer lists, merge them into one sorted list.\n\n\
         Use a min-heap to efficiently merge.\n\n\
         Constraints:\n\
         - 0 <= k <= 20\n\
         - 0 <= list[i].len() <= 30\n\
         - -1000 <= list[i][j] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let k = rng.random_range(0..=8);
                let lists: Vec<Vec<i32>> = (0..k)
                    .map(|_| {
                        let len = rng.random_range(0..=10);
                        let mut v: Vec<i32> =
                            (0..len).map(|_| rng.random_range(-100..=100)).collect();
                        v.sort();
                        v
                    })
                    .collect();
                TestCase {
                    data: Box::new(MergeKSortedTest { lists }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MergeKSortedTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let mut expected: Vec<i32> = t.lists.iter().flatten().copied().collect();
        expected.sort();
        let tracked_lists: Vec<Vec<_>> = t
            .lists
            .iter()
            .map(|v| track_slice(v, shared_log.clone()))
            .collect();
        let actual = solutions::merge_k_sorted(&tracked_lists);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("lists={:?}", t.lists),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 5: Task Scheduler
// ═══════════════════════════════════════════════════════════════════════

struct TaskScheduler;
struct TaskSchedulerTest {
    tasks: Vec<char>,
    n: i32,
}

impl Problem for TaskScheduler {
    fn id(&self) -> &str {
        "heap_sort_task_scheduler"
    }
    fn name(&self) -> &str {
        "Task Scheduler"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a list of tasks represented by characters and a cooldown period `n`, \
         find the minimum number of intervals the CPU needs to execute all tasks.\n\n\
         Between two same tasks, there must be at least `n` intervals (including idle).\n\n\
         Constraints:\n\
         - 1 <= tasks.len() <= 100\n\
         - tasks[i] is an uppercase English letter\n\
         - 0 <= n <= 10"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=30);
                let distinct = rng.random_range(1..=std::cmp::min(6, len));
                let chars: Vec<char> = (0..distinct).map(|i| (b'A' + i as u8) as char).collect();
                let tasks: Vec<char> = (0..len)
                    .map(|_| chars[rng.random_range(0..chars.len())])
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
    for &c in tasks {
        freq[(c as u8 - b'A') as usize] += 1;
    }
    let max_freq = *freq.iter().max().unwrap();
    let max_count = freq.iter().filter(|&&f| f == max_freq).count() as i32;
    let result = (max_freq - 1) * (n + 1) + max_count;
    std::cmp::max(result, tasks.len() as i32)
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 1: Find Median from Data Stream
// ═══════════════════════════════════════════════════════════════════════

struct FindMedianStream;
struct FindMedianStreamTest {
    nums: Vec<i32>,
}

impl Problem for FindMedianStream {
    fn id(&self) -> &str {
        "heap_sort_find_median_stream"
    }
    fn name(&self) -> &str {
        "Find Median from Data Stream"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a stream of integers, compute the running median after each insertion.\n\n\
         Return a Vec<f64> where result[i] is the median after inserting nums[0..=i].\n\n\
         Use two heaps: a max-heap for the lower half, a min-heap for the upper half.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(FindMedianStreamTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FindMedianStreamTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_find_median_stream(&t.nums);
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

fn ref_find_median_stream(nums: &[i32]) -> Vec<f64> {
    let mut sorted = Vec::new();
    let mut medians = Vec::new();
    for &n in nums {
        let pos = sorted.partition_point(|&x| x < n);
        sorted.insert(pos, n);
        let len = sorted.len();
        let median = if len % 2 == 1 {
            sorted[len / 2] as f64
        } else {
            (sorted[len / 2 - 1] as f64 + sorted[len / 2] as f64) / 2.0
        };
        medians.push(median);
    }
    medians
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 2: Sliding Window Median
// ═══════════════════════════════════════════════════════════════════════

struct SlidingWindowMedian;
struct SlidingWindowMedianTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for SlidingWindowMedian {
    fn id(&self) -> &str {
        "heap_sort_sliding_window_median"
    }
    fn name(&self) -> &str {
        "Sliding Window Median"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array `nums` and window size `k`, return the median of each sliding \
         window of size k as the window moves from left to right.\n\n\
         Return Vec<f64>.\n\n\
         Constraints:\n\
         - 1 <= k <= nums.len() <= 100\n\
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
                    data: Box::new(SlidingWindowMedianTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SlidingWindowMedianTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_sliding_window_median(&t.nums, t.k);
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
    for i in 0..=(nums.len() - k) {
        let mut window: Vec<i32> = nums[i..i + k].to_vec();
        window.sort();
        let median = if k % 2 == 1 {
            window[k / 2] as f64
        } else {
            (window[k / 2 - 1] as f64 + window[k / 2] as f64) / 2.0
        };
        result.push(median);
    }
    result
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 3: Trapping Rain Water II (2D Heightmap)
// ═══════════════════════════════════════════════════════════════════════

struct TrappingRainWaterII;
struct TrappingRainWaterIITest {
    heightmap: Vec<Vec<i32>>,
}

impl Problem for TrappingRainWaterII {
    fn id(&self) -> &str {
        "heap_sort_trapping_rain_water_ii"
    }
    fn name(&self) -> &str {
        "Trapping Rain Water II"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an m x n matrix of positive integers representing the height of each \
         cell in a 2D elevation map, compute the volume of water it can trap after raining.\n\n\
         Use a min-heap (priority queue) BFS approach from the borders inward.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 15\n\
         - 0 <= heightmap[i][j] <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let m = rng.random_range(3..=8);
                let n = rng.random_range(3..=8);
                let heightmap: Vec<Vec<i32>> = (0..m)
                    .map(|_| (0..n).map(|_| rng.random_range(0..=20)).collect())
                    .collect();
                TestCase {
                    data: Box::new(TrappingRainWaterIITest { heightmap }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TrappingRainWaterIITest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_trap_rain_water_ii(&t.heightmap);
        let tracked_heightmap: Vec<Vec<_>> = t
            .heightmap
            .iter()
            .map(|v| track_slice(v, shared_log.clone()))
            .collect();
        let actual = solutions::trapping_rain_water_ii(&tracked_heightmap);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("heightmap={:?}", t.heightmap),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_trap_rain_water_ii(heightmap: &[Vec<i32>]) -> i32 {
    use std::cmp::Reverse;
    let m = heightmap.len();
    if m < 3 {
        return 0;
    }
    let n = heightmap[0].len();
    if n < 3 {
        return 0;
    }
    let mut visited = vec![vec![false; n]; m];
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

    for r in 0..m {
        for c in 0..n {
            if r == 0 || r == m - 1 || c == 0 || c == n - 1 {
                heap.push(Reverse((heightmap[r][c], r, c)));
                visited[r][c] = true;
            }
        }
    }

    let dirs = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
    let mut water = 0;
    let mut max_height = 0;

    while let Some(Reverse((h, r, c))) = heap.pop() {
        max_height = std::cmp::max(max_height, h);
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                if !visited[nr][nc] {
                    visited[nr][nc] = true;
                    if heightmap[nr][nc] < max_height {
                        water += max_height - heightmap[nr][nc];
                    }
                    heap.push(Reverse((heightmap[nr][nc], nr, nc)));
                }
            }
        }
    }
    water
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 4: Smallest Range Covering Elements from K Lists
// ═══════════════════════════════════════════════════════════════════════

struct SmallestRange;
struct SmallestRangeTest {
    lists: Vec<Vec<i32>>,
}

impl Problem for SmallestRange {
    fn id(&self) -> &str {
        "heap_sort_smallest_range"
    }
    fn name(&self) -> &str {
        "Smallest Range Covering Elements from K Lists"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given k sorted lists of integers, find the smallest range [a, b] such that \
         at least one number from each list is included in the range.\n\n\
         Return (a, b). If multiple ranges have the same size, return the one with \
         the smallest `a`.\n\n\
         Constraints:\n\
         - 1 <= k <= 10\n\
         - 1 <= list[i].len() <= 20\n\
         - -1000 <= list[i][j] <= 1000\n\
         - Each list is sorted in ascending order"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let k = rng.random_range(1..=5);
                let lists: Vec<Vec<i32>> = (0..k)
                    .map(|_| {
                        let len = rng.random_range(1..=10);
                        let mut v: Vec<i32> =
                            (0..len).map(|_| rng.random_range(-100..=100)).collect();
                        v.sort();
                        v.dedup();
                        if v.is_empty() {
                            v.push(rng.random_range(-100..=100));
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
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_smallest_range(&t.lists);
        let tracked_lists: Vec<Vec<_>> = t
            .lists
            .iter()
            .map(|v| track_slice(v, shared_log.clone()))
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
    use std::cmp::Reverse;
    let k = lists.len();
    // Min-heap: (value, list_index, element_index)
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
    let mut cur_max = i32::MIN;

    for (i, list) in lists.iter().enumerate() {
        heap.push(Reverse((list[0], i, 0)));
        cur_max = std::cmp::max(cur_max, list[0]);
    }

    let mut best = (i32::MIN / 2, i32::MAX / 2);

    while heap.len() == k {
        let Reverse((cur_min, list_idx, elem_idx)) = heap.pop().unwrap();
        if cur_max - cur_min < best.1 - best.0
            || (cur_max - cur_min == best.1 - best.0 && cur_min < best.0)
        {
            best = (cur_min, cur_max);
        }
        if elem_idx + 1 < lists[list_idx].len() {
            let next_val = lists[list_idx][elem_idx + 1];
            heap.push(Reverse((next_val, list_idx, elem_idx + 1)));
            cur_max = std::cmp::max(cur_max, next_val);
        }
        // If any list is exhausted, we stop (heap.len() < k)
    }

    best
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 5: IPO
// ═══════════════════════════════════════════════════════════════════════

struct Ipo;
struct IPOTest {
    k: usize,
    w: i32,
    profits: Vec<i32>,
    capital: Vec<i32>,
}

impl Problem for Ipo {
    fn id(&self) -> &str {
        "heap_sort_ipo"
    }
    fn name(&self) -> &str {
        "IPO"
    }
    fn topic(&self) -> &str {
        "heap_sort"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "You are given `k` projects you can finish at most, initial capital `w`, \
         arrays `profits` and `capital` where profits[i] is the profit and capital[i] \
         is the minimum capital needed to start project i.\n\n\
         After completing a project, your capital increases by its profit. Maximize \
         your final capital.\n\n\
         Return the maximized capital.\n\n\
         Constraints:\n\
         - 1 <= k <= 100\n\
         - 0 <= w <= 10000\n\
         - profits.len() == capital.len() <= 100\n\
         - 0 <= profits[i], capital[i] <= 10000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let k = rng.random_range(1..=n);
                let w = rng.random_range(0..=50);
                let profits: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                let capital: Vec<i32> = (0..n).map(|_| rng.random_range(0..=50)).collect();
                TestCase {
                    data: Box::new(IPOTest {
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
        let t = test.data.downcast_ref::<IPOTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_ipo(t.k, t.w, &t.profits, &t.capital);
        let tracked_profits = track_slice(&t.profits, shared_log.clone());
        let tracked_capital = track_slice(&t.capital, shared_log.clone());
        let actual = solutions::ipo(t.k, t.w, &tracked_profits, &tracked_capital);
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

    let mut cur_w = w;
    let mut profit_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut idx = 0;

    for _ in 0..k {
        while idx < n && projects[idx].0 <= cur_w {
            profit_heap.push(projects[idx].1);
            idx += 1;
        }
        if let Some(best_profit) = profit_heap.pop() {
            cur_w += best_profit;
        } else {
            break;
        }
    }
    cur_w
}
