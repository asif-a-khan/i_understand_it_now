use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part6_advanced::bit_manipulation as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy (5)
        Box::new(BitsSingleNumber),
        Box::new(BitsCountOnes),
        Box::new(BitsIsPowerOfTwo),
        Box::new(BitsReverseBits),
        Box::new(BitsMissingNumber),
        // Medium (5)
        Box::new(BitsCountingBits),
        Box::new(BitsSubsetsBitmask),
        Box::new(BitsSumWithoutArithmetic),
        Box::new(BitsTotalHammingDistance),
        Box::new(BitsMaximumXor),
        // Hard (5)
        Box::new(BitsSingleNumberII),
        Box::new(BitsSingleNumberIII),
        Box::new(BitsMaxAndPair),
        Box::new(BitsMinFlips),
        Box::new(BitsGrayCode),
    ]
}

// ── Reference implementations ────────────────────────────────────────

fn ref_single_number(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &x| acc ^ x)
}

fn ref_count_ones(n: u32) -> u32 {
    n.count_ones()
}

fn ref_is_power_of_two(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

fn ref_reverse_bits(n: u32) -> u32 {
    n.reverse_bits()
}

fn ref_missing_number(nums: &[i32]) -> i32 {
    let n = nums.len() as i32;
    let expected_sum = n * (n + 1) / 2;
    let actual_sum: i32 = nums.iter().sum();
    expected_sum - actual_sum
}

fn ref_counting_bits(n: i32) -> Vec<i32> {
    (0..=n).map(|i| (i as u32).count_ones() as i32).collect()
}

fn ref_subsets_bitmask(nums: &[i32]) -> Vec<Vec<i32>> {
    let n = nums.len();
    let total = 1 << n;
    let mut result = Vec::new();
    for mask in 0..total {
        let mut subset = Vec::new();
        for (i, &num) in nums.iter().enumerate().take(n) {
            if mask & (1 << i) != 0 {
                subset.push(num);
            }
        }
        result.push(subset);
    }
    result.sort();
    result
}

fn ref_sum_without_arithmetic(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let carry = a & b;
        a ^= b;
        b = carry << 1;
    }
    a
}

fn ref_total_hamming_distance(nums: &[i32]) -> i32 {
    let n = nums.len() as i32;
    let mut total = 0;
    for bit in 0..32 {
        let ones = nums.iter().filter(|&&x| (x >> bit) & 1 == 1).count() as i32;
        let zeros = n - ones;
        total += ones * zeros;
    }
    total
}

fn ref_maximum_xor(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut max_xor = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            max_xor = max_xor.max(nums[i] ^ nums[j]);
        }
    }
    max_xor
}

fn ref_single_number_ii(nums: &[i32]) -> i32 {
    let mut result = 0i32;
    for bit in 0..32 {
        let sum: i32 = nums.iter().map(|&x| (x >> bit) & 1).sum();
        if sum % 3 != 0 {
            result |= 1 << bit;
        }
    }
    result
}

fn ref_single_number_iii(nums: &[i32]) -> Vec<i32> {
    let xor_all = nums.iter().fold(0, |acc, &x| acc ^ x);
    let diff_bit = xor_all & (-xor_all); // lowest set bit
    let mut a = 0;
    let mut b = 0;
    for &x in nums {
        if x & diff_bit != 0 {
            a ^= x;
        } else {
            b ^= x;
        }
    }
    let mut result = vec![a, b];
    result.sort();
    result
}

fn ref_max_and_pair(nums: &[i32]) -> i32 {
    let mut result = 0;
    for bit in (0..30).rev() {
        let candidate = result | (1 << bit);
        let count = nums
            .iter()
            .filter(|&&x| (x & candidate) == candidate)
            .count();
        if count >= 2 {
            result = candidate;
        }
    }
    result
}

fn ref_min_flips(a: i32, b: i32) -> i32 {
    (a ^ b).count_ones() as i32
}

fn ref_gray_code(n: i32) -> Vec<i32> {
    let total = 1 << n;
    (0..total).map(|i| i ^ (i >> 1)).collect()
}

// ── Easy 1: Single Number ────────────────────────────────────────────

struct BitsSingleNumber;

struct SingleNumTest {
    nums: Vec<i32>,
}

impl Problem for BitsSingleNumber {
    fn id(&self) -> &str {
        "bits_single_number"
    }
    fn name(&self) -> &str {
        "Single Number"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Every element appears twice except one. Find the element that appears once.\n\n\
         Input: Vec<i32>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let pairs = rng.random_range(2..=10);
                let single = rng.random_range(-100..=100);
                let mut nums: Vec<i32> = Vec::new();
                for _ in 0..pairs {
                    let val = rng.random_range(-100..=100);
                    nums.push(val);
                    nums.push(val);
                }
                nums.push(single);
                // Shuffle
                for i in (1..nums.len()).rev() {
                    let j = rng.random_range(0..=i);
                    nums.swap(i, j);
                }
                TestCase {
                    data: Box::new(SingleNumTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SingleNumTest>().unwrap();
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

// ── Easy 2: Count Set Bits ───────────────────────────────────────────

struct BitsCountOnes;

struct CountOnesTest {
    n: u32,
}

impl Problem for BitsCountOnes {
    fn id(&self) -> &str {
        "bits_count_ones"
    }
    fn name(&self) -> &str {
        "Count Set Bits (Hamming Weight)"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Count the number of set bits (1s) in a 32-bit unsigned integer.\n\n\
         Input: u32\n\
         Output: u32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=u32::MAX);
                TestCase {
                    data: Box::new(CountOnesTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountOnesTest>().unwrap();
        let expected = ref_count_ones(t.n);
        let actual = solutions::count_ones(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={} (0b{:b})", t.n, t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 3: Is Power of Two ──────────────────────────────────────────

struct BitsIsPowerOfTwo;

struct PowerOfTwoTest {
    n: i32,
}

impl Problem for BitsIsPowerOfTwo {
    fn id(&self) -> &str {
        "bits_is_power_of_two"
    }
    fn name(&self) -> &str {
        "Is Power of Two"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Check if a given integer is a power of two.\n\n\
         Input: i32\n\
         Output: bool"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..7)
            .map(|_| {
                let n = rng.random_range(-10..=1000);
                TestCase {
                    data: Box::new(PowerOfTwoTest { n }),
                }
            })
            .collect();
        // Ensure some powers of two are included
        for &p in &[1, 2, 4, 8, 16, 64, 256, 1024] {
            if tests.len() < 10 {
                tests.push(TestCase {
                    data: Box::new(PowerOfTwoTest { n: p }),
                });
            }
        }
        tests.truncate(10);
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PowerOfTwoTest>().unwrap();
        let expected = ref_is_power_of_two(t.n);
        let actual = solutions::is_power_of_two(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 4: Reverse Bits ─────────────────────────────────────────────

struct BitsReverseBits;

struct ReverseBitsTest {
    n: u32,
}

impl Problem for BitsReverseBits {
    fn id(&self) -> &str {
        "bits_reverse_bits"
    }
    fn name(&self) -> &str {
        "Reverse Bits"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Reverse the bits of a 32-bit unsigned integer.\n\n\
         Input: u32\n\
         Output: u32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=u32::MAX);
                TestCase {
                    data: Box::new(ReverseBitsTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReverseBitsTest>().unwrap();
        let expected = ref_reverse_bits(t.n);
        let actual = solutions::reverse_bits(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={} (0b{:032b})", t.n, t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 5: Missing Number ───────────────────────────────────────────

struct BitsMissingNumber;

struct MissingNumTest {
    nums: Vec<i32>,
}

impl Problem for BitsMissingNumber {
    fn id(&self) -> &str {
        "bits_missing_number"
    }
    fn name(&self) -> &str {
        "Missing Number"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array containing n distinct numbers from [0, n], find the missing one.\n\
         Use XOR for O(1) space.\n\n\
         Input: Vec<i32>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=20);
                let missing = rng.random_range(0..=n as i32);
                let mut nums: Vec<i32> = (0..=n as i32).filter(|&x| x != missing).collect();
                // Shuffle
                for i in (1..nums.len()).rev() {
                    let j = rng.random_range(0..=i);
                    nums.swap(i, j);
                }
                TestCase {
                    data: Box::new(MissingNumTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MissingNumTest>().unwrap();
        let expected = ref_missing_number(&t.nums);
        let actual = solutions::missing_number(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 1: Counting Bits ──────────────────────────────────────────

struct BitsCountingBits;

struct CountingBitsTest {
    n: i32,
}

impl Problem for BitsCountingBits {
    fn id(&self) -> &str {
        "bits_counting_bits"
    }
    fn name(&self) -> &str {
        "Counting Bits"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "For each i in [0, n], return the number of 1-bits.\n\n\
         Input: i32 (n)\n\
         Output: Vec<i32>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=50);
                TestCase {
                    data: Box::new(CountingBitsTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountingBitsTest>().unwrap();
        let expected = ref_counting_bits(t.n);
        let actual = solutions::counting_bits(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 2: Subsets via Bitmask ────────────────────────────────────

struct BitsSubsetsBitmask;

struct SubsetsTest {
    nums: Vec<i32>,
}

impl Problem for BitsSubsetsBitmask {
    fn id(&self) -> &str {
        "bits_subsets_bitmask"
    }
    fn name(&self) -> &str {
        "Generate Subsets (Bitmask)"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Generate all subsets of the input array using bitmask enumeration.\n\
         Return subsets sorted lexicographically.\n\n\
         Input: Vec<i32>\n\
         Output: Vec<Vec<i32>> (sorted)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=6);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(1..=20)).collect();
                TestCase {
                    data: Box::new(SubsetsTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SubsetsTest>().unwrap();
        let expected = ref_subsets_bitmask(&t.nums);
        let mut actual = solutions::subsets_bitmask(&t.nums);
        actual.sort();
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 3: Sum Without Arithmetic ─────────────────────────────────

struct BitsSumWithoutArithmetic;

struct SumTest {
    a: i32,
    b: i32,
}

impl Problem for BitsSumWithoutArithmetic {
    fn id(&self) -> &str {
        "bits_sum_without_arithmetic"
    }
    fn name(&self) -> &str {
        "Add Without +/-"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Add two integers without using + or - operators. Use bit manipulation.\n\n\
         Input: (i32, i32)\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let a = rng.random_range(-1000..=1000);
                let b = rng.random_range(-1000..=1000);
                TestCase {
                    data: Box::new(SumTest { a, b }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SumTest>().unwrap();
        let expected = ref_sum_without_arithmetic(t.a, t.b);
        let actual = solutions::sum_without_arithmetic(t.a, t.b);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("a={}, b={}", t.a, t.b),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 4: Total Hamming Distance ─────────────────────────────────

struct BitsTotalHammingDistance;

struct HammingDistTest {
    nums: Vec<i32>,
}

impl Problem for BitsTotalHammingDistance {
    fn id(&self) -> &str {
        "bits_total_hamming_distance"
    }
    fn name(&self) -> &str {
        "Total Hamming Distance"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Return the total Hamming distance between all pairs of numbers.\n\n\
         Input: Vec<i32>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=1000)).collect();
                TestCase {
                    data: Box::new(HammingDistTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<HammingDistTest>().unwrap();
        let expected = ref_total_hamming_distance(&t.nums);
        let actual = solutions::total_hamming_distance(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 5: Maximum XOR of Two Numbers ─────────────────────────────

struct BitsMaximumXor;

struct MaxXorTest {
    nums: Vec<i32>,
}

impl Problem for BitsMaximumXor {
    fn id(&self) -> &str {
        "bits_maximum_xor"
    }
    fn name(&self) -> &str {
        "Maximum XOR of Two Numbers"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Find the maximum XOR of any two numbers in the array.\n\
         Use a trie for O(n * 32) solution.\n\n\
         Input: Vec<i32>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=10000)).collect();
                TestCase {
                    data: Box::new(MaxXorTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxXorTest>().unwrap();
        let expected = ref_maximum_xor(&t.nums);
        let actual = solutions::maximum_xor(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 1: Single Number II ─────────────────────────────────────────

struct BitsSingleNumberII;

struct SingleNumIITest {
    nums: Vec<i32>,
}

impl Problem for BitsSingleNumberII {
    fn id(&self) -> &str {
        "bits_single_number_ii"
    }
    fn name(&self) -> &str {
        "Single Number II (Others x3)"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Every element appears three times except one. Find the element that appears once.\n\n\
         Input: Vec<i32>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let triples = rng.random_range(2..=8);
                let single = rng.random_range(-100..=100);
                let mut nums: Vec<i32> = Vec::new();
                for _ in 0..triples {
                    let val = loop {
                        let v = rng.random_range(-100..=100);
                        if v != single {
                            break v;
                        }
                    };
                    nums.push(val);
                    nums.push(val);
                    nums.push(val);
                }
                nums.push(single);
                for i in (1..nums.len()).rev() {
                    let j = rng.random_range(0..=i);
                    nums.swap(i, j);
                }
                TestCase {
                    data: Box::new(SingleNumIITest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SingleNumIITest>().unwrap();
        let expected = ref_single_number_ii(&t.nums);
        let actual = solutions::single_number_ii(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 2: Single Number III ────────────────────────────────────────

struct BitsSingleNumberIII;

struct SingleNumIIITest {
    nums: Vec<i32>,
}

impl Problem for BitsSingleNumberIII {
    fn id(&self) -> &str {
        "bits_single_number_iii"
    }
    fn name(&self) -> &str {
        "Single Number III (Two Singles)"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Two elements appear once, all others appear twice.\n\
         Find the two unique elements and return them sorted.\n\n\
         Input: Vec<i32>\n\
         Output: Vec<i32> (sorted, length 2)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let pairs = rng.random_range(2..=8);
                let a = rng.random_range(-100..=100);
                let b = loop {
                    let v = rng.random_range(-100..=100);
                    if v != a {
                        break v;
                    }
                };
                let mut nums: Vec<i32> = Vec::new();
                for _ in 0..pairs {
                    let val = loop {
                        let v = rng.random_range(-100..=100);
                        if v != a && v != b {
                            break v;
                        }
                    };
                    nums.push(val);
                    nums.push(val);
                }
                nums.push(a);
                nums.push(b);
                for i in (1..nums.len()).rev() {
                    let j = rng.random_range(0..=i);
                    nums.swap(i, j);
                }
                TestCase {
                    data: Box::new(SingleNumIIITest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SingleNumIIITest>().unwrap();
        let expected = ref_single_number_iii(&t.nums);
        let mut actual = solutions::single_number_iii(&t.nums);
        actual.sort();
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 3: Maximum AND Pair ─────────────────────────────────────────

struct BitsMaxAndPair;

struct MaxAndTest {
    nums: Vec<i32>,
}

impl Problem for BitsMaxAndPair {
    fn id(&self) -> &str {
        "bits_max_and_pair"
    }
    fn name(&self) -> &str {
        "Maximum AND of Any Pair"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find the maximum bitwise AND of any pair of numbers in the array.\n\n\
         Input: Vec<i32>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=15);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100000)).collect();
                TestCase {
                    data: Box::new(MaxAndTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxAndTest>().unwrap();
        let expected = ref_max_and_pair(&t.nums);
        let actual = solutions::max_and_pair(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 4: Minimum Bit Flips ────────────────────────────────────────

struct BitsMinFlips;

struct MinFlipsTest {
    a: i32,
    b: i32,
}

impl Problem for BitsMinFlips {
    fn id(&self) -> &str {
        "bits_min_flips"
    }
    fn name(&self) -> &str {
        "Minimum Bit Flips to Convert"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find the minimum number of bit flips to convert integer a to integer b.\n\n\
         Input: (i32, i32)\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let a = rng.random_range(0..=1_000_000);
                let b = rng.random_range(0..=1_000_000);
                TestCase {
                    data: Box::new(MinFlipsTest { a, b }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinFlipsTest>().unwrap();
        let expected = ref_min_flips(t.a, t.b);
        let actual = solutions::min_flips(t.a, t.b);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("a={}, b={}", t.a, t.b),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 5: Gray Code ────────────────────────────────────────────────

struct BitsGrayCode;

struct GrayCodeTest {
    n: i32,
}

impl Problem for BitsGrayCode {
    fn id(&self) -> &str {
        "bits_gray_code"
    }
    fn name(&self) -> &str {
        "Gray Code Sequence"
    }
    fn topic(&self) -> &str {
        "bit_manipulation"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Generate the n-bit Gray code sequence. Each consecutive pair differs by one bit.\n\
         Start with 0.\n\n\
         Input: i32 (n)\n\
         Output: Vec<i32>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=8);
                TestCase {
                    data: Box::new(GrayCodeTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<GrayCodeTest>().unwrap();
        let expected = ref_gray_code(t.n);
        let actual = solutions::gray_code(t.n);
        // Validate: correct length, starts at 0, all present, adjacent differ by 1 bit
        let total = 1 << t.n;
        let valid_len = actual.len() == total as usize;
        let valid_start = actual.first() == Some(&0);
        let valid_set: bool = {
            let mut s = std::collections::HashSet::new();
            actual.iter().all(|x| s.insert(*x)) && s.len() == total as usize
        };
        let valid_gray = actual.windows(2).all(|w| (w[0] ^ w[1]).count_ones() == 1);
        let is_correct = valid_len && valid_start && valid_set && valid_gray;
        SolutionResult {
            is_correct,
            input_description: format!("n={}", t.n),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}
