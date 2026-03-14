use rand::Rng;
use std::collections::{HashMap, HashSet};

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part1_foundations::hash_maps as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy (5)
        Box::new(ContainsDuplicate),
        Box::new(SingleNumber),
        Box::new(Intersection),
        Box::new(HappyNumber),
        Box::new(IsomorphicStrings),
        // Medium (5)
        Box::new(GroupAnagrams),
        Box::new(TopKFrequent),
        Box::new(LongestConsecutive),
        Box::new(SubarraySumK),
        Box::new(EncodeDecode),
        // Hard (5)
        Box::new(MinWindowSubstring),
        Box::new(LongestKDistinct),
        Box::new(AlienDictionary),
        Box::new(AllO1),
        Box::new(MaxPointsLine),
    ]
}

// ── Easy 1: Contains Duplicate ────────────────────────────────────────

struct ContainsDuplicate;
struct ContainsDuplicateTest {
    nums: Vec<i32>,
}

impl Problem for ContainsDuplicate {
    fn id(&self) -> &str { "hash_maps_contains_duplicate" }
    fn name(&self) -> &str { "Contains Duplicate" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given an integer array `nums`, return `true` if any value appears at least twice, \
         and `false` if every element is distinct.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 1000\n\
         - -1000 <= nums[i] <= 1000\n\n\
         Use a HashSet for O(n) solution."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
            TestCase { data: Box::new(ContainsDuplicateTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ContainsDuplicateTest>().unwrap();
        let expected = ref_contains_duplicate(&t.nums);
        let actual = solutions::contains_duplicate(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_contains_duplicate(nums: &[i32]) -> bool {
    let mut seen = HashSet::new();
    nums.iter().any(|x| !seen.insert(x))
}

// ── Easy 2: Single Number ─────────────────────────────────────────────

struct SingleNumber;
struct SingleNumberTest {
    nums: Vec<i32>,
}

impl Problem for SingleNumber {
    fn id(&self) -> &str { "hash_maps_single_number" }
    fn name(&self) -> &str { "Single Number" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a non-empty array of integers `nums`, every element appears twice except \
         for one. Find that single one.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 1000 (always odd length)\n\
         - Every element appears exactly twice except for one element which appears once.\n\n\
         Hint: XOR all elements together, or use a HashSet."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let pair_count = rng.random_range(1..=15);
            let mut nums: Vec<i32> = Vec::with_capacity(pair_count * 2 + 1);
            for _ in 0..pair_count {
                let v = rng.random_range(-500..=500);
                nums.push(v);
                nums.push(v);
            }
            // Add the unique element
            let unique = rng.random_range(-500..=500);
            nums.push(unique);
            // Shuffle
            for i in (1..nums.len()).rev() {
                let j = rng.random_range(0..=i);
                nums.swap(i, j);
            }
            TestCase { data: Box::new(SingleNumberTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SingleNumberTest>().unwrap();
        let expected = ref_single_number(&t.nums);
        let actual = solutions::single_number(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_single_number(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &x| acc ^ x)
}

// ── Easy 3: Intersection of Two Arrays ────────────────────────────────

struct Intersection;
struct IntersectionTest {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

impl Problem for Intersection {
    fn id(&self) -> &str { "hash_maps_intersection" }
    fn name(&self) -> &str { "Intersection of Two Arrays" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given two integer arrays `nums1` and `nums2`, return an array of their intersection. \
         Each element in the result must be unique. Return the result sorted in ascending order.\n\n\
         Constraints:\n\
         - 1 <= nums1.len(), nums2.len() <= 1000\n\
         - 0 <= nums1[i], nums2[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n1 = rng.random_range(1..=20);
            let n2 = rng.random_range(1..=20);
            let nums1: Vec<i32> = (0..n1).map(|_| rng.random_range(0..=50)).collect();
            let nums2: Vec<i32> = (0..n2).map(|_| rng.random_range(0..=50)).collect();
            TestCase { data: Box::new(IntersectionTest { nums1, nums2 }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IntersectionTest>().unwrap();
        let expected = ref_intersection(&t.nums1, &t.nums2);
        let actual = solutions::intersection(&t.nums1, &t.nums2);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums1={:?}, nums2={:?}", t.nums1, t.nums2),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_intersection(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let set1: HashSet<i32> = nums1.iter().copied().collect();
    let mut result: Vec<i32> = nums2.iter().copied().filter(|x| set1.contains(x)).collect();
    result.sort();
    result.dedup();
    result
}

// ── Easy 4: Happy Number ─────────────────────────────────────────────

struct HappyNumber;
struct HappyNumberTest {
    n: i32,
}

impl Problem for HappyNumber {
    fn id(&self) -> &str { "hash_maps_happy_number" }
    fn name(&self) -> &str { "Happy Number" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Determine if a number `n` is happy.\n\n\
         A happy number is defined by the following process:\n\
         - Starting with any positive integer, replace the number by the sum of the squares \
           of its digits.\n\
         - Repeat until the number equals 1 (happy) or loops endlessly in a cycle (not happy).\n\n\
         Constraints:\n\
         - 1 <= n <= 2^31 - 1\n\n\
         Hint: Use a HashSet to detect cycles."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=10000);
            TestCase { data: Box::new(HappyNumberTest { n }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<HappyNumberTest>().unwrap();
        let expected = ref_happy_number(t.n);
        let actual = solutions::is_happy(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_happy_number(n: i32) -> bool {
    let mut seen = HashSet::new();
    let mut current = n;
    loop {
        if current == 1 { return true; }
        if !seen.insert(current) { return false; }
        let mut sum = 0;
        let mut x = current;
        while x > 0 {
            let d = x % 10;
            sum += d * d;
            x /= 10;
        }
        current = sum;
    }
}

// ── Easy 5: Isomorphic Strings ───────────────────────────────────────

struct IsomorphicStrings;
struct IsomorphicTest {
    s: String,
    t: String,
}

impl Problem for IsomorphicStrings {
    fn id(&self) -> &str { "hash_maps_isomorphic_strings" }
    fn name(&self) -> &str { "Isomorphic Strings" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given two strings `s` and `t`, determine if they are isomorphic.\n\n\
         Two strings are isomorphic if the characters in `s` can be replaced to get `t`, \
         with a consistent one-to-one mapping. No two characters may map to the same character, \
         but a character may map to itself.\n\n\
         Constraints:\n\
         - 1 <= s.len() == t.len() <= 1000\n\
         - `s` and `t` consist of lowercase ASCII letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let len = rng.random_range(1..=20);
            let alphabet_size = rng.random_range(2..=8usize);
            let s: String = (0..len)
                .map(|_| (b'a' + rng.random_range(0..alphabet_size as u8)) as char)
                .collect();
            // Sometimes generate isomorphic, sometimes not
            let t = if rng.random_range(0..2) == 0 {
                // Create isomorphic mapping
                let mut mapping: HashMap<u8, u8> = HashMap::new();
                let mut used: HashSet<u8> = HashSet::new();
                let mut result = Vec::with_capacity(len);
                for &b in s.as_bytes() {
                    let mapped = *mapping.entry(b).or_insert_with(|| {
                        let mut c = b'a' + rng.random_range(0..26u8);
                        while used.contains(&c) {
                            c = b'a' + rng.random_range(0..26u8);
                        }
                        used.insert(c);
                        c
                    });
                    result.push(mapped);
                }
                String::from_utf8(result).unwrap()
            } else {
                // Random string (may or may not be isomorphic)
                (0..len)
                    .map(|_| (b'a' + rng.random_range(0..alphabet_size as u8)) as char)
                    .collect()
            };
            TestCase { data: Box::new(IsomorphicTest { s, t }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IsomorphicTest>().unwrap();
        let expected = ref_isomorphic(&t.s, &t.t);
        let actual = solutions::is_isomorphic(&t.s, &t.t);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\", t=\"{}\"", t.s, t.t),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_isomorphic(s: &str, t: &str) -> bool {
    if s.len() != t.len() { return false; }
    let mut s_to_t: HashMap<u8, u8> = HashMap::new();
    let mut t_to_s: HashMap<u8, u8> = HashMap::new();
    for (&sb, &tb) in s.as_bytes().iter().zip(t.as_bytes()) {
        match s_to_t.get(&sb) {
            Some(&mapped) if mapped != tb => return false,
            None => {
                if let Some(&mapped_back) = t_to_s.get(&tb) {
                    if mapped_back != sb { return false; }
                }
                s_to_t.insert(sb, tb);
                t_to_s.insert(tb, sb);
            }
            _ => {}
        }
    }
    true
}

// ── Medium 1: Group Anagrams ──────────────────────────────────────────

struct GroupAnagrams;
struct GroupAnagramsTest {
    strs: Vec<String>,
}

impl Problem for GroupAnagrams {
    fn id(&self) -> &str { "hash_maps_group_anagrams" }
    fn name(&self) -> &str { "Group Anagrams" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an array of strings `strs`, group the anagrams together. \
         Return a list of groups where each group is sorted alphabetically, \
         and the groups themselves are sorted by their first element.\n\n\
         Two strings are anagrams if they contain the same characters with the same frequencies.\n\n\
         Constraints:\n\
         - 1 <= strs.len() <= 100\n\
         - 0 <= strs[i].len() <= 10\n\
         - strs[i] consists of lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=15);
            let strs: Vec<String> = (0..n).map(|_| {
                let len = rng.random_range(1..=6);
                let alphabet_size = rng.random_range(2..=6u8);
                (0..len)
                    .map(|_| (b'a' + rng.random_range(0..alphabet_size)) as char)
                    .collect()
            }).collect();
            TestCase { data: Box::new(GroupAnagramsTest { strs }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<GroupAnagramsTest>().unwrap();
        let expected = ref_group_anagrams(&t.strs);
        let actual = solutions::group_anagrams(&t.strs);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("strs={:?}", t.strs),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_group_anagrams(strs: &[String]) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
    for s in strs {
        let mut key: Vec<u8> = s.bytes().collect();
        key.sort();
        map.entry(key).or_default().push(s.clone());
    }
    let mut groups: Vec<Vec<String>> = map.into_values().collect();
    for group in &mut groups {
        group.sort();
    }
    groups.sort_by(|a, b| a[0].cmp(&b[0]));
    groups
}

// ── Medium 2: Top K Frequent Elements ─────────────────────────────────

struct TopKFrequent;
struct TopKFrequentTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for TopKFrequent {
    fn id(&self) -> &str { "hash_maps_top_k_frequent" }
    fn name(&self) -> &str { "Top K Frequent Elements" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an integer array `nums` and an integer `k`, return the `k` most frequent elements. \
         Return them sorted by frequency in descending order. If two elements have the same \
         frequency, sort them by value in ascending order.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 1000\n\
         - 1 <= k <= number of distinct elements\n\
         - The answer is guaranteed to be unique for the given k."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-20..=20)).collect();
            let distinct: HashSet<i32> = nums.iter().copied().collect();
            let k = rng.random_range(1..=distinct.len());
            TestCase { data: Box::new(TopKFrequentTest { nums, k }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TopKFrequentTest>().unwrap();
        let expected = ref_top_k_frequent(&t.nums, t.k);
        let actual = solutions::top_k_frequent(&t.nums, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &n in nums {
        *counts.entry(n).or_default() += 1;
    }
    let mut pairs: Vec<(i32, usize)> = counts.into_iter().collect();
    // Sort by frequency descending, then by value ascending for ties
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    pairs.into_iter().take(k).map(|(val, _)| val).collect()
}

// ── Medium 3: Longest Consecutive Sequence ────────────────────────────

struct LongestConsecutive;
struct LongestConsecutiveTest {
    nums: Vec<i32>,
}

impl Problem for LongestConsecutive {
    fn id(&self) -> &str { "hash_maps_longest_consecutive" }
    fn name(&self) -> &str { "Longest Consecutive Sequence" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an unsorted array of integers `nums`, return the length of the longest \
         consecutive elements sequence.\n\n\
         Must run in O(n) time.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 1000\n\
         - -1000 <= nums[i] <= 1000\n\n\
         Hint: Use a HashSet. For each number that is the start of a sequence \
         (i.e., num-1 is not in the set), count how long the sequence extends."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=30);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
            TestCase { data: Box::new(LongestConsecutiveTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LongestConsecutiveTest>().unwrap();
        let expected = ref_longest_consecutive(&t.nums);
        let actual = solutions::longest_consecutive(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_longest_consecutive(nums: &[i32]) -> i32 {
    let set: HashSet<i32> = nums.iter().copied().collect();
    let mut best = 0i32;
    for &n in &set {
        if !set.contains(&(n - 1)) {
            let mut len = 1;
            let mut cur = n;
            while set.contains(&(cur + 1)) {
                cur += 1;
                len += 1;
            }
            best = best.max(len);
        }
    }
    best
}

// ── Medium 4: Subarray Sum Equals K ───────────────────────────────────

struct SubarraySumK;
struct SubarraySumKTest {
    nums: Vec<i32>,
    k: i32,
}

impl Problem for SubarraySumK {
    fn id(&self) -> &str { "hash_maps_subarray_sum_k" }
    fn name(&self) -> &str { "Subarray Sum Equals K" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an array of integers `nums` and an integer `k`, return the total number of \
         subarrays whose sum equals `k`.\n\n\
         A subarray is a contiguous non-empty sequence of elements.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 1000\n\
         - -1000 <= nums[i] <= 1000\n\
         - -10^7 <= k <= 10^7\n\n\
         Hint: Use a prefix sum HashMap. For each prefix sum, check how many previous \
         prefix sums equal (current_prefix - k)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-10..=10)).collect();
            let k = rng.random_range(-20..=20);
            TestCase { data: Box::new(SubarraySumKTest { nums, k }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SubarraySumKTest>().unwrap();
        let expected = ref_subarray_sum_k(&t.nums, t.k);
        let actual = solutions::subarray_sum_k(&t.nums, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_subarray_sum_k(nums: &[i32], k: i32) -> i32 {
    let mut count = 0;
    let mut prefix_sum = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, 1);
    for &n in nums {
        prefix_sum += n;
        if let Some(&c) = map.get(&(prefix_sum - k)) {
            count += c;
        }
        *map.entry(prefix_sum).or_default() += 1;
    }
    count
}

// ── Medium 5: Encode and Decode Strings ───────────────────────────────

struct EncodeDecode;
struct EncodeDecodeTest {
    strs: Vec<String>,
}

impl Problem for EncodeDecode {
    fn id(&self) -> &str { "hash_maps_encode_decode" }
    fn name(&self) -> &str { "Encode and Decode Strings" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Design an algorithm to encode a list of strings to a single string, and decode \
         it back to the original list.\n\n\
         Implement two functions:\n\
         - `encode(strs: &[String]) -> String` — encodes the list into a single string.\n\
         - `decode(s: &str) -> Vec<String>` — decodes it back to the original list.\n\n\
         The encoded string must be decodable. The strings may contain any characters, \
         including delimiters.\n\n\
         This problem tests your `encode` function's output fed into your `decode` function. \
         The round-trip must produce the original list.\n\n\
         Hint: Use length-prefix encoding, e.g. \"3#abc5#hello\" for [\"abc\", \"hello\"]."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=10);
            let strs: Vec<String> = (0..n).map(|_| {
                let len = rng.random_range(0..=10);
                // Include some tricky characters: #, digits, spaces
                let alphabet = b"abcdefghij0123456789# ";
                (0..len)
                    .map(|_| alphabet[rng.random_range(0..alphabet.len())] as char)
                    .collect()
            }).collect();
            TestCase { data: Box::new(EncodeDecodeTest { strs }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<EncodeDecodeTest>().unwrap();
        let encoded = solutions::encode(&t.strs);
        let decoded = solutions::decode(&encoded);
        SolutionResult {
            is_correct: decoded == t.strs,
            input_description: format!("strs={:?}", t.strs),
            expected: format!("{:?}", t.strs),
            actual: format!("{decoded:?}"),
        }
    }
}

// ── Hard 1: Minimum Window Substring ──────────────────────────────────

struct MinWindowSubstring;
struct MinWindowTest {
    s: String,
    t: String,
}

impl Problem for MinWindowSubstring {
    fn id(&self) -> &str { "hash_maps_min_window_substring" }
    fn name(&self) -> &str { "Minimum Window Substring" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given two strings `s` and `t`, return the minimum window substring of `s` such \
         that every character in `t` (including duplicates) is included in the window. \
         If there is no such substring, return the empty string.\n\n\
         Constraints:\n\
         - 1 <= s.len(), t.len() <= 1000\n\
         - `s` and `t` consist of lowercase and uppercase English letters.\n\n\
         Hint: Use a sliding window with two HashMaps tracking character counts."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let s_len = rng.random_range(5..=30);
            let t_len = rng.random_range(1..=s_len.min(10));
            let alphabet_size = rng.random_range(3..=8u8);
            let s: String = (0..s_len)
                .map(|_| (b'a' + rng.random_range(0..alphabet_size)) as char)
                .collect();
            // Build t from characters that appear in s (so a solution is more likely)
            let s_bytes: Vec<u8> = s.bytes().collect();
            let t: String = (0..t_len)
                .map(|_| s_bytes[rng.random_range(0..s_bytes.len())] as char)
                .collect();
            TestCase { data: Box::new(MinWindowTest { s, t }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinWindowTest>().unwrap();
        let expected = ref_min_window(&t.s, &t.t);
        let actual = solutions::min_window_substring(&t.s, &t.t);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\", t=\"{}\"", t.s, t.t),
            expected: format!("\"{expected}\""),
            actual: format!("\"{actual}\""),
        }
    }
}

fn ref_min_window(s: &str, t: &str) -> String {
    if t.is_empty() || s.len() < t.len() { return String::new(); }
    let s_bytes = s.as_bytes();
    let mut need: HashMap<u8, i32> = HashMap::new();
    for &b in t.as_bytes() {
        *need.entry(b).or_default() += 1;
    }
    let required = need.len();
    let mut have = 0usize;
    let mut window: HashMap<u8, i32> = HashMap::new();
    let mut best: Option<(usize, usize)> = None;
    let mut left = 0;

    for right in 0..s_bytes.len() {
        let c = s_bytes[right];
        *window.entry(c).or_default() += 1;
        if let Some(&needed) = need.get(&c) {
            if window[&c] == needed {
                have += 1;
            }
        }
        while have == required {
            let window_len = right - left + 1;
            if best.is_none() || window_len < best.unwrap().1 - best.unwrap().0 {
                best = Some((left, left + window_len));
            }
            let lc = s_bytes[left];
            *window.get_mut(&lc).unwrap() -= 1;
            if let Some(&needed) = need.get(&lc) {
                if window[&lc] < needed {
                    have -= 1;
                }
            }
            left += 1;
        }
    }
    match best {
        Some((l, r)) => s[l..r].to_string(),
        None => String::new(),
    }
}

// ── Hard 2: Longest Substring with At Most K Distinct Characters ──────

struct LongestKDistinct;
struct LongestKDistinctTest {
    s: String,
    k: usize,
}

impl Problem for LongestKDistinct {
    fn id(&self) -> &str { "hash_maps_longest_k_distinct" }
    fn name(&self) -> &str { "Longest Substring with At Most K Distinct Characters" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a string `s` and an integer `k`, return the length of the longest substring \
         that contains at most `k` distinct characters.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 1000\n\
         - 1 <= k <= 26\n\
         - `s` consists of lowercase English letters.\n\n\
         Hint: Use a sliding window with a HashMap counting character frequencies."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let len = rng.random_range(1..=30);
            let alphabet_size = rng.random_range(2..=8u8);
            let s: String = (0..len)
                .map(|_| (b'a' + rng.random_range(0..alphabet_size)) as char)
                .collect();
            let k = rng.random_range(1..=alphabet_size as usize);
            TestCase { data: Box::new(LongestKDistinctTest { s, k }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LongestKDistinctTest>().unwrap();
        let expected = ref_longest_k_distinct(&t.s, t.k);
        let actual = solutions::longest_k_distinct(&t.s, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\", k={}", t.s, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_longest_k_distinct(s: &str, k: usize) -> i32 {
    let bytes = s.as_bytes();
    let mut counts: HashMap<u8, usize> = HashMap::new();
    let mut best = 0i32;
    let mut left = 0;
    for right in 0..bytes.len() {
        *counts.entry(bytes[right]).or_default() += 1;
        while counts.len() > k {
            let lc = bytes[left];
            let count = counts.get_mut(&lc).unwrap();
            *count -= 1;
            if *count == 0 {
                counts.remove(&lc);
            }
            left += 1;
        }
        best = best.max((right - left + 1) as i32);
    }
    best
}

// ── Hard 3: Alien Dictionary ──────────────────────────────────────────

struct AlienDictionary;
struct AlienDictionaryTest {
    words: Vec<String>,
}

impl Problem for AlienDictionary {
    fn id(&self) -> &str { "hash_maps_alien_dictionary" }
    fn name(&self) -> &str { "Alien Dictionary" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "There is a new alien language that uses the English lowercase alphabet. The order \
         among the letters is unknown to you.\n\n\
         You are given a list of strings `words` from the alien language's dictionary, where \
         the strings are sorted lexicographically by the rules of this new language.\n\n\
         Derive the order of letters in this language. Return the letters as a string sorted \
         in the alien order. If the order is invalid, return an empty string. If there are \
         multiple valid orderings, return the smallest one lexicographically (in English).\n\n\
         Constraints:\n\
         - 1 <= words.len() <= 100\n\
         - 1 <= words[i].len() <= 20\n\
         - words[i] consists of lowercase English letters.\n\n\
         Hint: Build a directed graph from adjacent word comparisons, then topological sort."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let letter_count = rng.random_range(2..=6usize);
            // Create a random permutation of letters as the alien order
            let mut alphabet: Vec<u8> = (0..letter_count as u8).map(|i| b'a' + i).collect();
            for i in (1..alphabet.len()).rev() {
                let j = rng.random_range(0..=i);
                alphabet.swap(i, j);
            }
            // Generate words that respect this ordering
            let word_count = rng.random_range(2..=8);
            let mut words: Vec<String> = (0..word_count).map(|_| {
                let len = rng.random_range(1..=4);
                (0..len)
                    .map(|_| alphabet[rng.random_range(0..alphabet.len())] as char)
                    .collect()
            }).collect();
            // Sort by alien order
            let order_map: HashMap<u8, usize> = alphabet.iter().enumerate()
                .map(|(i, &c)| (c, i)).collect();
            words.sort_by(|a, b| {
                for (ac, bc) in a.bytes().zip(b.bytes()) {
                    let ord = order_map[&ac].cmp(&order_map[&bc]);
                    if ord != std::cmp::Ordering::Equal {
                        return ord;
                    }
                }
                a.len().cmp(&b.len())
            });
            words.dedup();
            if words.len() < 2 {
                // Ensure at least 2 words
                words.push(String::from(alphabet[0] as char));
                words.sort_by(|a, b| {
                    for (ac, bc) in a.bytes().zip(b.bytes()) {
                        let ord = order_map[&ac].cmp(&order_map[&bc]);
                        if ord != std::cmp::Ordering::Equal {
                            return ord;
                        }
                    }
                    a.len().cmp(&b.len())
                });
                words.dedup();
            }
            TestCase { data: Box::new(AlienDictionaryTest { words }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AlienDictionaryTest>().unwrap();
        let expected = ref_alien_dictionary(&t.words);
        let actual = solutions::alien_dictionary(&t.words);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("words={:?}", t.words),
            expected: format!("\"{expected}\""),
            actual: format!("\"{actual}\""),
        }
    }
}

fn ref_alien_dictionary(words: &[String]) -> String {
    // Collect all unique characters
    let mut chars: HashSet<u8> = HashSet::new();
    for w in words {
        for &b in w.as_bytes() {
            chars.insert(b);
        }
    }

    // Build adjacency list and in-degree map
    let mut adj: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut in_degree: HashMap<u8, usize> = HashMap::new();
    for &c in &chars {
        adj.entry(c).or_default();
        in_degree.entry(c).or_insert(0);
    }

    for i in 0..words.len() - 1 {
        let w1 = words[i].as_bytes();
        let w2 = words[i + 1].as_bytes();
        let min_len = w1.len().min(w2.len());

        // Check for invalid: prefix case where w1 is longer
        if w1.len() > w2.len() && w1[..min_len] == w2[..min_len] {
            return String::new();
        }

        for j in 0..min_len {
            if w1[j] != w2[j] {
                if adj.get(&w1[j]).map_or(true, |s| !s.contains(&w2[j])) {
                    adj.entry(w1[j]).or_default().insert(w2[j]);
                    *in_degree.entry(w2[j]).or_default() += 1;
                }
                break;
            }
        }
    }

    // Topological sort using BTreeMap-based min-heap for deterministic smallest ordering
    let mut queue: std::collections::BTreeSet<u8> = std::collections::BTreeSet::new();
    for (&c, &deg) in &in_degree {
        if deg == 0 {
            queue.insert(c);
        }
    }

    let mut result = Vec::new();
    while let Some(&c) = queue.iter().next() {
        queue.remove(&c);
        result.push(c);
        if let Some(neighbors) = adj.get(&c) {
            for &next in neighbors {
                let deg = in_degree.get_mut(&next).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.insert(next);
                }
            }
        }
    }

    if result.len() != chars.len() {
        return String::new(); // Cycle detected
    }

    String::from_utf8(result).unwrap()
}

// ── Hard 4: All O(1) Data Structure ───────────────────────────────────

struct AllO1;

/// Operations for the All O(1) data structure test.
#[derive(Debug, Clone)]
enum AllO1Op {
    Inc(String),
    Dec(String),
    GetMaxKey,
    GetMinKey,
}

struct AllO1Test {
    ops: Vec<AllO1Op>,
    expected_results: Vec<Option<String>>,
}

impl Problem for AllO1 {
    fn id(&self) -> &str { "hash_maps_all_o1" }
    fn name(&self) -> &str { "All O(1) Data Structure" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Design a data structure that supports the following operations, all in O(1) time:\n\n\
         - `inc(key)` — Increments the count of `key` by 1. If `key` doesn't exist, insert \
           it with count 1.\n\
         - `dec(key)` — Decrements the count of `key` by 1. If the count reaches 0, remove it.\n\
         - `get_max_key()` — Returns one of the keys with the maximum count. Returns \"\" if empty.\n\
         - `get_min_key()` — Returns one of the keys with the minimum count. Returns \"\" if empty.\n\n\
         Implement the `AllO1` struct with these methods. When multiple keys share the \
         max/min count, return the lexicographically smallest one.\n\n\
         You will receive a sequence of operations and must produce the correct output for \
         each `get_max_key` / `get_min_key` call.\n\n\
         Input: `ops: Vec<(String, String)>` where each pair is (operation, key).\n\
         - \"inc\"/\"dec\" operations have a key.\n\
         - \"max\"/\"min\" operations have an empty key.\n\
         Output: `Vec<String>` — the results of all \"max\" and \"min\" operations in order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let keys: Vec<String> = (0..rng.random_range(2..=5))
                .map(|_| {
                    let len = rng.random_range(1..=4);
                    (0..len).map(|_| (b'a' + rng.random_range(0..6u8)) as char).collect()
                })
                .collect();

            let op_count = rng.random_range(5..=20);
            let mut ops = Vec::new();
            let mut counts: HashMap<String, i32> = HashMap::new();
            let mut expected_results = Vec::new();

            for _ in 0..op_count {
                let op_type = rng.random_range(0..4);
                match op_type {
                    0 => {
                        let key = keys[rng.random_range(0..keys.len())].clone();
                        *counts.entry(key.clone()).or_default() += 1;
                        ops.push(AllO1Op::Inc(key));
                        expected_results.push(None);
                    }
                    1 => {
                        // Only dec if there are keys with positive counts
                        let positive: Vec<String> = counts.iter()
                            .filter(|(_, &v)| v > 0)
                            .map(|(k, _)| k.clone())
                            .collect();
                        if positive.is_empty() {
                            // Do an inc instead
                            let key = keys[rng.random_range(0..keys.len())].clone();
                            *counts.entry(key.clone()).or_default() += 1;
                            ops.push(AllO1Op::Inc(key));
                            expected_results.push(None);
                        } else {
                            let key = positive[rng.random_range(0..positive.len())].clone();
                            *counts.get_mut(&key).unwrap() -= 1;
                            if counts[&key] == 0 {
                                counts.remove(&key);
                            }
                            ops.push(AllO1Op::Dec(key));
                            expected_results.push(None);
                        }
                    }
                    2 => {
                        ops.push(AllO1Op::GetMaxKey);
                        let max_key = ref_get_max_key(&counts);
                        expected_results.push(Some(max_key));
                    }
                    _ => {
                        ops.push(AllO1Op::GetMinKey);
                        let min_key = ref_get_min_key(&counts);
                        expected_results.push(Some(min_key));
                    }
                }
            }

            TestCase { data: Box::new(AllO1Test { ops, expected_results }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AllO1Test>().unwrap();

        // Convert ops to the format the user solution expects: Vec<(String, String)>
        let user_ops: Vec<(String, String)> = t.ops.iter().map(|op| match op {
            AllO1Op::Inc(k) => ("inc".to_string(), k.clone()),
            AllO1Op::Dec(k) => ("dec".to_string(), k.clone()),
            AllO1Op::GetMaxKey => ("max".to_string(), String::new()),
            AllO1Op::GetMinKey => ("min".to_string(), String::new()),
        }).collect();

        let actual = solutions::all_o1(&user_ops);

        let expected: Vec<String> = t.expected_results.iter()
            .filter_map(|r| r.clone())
            .collect();

        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("ops={:?}", user_ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_get_max_key(counts: &HashMap<String, i32>) -> String {
    if counts.is_empty() { return String::new(); }
    let max_val = *counts.values().max().unwrap();
    let mut candidates: Vec<&String> = counts.iter()
        .filter(|(_, &v)| v == max_val)
        .map(|(k, _)| k)
        .collect();
    candidates.sort();
    candidates[0].clone()
}

fn ref_get_min_key(counts: &HashMap<String, i32>) -> String {
    if counts.is_empty() { return String::new(); }
    let min_val = *counts.values().min().unwrap();
    let mut candidates: Vec<&String> = counts.iter()
        .filter(|(_, &v)| v == min_val)
        .map(|(k, _)| k)
        .collect();
    candidates.sort();
    candidates[0].clone()
}

// ── Hard 5: Max Points on a Line ──────────────────────────────────────

struct MaxPointsLine;
struct MaxPointsTest {
    points: Vec<(i32, i32)>,
}

impl Problem for MaxPointsLine {
    fn id(&self) -> &str { "hash_maps_max_points_line" }
    fn name(&self) -> &str { "Max Points on a Line" }
    fn topic(&self) -> &str { "hash_maps" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an array of points on the X-Y plane `points[i] = (xi, yi)`, return the \
         maximum number of points that lie on the same straight line.\n\n\
         Constraints:\n\
         - 1 <= points.len() <= 300\n\
         - -10^4 <= xi, yi <= 10^4\n\
         - All points are unique.\n\n\
         Hint: For each point, compute slopes to all other points as normalized (dx, dy) \
         pairs using GCD to avoid floating-point issues."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=15);
            let mut used: HashSet<(i32, i32)> = HashSet::new();
            let mut points = Vec::with_capacity(n);
            while points.len() < n {
                let x = rng.random_range(-100..=100);
                let y = rng.random_range(-100..=100);
                if used.insert((x, y)) {
                    points.push((x, y));
                }
            }
            TestCase { data: Box::new(MaxPointsTest { points }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxPointsTest>().unwrap();
        let expected = ref_max_points(&t.points);
        let actual = solutions::max_points_on_line(&t.points);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("points={:?}", t.points),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_points(points: &[(i32, i32)]) -> i32 {
    let n = points.len();
    if n <= 2 { return n as i32; }
    let mut best = 2i32;

    for i in 0..n {
        let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();
        for j in (i + 1)..n {
            let mut dx = points[j].0 - points[i].0;
            let mut dy = points[j].1 - points[i].1;

            // Normalize the slope: divide by GCD, ensure consistent sign
            let g = gcd(dx.abs(), dy.abs());
            if g != 0 {
                dx /= g;
                dy /= g;
            }
            // Ensure canonical direction: dx positive, or if dx == 0 then dy positive
            if dx < 0 || (dx == 0 && dy < 0) {
                dx = -dx;
                dy = -dy;
            }

            *slopes.entry((dx, dy)).or_default() += 1;
        }
        for &count in slopes.values() {
            best = best.max(count + 1);
        }
    }
    best
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
