use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part5_paradigms::backtracking as solutions;
use crate::tracker::{track_slice, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(Subsets),
        Box::new(Permutations),
        Box::new(Combinations),
        Box::new(LetterCombinations),
        Box::new(BinaryStrings),
        Box::new(CombinationSum),
        Box::new(CombinationSumII),
        Box::new(PalindromePartition),
        Box::new(GenerateParentheses),
        Box::new(WordSearch),
        Box::new(NQueens),
        Box::new(SudokuSolver),
        Box::new(WordBreakII),
        Box::new(RestoreIp),
        Box::new(ExpressionAddOperators),
    ]
}

// ── Easy 1: Generate All Subsets ─────────────────────────────────────

struct Subsets;

struct SubsetsTest {
    nums: Vec<i32>,
}

impl Problem for Subsets {
    fn id(&self) -> &str {
        "backtracking_subsets"
    }
    fn name(&self) -> &str {
        "Generate All Subsets"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a vector of distinct integers, return all possible subsets (the power set).\n\n\
         The solution must not contain duplicate subsets. Return them sorted \
         (each subset sorted, and the list of subsets sorted lexicographically).\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 10\n\
         - All elements are distinct\n\n\
         Example: nums=[1,2,3] => [[], [1], [1,2], [1,2,3], [1,3], [2], [2,3], [3]]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=6);
                let nums = crate::problems::helpers::random_unique_vec(&mut rng, n, -10, 10);
                TestCase {
                    data: Box::new(SubsetsTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SubsetsTest>().unwrap();
        let expected = ref_subsets(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::subsets(&tracked_nums);
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

fn ref_subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut sorted = nums.to_vec();
    sorted.sort();
    let mut current = Vec::new();
    fn backtrack(sorted: &[i32], start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(current.clone());
        for i in start..sorted.len() {
            current.push(sorted[i]);
            backtrack(sorted, i + 1, current, result);
            current.pop();
        }
    }
    backtrack(&sorted, 0, &mut current, &mut result);
    result.sort();
    result
}

// ── Easy 2: Generate All Permutations ────────────────────────────────

struct Permutations;

struct PermutationsTest {
    nums: Vec<i32>,
}

impl Problem for Permutations {
    fn id(&self) -> &str {
        "backtracking_permutations"
    }
    fn name(&self) -> &str {
        "Generate All Permutations"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a vector of distinct integers, return all possible permutations.\n\n\
         Return them sorted lexicographically.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 6\n\
         - All elements are distinct\n\n\
         Example: nums=[1,2,3] => [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=5);
                let nums = crate::problems::helpers::random_unique_vec(&mut rng, n, -10, 10);
                TestCase {
                    data: Box::new(PermutationsTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PermutationsTest>().unwrap();
        let expected = ref_permutations(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::permutations(&tracked_nums);
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

fn ref_permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut sorted = nums.to_vec();
    sorted.sort();
    let mut used = vec![false; sorted.len()];
    let mut current = Vec::new();
    fn backtrack(
        sorted: &[i32],
        used: &mut Vec<bool>,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if current.len() == sorted.len() {
            result.push(current.clone());
            return;
        }
        for i in 0..sorted.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            current.push(sorted[i]);
            backtrack(sorted, used, current, result);
            current.pop();
            used[i] = false;
        }
    }
    backtrack(&sorted, &mut used, &mut current, &mut result);
    result.sort();
    result
}

// ── Easy 3: Combinations C(n,k) ─────────────────────────────────────

struct Combinations;

struct CombinationsTest {
    n: i32,
    k: i32,
}

impl Problem for Combinations {
    fn id(&self) -> &str {
        "backtracking_combinations"
    }
    fn name(&self) -> &str {
        "Combinations C(n,k)"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given two integers n and k, return all possible combinations of k numbers \
         chosen from the range [1, n]. Return them sorted.\n\n\
         Constraints:\n\
         - 1 <= n <= 20\n\
         - 1 <= k <= n\n\n\
         Example: n=4, k=2 => [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=10);
                let k = rng.random_range(1..=n);
                TestCase {
                    data: Box::new(CombinationsTest { n, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CombinationsTest>().unwrap();
        let expected = ref_combinations(t.n, t.k);
        let actual = solutions::combinations(t.n, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}, k={}", t.n, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_combinations(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    fn backtrack(start: i32, n: i32, k: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if current.len() == k as usize {
            result.push(current.clone());
            return;
        }
        for i in start..=n {
            current.push(i);
            backtrack(i + 1, n, k, current, result);
            current.pop();
        }
    }
    backtrack(1, n, k, &mut current, &mut result);
    result.sort();
    result
}

// ── Easy 4: Letter Combinations of a Phone Number ────────────────────

struct LetterCombinations;

struct LetterCombinationsTest {
    digits: String,
}

impl Problem for LetterCombinations {
    fn id(&self) -> &str {
        "backtracking_letter_combinations"
    }
    fn name(&self) -> &str {
        "Letter Combinations of a Phone Number"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a string containing digits from 2-9, return all possible letter combinations \
         that the number could represent (like on a phone keypad). Return them sorted.\n\n\
         Mapping: 2->abc, 3->def, 4->ghi, 5->jkl, 6->mno, 7->pqrs, 8->tuv, 9->wxyz\n\n\
         If the input is empty, return an empty vector.\n\n\
         Constraints:\n\
         - 0 <= digits.len() <= 4\n\
         - digits[i] is a digit in ['2', '9']\n\n\
         Example: digits=\"23\" => [\"ad\",\"ae\",\"af\",\"bd\",\"be\",\"bf\",\"cd\",\"ce\",\"cf\"]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=4);
                let digits: String = (0..n)
                    .map(|_| (b'2' + rng.random_range(0..=7u8)) as char)
                    .collect();
                TestCase {
                    data: Box::new(LetterCombinationsTest { digits }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LetterCombinationsTest>().unwrap();
        let expected = ref_letter_combinations(&t.digits);
        let actual = solutions::letter_combinations(&t.digits);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("digits=\"{}\"", t.digits),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_letter_combinations(digits: &str) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let mapping: [&str; 10] = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    let mut result = Vec::new();
    let mut current = String::new();
    fn backtrack(
        digits: &[u8],
        idx: usize,
        mapping: &[&str; 10],
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if idx == digits.len() {
            result.push(current.clone());
            return;
        }
        let digit = (digits[idx] - b'0') as usize;
        for ch in mapping[digit].chars() {
            current.push(ch);
            backtrack(digits, idx + 1, mapping, current, result);
            current.pop();
        }
    }
    backtrack(digits.as_bytes(), 0, &mapping, &mut current, &mut result);
    result.sort();
    result
}

// ── Easy 5: Generate All Binary Strings of Length n ──────────────────

struct BinaryStrings;

struct BinaryStringsTest {
    n: usize,
}

impl Problem for BinaryStrings {
    fn id(&self) -> &str {
        "backtracking_binary_strings"
    }
    fn name(&self) -> &str {
        "Generate All Binary Strings"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an integer n, return all binary strings of length n, sorted.\n\n\
         Constraints:\n\
         - 0 <= n <= 16\n\n\
         Example: n=2 => [\"00\", \"01\", \"10\", \"11\"]\n\
         Example: n=0 => [\"\"]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=8);
                TestCase {
                    data: Box::new(BinaryStringsTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BinaryStringsTest>().unwrap();
        let expected = ref_binary_strings(t.n);
        let actual = solutions::binary_strings(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_binary_strings(n: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    fn backtrack(n: usize, current: &mut String, result: &mut Vec<String>) {
        if current.len() == n {
            result.push(current.clone());
            return;
        }
        current.push('0');
        backtrack(n, current, result);
        current.pop();
        current.push('1');
        backtrack(n, current, result);
        current.pop();
    }
    backtrack(n, &mut current, &mut result);
    result.sort();
    result
}

// ── Medium 1: Combination Sum ────────────────────────────────────────

struct CombinationSum;

struct CombinationSumTest {
    candidates: Vec<i32>,
    target: i32,
}

impl Problem for CombinationSum {
    fn id(&self) -> &str {
        "backtracking_combination_sum"
    }
    fn name(&self) -> &str {
        "Combination Sum"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array of distinct integers `candidates` and a target integer, return all \
         unique combinations of candidates where the chosen numbers sum to target.\n\n\
         The same number may be chosen an unlimited number of times. Each combination \
         should be sorted, and the list of combinations sorted lexicographically.\n\n\
         Constraints:\n\
         - 1 <= candidates.len() <= 30\n\
         - 2 <= candidates[i] <= 40\n\
         - 1 <= target <= 40\n\n\
         Example: candidates=[2,3,6,7], target=7 => [[2,2,3],[7]]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=8);
                let candidates = crate::problems::helpers::random_unique_vec(&mut rng, n, 2, 20);
                let target = rng.random_range(3..=25);
                TestCase {
                    data: Box::new(CombinationSumTest { candidates, target }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CombinationSumTest>().unwrap();
        let expected = ref_combination_sum(&t.candidates, t.target);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_candidates = track_slice(&t.candidates, shared_log.clone());
        let actual = solutions::combination_sum(&tracked_candidates, t.target);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("candidates={:?}, target={}", t.candidates, t.target),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_combination_sum(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut sorted = candidates.to_vec();
    sorted.sort();
    let mut current = Vec::new();
    fn backtrack(
        sorted: &[i32],
        start: usize,
        remaining: i32,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if remaining == 0 {
            result.push(current.clone());
            return;
        }
        for i in start..sorted.len() {
            if sorted[i] > remaining {
                break;
            }
            current.push(sorted[i]);
            backtrack(sorted, i, remaining - sorted[i], current, result);
            current.pop();
        }
    }
    backtrack(&sorted, 0, target, &mut current, &mut result);
    result.sort();
    result
}

// ── Medium 2: Combination Sum II ─────────────────────────────────────

struct CombinationSumII;

struct CombinationSumIITest {
    candidates: Vec<i32>,
    target: i32,
}

impl Problem for CombinationSumII {
    fn id(&self) -> &str {
        "backtracking_combination_sum_ii"
    }
    fn name(&self) -> &str {
        "Combination Sum II"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a collection of candidate numbers (may contain duplicates) and a target, \
         find all unique combinations where the candidates sum to target.\n\n\
         Each number in candidates may only be used once. The solution set must not contain \
         duplicate combinations. Return sorted.\n\n\
         Constraints:\n\
         - 1 <= candidates.len() <= 30\n\
         - 1 <= candidates[i] <= 50\n\
         - 1 <= target <= 30\n\n\
         Example: candidates=[10,1,2,7,6,1,5], target=8 => [[1,1,6],[1,2,5],[1,7],[2,6]]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=10);
                let candidates: Vec<i32> = (0..n).map(|_| rng.random_range(1..=15)).collect();
                let target = rng.random_range(3..=20);
                TestCase {
                    data: Box::new(CombinationSumIITest { candidates, target }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CombinationSumIITest>().unwrap();
        let expected = ref_combination_sum_ii(&t.candidates, t.target);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_candidates = track_slice(&t.candidates, shared_log.clone());
        let actual = solutions::combination_sum_ii(&tracked_candidates, t.target);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("candidates={:?}, target={}", t.candidates, t.target),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_combination_sum_ii(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut sorted = candidates.to_vec();
    sorted.sort();
    let mut current = Vec::new();
    fn backtrack(
        sorted: &[i32],
        start: usize,
        remaining: i32,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if remaining == 0 {
            result.push(current.clone());
            return;
        }
        for i in start..sorted.len() {
            if sorted[i] > remaining {
                break;
            }
            if i > start && sorted[i] == sorted[i - 1] {
                continue;
            }
            current.push(sorted[i]);
            backtrack(sorted, i + 1, remaining - sorted[i], current, result);
            current.pop();
        }
    }
    backtrack(&sorted, 0, target, &mut current, &mut result);
    result.sort();
    result
}

// ── Medium 3: Palindrome Partitioning ────────────────────────────────

struct PalindromePartition;

struct PalindromePartitionTest {
    s: String,
}

impl Problem for PalindromePartition {
    fn id(&self) -> &str {
        "backtracking_palindrome_partition"
    }
    fn name(&self) -> &str {
        "Palindrome Partitioning"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a string s, partition s such that every substring of the partition is a \
         palindrome. Return all possible palindrome partitions, sorted.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 16\n\
         - s contains only lowercase English letters\n\n\
         Example: s=\"aab\" => [[\"a\",\"a\",\"b\"],[\"aa\",\"b\"]]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=8);
                // Use a small alphabet to increase palindrome likelihood
                let s = crate::problems::helpers::random_string_from(&mut rng, n, b"abc");
                TestCase {
                    data: Box::new(PalindromePartitionTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PalindromePartitionTest>().unwrap();
        let expected = ref_palindrome_partition(&t.s);
        let actual = solutions::palindrome_partition(&t.s);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_palindrome_partition(s: &str) -> Vec<Vec<String>> {
    let bytes = s.as_bytes();
    let mut result = Vec::new();
    let mut current: Vec<String> = Vec::new();
    fn is_palindrome(b: &[u8], lo: usize, hi: usize) -> bool {
        let (mut l, mut r) = (lo, hi);
        while l < r {
            if b[l] != b[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
    fn backtrack(
        bytes: &[u8],
        start: usize,
        current: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if start == bytes.len() {
            result.push(current.clone());
            return;
        }
        for end in start..bytes.len() {
            if is_palindrome(bytes, start, end) {
                current.push(String::from_utf8_lossy(&bytes[start..=end]).to_string());
                backtrack(bytes, end + 1, current, result);
                current.pop();
            }
        }
    }
    backtrack(bytes, 0, &mut current, &mut result);
    result.sort();
    result
}

// ── Medium 4: Generate Parentheses ───────────────────────────────────

struct GenerateParentheses;

struct GenerateParenthesesTest {
    n: usize,
}

impl Problem for GenerateParentheses {
    fn id(&self) -> &str {
        "backtracking_generate_parentheses"
    }
    fn name(&self) -> &str {
        "Generate Parentheses"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given n pairs of parentheses, generate all combinations of well-formed \
         parentheses. Return them sorted.\n\n\
         Constraints:\n\
         - 1 <= n <= 8\n\n\
         Example: n=3 => [\"((()))\",\"(()())\",\"(())()\",\"()(())\",\"()()()\"]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=6);
                TestCase {
                    data: Box::new(GenerateParenthesesTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<GenerateParenthesesTest>().unwrap();
        let expected = ref_generate_parentheses(t.n);
        let actual = solutions::generate_parentheses(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_generate_parentheses(n: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    fn backtrack(
        n: usize,
        open: usize,
        close: usize,
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if current.len() == 2 * n {
            result.push(current.clone());
            return;
        }
        if open < n {
            current.push('(');
            backtrack(n, open + 1, close, current, result);
            current.pop();
        }
        if close < open {
            current.push(')');
            backtrack(n, open, close + 1, current, result);
            current.pop();
        }
    }
    backtrack(n, 0, 0, &mut current, &mut result);
    result.sort();
    result
}

// ── Medium 5: Word Search ────────────────────────────────────────────

struct WordSearch;

struct WordSearchTest {
    board: Vec<Vec<char>>,
    word: String,
}

impl Problem for WordSearch {
    fn id(&self) -> &str {
        "backtracking_word_search"
    }
    fn name(&self) -> &str {
        "Word Search"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an m x n grid of characters and a string word, return true if the word \
         exists in the grid.\n\n\
         The word can be constructed from letters of sequentially adjacent cells, where \
         adjacent cells are horizontally or vertically neighboring. The same cell may not \
         be used more than once.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 6\n\
         - 1 <= word.len() <= 15\n\n\
         Example: board=[[A,B,C,E],[S,F,C,S],[A,D,E,E]], word=\"ABCCED\" => true"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let rows = rng.random_range(2..=4);
                let cols = rng.random_range(2..=4);
                let board: Vec<Vec<char>> = (0..rows)
                    .map(|_| {
                        (0..cols)
                            .map(|_| (b'A' + rng.random_range(0..6u8)) as char)
                            .collect()
                    })
                    .collect();
                // 50% chance to build word from the board for a positive case
                let word = if rng.random_range(0..2) == 0 {
                    ref_random_path_word(&board, &mut rng)
                } else {
                    let wlen = rng.random_range(2..=6);
                    (0..wlen)
                        .map(|_| (b'A' + rng.random_range(0..6u8)) as char)
                        .collect()
                };
                TestCase {
                    data: Box::new(WordSearchTest { board, word }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WordSearchTest>().unwrap();
        let expected = ref_word_search(&t.board, &t.word);
        let actual = solutions::word_search(&t.board, &t.word);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("board={:?}, word=\"{}\"", t.board, t.word),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_random_path_word(board: &[Vec<char>], rng: &mut impl Rng) -> String {
    let rows = board.len();
    let cols = board[0].len();
    let len = rng.random_range(2..=std::cmp::min(6, rows * cols));
    let mut r = rng.random_range(0..rows);
    let mut c = rng.random_range(0..cols);
    let mut word = String::new();
    let mut visited = vec![vec![false; cols]; rows];
    for _ in 0..len {
        word.push(board[r][c]);
        visited[r][c] = true;
        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut moved = false;
        let mut shuffled_dirs = dirs;
        for i in (1..4).rev() {
            let j = rng.random_range(0..=i);
            shuffled_dirs.swap(i, j);
        }
        for (dr, dc) in &shuffled_dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if !visited[nr][nc] {
                    r = nr;
                    c = nc;
                    moved = true;
                    break;
                }
            }
        }
        if !moved {
            break;
        }
    }
    word
}

fn ref_word_search(board: &[Vec<char>], word: &str) -> bool {
    let rows = board.len();
    let cols = board[0].len();
    let word_bytes = word.as_bytes();
    let mut visited = vec![vec![false; cols]; rows];
    fn dfs(
        board: &[Vec<char>],
        word: &[u8],
        idx: usize,
        r: usize,
        c: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if idx == word.len() {
            return true;
        }
        if r >= board.len() || c >= board[0].len() {
            return false;
        }
        if visited[r][c] {
            return false;
        }
        if board[r][c] as u8 != word[idx] {
            return false;
        }
        visited[r][c] = true;
        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nc >= 0 && dfs(board, word, idx + 1, nr as usize, nc as usize, visited) {
                visited[r][c] = false;
                return true;
            }
        }
        // Also check if we're at the last character
        if idx + 1 == word.len() {
            visited[r][c] = false;
            return true;
        }
        visited[r][c] = false;
        false
    }
    for r in 0..rows {
        for c in 0..cols {
            if dfs(board, word_bytes, 0, r, c, &mut visited) {
                return true;
            }
        }
    }
    false
}

// ── Hard 1: N-Queens ─────────────────────────────────────────────────

struct NQueens;

struct NQueensTest {
    n: usize,
}

impl Problem for NQueens {
    fn id(&self) -> &str {
        "backtracking_n_queens"
    }
    fn name(&self) -> &str {
        "N-Queens"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Place n queens on an n x n chessboard such that no two queens attack each other. \
         Return all distinct solutions.\n\n\
         Each solution is a Vec<String> where each string represents a row: '.' for empty, \
         'Q' for a queen. Return all solutions sorted.\n\n\
         Constraints:\n\
         - 1 <= n <= 9\n\n\
         Example: n=4 => [\n\
         [\".Q..\",\"...Q\",\"Q...\",\"..Q.\"],\n\
         [\"..Q.\",\"Q...\",\"...Q\",\".Q..\"]\n\
         ]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=8);
                TestCase {
                    data: Box::new(NQueensTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NQueensTest>().unwrap();
        let expected = ref_n_queens(t.n);
        let actual = solutions::n_queens(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_n_queens(n: usize) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut board = vec![vec!['.'; n]; n];
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n]; // row - col + n
    let mut diag2 = vec![false; 2 * n]; // row + col
    fn backtrack(
        n: usize,
        row: usize,
        board: &mut Vec<Vec<char>>,
        cols: &mut Vec<bool>,
        diag1: &mut Vec<bool>,
        diag2: &mut Vec<bool>,
        result: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            let solution: Vec<String> = board.iter().map(|r| r.iter().collect()).collect();
            result.push(solution);
            return;
        }
        for col in 0..n {
            let d1 = row + n - col;
            let d2 = row + col;
            if cols[col] || diag1[d1] || diag2[d2] {
                continue;
            }
            board[row][col] = 'Q';
            cols[col] = true;
            diag1[d1] = true;
            diag2[d2] = true;
            backtrack(n, row + 1, board, cols, diag1, diag2, result);
            board[row][col] = '.';
            cols[col] = false;
            diag1[d1] = false;
            diag2[d2] = false;
        }
    }
    backtrack(
        n,
        0,
        &mut board,
        &mut cols,
        &mut diag1,
        &mut diag2,
        &mut result,
    );
    result.sort();
    result
}

// ── Hard 2: Sudoku Solver ────────────────────────────────────────────

struct SudokuSolver;

struct SudokuSolverTest {
    board: Vec<Vec<u8>>,
}

impl Problem for SudokuSolver {
    fn id(&self) -> &str {
        "backtracking_sudoku_solver"
    }
    fn name(&self) -> &str {
        "Sudoku Solver"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Solve a Sudoku puzzle. The input is a 9x9 grid where 0 represents empty cells.\n\n\
         Fill in the grid so every row, column, and 3x3 box contains digits 1-9 exactly once.\n\n\
         The input is guaranteed to have a unique solution.\n\n\
         Constraints:\n\
         - board is always 9x9\n\
         - board[i][j] is 0 (empty) or 1-9\n\n\
         Example: A standard Sudoku puzzle => Its unique solution"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        // Generate valid Sudoku puzzles by solving an empty board, then removing cells
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let board = ref_generate_sudoku(&mut rng);
                TestCase {
                    data: Box::new(SudokuSolverTest { board }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SudokuSolverTest>().unwrap();
        let expected = ref_solve_sudoku(&t.board);
        let actual = solutions::sudoku_solver(&t.board);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("board={:?}", t.board),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_generate_sudoku(rng: &mut impl Rng) -> Vec<Vec<u8>> {
    // Start with an empty board and fill diagonals, then solve
    let mut board = vec![vec![0u8; 9]; 9];

    // Fill diagonal 3x3 boxes (they don't interfere with each other)
    for box_start in (0..9).step_by(3) {
        let mut nums: Vec<u8> = (1..=9).collect();
        for i in (1..9).rev() {
            let j = rng.random_range(0..=i);
            nums.swap(i, j);
        }
        let mut idx = 0;
        for row in board.iter_mut().skip(box_start).take(3) {
            for cell in row.iter_mut().skip(box_start).take(3) {
                *cell = nums[idx];
                idx += 1;
            }
        }
    }

    // Solve the board
    ref_solve_sudoku_inplace(&mut board);

    // Remove cells to create the puzzle (keep ~25-35 clues)
    let clues = rng.random_range(25..=35);
    let remove = 81 - clues;
    let mut positions: Vec<(usize, usize)> =
        (0..9).flat_map(|r| (0..9).map(move |c| (r, c))).collect();
    for i in (1..81).rev() {
        let j = rng.random_range(0..=i);
        positions.swap(i, j);
    }
    for &(r, c) in positions.iter().take(remove) {
        board[r][c] = 0;
    }
    board
}

fn ref_solve_sudoku_inplace(board: &mut Vec<Vec<u8>>) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] == 0 {
                for num in 1..=9u8 {
                    if ref_sudoku_valid(board, r, c, num) {
                        board[r][c] = num;
                        if ref_solve_sudoku_inplace(board) {
                            return true;
                        }
                        board[r][c] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}

fn ref_sudoku_valid(board: &[Vec<u8>], row: usize, col: usize, num: u8) -> bool {
    for c in 0..9 {
        if board[row][c] == num {
            return false;
        }
    }
    for row in board.iter().take(9) {
        if row[col] == num {
            return false;
        }
    }
    let box_r = (row / 3) * 3;
    let box_c = (col / 3) * 3;
    for board_row in board.iter().skip(box_r).take(3) {
        for &cell in board_row.iter().skip(box_c).take(3) {
            if cell == num {
                return false;
            }
        }
    }
    true
}

fn ref_solve_sudoku(board: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut b = board.to_vec();
    ref_solve_sudoku_inplace(&mut b);
    b
}

// ── Hard 3: Word Break II ────────────────────────────────────────────

struct WordBreakII;

struct WordBreakIITest {
    s: String,
    word_dict: Vec<String>,
}

impl Problem for WordBreakII {
    fn id(&self) -> &str {
        "backtracking_word_break_ii"
    }
    fn name(&self) -> &str {
        "Word Break II"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a string s and a dictionary of strings wordDict, add spaces in s to \
         construct a sentence where each word is a valid dictionary word. Return all \
         such possible sentences, sorted.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 20\n\
         - 1 <= wordDict.len() <= 1000\n\
         - 1 <= wordDict[i].len() <= 10\n\n\
         Example: s=\"catsanddog\", wordDict=[\"cat\",\"cats\",\"and\",\"sand\",\"dog\"]\n\
         => [\"cat sand dog\",\"cats and dog\"]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                // Build a string from dictionary words to guarantee at least one solution
                let dict_size = rng.random_range(3..=8);
                let word_dict: Vec<String> = (0..dict_size)
                    .map(|_| {
                        let wlen = rng.random_range(1..=4);
                        crate::problems::helpers::random_string_from(&mut rng, wlen, b"abcde")
                    })
                    .collect();
                // Build s by concatenating 2-4 random words from dict
                let word_count = rng.random_range(2..=4);
                let s: String = (0..word_count)
                    .map(|_| {
                        let idx = rng.random_range(0..word_dict.len());
                        word_dict[idx].clone()
                    })
                    .collect();
                TestCase {
                    data: Box::new(WordBreakIITest { s, word_dict }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WordBreakIITest>().unwrap();
        let expected = ref_word_break_ii(&t.s, &t.word_dict);
        let actual = solutions::word_break_ii(&t.s, &t.word_dict);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\", wordDict={:?}", t.s, t.word_dict),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_word_break_ii(s: &str, word_dict: &[String]) -> Vec<String> {
    use std::collections::HashSet;
    let dict: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let mut result = Vec::new();
    let mut current: Vec<String> = Vec::new();
    fn backtrack(
        s: &str,
        start: usize,
        dict: &HashSet<&str>,
        current: &mut Vec<String>,
        result: &mut Vec<String>,
    ) {
        if start == s.len() {
            result.push(current.join(" "));
            return;
        }
        for end in start + 1..=s.len() {
            let word = &s[start..end];
            if dict.contains(word) {
                current.push(word.to_string());
                backtrack(s, end, dict, current, result);
                current.pop();
            }
        }
    }
    backtrack(s, 0, &dict, &mut current, &mut result);
    result.sort();
    result
}

// ── Hard 4: Restore IP Addresses ─────────────────────────────────────

struct RestoreIp;

struct RestoreIpTest {
    s: String,
}

impl Problem for RestoreIp {
    fn id(&self) -> &str {
        "backtracking_restore_ip"
    }
    fn name(&self) -> &str {
        "Restore IP Addresses"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a string s containing only digits, return all possible valid IP addresses \
         that can be formed by inserting dots into s. Return them sorted.\n\n\
         A valid IP address consists of exactly four integers (each 0-255), separated by \
         dots. No integer may have leading zeros (except \"0\" itself).\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 20\n\
         - s consists of digits only\n\n\
         Example: s=\"25525511135\" => [\"255.255.11.135\",\"255.255.111.35\"]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                // 50% chance to generate from valid IP, 50% random digits
                let s = if rng.random_range(0..2) == 0 {
                    let a = rng.random_range(0..=255);
                    let b = rng.random_range(0..=255);
                    let c = rng.random_range(0..=255);
                    let d = rng.random_range(0..=255);
                    format!("{a}{b}{c}{d}")
                } else {
                    let len = rng.random_range(4..=12);
                    (0..len)
                        .map(|_| (b'0' + rng.random_range(0..=9u8)) as char)
                        .collect()
                };
                TestCase {
                    data: Box::new(RestoreIpTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RestoreIpTest>().unwrap();
        let expected = ref_restore_ip(&t.s);
        let actual = solutions::restore_ip(&t.s);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_restore_ip(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut parts: Vec<String> = Vec::new();
    fn backtrack(s: &str, start: usize, parts: &mut Vec<String>, result: &mut Vec<String>) {
        if parts.len() == 4 {
            if start == s.len() {
                result.push(parts.join("."));
            }
            return;
        }
        let remaining_parts = 4 - parts.len();
        let remaining_chars = s.len() - start;
        // Prune: not enough or too many characters left
        if remaining_chars < remaining_parts || remaining_chars > remaining_parts * 3 {
            return;
        }
        for len in 1..=3 {
            if start + len > s.len() {
                break;
            }
            let segment = &s[start..start + len];
            // No leading zeros (except "0" itself)
            if segment.len() > 1 && segment.starts_with('0') {
                continue;
            }
            let val: u32 = segment.parse().unwrap();
            if val > 255 {
                continue;
            }
            parts.push(segment.to_string());
            backtrack(s, start + len, parts, result);
            parts.pop();
        }
    }
    backtrack(s, 0, &mut parts, &mut result);
    result.sort();
    result
}

// ── Hard 5: Expression Add Operators ─────────────────────────────────

struct ExpressionAddOperators;

struct ExpressionAddOperatorsTest {
    num: String,
    target: i64,
}

impl Problem for ExpressionAddOperators {
    fn id(&self) -> &str {
        "backtracking_expression_add_operators"
    }
    fn name(&self) -> &str {
        "Expression Add Operators"
    }
    fn topic(&self) -> &str {
        "backtracking"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a string num that contains only digits and an integer target, return all \
         possibilities to insert the binary operators +, -, and * between the digits \
         so that the resulting expression evaluates to the target value. Return sorted.\n\n\
         Operands should not contain leading zeros (the number \"0\" itself is ok).\n\n\
         Constraints:\n\
         - 1 <= num.len() <= 10\n\
         - num consists of only digits\n\
         - -2^31 <= target <= 2^31 - 1\n\n\
         Example: num=\"123\", target=6 => [\"1*2*3\",\"1+2+3\"]\n\
         Example: num=\"105\", target=5 => [\"1*0+5\",\"10-5\"]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=6);
                let num: String = (0..len)
                    .map(|_| (b'0' + rng.random_range(0..=9u8)) as char)
                    .collect();
                let target = rng.random_range(-50..=50) as i64;
                TestCase {
                    data: Box::new(ExpressionAddOperatorsTest { num, target }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<ExpressionAddOperatorsTest>()
            .unwrap();
        let expected = ref_add_operators(&t.num, t.target);
        let actual = solutions::expression_add_operators(&t.num, t.target);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("num=\"{}\", target={}", t.num, t.target),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_add_operators(num: &str, target: i64) -> Vec<String> {
    let mut result = Vec::new();
    let digits = num.as_bytes();
    fn backtrack(
        digits: &[u8],
        idx: usize,
        target: i64,
        expr: &mut String,
        eval: i64,
        prev_operand: i64,
        result: &mut Vec<String>,
    ) {
        if idx == digits.len() {
            if eval == target {
                result.push(expr.clone());
            }
            return;
        }
        let mut operand: i64 = 0;
        let expr_len = expr.len();
        for i in idx..digits.len() {
            // Skip numbers with leading zeros
            if i > idx && digits[idx] == b'0' {
                break;
            }
            operand = operand * 10 + (digits[i] - b'0') as i64;
            let operand_str = std::str::from_utf8(&digits[idx..=i]).unwrap();
            if idx == 0 {
                // First number, no operator
                expr.push_str(operand_str);
                backtrack(digits, i + 1, target, expr, operand, operand, result);
                expr.truncate(expr_len);
            } else {
                // Try +
                expr.push('+');
                expr.push_str(operand_str);
                backtrack(digits, i + 1, target, expr, eval + operand, operand, result);
                expr.truncate(expr_len);

                // Try -
                expr.push('-');
                expr.push_str(operand_str);
                backtrack(
                    digits,
                    i + 1,
                    target,
                    expr,
                    eval - operand,
                    -operand,
                    result,
                );
                expr.truncate(expr_len);

                // Try *
                expr.push('*');
                expr.push_str(operand_str);
                backtrack(
                    digits,
                    i + 1,
                    target,
                    expr,
                    eval - prev_operand + prev_operand * operand,
                    prev_operand * operand,
                    result,
                );
                expr.truncate(expr_len);
            }
        }
    }
    let mut expr = String::new();
    backtrack(digits, 0, target, &mut expr, 0, 0, &mut result);
    result.sort();
    result
}
