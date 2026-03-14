use rand::Rng;

use std::cell::RefCell;
use std::rc::Rc;

use crate::problems::helpers;
use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part1_foundations::recursion as solutions;
use crate::tracker::{track_slice, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy (5)
        Box::new(Fibonacci),
        Box::new(PowerOfTwo),
        Box::new(ReverseString),
        Box::new(SumList),
        Box::new(MaxDepthTree),
        // Medium (5)
        Box::new(Permutations),
        Box::new(Subsets),
        Box::new(LetterCombinations),
        Box::new(Pow),
        Box::new(TowerOfHanoi),
        // Hard (5)
        Box::new(NQueens),
        Box::new(SudokuSolver),
        Box::new(RegexMatch),
        Box::new(WordSearch),
        Box::new(StrobogrammaticIII),
    ]
}

// ── Easy 1: Fibonacci ───────────────────────────────────────────────────

struct Fibonacci;
struct FibonacciTest {
    n: u32,
}

impl Problem for Fibonacci {
    fn id(&self) -> &str {
        "recursion_fibonacci"
    }
    fn name(&self) -> &str {
        "Fibonacci Number"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Return the nth Fibonacci number.\n\n\
         F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2).\n\n\
         Constraints:\n\
         - 0 <= n <= 30\n\n\
         Implement this recursively (memoization is fine)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                TestCase {
                    data: Box::new(FibonacciTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FibonacciTest>().unwrap();
        let expected = ref_fibonacci(t.n);
        let actual = solutions::fibonacci(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

// ── Easy 2: Power of Two ────────────────────────────────────────────────

struct PowerOfTwo;
struct PowerOfTwoTest {
    n: i32,
}

impl Problem for PowerOfTwo {
    fn id(&self) -> &str {
        "recursion_power_of_two"
    }
    fn name(&self) -> &str {
        "Power of Two"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Check if `n` is a power of two using recursion.\n\n\
         A power of two is a number of the form 2^k where k >= 0.\n\n\
         Return `true` if n is a power of two, `false` otherwise.\n\n\
         Constraints:\n\
         - -2^31 <= n <= 2^31 - 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..7)
            .map(|_| {
                let n = rng.random_range(-100..=10000);
                TestCase {
                    data: Box::new(PowerOfTwoTest { n }),
                }
            })
            .collect();
        // Ensure some known powers of two
        for &n in &[1, 2, 4, 16, 64, 1024, 0, -1] {
            tests.push(TestCase {
                data: Box::new(PowerOfTwoTest { n }),
            });
        }
        // Trim to 10
        tests.truncate(10);
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PowerOfTwoTest>().unwrap();
        let expected = ref_power_of_two(t.n);
        let actual = solutions::is_power_of_two(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_power_of_two(n: i32) -> bool {
    n > 0 && n & (n - 1) == 0
}

// ── Easy 3: Reverse String ──────────────────────────────────────────────

struct ReverseString;
struct ReverseStringTest {
    s: String,
}

impl Problem for ReverseString {
    fn id(&self) -> &str {
        "recursion_reverse_string"
    }
    fn name(&self) -> &str {
        "Reverse String Recursively"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Reverse a string using recursion.\n\n\
         Do NOT use `.rev()` or `.chars().rev()`. Implement the reversal recursively.\n\n\
         Constraints:\n\
         - 0 <= s.len() <= 100\n\
         - s contains only lowercase ASCII letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(0..=20);
                let s = helpers::random_string(&mut rng, len);
                TestCase {
                    data: Box::new(ReverseStringTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReverseStringTest>().unwrap();
        let expected: String = t.s.chars().rev().collect();
        let actual = solutions::reverse_string(&t.s);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("\"{expected}\""),
            actual: format!("\"{actual}\""),
        }
    }
}

// ── Easy 4: Sum List ────────────────────────────────────────────────────

struct SumList;
struct SumListTest {
    nums: Vec<i32>,
}

impl Problem for SumList {
    fn id(&self) -> &str {
        "recursion_sum_list"
    }
    fn name(&self) -> &str {
        "Sum List Recursively"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Sum all elements in a list using recursion.\n\n\
         Do NOT use `.iter().sum()` or loops. Implement the sum recursively:\n\
         sum([]) = 0, sum([head, ...rest]) = head + sum(rest).\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 100\n\
         - -1000 <= nums[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let nums = helpers::random_vec(&mut rng, n, -1000, 1000);
                TestCase {
                    data: Box::new(SumListTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SumListTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected: i32 = t.nums.iter().sum();
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sum_list(&tracked_nums);
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

// ── Easy 5: Max Depth of Binary Tree ────────────────────────────────────

struct MaxDepthTree;
struct MaxDepthTreeTest {
    level_order: Vec<Option<i32>>,
}

impl Problem for MaxDepthTree {
    fn id(&self) -> &str {
        "recursion_max_depth_tree"
    }
    fn name(&self) -> &str {
        "Maximum Depth of Binary Tree"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a binary tree represented as a level-order `Vec<Option<i32>>`, \
         return the maximum depth (number of nodes on the longest root-to-leaf path).\n\n\
         An empty tree has depth 0. A single-node tree has depth 1.\n\n\
         Build the tree using `crate::problems::helpers::build_tree`, then \
         recursively compute max(depth(left), depth(right)) + 1.\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let size = rng.random_range(1..=15);
                let level_order = helpers::random_tree(&mut rng, size, -100, 100);
                TestCase {
                    data: Box::new(MaxDepthTreeTest { level_order }),
                }
            })
            .collect();
        // Add edge case: empty tree
        tests.push(TestCase {
            data: Box::new(MaxDepthTreeTest {
                level_order: vec![],
            }),
        });
        // Add edge case: single node
        tests.push(TestCase {
            data: Box::new(MaxDepthTreeTest {
                level_order: vec![Some(1)],
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxDepthTreeTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_max_depth(&t.level_order);
        let tracked_level_order = track_slice(&t.level_order, shared_log.clone());
        let actual = solutions::max_depth_tree(&tracked_level_order);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.level_order),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_depth(level_order: &[Option<i32>]) -> i32 {
    let (arena, root) = helpers::build_tree(level_order);
    fn depth(arena: &[helpers::TreeNode], node: Option<usize>) -> i32 {
        match node {
            None => 0,
            Some(idx) => {
                let l = depth(arena, arena[idx].left);
                let r = depth(arena, arena[idx].right);
                1 + l.max(r)
            }
        }
    }
    depth(&arena, root)
}

// ── Medium 1: Permutations ──────────────────────────────────────────────

struct Permutations;
struct PermutationsTest {
    nums: Vec<i32>,
}

impl Problem for Permutations {
    fn id(&self) -> &str {
        "recursion_permutations"
    }
    fn name(&self) -> &str {
        "Permutations"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a vector of distinct integers, return all possible permutations.\n\n\
         Return the permutations sorted lexicographically.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 6\n\
         - All elements are distinct."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=5);
                let nums = helpers::random_unique_vec(&mut rng, n, -10, 10);
                TestCase {
                    data: Box::new(PermutationsTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PermutationsTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_permutations(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::permutations(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{} permutations", expected.len()),
            actual: format!("{} permutations", actual.len()),
        }
    }
}

fn ref_permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = nums.to_vec();
    fn backtrack(start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if start == current.len() {
            result.push(current.clone());
            return;
        }
        for i in start..current.len() {
            current.swap(start, i);
            backtrack(start + 1, current, result);
            current.swap(start, i);
        }
    }
    backtrack(0, &mut current, &mut result);
    result.sort();
    result
}

// ── Medium 2: Subsets ───────────────────────────────────────────────────

struct Subsets;
struct SubsetsTest {
    nums: Vec<i32>,
}

impl Problem for Subsets {
    fn id(&self) -> &str {
        "recursion_subsets"
    }
    fn name(&self) -> &str {
        "Subsets"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a vector of distinct integers, return all possible subsets (the power set).\n\n\
         Return the subsets sorted: each subset sorted internally, then all subsets \
         sorted lexicographically.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 8\n\
         - All elements are distinct."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=6);
                let nums = helpers::random_unique_vec(&mut rng, n, -10, 10);
                TestCase {
                    data: Box::new(SubsetsTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SubsetsTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_subsets(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::subsets(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{} subsets", expected.len()),
            actual: format!("{} subsets", actual.len()),
        }
    }
}

fn ref_subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut current = Vec::new();
    let mut sorted = nums.to_vec();
    sorted.sort();
    fn backtrack(idx: usize, sorted: &[i32], current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(current.clone());
        for i in idx..sorted.len() {
            current.push(sorted[i]);
            backtrack(i + 1, sorted, current, result);
            current.pop();
        }
    }
    backtrack(0, &sorted, &mut current, &mut result);
    result.sort();
    result
}

// ── Medium 3: Letter Combinations ───────────────────────────────────────

struct LetterCombinations;
struct LetterCombinationsTest {
    digits: String,
}

impl Problem for LetterCombinations {
    fn id(&self) -> &str {
        "recursion_letter_combinations"
    }
    fn name(&self) -> &str {
        "Letter Combinations of a Phone Number"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a string containing digits from '2' to '9', return all possible letter \
         combinations that the number could represent (like a phone keypad).\n\n\
         Mapping:\n\
         2->abc, 3->def, 4->ghi, 5->jkl, 6->mno, 7->pqrs, 8->tuv, 9->wxyz\n\n\
         Return the combinations sorted lexicographically.\n\
         If the input is empty, return an empty vector.\n\n\
         Constraints:\n\
         - 0 <= digits.len() <= 4\n\
         - Each digit is in '2'..='9'."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let len = rng.random_range(1..=4);
                let digits: String = (0..len)
                    .map(|_| (b'2' + rng.random_range(0..8u8)) as char)
                    .collect();
                TestCase {
                    data: Box::new(LetterCombinationsTest { digits }),
                }
            })
            .collect();
        // Edge case: empty string
        tests.push(TestCase {
            data: Box::new(LetterCombinationsTest {
                digits: String::new(),
            }),
        });
        // Edge case: single digit
        tests.push(TestCase {
            data: Box::new(LetterCombinationsTest {
                digits: "7".to_string(),
            }),
        });
        tests
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
    let mapping: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut result = Vec::new();
    let mut current = String::new();
    fn backtrack(
        idx: usize,
        digits: &[u8],
        mapping: &[&str; 8],
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if idx == digits.len() {
            result.push(current.clone());
            return;
        }
        let digit = (digits[idx] - b'2') as usize;
        for ch in mapping[digit].chars() {
            current.push(ch);
            backtrack(idx + 1, digits, mapping, current, result);
            current.pop();
        }
    }
    backtrack(0, digits.as_bytes(), &mapping, &mut current, &mut result);
    result.sort();
    result
}

// ── Medium 4: Pow(x, n) ────────────────────────────────────────────────

struct Pow;
struct PowTest {
    x: f64,
    n: i32,
}

impl Problem for Pow {
    fn id(&self) -> &str {
        "recursion_pow"
    }
    fn name(&self) -> &str {
        "Pow(x, n)"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Implement pow(x, n), which calculates x raised to the power n (x^n).\n\n\
         Use recursive fast exponentiation:\n\
         - x^0 = 1\n\
         - x^n = (x^(n/2))^2 if n is even\n\
         - x^n = x * (x^(n/2))^2 if n is odd\n\
         - Handle negative exponents: x^(-n) = 1 / x^n\n\n\
         Constraints:\n\
         - -100.0 < x < 100.0\n\
         - -30 <= n <= 30\n\
         - Result within 1e-5 of expected."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..7)
            .map(|_| {
                let x = (rng.random_range(-100..=100) as f64) / 10.0;
                let n = rng.random_range(-15..=15);
                TestCase {
                    data: Box::new(PowTest { x, n }),
                }
            })
            .collect();
        // Ensure some edge cases
        tests.push(TestCase {
            data: Box::new(PowTest { x: 2.0, n: 10 }),
        });
        tests.push(TestCase {
            data: Box::new(PowTest { x: 2.0, n: -2 }),
        });
        tests.push(TestCase {
            data: Box::new(PowTest { x: 0.0, n: 5 }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PowTest>().unwrap();
        // Skip cases where x==0 and n<0 (undefined)
        if t.x == 0.0 && t.n < 0 {
            return SolutionResult {
                is_correct: true,
                input_description: format!("x={}, n={} (skipped: 0^negative undefined)", t.x, t.n),
                expected: "skipped".to_string(),
                actual: "skipped".to_string(),
            };
        }
        let expected = ref_pow(t.x, t.n);
        let actual = solutions::pow(t.x, t.n);
        let correct = if expected.is_infinite() || expected.is_nan() {
            (actual.is_infinite() && expected.is_infinite())
                || (actual.is_nan() && expected.is_nan())
        } else {
            (expected - actual).abs() < 1e-5
                || (expected != 0.0 && ((expected - actual) / expected).abs() < 1e-5)
        };
        SolutionResult {
            is_correct: correct,
            input_description: format!("x={}, n={}", t.x, t.n),
            expected: format!("{expected:.10}"),
            actual: format!("{actual:.10}"),
        }
    }
}

fn ref_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n < 0 {
        return 1.0 / ref_pow(x, -n);
    }
    let half = ref_pow(x, n / 2);
    if n % 2 == 0 {
        half * half
    } else {
        x * half * half
    }
}

// ── Medium 5: Tower of Hanoi ────────────────────────────────────────────

struct TowerOfHanoi;
struct TowerOfHanoiTest {
    num_disks: u32,
}

impl Problem for TowerOfHanoi {
    fn id(&self) -> &str {
        "recursion_tower_of_hanoi"
    }
    fn name(&self) -> &str {
        "Tower of Hanoi"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Solve the Tower of Hanoi puzzle. Return the list of moves as `(from_peg, to_peg)` pairs.\n\n\
         Pegs are numbered 0, 1, 2. Move all disks from peg 0 to peg 2 using peg 1 as auxiliary.\n\n\
         Rules:\n\
         - Move one disk at a time.\n\
         - Never place a larger disk on a smaller one.\n\n\
         Constraints:\n\
         - 1 <= num_disks <= 10"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let num_disks = rng.random_range(1..=8);
                TestCase {
                    data: Box::new(TowerOfHanoiTest { num_disks }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TowerOfHanoiTest>().unwrap();
        let expected = ref_hanoi(t.num_disks);
        let actual = solutions::tower_of_hanoi(t.num_disks);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("num_disks={}", t.num_disks),
            expected: format!("{} moves", expected.len()),
            actual: format!(
                "{} moves, {:?}",
                actual.len(),
                if actual.len() <= 15 {
                    &actual[..]
                } else {
                    &actual[..15]
                }
            ),
        }
    }
}

fn ref_hanoi(n: u32) -> Vec<(u8, u8)> {
    let mut moves = Vec::new();
    fn solve(n: u32, from: u8, to: u8, aux: u8, moves: &mut Vec<(u8, u8)>) {
        if n == 0 {
            return;
        }
        solve(n - 1, from, aux, to, moves);
        moves.push((from, to));
        solve(n - 1, aux, to, from, moves);
    }
    solve(n, 0, 2, 1, &mut moves);
    moves
}

// ── Hard 1: N-Queens ────────────────────────────────────────────────────

struct NQueens;
struct NQueensTest {
    n: u32,
}

impl Problem for NQueens {
    fn id(&self) -> &str {
        "recursion_n_queens"
    }
    fn name(&self) -> &str {
        "N-Queens"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "The N-Queens puzzle: place N queens on an NxN chessboard so that no two \
         queens threaten each other.\n\n\
         Return the number of distinct solutions.\n\n\
         Constraints:\n\
         - 1 <= n <= 10"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        // N-Queens is expensive for large n, use specific values
        let test_values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        test_values
            .iter()
            .map(|&n| TestCase {
                data: Box::new(NQueensTest { n }),
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
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_n_queens(n: u32) -> u32 {
    let n = n as usize;
    let mut count = 0u32;
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n]; // row + col
    let mut diag2 = vec![false; 2 * n]; // row - col + n
    fn solve(
        row: usize,
        n: usize,
        cols: &mut Vec<bool>,
        diag1: &mut Vec<bool>,
        diag2: &mut Vec<bool>,
        count: &mut u32,
    ) {
        if row == n {
            *count += 1;
            return;
        }
        for col in 0..n {
            let d1 = row + col;
            let d2 = row + n - col;
            if !cols[col] && !diag1[d1] && !diag2[d2] {
                cols[col] = true;
                diag1[d1] = true;
                diag2[d2] = true;
                solve(row + 1, n, cols, diag1, diag2, count);
                cols[col] = false;
                diag1[d1] = false;
                diag2[d2] = false;
            }
        }
    }
    solve(0, n, &mut cols, &mut diag1, &mut diag2, &mut count);
    count
}

// ── Hard 2: Sudoku Solver ───────────────────────────────────────────────

struct SudokuSolver;
struct SudokuTest {
    board: Vec<Vec<u8>>,
}

impl Problem for SudokuSolver {
    fn id(&self) -> &str {
        "recursion_sudoku_solver"
    }
    fn name(&self) -> &str {
        "Sudoku Solver"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Solve a 9x9 Sudoku puzzle using recursive backtracking.\n\n\
         Input: a 9x9 grid where 0 represents an empty cell.\n\
         Output: the solved 9x9 grid with all cells filled (1-9).\n\n\
         The solution must satisfy standard Sudoku rules:\n\
         - Each row contains 1-9 exactly once.\n\
         - Each column contains 1-9 exactly once.\n\
         - Each 3x3 sub-box contains 1-9 exactly once.\n\n\
         The input puzzle is guaranteed to have exactly one solution."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        // Use a set of known solvable puzzles (generating random valid Sudoku is nontrivial).
        let puzzles = vec![
            vec![
                vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
                vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
                vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
                vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
                vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
                vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
                vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
                vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
                vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
            ],
            vec![
                vec![0, 0, 0, 2, 6, 0, 7, 0, 1],
                vec![6, 8, 0, 0, 7, 0, 0, 9, 0],
                vec![1, 9, 0, 0, 0, 4, 5, 0, 0],
                vec![8, 2, 0, 1, 0, 0, 0, 4, 0],
                vec![0, 0, 4, 6, 0, 2, 9, 0, 0],
                vec![0, 5, 0, 0, 0, 3, 0, 2, 8],
                vec![0, 0, 9, 3, 0, 0, 0, 7, 4],
                vec![0, 4, 0, 0, 5, 0, 0, 3, 6],
                vec![7, 0, 3, 0, 1, 8, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 6, 0, 0, 4, 0, 0],
                vec![7, 0, 0, 0, 0, 3, 6, 0, 0],
                vec![0, 0, 0, 0, 9, 1, 0, 8, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 5, 0, 1, 8, 0, 0, 0, 3],
                vec![0, 0, 0, 3, 0, 6, 0, 4, 5],
                vec![0, 4, 0, 2, 0, 0, 0, 6, 0],
                vec![9, 0, 3, 0, 0, 0, 0, 0, 0],
                vec![0, 2, 0, 0, 0, 0, 1, 0, 0],
            ],
            vec![
                vec![2, 0, 0, 3, 0, 0, 0, 0, 0],
                vec![8, 0, 4, 0, 6, 2, 0, 0, 3],
                vec![0, 1, 3, 8, 0, 0, 2, 0, 0],
                vec![0, 0, 0, 0, 2, 0, 3, 9, 0],
                vec![5, 0, 7, 0, 0, 0, 6, 2, 1],
                vec![0, 3, 2, 0, 0, 6, 0, 0, 0],
                vec![0, 2, 0, 0, 0, 9, 1, 4, 0],
                vec![6, 0, 1, 2, 5, 0, 8, 0, 9],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 2],
            ],
            vec![
                vec![0, 2, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 6, 0, 0, 0, 0, 3],
                vec![0, 7, 4, 0, 8, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 3, 0, 0, 2],
                vec![0, 8, 0, 0, 4, 0, 0, 1, 0],
                vec![6, 0, 0, 5, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 7, 8, 0],
                vec![5, 0, 0, 0, 0, 9, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 4, 0],
            ],
        ];
        puzzles
            .into_iter()
            .map(|board| TestCase {
                data: Box::new(SudokuTest { board }),
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SudokuTest>().unwrap();
        let expected = ref_solve_sudoku(&t.board);
        let actual = solutions::solve_sudoku(&t.board);
        let correct = expected == actual && is_valid_sudoku(&actual);
        SolutionResult {
            is_correct: correct,
            input_description: format!(
                "sudoku ({}  empty cells)",
                t.board.iter().flatten().filter(|&&v| v == 0).count()
            ),
            expected: format_sudoku(&expected),
            actual: format_sudoku(&actual),
        }
    }
}

fn ref_solve_sudoku(board: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = board.to_vec();
    fn solve(grid: &mut Vec<Vec<u8>>) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if grid[r][c] == 0 {
                    for num in 1..=9u8 {
                        if is_valid_placement(grid, r, c, num) {
                            grid[r][c] = num;
                            if solve(grid) {
                                return true;
                            }
                            grid[r][c] = 0;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
    fn is_valid_placement(grid: &[Vec<u8>], row: usize, col: usize, num: u8) -> bool {
        for i in 0..9 {
            if grid[row][i] == num {
                return false;
            }
            if grid[i][col] == num {
                return false;
            }
        }
        let box_r = (row / 3) * 3;
        let box_c = (col / 3) * 3;
        for row in grid.iter().skip(box_r).take(3) {
            for &cell in row.iter().skip(box_c).take(3) {
                if cell == num {
                    return false;
                }
            }
        }
        true
    }
    solve(&mut grid);
    grid
}

fn is_valid_sudoku(board: &[Vec<u8>]) -> bool {
    if board.len() != 9 {
        return false;
    }
    for row in board {
        if row.len() != 9 {
            return false;
        }
    }
    // Check rows
    for row in board {
        let mut seen = [false; 10];
        for &val in row {
            if !(1..=9).contains(&val) || seen[val as usize] {
                return false;
            }
            seen[val as usize] = true;
        }
    }
    // Check columns
    for c in 0..9 {
        let mut seen = [false; 10];
        for row in board.iter().take(9) {
            let val = row[c];
            if !(1..=9).contains(&val) || seen[val as usize] {
                return false;
            }
            seen[val as usize] = true;
        }
    }
    // Check 3x3 boxes
    for box_r in 0..3 {
        for box_c in 0..3 {
            let mut seen = [false; 10];
            for r in 0..3 {
                for c in 0..3 {
                    let val = board[box_r * 3 + r][box_c * 3 + c];
                    if !(1..=9).contains(&val) || seen[val as usize] {
                        return false;
                    }
                    seen[val as usize] = true;
                }
            }
        }
    }
    true
}

fn format_sudoku(board: &[Vec<u8>]) -> String {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ── Hard 3: Regex Match ─────────────────────────────────────────────────

struct RegexMatch;
struct RegexMatchTest {
    s: String,
    p: String,
}

impl Problem for RegexMatch {
    fn id(&self) -> &str {
        "recursion_regex_match"
    }
    fn name(&self) -> &str {
        "Regular Expression Matching"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Implement regular expression matching with support for '.' and '*'.\n\n\
         '.' matches any single character.\n\
         '*' matches zero or more of the preceding element.\n\n\
         The matching should cover the entire input string (not partial).\n\n\
         Examples:\n\
         - is_match(\"aa\", \"a\") -> false\n\
         - is_match(\"aa\", \"a*\") -> true\n\
         - is_match(\"ab\", \".*\") -> true\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 20\n\
         - 1 <= p.len() <= 20\n\
         - s contains only lowercase letters\n\
         - p contains only lowercase letters, '.', and '*'"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        // Regex matching is tricky to randomly generate valid patterns, so use curated cases
        let cases = vec![
            ("aa", "a"),
            ("aa", "a*"),
            ("ab", ".*"),
            ("aab", "c*a*b"),
            ("mississippi", "mis*is*p*."),
            ("ab", ".*c"),
            ("aaa", "a*a"),
            ("", "c*"),
            ("a", "ab*"),
            ("abcd", "d*"),
        ];
        cases
            .into_iter()
            .map(|(s, p)| TestCase {
                data: Box::new(RegexMatchTest {
                    s: s.to_string(),
                    p: p.to_string(),
                }),
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RegexMatchTest>().unwrap();
        let expected = ref_regex_match(&t.s, &t.p);
        let actual = solutions::regex_match(&t.s, &t.p);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\", p=\"{}\"", t.s, t.p),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_regex_match(s: &str, p: &str) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    fn dp(s: &[u8], p: &[u8], si: usize, pi: usize) -> bool {
        if pi == p.len() {
            return si == s.len();
        }
        let first_match = si < s.len() && (p[pi] == b'.' || p[pi] == s[si]);
        if pi + 1 < p.len() && p[pi + 1] == b'*' {
            // '*' matches zero occurrences, OR one-and-advance-in-s
            dp(s, p, si, pi + 2) || (first_match && dp(s, p, si + 1, pi))
        } else {
            first_match && dp(s, p, si + 1, pi + 1)
        }
    }
    dp(s, p, 0, 0)
}

// ── Hard 4: Word Search ─────────────────────────────────────────────────

struct WordSearch;
struct WordSearchTest {
    board: Vec<Vec<char>>,
    word: String,
}

impl Problem for WordSearch {
    fn id(&self) -> &str {
        "recursion_word_search"
    }
    fn name(&self) -> &str {
        "Word Search"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an m x n grid of characters `board` and a string `word`, return `true` if \
         `word` exists in the grid.\n\n\
         The word can be constructed from letters of sequentially adjacent cells, where \
         adjacent cells are horizontally or vertically neighboring. The same cell may not \
         be used more than once.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 6\n\
         - 1 <= word.len() <= 10\n\
         - board and word contain only lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests = Vec::new();
        for _ in 0..7 {
            let rows = rng.random_range(2..=5);
            let cols = rng.random_range(2..=5);
            let board: Vec<Vec<char>> = (0..rows)
                .map(|_| {
                    (0..cols)
                        .map(|_| (b'a' + rng.random_range(0..6u8)) as char)
                        .collect()
                })
                .collect();
            // Pick a word that either exists in the grid or doesn't
            let word_len = rng.random_range(2..=5);
            let word: String = if rng.random_range(0..2) == 0 {
                // Try to create a word from the grid (walk a random path)
                extract_random_word(&board, word_len, &mut rng)
            } else {
                helpers::random_string_from(&mut rng, word_len, b"abcdef")
            };
            tests.push(TestCase {
                data: Box::new(WordSearchTest { board, word }),
            });
        }
        // Known positive case
        tests.push(TestCase {
            data: Box::new(WordSearchTest {
                board: vec![
                    vec!['a', 'b', 'c', 'e'],
                    vec!['s', 'f', 'c', 's'],
                    vec!['a', 'd', 'e', 'e'],
                ],
                word: "abcced".to_string(),
            }),
        });
        // Known negative case
        tests.push(TestCase {
            data: Box::new(WordSearchTest {
                board: vec![
                    vec!['a', 'b', 'c', 'e'],
                    vec!['s', 'f', 'c', 's'],
                    vec!['a', 'd', 'e', 'e'],
                ],
                word: "abcb".to_string(),
            }),
        });
        // Single cell
        tests.push(TestCase {
            data: Box::new(WordSearchTest {
                board: vec![vec!['a']],
                word: "a".to_string(),
            }),
        });
        tests
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

fn extract_random_word(board: &[Vec<char>], len: usize, rng: &mut impl Rng) -> String {
    let rows = board.len();
    let cols = board[0].len();
    let mut r = rng.random_range(0..rows);
    let mut c = rng.random_range(0..cols);
    let mut word = String::new();
    let mut visited = vec![vec![false; cols]; rows];
    for _ in 0..len {
        word.push(board[r][c]);
        visited[r][c] = true;
        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut moved = false;
        let start = rng.random_range(0..4usize);
        for di in 0..4 {
            let (dr, dc) = dirs[(start + di) % 4];
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
    let word = word.as_bytes();
    let rows = board.len();
    let cols = board[0].len();
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
        if r >= board.len() || c >= board[0].len() || visited[r][c] {
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
        // Also check if we are at the last character
        if idx + 1 == word.len() {
            visited[r][c] = false;
            return true;
        }
        visited[r][c] = false;
        false
    }
    for r in 0..rows {
        for c in 0..cols {
            if dfs(board, word, 0, r, c, &mut visited) {
                return true;
            }
        }
    }
    false
}

// ── Hard 5: Strobogrammatic Number III ──────────────────────────────────

struct StrobogrammaticIII;
struct StrobogrammaticTest {
    low: String,
    high: String,
}

impl Problem for StrobogrammaticIII {
    fn id(&self) -> &str {
        "recursion_strobogrammatic_iii"
    }
    fn name(&self) -> &str {
        "Strobogrammatic Number III"
    }
    fn topic(&self) -> &str {
        "recursion"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "A strobogrammatic number is a number that looks the same when rotated 180 degrees \
         (looked at upside down). The strobogrammatic digits are: 0, 1, 8, and the pairs \
         (6, 9) and (9, 6).\n\n\
         Given two strings `low` and `high` representing the range [low, high], return \
         the count of strobogrammatic numbers in that range (inclusive).\n\n\
         Examples of strobogrammatic numbers: 0, 1, 8, 11, 69, 88, 96, 101, ...\n\n\
         Constraints:\n\
         - 1 <= low.len(), high.len() <= 14\n\
         - low and high consist of only digits\n\
         - low <= high (as numeric values)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let cases = vec![
            ("50", "100"),
            ("0", "0"),
            ("0", "1"),
            ("0", "100"),
            ("100", "1000"),
            ("1000", "10000"),
            ("0", "10000"),
            ("10000", "100000"),
            ("1", "1000000"),
            ("50000", "100000"),
        ];
        cases
            .into_iter()
            .map(|(low, high)| TestCase {
                data: Box::new(StrobogrammaticTest {
                    low: low.to_string(),
                    high: high.to_string(),
                }),
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<StrobogrammaticTest>().unwrap();
        let expected = ref_strobogrammatic_iii(&t.low, &t.high);
        let actual = solutions::strobogrammatic_in_range(&t.low, &t.high);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("low=\"{}\", high=\"{}\"", t.low, t.high),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_strobogrammatic_iii(low: &str, high: &str) -> i32 {
    let lo_len = low.len();
    let hi_len = high.len();
    let mut count = 0;
    for len in lo_len..=hi_len {
        let nums = generate_strobogrammatic(len);
        for num in &nums {
            if num.len() == lo_len && num.as_str() < low {
                continue;
            }
            if num.len() == hi_len && num.as_str() > high {
                continue;
            }
            count += 1;
        }
    }
    count
}

fn generate_strobogrammatic(n: usize) -> Vec<String> {
    fn helper(n: usize, total: usize) -> Vec<String> {
        if n == 0 {
            return vec![String::new()];
        }
        if n == 1 {
            return vec!["0".to_string(), "1".to_string(), "8".to_string()];
        }
        let middles = helper(n - 2, total);
        let mut result = Vec::new();
        let pairs = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];
        for mid in &middles {
            for &(l, r) in &pairs {
                // Skip leading zeros for multi-digit numbers
                if l == '0' && n == total && total > 1 {
                    continue;
                }
                result.push(format!("{l}{mid}{r}"));
            }
        }
        result
    }
    helper(n, n)
}

// The comparison for strobogrammatic uses string comparison which works correctly
// when strings have the same length. For different lengths, shorter is always smaller
// (which is guaranteed by iterating by length).
