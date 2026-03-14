use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part5_paradigms::dynamic_programming as solutions;
use crate::tracker::{track_slice, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(ClimbingStairs),
        Box::new(HouseRobber),
        Box::new(MinCostStairs),
        Box::new(Fibonacci),
        Box::new(MaxSubarray),
        Box::new(CoinChange),
        Box::new(LongestIncreasingSubsequence),
        Box::new(UniquePaths),
        Box::new(WordBreak),
        Box::new(LongestCommonSubsequence),
        Box::new(EditDistance),
        Box::new(BurstBalloons),
        Box::new(RegularExpression),
        Box::new(LongestValidParentheses),
        Box::new(MaxProfitWithCooldown),
    ]
}

// ── Easy 1: Climbing Stairs ──────────────────────────────────────────

struct ClimbingStairs;
struct ClimbingStairsTest {
    n: i32,
}

impl Problem for ClimbingStairs {
    fn id(&self) -> &str {
        "dp_climbing_stairs"
    }
    fn name(&self) -> &str {
        "Climbing Stairs"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "You are climbing a staircase. It takes `n` steps to reach the top. \
         Each time you can climb 1 or 2 steps. In how many distinct ways \
         can you climb to the top?\n\n\
         Constraints:\n\
         - 1 <= n <= 45\n\
         - Return the number of distinct ways."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                TestCase {
                    data: Box::new(ClimbingStairsTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ClimbingStairsTest>().unwrap();
        let expected = ref_climbing_stairs(t.n);
        let actual = solutions::climbing_stairs(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_climbing_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }
    let (mut a, mut b) = (1, 2);
    for _ in 3..=n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

// ── Easy 2: House Robber ─────────────────────────────────────────────

struct HouseRobber;
struct HouseRobberTest {
    nums: Vec<i32>,
}

impl Problem for HouseRobber {
    fn id(&self) -> &str {
        "dp_house_robber"
    }
    fn name(&self) -> &str {
        "House Robber"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "You are a robber planning to rob houses along a street. Each house has \
         a certain amount of money stashed. Adjacent houses have security systems \
         connected -- if two adjacent houses are robbed the same night, the police \
         will be alerted.\n\n\
         Return the maximum amount of money you can rob without alerting the police.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - 0 <= nums[i] <= 400"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                TestCase {
                    data: Box::new(HouseRobberTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<HouseRobberTest>().unwrap();
        let expected = ref_house_robber(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::house_robber(&tracked_nums);
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

fn ref_house_robber(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return nums[0];
    }
    let (mut prev2, mut prev1) = (nums[0], nums[0].max(nums[1]));
    for &num in nums.iter().take(n).skip(2) {
        let cur = prev1.max(prev2 + num);
        prev2 = prev1;
        prev1 = cur;
    }
    prev1
}

// ── Easy 3: Min Cost Climbing Stairs ─────────────────────────────────

struct MinCostStairs;
struct MinCostStairsTest {
    cost: Vec<i32>,
}

impl Problem for MinCostStairs {
    fn id(&self) -> &str {
        "dp_min_cost_stairs"
    }
    fn name(&self) -> &str {
        "Min Cost Climbing Stairs"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an integer array `cost` where `cost[i]` is the cost of the i-th step, \
         once you pay the cost you can climb 1 or 2 steps. You can start from step 0 or 1. \
         Return the minimum cost to reach the top of the floor (past the last step).\n\n\
         Constraints:\n\
         - 2 <= cost.len() <= 1000\n\
         - 0 <= cost[i] <= 999"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=20);
                let cost: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                TestCase {
                    data: Box::new(MinCostStairsTest { cost }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinCostStairsTest>().unwrap();
        let expected = ref_min_cost_stairs(&t.cost);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_cost = track_slice(&t.cost, shared_log.clone());
        let actual = solutions::min_cost_stairs(&tracked_cost);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("cost={:?}", t.cost),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_cost_stairs(cost: &[i32]) -> i32 {
    let n = cost.len();
    let (mut a, mut b) = (cost[0], cost[1]);
    for &c in cost.iter().take(n).skip(2) {
        let cur = c + a.min(b);
        a = b;
        b = cur;
    }
    a.min(b)
}

// ── Easy 4: Fibonacci ────────────────────────────────────────────────

struct Fibonacci;
struct FibonacciTest {
    n: i32,
}

impl Problem for Fibonacci {
    fn id(&self) -> &str {
        "dp_fibonacci"
    }
    fn name(&self) -> &str {
        "Fibonacci (DP)"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Compute the nth Fibonacci number using dynamic programming.\n\
         F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n >= 2.\n\n\
         Constraints:\n\
         - 0 <= n <= 60\n\
         - Return as i64."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=50);
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

fn ref_fibonacci(n: i32) -> i64 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let (mut a, mut b): (i64, i64) = (0, 1);
    for _ in 2..=n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

// ── Easy 5: Maximum Subarray ─────────────────────────────────────────

struct MaxSubarray;
struct MaxSubarrayTest {
    nums: Vec<i32>,
}

impl Problem for MaxSubarray {
    fn id(&self) -> &str {
        "dp_max_subarray"
    }
    fn name(&self) -> &str {
        "Maximum Subarray (DP)"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an integer array `nums`, find the contiguous subarray with the \
         largest sum and return that sum (Kadane's algorithm).\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 10^5\n\
         - -10^4 <= nums[i] <= 10^4"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=50);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(MaxSubarrayTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxSubarrayTest>().unwrap();
        let expected = ref_max_subarray(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::max_subarray(&tracked_nums);
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

fn ref_max_subarray(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut cur = nums[0];
    for &n in &nums[1..] {
        cur = n.max(cur + n);
        max_sum = max_sum.max(cur);
    }
    max_sum
}

// ── Medium 1: Coin Change ────────────────────────────────────────────

struct CoinChange;
struct CoinChangeTest {
    coins: Vec<i32>,
    amount: i32,
}

impl Problem for CoinChange {
    fn id(&self) -> &str {
        "dp_coin_change"
    }
    fn name(&self) -> &str {
        "Coin Change"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array of coin denominations and a target amount, return the fewest \
         number of coins needed to make up that amount. Return -1 if it cannot be done.\n\n\
         Constraints:\n\
         - 1 <= coins.len() <= 12\n\
         - 1 <= coins[i] <= 2^31 - 1\n\
         - 0 <= amount <= 10^4"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let num_coins = rng.random_range(1..=5);
                let coins: Vec<i32> = (0..num_coins).map(|_| rng.random_range(1..=20)).collect();
                let amount = rng.random_range(0..=100);
                TestCase {
                    data: Box::new(CoinChangeTest { coins, amount }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CoinChangeTest>().unwrap();
        let expected = ref_coin_change(&t.coins, t.amount);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_coins = track_slice(&t.coins, shared_log.clone());
        let actual = solutions::coin_change(&tracked_coins, t.amount);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("coins={:?}, amount={}", t.coins, t.amount),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_coin_change(coins: &[i32], amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![i32::MAX; amount + 1];
    dp[0] = 0;
    for i in 1..=amount {
        for &c in coins {
            let c = c as usize;
            if c <= i && dp[i - c] != i32::MAX {
                dp[i] = dp[i].min(dp[i - c] + 1);
            }
        }
    }
    if dp[amount] == i32::MAX {
        -1
    } else {
        dp[amount]
    }
}

// ── Medium 2: Longest Increasing Subsequence ─────────────────────────

struct LongestIncreasingSubsequence;
struct LISTest {
    nums: Vec<i32>,
}

impl Problem for LongestIncreasingSubsequence {
    fn id(&self) -> &str {
        "dp_longest_increasing_subsequence"
    }
    fn name(&self) -> &str {
        "Longest Increasing Subsequence"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an integer array `nums`, return the length of the longest strictly \
         increasing subsequence.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 2500\n\
         - -10^4 <= nums[i] <= 10^4"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-50..=50)).collect();
                TestCase {
                    data: Box::new(LISTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LISTest>().unwrap();
        let expected = ref_lis(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::longest_increasing_subsequence(&tracked_nums);
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

fn ref_lis(nums: &[i32]) -> i32 {
    // patience sorting / binary search approach
    let mut tails: Vec<i32> = Vec::new();
    for &num in nums {
        let pos = tails.partition_point(|&x| x < num);
        if pos == tails.len() {
            tails.push(num);
        } else {
            tails[pos] = num;
        }
    }
    tails.len() as i32
}

// ── Medium 3: Unique Paths ───────────────────────────────────────────

struct UniquePaths;
struct UniquePathsTest {
    m: usize,
    n: usize,
}

impl Problem for UniquePaths {
    fn id(&self) -> &str {
        "dp_unique_paths"
    }
    fn name(&self) -> &str {
        "Unique Paths"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "A robot is on an m x n grid starting at top-left corner. It can only \
         move right or down. How many unique paths are there to reach the \
         bottom-right corner?\n\n\
         Constraints:\n\
         - 1 <= m, n <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let m = rng.random_range(1..=15);
                let n = rng.random_range(1..=15);
                TestCase {
                    data: Box::new(UniquePathsTest { m, n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UniquePathsTest>().unwrap();
        let expected = ref_unique_paths(t.m, t.n);
        let actual = solutions::unique_paths(t.m, t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("m={}, n={}", t.m, t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_unique_paths(m: usize, n: usize) -> i32 {
    let mut dp = vec![1i32; n];
    for _ in 1..m {
        for j in 1..n {
            dp[j] += dp[j - 1];
        }
    }
    dp[n - 1]
}

// ── Medium 4: Word Break ────────────────────────────────────────────

struct WordBreak;
struct WordBreakTest {
    s: String,
    word_dict: Vec<String>,
}

impl Problem for WordBreak {
    fn id(&self) -> &str {
        "dp_word_break"
    }
    fn name(&self) -> &str {
        "Word Break"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a string `s` and a dictionary of words, return true if `s` can \
         be segmented into a space-separated sequence of one or more dictionary words.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 300\n\
         - 1 <= word_dict.len() <= 1000\n\
         - All strings consist of lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let alphabet = b"abcdefghij";
        (0..10)
            .map(|_| {
                let num_words = rng.random_range(2..=6);
                let word_dict: Vec<String> = (0..num_words)
                    .map(|_| {
                        let wlen = rng.random_range(1..=5);
                        (0..wlen)
                            .map(|_| {
                                let idx = rng.random_range(0..alphabet.len());
                                alphabet[idx] as char
                            })
                            .collect()
                    })
                    .collect();
                // Build s from some dictionary words (sometimes add random chars)
                let num_parts = rng.random_range(1..=4);
                let mut s = String::new();
                for _ in 0..num_parts {
                    if rng.random_range(0..=3) == 0 {
                        // Add a random segment to sometimes make it unsolvable
                        let rlen = rng.random_range(1..=3);
                        for _ in 0..rlen {
                            let idx = rng.random_range(0..alphabet.len());
                            s.push(alphabet[idx] as char);
                        }
                    } else {
                        let idx = rng.random_range(0..word_dict.len());
                        s.push_str(&word_dict[idx]);
                    }
                }
                TestCase {
                    data: Box::new(WordBreakTest { s, word_dict }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WordBreakTest>().unwrap();
        let expected = ref_word_break(&t.s, &t.word_dict);
        let actual = solutions::word_break(&t.s, &t.word_dict);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\", word_dict={:?}", t.s, t.word_dict),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_word_break(s: &str, word_dict: &[String]) -> bool {
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for w in word_dict {
            let wl = w.len();
            if wl <= i && dp[i - wl] && &s[i - wl..i] == w.as_str() {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}

// ── Medium 5: Longest Common Subsequence ─────────────────────────────

struct LongestCommonSubsequence;
struct LCSTest {
    text1: String,
    text2: String,
}

impl Problem for LongestCommonSubsequence {
    fn id(&self) -> &str {
        "dp_longest_common_subsequence"
    }
    fn name(&self) -> &str {
        "Longest Common Subsequence"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given two strings, return the length of their longest common subsequence. \
         If there is no common subsequence, return 0.\n\n\
         Constraints:\n\
         - 1 <= text1.len(), text2.len() <= 1000\n\
         - Both strings consist of lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let alphabet = b"abcdef";
        (0..10)
            .map(|_| {
                let len1 = rng.random_range(1..=15);
                let len2 = rng.random_range(1..=15);
                let text1: String = (0..len1)
                    .map(|_| alphabet[rng.random_range(0..alphabet.len())] as char)
                    .collect();
                let text2: String = (0..len2)
                    .map(|_| alphabet[rng.random_range(0..alphabet.len())] as char)
                    .collect();
                TestCase {
                    data: Box::new(LCSTest { text1, text2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LCSTest>().unwrap();
        let expected = ref_lcs(&t.text1, &t.text2);
        let actual = solutions::longest_common_subsequence(&t.text1, &t.text2);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("text1=\"{}\", text2=\"{}\"", t.text1, t.text2),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_lcs(text1: &str, text2: &str) -> i32 {
    let (a, b): (Vec<u8>, Vec<u8>) = (text1.bytes().collect(), text2.bytes().collect());
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0i32; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    dp[m][n]
}

// ── Hard 1: Edit Distance ────────────────────────────────────────────

struct EditDistance;
struct EditDistanceTest {
    word1: String,
    word2: String,
}

impl Problem for EditDistance {
    fn id(&self) -> &str {
        "dp_edit_distance"
    }
    fn name(&self) -> &str {
        "Edit Distance"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given two strings, return the minimum number of operations required to \
         convert word1 to word2. Operations: insert, delete, or replace a character.\n\n\
         Constraints:\n\
         - 0 <= word1.len(), word2.len() <= 500\n\
         - Both strings consist of lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let alphabet = b"abcde";
        (0..10)
            .map(|_| {
                let len1 = rng.random_range(0..=12);
                let len2 = rng.random_range(0..=12);
                let word1: String = (0..len1)
                    .map(|_| alphabet[rng.random_range(0..alphabet.len())] as char)
                    .collect();
                let word2: String = (0..len2)
                    .map(|_| alphabet[rng.random_range(0..alphabet.len())] as char)
                    .collect();
                TestCase {
                    data: Box::new(EditDistanceTest { word1, word2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<EditDistanceTest>().unwrap();
        let expected = ref_edit_distance(&t.word1, &t.word2);
        let actual = solutions::edit_distance(&t.word1, &t.word2);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("word1=\"{}\", word2=\"{}\"", t.word1, t.word2),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_edit_distance(word1: &str, word2: &str) -> i32 {
    let (a, b): (Vec<u8>, Vec<u8>) = (word1.bytes().collect(), word2.bytes().collect());
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0i32; n + 1]; m + 1];
    for (i, row) in dp.iter_mut().enumerate().take(m + 1) {
        row[0] = i as i32;
    }
    for j in 0..=n {
        dp[0][j] = j as i32;
    }
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}

// ── Hard 2: Burst Balloons ──────────────────────────────────────────

struct BurstBalloons;
struct BurstBalloonsTest {
    nums: Vec<i32>,
}

impl Problem for BurstBalloons {
    fn id(&self) -> &str {
        "dp_burst_balloons"
    }
    fn name(&self) -> &str {
        "Burst Balloons"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "You have `n` balloons, each with a number painted on it. Bursting balloon `i` \
         gives you `nums[left] * nums[i] * nums[right]` coins, where left and right are \
         adjacent remaining balloons. Find the maximum coins you can collect by bursting \
         all the balloons wisely.\n\n\
         Treat boundaries as implicit balloons with value 1.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 300\n\
         - 0 <= nums[i] <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=8);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(1..=20)).collect();
                TestCase {
                    data: Box::new(BurstBalloonsTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BurstBalloonsTest>().unwrap();
        let expected = ref_burst_balloons(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::burst_balloons(&tracked_nums);
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

fn ref_burst_balloons(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut vals = vec![1i32];
    vals.extend_from_slice(nums);
    vals.push(1);
    let total = vals.len();
    let mut dp = vec![vec![0i32; total]; total];
    for length in 2..total {
        for left in 0..total - length {
            let right = left + length;
            for k in left + 1..right {
                dp[left][right] = dp[left][right]
                    .max(dp[left][k] + dp[k][right] + vals[left] * vals[k] * vals[right]);
            }
        }
    }
    dp[0][n + 1]
}

// ── Hard 3: Regular Expression Matching ──────────────────────────────

struct RegularExpression;
struct RegexTest {
    s: String,
    p: String,
}

impl Problem for RegularExpression {
    fn id(&self) -> &str {
        "dp_regular_expression"
    }
    fn name(&self) -> &str {
        "Regular Expression Matching"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Implement regular expression matching with support for '.' and '*'.\n\
         '.' matches any single character.\n\
         '*' matches zero or more of the preceding element.\n\n\
         The matching should cover the entire input string (not partial).\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 20\n\
         - 1 <= p.len() <= 20\n\
         - s contains only lowercase English letters.\n\
         - p contains only lowercase English letters, '.', and '*'."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let alphabet = b"abc";
        (0..10)
            .map(|_| {
                let slen = rng.random_range(1..=8);
                let s: String = (0..slen)
                    .map(|_| alphabet[rng.random_range(0..alphabet.len())] as char)
                    .collect();
                // Build a pattern that might or might not match
                let plen = rng.random_range(1..=6);
                let mut p = String::new();
                for _ in 0..plen {
                    let r = rng.random_range(0..=4);
                    match r {
                        0 => {
                            p.push(alphabet[rng.random_range(0..alphabet.len())] as char);
                            p.push('*');
                        }
                        1 => {
                            p.push('.');
                        }
                        2 => {
                            p.push('.');
                            p.push('*');
                        }
                        _ => {
                            p.push(alphabet[rng.random_range(0..alphabet.len())] as char);
                        }
                    }
                }
                TestCase {
                    data: Box::new(RegexTest { s, p }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RegexTest>().unwrap();
        let expected = ref_regex_match(&t.s, &t.p);
        let actual = solutions::regular_expression(&t.s, &t.p);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\", p=\"{}\"", t.s, t.p),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_regex_match(s: &str, p: &str) -> bool {
    let (sb, pb): (Vec<u8>, Vec<u8>) = (s.bytes().collect(), p.bytes().collect());
    let (m, n) = (sb.len(), pb.len());
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;
    for j in 1..=n {
        if pb[j - 1] == b'*' && j >= 2 {
            dp[0][j] = dp[0][j - 2];
        }
    }
    for i in 1..=m {
        for j in 1..=n {
            if pb[j - 1] == b'*' {
                // zero occurrences of the preceding element
                dp[i][j] = j >= 2 && dp[i][j - 2];
                // one or more occurrences
                if j >= 2 && (pb[j - 2] == b'.' || pb[j - 2] == sb[i - 1]) {
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
                }
            } else if pb[j - 1] == b'.' || pb[j - 1] == sb[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    dp[m][n]
}

// ── Hard 4: Longest Valid Parentheses ────────────────────────────────

struct LongestValidParentheses;
struct LVPTest {
    s: String,
}

impl Problem for LongestValidParentheses {
    fn id(&self) -> &str {
        "dp_longest_valid_parentheses"
    }
    fn name(&self) -> &str {
        "Longest Valid Parentheses"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a string containing only '(' and ')', return the length of the \
         longest valid (well-formed) parentheses substring.\n\n\
         Constraints:\n\
         - 0 <= s.len() <= 3 * 10^4\n\
         - s[i] is '(' or ')'."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let s: String = (0..n)
                    .map(|_| {
                        if rng.random_range(0..=1) == 0 {
                            '('
                        } else {
                            ')'
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(LVPTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LVPTest>().unwrap();
        let expected = ref_longest_valid_parens(&t.s);
        let actual = solutions::longest_valid_parentheses(&t.s);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_longest_valid_parens(s: &str) -> i32 {
    let chars: Vec<u8> = s.bytes().collect();
    let n = chars.len();
    if n == 0 {
        return 0;
    }
    let mut dp = vec![0i32; n];
    let mut max_len = 0;
    for i in 1..n {
        if chars[i] == b')' {
            if chars[i - 1] == b'(' {
                dp[i] = if i >= 2 { dp[i - 2] } else { 0 } + 2;
            } else if dp[i - 1] > 0 {
                let prev = i as i32 - dp[i - 1] - 1;
                if prev >= 0 && chars[prev as usize] == b'(' {
                    dp[i] = dp[i - 1] + 2;
                    if prev >= 1 {
                        dp[i] += dp[prev as usize - 1];
                    }
                }
            }
            max_len = max_len.max(dp[i]);
        }
    }
    max_len
}

// ── Hard 5: Max Profit with Cooldown ─────────────────────────────────

struct MaxProfitWithCooldown;
struct CooldownTest {
    prices: Vec<i32>,
}

impl Problem for MaxProfitWithCooldown {
    fn id(&self) -> &str {
        "dp_max_profit_with_cooldown"
    }
    fn name(&self) -> &str {
        "Max Profit with Cooldown"
    }
    fn topic(&self) -> &str {
        "dynamic_programming"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "You can complete as many buy/sell transactions as you like, but after \
         selling a stock, you must wait one day before buying again (cooldown). \
         Return the maximum profit.\n\n\
         Constraints:\n\
         - 1 <= prices.len() <= 5000\n\
         - 0 <= prices[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let prices: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                TestCase {
                    data: Box::new(CooldownTest { prices }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CooldownTest>().unwrap();
        let expected = ref_max_profit_cooldown(&t.prices);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_prices = track_slice(&t.prices, shared_log.clone());
        let actual = solutions::max_profit_with_cooldown(&tracked_prices);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("prices={:?}", t.prices),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_profit_cooldown(prices: &[i32]) -> i32 {
    let n = prices.len();
    if n <= 1 {
        return 0;
    }
    // held = max profit while holding stock
    // sold = max profit on day we just sold
    // rest = max profit while resting (cooldown or idle)
    let mut held = -prices[0];
    let mut sold = 0;
    let mut rest = 0;
    for &price in prices.iter().take(n).skip(1) {
        let prev_held = held;
        let prev_sold = sold;
        let prev_rest = rest;
        held = prev_held.max(prev_rest - price);
        sold = prev_held + price;
        rest = prev_rest.max(prev_sold);
    }
    sold.max(rest)
}
