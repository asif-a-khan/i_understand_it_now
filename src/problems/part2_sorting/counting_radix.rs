use rand::Rng;
use std::collections::HashMap;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part2_sorting::counting_radix as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(CountingSortBasic),
        Box::new(SortColors),
        Box::new(RelativeSort),
        Box::new(HeightChecker),
        Box::new(SortByFrequency),
        Box::new(RadixSortBasic),
        Box::new(MaximumGap),
        Box::new(BucketSortBasic),
        Box::new(TopKFrequentWords),
        Box::new(ReorganizeString),
        Box::new(RadixSortMaxGap),
        Box::new(SmallestMissingPositive),
        Box::new(CreateMaximumNumber),
        Box::new(RadixSortSuffixArray),
        Box::new(SortTransformed),
    ]
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 1: Counting Sort Basic
// ═══════════════════════════════════════════════════════════════════════

struct CountingSortBasic;
struct CountingSortBasicTest {
    nums: Vec<i32>,
}

impl Problem for CountingSortBasic {
    fn id(&self) -> &str {
        "counting_sort_basic"
    }
    fn name(&self) -> &str {
        "Counting Sort"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Implement counting sort for non-negative integers.\n\n\
         Return a new sorted vector.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - 0 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                TestCase {
                    data: Box::new(CountingSortBasicTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountingSortBasicTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();
        let actual = solutions::counting_sort(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 2: Sort Colors (Dutch National Flag)
// ═══════════════════════════════════════════════════════════════════════

struct SortColors;
struct SortColorsTest {
    nums: Vec<i32>,
}

impl Problem for SortColors {
    fn id(&self) -> &str {
        "counting_sort_sort_colors"
    }
    fn name(&self) -> &str {
        "Sort Colors"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array containing only 0s, 1s, and 2s, sort it in-place.\n\n\
         Return the sorted array.\n\n\
         Hint: use counting sort or the Dutch National Flag algorithm.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - nums[i] is 0, 1, or 2"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
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

// ═══════════════════════════════════════════════════════════════════════
// Easy 3: Relative Sort Array
// ═══════════════════════════════════════════════════════════════════════

struct RelativeSort;
struct RelativeSortTest {
    arr1: Vec<i32>,
    arr2: Vec<i32>,
}

impl Problem for RelativeSort {
    fn id(&self) -> &str {
        "counting_sort_relative_sort"
    }
    fn name(&self) -> &str {
        "Relative Sort Array"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Sort `arr1` such that the relative ordering of items in `arr1` matches `arr2`.\n\n\
         Elements not in `arr2` should be placed at the end in ascending order.\n\n\
         Constraints:\n\
         - 1 <= arr1.len() <= 100\n\
         - 0 <= arr2.len() <= arr1.len()\n\
         - All elements of arr2 are distinct and present in arr1\n\
         - 0 <= arr1[i], arr2[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let arr1: Vec<i32> = (0..n).map(|_| rng.random_range(0..=30)).collect();
                // arr2 is a subset of unique values from arr1
                let mut unique: Vec<i32> = arr1
                    .iter()
                    .copied()
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();
                // Shuffle unique
                for i in (1..unique.len()).rev() {
                    let j = rng.random_range(0..=i);
                    unique.swap(i, j);
                }
                let take = rng.random_range(0..=unique.len());
                let arr2: Vec<i32> = unique[..take].to_vec();
                TestCase {
                    data: Box::new(RelativeSortTest { arr1, arr2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RelativeSortTest>().unwrap();
        let expected = ref_relative_sort(&t.arr1, &t.arr2);
        let actual = solutions::relative_sort(&t.arr1, &t.arr2);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("arr1={:?}, arr2={:?}", t.arr1, t.arr2),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_relative_sort(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let order: HashMap<i32, usize> = arr2.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let mut arr = arr1.to_vec();
    arr.sort_by(|a, b| match (order.get(a), order.get(b)) {
        (Some(ia), Some(ib)) => ia.cmp(ib),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.cmp(b),
    });
    arr
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 4: Height Checker
// ═══════════════════════════════════════════════════════════════════════

struct HeightChecker;
struct HeightCheckerTest {
    heights: Vec<i32>,
}

impl Problem for HeightChecker {
    fn id(&self) -> &str {
        "counting_sort_height_checker"
    }
    fn name(&self) -> &str {
        "Height Checker"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Students are asked to stand in non-decreasing order of height. \
         Return the number of students not standing in the correct position.\n\n\
         Hint: use counting sort to get the expected order, then compare.\n\n\
         Constraints:\n\
         - 1 <= heights.len() <= 100\n\
         - 1 <= heights[i] <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let heights: Vec<i32> = (0..n).map(|_| rng.random_range(1..=100)).collect();
                TestCase {
                    data: Box::new(HeightCheckerTest { heights }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<HeightCheckerTest>().unwrap();
        let mut sorted = t.heights.clone();
        sorted.sort();
        let expected: i32 = t
            .heights
            .iter()
            .zip(sorted.iter())
            .filter(|(a, b)| a != b)
            .count() as i32;
        let actual = solutions::height_checker(&t.heights);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("heights={:?}", t.heights),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Easy 5: Sort Characters By Frequency
// ═══════════════════════════════════════════════════════════════════════

struct SortByFrequency;
struct SortByFrequencyTest {
    s: String,
}

impl Problem for SortByFrequency {
    fn id(&self) -> &str {
        "counting_sort_sort_by_frequency"
    }
    fn name(&self) -> &str {
        "Sort Characters By Frequency"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a string, sort its characters by frequency (most frequent first).\n\n\
         Characters with the same frequency should be grouped together. \
         Ties between different characters broken by character value (ascending).\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 100\n\
         - s consists of lowercase English letters"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=30);
                let s = crate::problems::helpers::random_string(&mut rng, len);
                TestCase {
                    data: Box::new(SortByFrequencyTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortByFrequencyTest>().unwrap();
        let expected = ref_sort_by_frequency(&t.s);
        let actual = solutions::sort_by_frequency(&t.s);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("\"{expected}\""),
            actual: format!("\"{actual}\""),
        }
    }
}

fn ref_sort_by_frequency(s: &str) -> String {
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| freq[b].cmp(&freq[a]).then(a.cmp(b)));
    chars.into_iter().collect()
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 1: Radix Sort Basic
// ═══════════════════════════════════════════════════════════════════════

struct RadixSortBasic;
struct RadixSortBasicTest {
    nums: Vec<i32>,
}

impl Problem for RadixSortBasic {
    fn id(&self) -> &str {
        "radix_sort_basic"
    }
    fn name(&self) -> &str {
        "Radix Sort"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Implement radix sort (LSD) for non-negative integers.\n\n\
         Process digits from least significant to most significant, using \
         counting sort as a stable subroutine at each digit position.\n\n\
         Return a new sorted vector.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - 0 <= nums[i] <= 100000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=10000)).collect();
                TestCase {
                    data: Box::new(RadixSortBasicTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RadixSortBasicTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort();
        let actual = solutions::radix_sort(&t.nums);
        SolutionResult {
            is_correct: actual == expected,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 2: Maximum Gap
// ═══════════════════════════════════════════════════════════════════════

struct MaximumGap;
struct MaximumGapTest {
    nums: Vec<i32>,
}

impl Problem for MaximumGap {
    fn id(&self) -> &str {
        "counting_sort_maximum_gap"
    }
    fn name(&self) -> &str {
        "Maximum Gap"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an unsorted array of non-negative integers, find the maximum difference \
         between successive elements in the sorted form.\n\n\
         Return 0 if the array has fewer than 2 elements.\n\n\
         Must run in O(n) time and O(n) space. Hint: use a bucket/pigeonhole approach.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - 0 <= nums[i] <= 10000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=1000)).collect();
                TestCase {
                    data: Box::new(MaximumGapTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaximumGapTest>().unwrap();
        let expected = ref_maximum_gap(&t.nums);
        let actual = solutions::maximum_gap(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_maximum_gap(nums: &[i32]) -> i32 {
    if nums.len() < 2 {
        return 0;
    }
    let mut sorted = nums.to_vec();
    sorted.sort();
    sorted.windows(2).map(|w| w[1] - w[0]).max().unwrap_or(0)
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 3: Bucket Sort for Floats
// ═══════════════════════════════════════════════════════════════════════

struct BucketSortBasic;
struct BucketSortBasicTest {
    nums: Vec<f64>,
}

impl Problem for BucketSortBasic {
    fn id(&self) -> &str {
        "bucket_sort_basic"
    }
    fn name(&self) -> &str {
        "Bucket Sort"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Implement bucket sort for floating-point numbers in the range [0.0, 1.0).\n\n\
         Distribute elements into n buckets, sort each bucket, then concatenate.\n\n\
         Return a new sorted vector.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - 0.0 <= nums[i] < 1.0"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<f64> = (0..n)
                    .map(|_| rng.random_range(0..1000) as f64 / 1000.0)
                    .collect();
                TestCase {
                    data: Box::new(BucketSortBasicTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BucketSortBasicTest>().unwrap();
        let mut expected = t.nums.clone();
        expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let actual = solutions::bucket_sort(&t.nums);
        let is_correct = expected.len() == actual.len()
            && expected
                .iter()
                .zip(actual.iter())
                .all(|(e, a)| (e - a).abs() < 1e-10);
        SolutionResult {
            is_correct,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 4: Top K Frequent Words
// ═══════════════════════════════════════════════════════════════════════

struct TopKFrequentWords;
struct TopKFrequentWordsTest {
    words: Vec<String>,
    k: usize,
}

impl Problem for TopKFrequentWords {
    fn id(&self) -> &str {
        "counting_sort_top_k_frequent_words"
    }
    fn name(&self) -> &str {
        "Top K Frequent Words"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a list of words and an integer k, return the k most frequent words.\n\n\
         Sort by frequency (descending). Ties broken by lexicographic order (ascending).\n\n\
         Constraints:\n\
         - 1 <= words.len() <= 100\n\
         - 1 <= k <= number of unique words\n\
         - Each word consists of lowercase English letters"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let vocab_size = rng.random_range(1..=8);
                let vocab: Vec<String> = (0..vocab_size)
                    .map(|_| {
                        let len = rng.random_range(1..=6);
                        crate::problems::helpers::random_string(&mut rng, len)
                    })
                    .collect();
                let n = rng.random_range(1..=30);
                let words: Vec<String> = (0..n)
                    .map(|_| vocab[rng.random_range(0..vocab.len())].clone())
                    .collect();
                let unique_count = words.iter().collect::<std::collections::HashSet<_>>().len();
                let k = rng.random_range(1..=unique_count);
                TestCase {
                    data: Box::new(TopKFrequentWordsTest { words, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TopKFrequentWordsTest>().unwrap();
        let expected = ref_top_k_frequent_words(&t.words, t.k);
        let actual = solutions::top_k_frequent_words(&t.words, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("words={:?}, k={}", t.words, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_top_k_frequent_words(words: &[String], k: usize) -> Vec<String> {
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for w in words {
        *freq.entry(w.as_str()).or_insert(0) += 1;
    }
    let mut items: Vec<(&str, usize)> = freq.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    items.iter().take(k).map(|(w, _)| w.to_string()).collect()
}

// ═══════════════════════════════════════════════════════════════════════
// Medium 5: Reorganize String
// ═══════════════════════════════════════════════════════════════════════

struct ReorganizeString;
struct ReorganizeStringTest {
    s: String,
}

impl Problem for ReorganizeString {
    fn id(&self) -> &str {
        "counting_sort_reorganize_string"
    }
    fn name(&self) -> &str {
        "Reorganize String"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a string s, rearrange the characters so that no two adjacent \
         characters are the same.\n\n\
         Return any valid rearrangement, or an empty string if impossible.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 100\n\
         - s consists of lowercase English letters"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=20);
                let distinct = rng.random_range(1..=std::cmp::min(5, len));
                let chars: Vec<u8> = (0..distinct).map(|i| b'a' + i as u8).collect();
                let s: String = (0..len)
                    .map(|_| chars[rng.random_range(0..chars.len())] as char)
                    .collect();
                TestCase {
                    data: Box::new(ReorganizeStringTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReorganizeStringTest>().unwrap();
        let is_possible = ref_is_reorganize_possible(&t.s);
        let actual = solutions::reorganize_string(&t.s);

        if !is_possible {
            let is_correct = actual.is_empty();
            return SolutionResult {
                is_correct,
                input_description: format!("s=\"{}\"", t.s),
                expected: "\"\" (impossible)".to_string(),
                actual: format!("\"{actual}\""),
            };
        }

        // Verify the result is valid
        let valid_len = actual.len() == t.s.len();
        let valid_chars = {
            let mut orig: Vec<char> = t.s.chars().collect();
            let mut res: Vec<char> = actual.chars().collect();
            orig.sort();
            res.sort();
            orig == res
        };
        let valid_no_adjacent = actual.as_bytes().windows(2).all(|w| w[0] != w[1]);

        SolutionResult {
            is_correct: valid_len && valid_chars && valid_no_adjacent,
            input_description: format!("s=\"{}\"", t.s),
            expected: "any valid rearrangement with no adjacent duplicates".to_string(),
            actual: format!("\"{actual}\""),
        }
    }
}

fn ref_is_reorganize_possible(s: &str) -> bool {
    let mut freq = [0usize; 26];
    for b in s.bytes() {
        freq[(b - b'a') as usize] += 1;
    }
    let max_freq = *freq.iter().max().unwrap();
    max_freq <= s.len().div_ceil(2)
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 1: Max Gap using Radix/Bucket Sort
// ═══════════════════════════════════════════════════════════════════════

struct RadixSortMaxGap;
struct RadixSortMaxGapTest {
    nums: Vec<i32>,
}

impl Problem for RadixSortMaxGap {
    fn id(&self) -> &str {
        "radix_sort_max_gap"
    }
    fn name(&self) -> &str {
        "Maximum Gap (Radix Sort)"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an unsorted array of non-negative integers, find the maximum difference \
         between successive elements in sorted form using a radix sort or bucket sort \
         approach.\n\n\
         Unlike the medium version, this must be solved using radix or bucket sort \
         concepts (not just sorting). Use the pigeonhole principle: the max gap is \
         at least ceil((max-min)/(n-1)), so gaps within a bucket can be ignored.\n\n\
         Return 0 if the array has fewer than 2 elements.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - 0 <= nums[i] <= 100000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=10000)).collect();
                TestCase {
                    data: Box::new(RadixSortMaxGapTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RadixSortMaxGapTest>().unwrap();
        let expected = ref_maximum_gap(&t.nums);
        let actual = solutions::radix_sort_max_gap(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 2: First Missing Positive (Counting Approach)
// ═══════════════════════════════════════════════════════════════════════

struct SmallestMissingPositive;
struct SmallestMissingPositiveTest {
    nums: Vec<i32>,
}

impl Problem for SmallestMissingPositive {
    fn id(&self) -> &str {
        "counting_sort_smallest_missing_positive"
    }
    fn name(&self) -> &str {
        "First Missing Positive"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an unsorted integer array, return the smallest missing positive integer.\n\n\
         Use a counting/placement approach: place each value v at index v-1 \
         (like an in-place counting sort for positives), then scan for the first \
         position where nums[i] != i+1.\n\n\
         Must run in O(n) time and O(1) extra space.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - -100 <= nums[i] <= 200"
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
                    data: Box::new(SmallestMissingPositiveTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<SmallestMissingPositiveTest>()
            .unwrap();
        let set: std::collections::HashSet<i32> = t.nums.iter().copied().collect();
        let mut expected = 1;
        while set.contains(&expected) {
            expected += 1;
        }
        let actual = solutions::smallest_missing_positive(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 3: Create Maximum Number
// ═══════════════════════════════════════════════════════════════════════

struct CreateMaximumNumber;
struct CreateMaximumNumberTest {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    k: usize,
}

impl Problem for CreateMaximumNumber {
    fn id(&self) -> &str {
        "counting_sort_create_maximum_number"
    }
    fn name(&self) -> &str {
        "Create Maximum Number"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given two arrays of digits and an integer k, create the maximum number of \
         length k by selecting digits from both arrays while preserving relative order.\n\n\
         Return the result as Vec<i32>.\n\n\
         Constraints:\n\
         - 0 <= nums1[i], nums2[i] <= 9\n\
         - 1 <= k <= nums1.len() + nums2.len() <= 40"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n1 = rng.random_range(1..=10);
                let n2 = rng.random_range(1..=10);
                let nums1: Vec<i32> = (0..n1).map(|_| rng.random_range(0..=9)).collect();
                let nums2: Vec<i32> = (0..n2).map(|_| rng.random_range(0..=9)).collect();
                let k = rng.random_range(1..=n1 + n2);
                TestCase {
                    data: Box::new(CreateMaximumNumberTest { nums1, nums2, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CreateMaximumNumberTest>().unwrap();
        let expected = ref_create_max_number(&t.nums1, &t.nums2, t.k);
        let actual = solutions::create_maximum_number(&t.nums1, &t.nums2, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums1={:?}, nums2={:?}, k={}", t.nums1, t.nums2, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_max_subsequence(nums: &[i32], k: usize) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut drop = nums.len() - k;
    for &n in nums {
        while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < n {
            stack.pop();
            drop -= 1;
        }
        stack.push(n);
    }
    stack.truncate(k);
    stack
}

fn ref_merge_sequences(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < a.len() || j < b.len() {
        if i >= a.len() {
            result.push(b[j]);
            j += 1;
        } else if j >= b.len() || a[i..] > b[j..] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }
    result
}

fn ref_create_max_number(nums1: &[i32], nums2: &[i32], k: usize) -> Vec<i32> {
    let n1 = nums1.len();
    let n2 = nums2.len();
    let mut best = Vec::new();
    let lo = k.saturating_sub(n2);
    let hi = std::cmp::min(k, n1);
    for i in lo..=hi {
        let sub1 = ref_max_subsequence(nums1, i);
        let sub2 = ref_max_subsequence(nums2, k - i);
        let merged = ref_merge_sequences(&sub1, &sub2);
        if merged > best {
            best = merged;
        }
    }
    best
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 4: Suffix Array via Radix Sort
// ═══════════════════════════════════════════════════════════════════════

struct RadixSortSuffixArray;
struct RadixSortSuffixArrayTest {
    s: String,
}

impl Problem for RadixSortSuffixArray {
    fn id(&self) -> &str {
        "radix_sort_suffix_array"
    }
    fn name(&self) -> &str {
        "Suffix Array"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Build a suffix array for the given string using radix sort.\n\n\
         A suffix array is a sorted array of all suffix indices. suffix_array[i] \
         gives the starting index of the i-th lexicographically smallest suffix.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 100\n\
         - s consists of lowercase English letters"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=20);
                let s = crate::problems::helpers::random_string(&mut rng, len);
                TestCase {
                    data: Box::new(RadixSortSuffixArrayTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<RadixSortSuffixArrayTest>()
            .unwrap();
        let expected = ref_suffix_array(&t.s);
        let actual = solutions::suffix_array(&t.s);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_suffix_array(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&a, &b| s[a..].cmp(&s[b..]));
    sa
}

// ═══════════════════════════════════════════════════════════════════════
// Hard 5: Sort Transformed Array
// ═══════════════════════════════════════════════════════════════════════

struct SortTransformed;
struct SortTransformedTest {
    nums: Vec<i32>,
    a: i32,
    b: i32,
    c: i32,
}

impl Problem for SortTransformed {
    fn id(&self) -> &str {
        "counting_sort_sort_transformed"
    }
    fn name(&self) -> &str {
        "Sort Transformed Array"
    }
    fn topic(&self) -> &str {
        "counting_radix"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a sorted integer array `nums` and integers a, b, c, apply the \
         function f(x) = a*x*x + b*x + c to each element, and return the result \
         in sorted order.\n\n\
         Hint: if a >= 0 the parabola opens upward (ends are largest), if a < 0 \
         it opens downward (ends are smallest). Use two pointers.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - nums is sorted in ascending order\n\
         - -10 <= a <= 10\n\
         - -100 <= b <= 100\n\
         - -1000 <= c <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                nums.sort();
                let a = rng.random_range(-5..=5);
                let b = rng.random_range(-20..=20);
                let c = rng.random_range(-100..=100);
                TestCase {
                    data: Box::new(SortTransformedTest { nums, a, b, c }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortTransformedTest>().unwrap();
        let expected = ref_sort_transformed(&t.nums, t.a, t.b, t.c);
        let actual = solutions::sort_transformed(&t.nums, t.a, t.b, t.c);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, a={}, b={}, c={}", t.nums, t.a, t.b, t.c),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_sort_transformed(nums: &[i32], a: i32, b: i32, c: i32) -> Vec<i32> {
    let f = |x: i32| -> i32 { a * x * x + b * x + c };
    let mut result: Vec<i32> = nums.iter().map(|&x| f(x)).collect();
    result.sort();
    result
}
