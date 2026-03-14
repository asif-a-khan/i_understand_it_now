use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;
use std::collections::HashMap;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part6_advanced::math_geometry as solutions;
use crate::tracker::{track_slice, OperationLog, Tracked};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy (5)
        Box::new(MathGcd),
        Box::new(MathIsPrime),
        Box::new(MathCountPrimes),
        Box::new(MathPowerMod),
        Box::new(MathReverseInteger),
        // Medium (5)
        Box::new(MathSieveOfEratosthenes),
        Box::new(MathMultiplyStrings),
        Box::new(MathMaxPointsOnLine),
        Box::new(MathUglyNumber),
        Box::new(MathNextPermutation),
        // Hard (5)
        Box::new(MathConvexHull),
        Box::new(MathModularInverse),
        Box::new(MathMatrixExponentiation),
        Box::new(MathChineseRemainder),
        Box::new(MathTrapezoidalIntegral),
    ]
}

// ── Reference implementations ────────────────────────────────────────

fn ref_gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn ref_is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i: i64 = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn ref_count_primes(n: i32) -> i32 {
    if n < 2 {
        return 0;
    }
    let n = n as usize;
    let mut sieve = vec![true; n];
    sieve[0] = false;
    if n > 1 {
        sieve[1] = false;
    }
    let mut i = 2;
    while i * i < n {
        if sieve[i] {
            let mut j = i * i;
            while j < n {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    sieve.iter().filter(|&&x| x).count() as i32
}

fn ref_power_mod(base: i64, exp: i64, m: i64) -> i64 {
    if m == 1 {
        return 0;
    }
    let mut result: i64 = 1;
    let mut base = base.rem_euclid(m);
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % m;
        }
        exp /= 2;
        base = base * base % m;
    }
    result
}

fn ref_reverse_integer(x: i32) -> i32 {
    let negative = x < 0;
    let mut n = (x as i64).abs();
    let mut reversed: i64 = 0;
    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    if negative {
        reversed = -reversed;
    }
    if reversed < i32::MIN as i64 || reversed > i32::MAX as i64 {
        0
    } else {
        reversed as i32
    }
}

fn ref_sieve(n: i32) -> Vec<i32> {
    if n < 2 {
        return vec![];
    }
    let n = n as usize;
    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;
    let mut i = 2;
    while i * i <= n {
        if sieve[i] {
            let mut j = i * i;
            while j <= n {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    sieve
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(i, _)| i as i32)
        .collect()
}

fn ref_multiply_strings(num1: &str, num2: &str) -> String {
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }
    let d1: Vec<u32> = num1.bytes().rev().map(|b| (b - b'0') as u32).collect();
    let d2: Vec<u32> = num2.bytes().rev().map(|b| (b - b'0') as u32).collect();
    let mut result = vec![0u32; d1.len() + d2.len()];
    for i in 0..d1.len() {
        for j in 0..d2.len() {
            result[i + j] += d1[i] * d2[j];
            result[i + j + 1] += result[i + j] / 10;
            result[i + j] %= 10;
        }
    }
    while result.len() > 1 && *result.last().unwrap() == 0 {
        result.pop();
    }
    result
        .iter()
        .rev()
        .map(|&d| (d as u8 + b'0') as char)
        .collect()
}

fn ref_max_points_on_line(points: &[(i32, i32)]) -> i32 {
    let n = points.len();
    if n <= 2 {
        return n as i32;
    }
    let mut max_count = 2;
    for i in 0..n {
        let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();
        for j in (i + 1)..n {
            let dx = points[j].0 - points[i].0;
            let dy = points[j].1 - points[i].1;
            let g = gcd_i32(dx.abs(), dy.abs());
            let (dx, dy) = if g == 0 {
                (0, 0)
            } else {
                let (dx, dy) = (dx / g, dy / g);
                if dx < 0 || (dx == 0 && dy < 0) {
                    (-dx, -dy)
                } else {
                    (dx, dy)
                }
            };
            *slopes.entry((dx, dy)).or_insert(0) += 1;
        }
        if let Some(&max_s) = slopes.values().max() {
            max_count = max_count.max(max_s + 1);
        }
    }
    max_count
}

fn gcd_i32(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd_i32(b, a % b)
    }
}

fn ref_ugly_number(n: i32) -> i32 {
    let mut uglies = vec![1i64];
    let (mut i2, mut i3, mut i5) = (0usize, 0usize, 0usize);
    for _ in 1..n {
        let next = (uglies[i2] * 2).min(uglies[i3] * 3).min(uglies[i5] * 5);
        if next == uglies[i2] * 2 {
            i2 += 1;
        }
        if next == uglies[i3] * 3 {
            i3 += 1;
        }
        if next == uglies[i5] * 5 {
            i5 += 1;
        }
        uglies.push(next);
    }
    *uglies.last().unwrap() as i32
}

fn ref_next_permutation(nums: &[i32]) -> Vec<i32> {
    let mut result = nums.to_vec();
    let n = result.len();
    if n <= 1 {
        return result;
    }
    let mut i = n - 2;
    while i < n && result[i] >= result[i + 1] {
        if i == 0 {
            result.sort();
            return result;
        }
        i -= 1;
    }
    let mut j = n - 1;
    while result[j] <= result[i] {
        j -= 1;
    }
    result.swap(i, j);
    result[i + 1..].reverse();
    result
}

fn ref_convex_hull(points: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut pts: Vec<(i64, i64)> = points.to_vec();
    pts.sort();
    pts.dedup();
    let n = pts.len();
    if n <= 1 {
        return pts;
    }

    fn cross(o: (i64, i64), a: (i64, i64), b: (i64, i64)) -> i64 {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    }

    let mut hull = Vec::new();
    // Lower hull
    for &p in &pts {
        while hull.len() >= 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0 {
            hull.pop();
        }
        hull.push(p);
    }
    // Upper hull
    let lower_len = hull.len() + 1;
    for &p in pts.iter().rev().skip(1) {
        while hull.len() >= lower_len && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0 {
            hull.pop();
        }
        hull.push(p);
    }
    hull.pop(); // Remove last point (duplicate of first)
    hull.sort();
    hull
}

fn ref_modular_inverse(a: i64, m: i64) -> i64 {
    // Extended Euclidean
    fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            return (b, 0, 1);
        }
        let (g, x1, y1) = ext_gcd(b % a, a);
        (g, y1 - (b / a) * x1, x1)
    }
    let (g, x, _) = ext_gcd(a.rem_euclid(m), m);
    if g != 1 {
        -1 // No inverse exists
    } else {
        x.rem_euclid(m)
    }
}

fn ref_matrix_exp(mat: &[Vec<i64>], exp: i64, m: i64) -> Vec<Vec<i64>> {
    let n = mat.len();
    let mat_mul = |a: &[Vec<i64>], b: &[Vec<i64>]| -> Vec<Vec<i64>> {
        let mut c = vec![vec![0i64; n]; n];
        for i in 0..n {
            for j in 0..n {
                for (k, b_row) in b.iter().enumerate().take(n) {
                    c[i][j] = (c[i][j] + a[i][k] * b_row[j]) % m;
                }
            }
        }
        c
    };
    let mut result = vec![vec![0i64; n]; n];
    for (i, row) in result.iter_mut().enumerate().take(n) {
        row[i] = 1;
    }
    let mut base = mat.to_vec();
    let mut exp = exp;
    // Ensure base values are positive mod m
    for row in &mut base {
        for val in row.iter_mut() {
            *val = val.rem_euclid(m);
        }
    }
    while exp > 0 {
        if exp % 2 == 1 {
            result = mat_mul(&result, &base);
        }
        base = mat_mul(&base, &base);
        exp /= 2;
    }
    result
}

fn ref_chinese_remainder(remainders: &[i64], moduli: &[i64]) -> i64 {
    // Simple brute force for small inputs
    let product: i64 = moduli.iter().product();
    for x in 0..product {
        let mut valid = true;
        for i in 0..moduli.len() {
            if x % moduli[i] != remainders[i] % moduli[i] {
                valid = false;
                break;
            }
        }
        if valid {
            return x;
        }
    }
    -1
}

fn ref_trapezoidal(ys: &[f64], dx: f64) -> f64 {
    if ys.len() < 2 {
        return 0.0;
    }
    let mut sum = 0.0;
    for i in 0..ys.len() - 1 {
        sum += (ys[i] + ys[i + 1]) * dx / 2.0;
    }
    sum
}

// ── Easy 1: GCD ──────────────────────────────────────────────────────

struct MathGcd;

struct GcdTest {
    a: i64,
    b: i64,
}

impl Problem for MathGcd {
    fn id(&self) -> &str {
        "math_gcd"
    }
    fn name(&self) -> &str {
        "GCD of Two Numbers"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Compute the greatest common divisor of two numbers using Euclidean algorithm.\n\n\
         Input: (i64, i64)\n\
         Output: i64"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let a = rng.random_range(1..=10000i64);
                let b = rng.random_range(1..=10000i64);
                TestCase {
                    data: Box::new(GcdTest { a, b }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<GcdTest>().unwrap();
        let expected = ref_gcd(t.a, t.b);
        let actual = solutions::gcd(t.a, t.b);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("a={}, b={}", t.a, t.b),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 2: Is Prime ─────────────────────────────────────────────────

struct MathIsPrime;

struct IsPrimeTest {
    n: i64,
}

impl Problem for MathIsPrime {
    fn id(&self) -> &str {
        "math_is_prime"
    }
    fn name(&self) -> &str {
        "Primality Test"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Determine if a number is prime.\n\n\
         Input: i64\n\
         Output: bool"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=1000i64);
                TestCase {
                    data: Box::new(IsPrimeTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IsPrimeTest>().unwrap();
        let expected = ref_is_prime(t.n);
        let actual = solutions::is_prime(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 3: Count Primes ─────────────────────────────────────────────

struct MathCountPrimes;

struct CountPrimesTest {
    n: i32,
}

impl Problem for MathCountPrimes {
    fn id(&self) -> &str {
        "math_count_primes"
    }
    fn name(&self) -> &str {
        "Count Primes Less Than N"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Count the number of primes strictly less than n.\n\n\
         Input: i32\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=1000);
                TestCase {
                    data: Box::new(CountPrimesTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountPrimesTest>().unwrap();
        let expected = ref_count_primes(t.n);
        let actual = solutions::count_primes(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 4: Power Mod ────────────────────────────────────────────────

struct MathPowerMod;

struct PowerModTest {
    base: i64,
    exp: i64,
    m: i64,
}

impl Problem for MathPowerMod {
    fn id(&self) -> &str {
        "math_power_mod"
    }
    fn name(&self) -> &str {
        "Modular Exponentiation"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Compute (base^exp) mod m using fast exponentiation.\n\n\
         Input: (base: i64, exp: i64, m: i64)\n\
         Output: i64"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let base = rng.random_range(1..=100i64);
                let exp = rng.random_range(0..=50i64);
                let m = rng.random_range(2..=1_000_000_007i64);
                TestCase {
                    data: Box::new(PowerModTest { base, exp, m }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PowerModTest>().unwrap();
        let expected = ref_power_mod(t.base, t.exp, t.m);
        let actual = solutions::power_mod(t.base, t.exp, t.m);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("base={}, exp={}, m={}", t.base, t.exp, t.m),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 5: Reverse Integer ──────────────────────────────────────────

struct MathReverseInteger;

struct ReverseIntTest {
    x: i32,
}

impl Problem for MathReverseInteger {
    fn id(&self) -> &str {
        "math_reverse_integer"
    }
    fn name(&self) -> &str {
        "Reverse Integer"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Reverse the digits of an integer. Return 0 if the result overflows i32.\n\n\
         Input: i32\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let x = rng.random_range(i32::MIN / 10..=i32::MAX / 10);
                TestCase {
                    data: Box::new(ReverseIntTest { x }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReverseIntTest>().unwrap();
        let expected = ref_reverse_integer(t.x);
        let actual = solutions::reverse_integer(t.x);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("x={}", t.x),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 1: Sieve of Eratosthenes ──────────────────────────────────

struct MathSieveOfEratosthenes;

struct SieveTest {
    n: i32,
}

impl Problem for MathSieveOfEratosthenes {
    fn id(&self) -> &str {
        "math_sieve_of_eratosthenes"
    }
    fn name(&self) -> &str {
        "Sieve of Eratosthenes"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "List all primes up to and including n.\n\n\
         Input: i32\n\
         Output: Vec<i32>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=200);
                TestCase {
                    data: Box::new(SieveTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SieveTest>().unwrap();
        let expected = ref_sieve(t.n);
        let actual = solutions::sieve_of_eratosthenes(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 2: Multiply Strings ───────────────────────────────────────

struct MathMultiplyStrings;

struct MulStrTest {
    num1: String,
    num2: String,
}

impl Problem for MathMultiplyStrings {
    fn id(&self) -> &str {
        "math_multiply_strings"
    }
    fn name(&self) -> &str {
        "Multiply Two Large Numbers"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Multiply two numbers represented as strings. Do not use BigInt.\n\n\
         Input: (num1: String, num2: String)\n\
         Output: String"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len1 = rng.random_range(1..=10);
                let len2 = rng.random_range(1..=10);
                let num1: String = (0..len1)
                    .map(|i| {
                        if i == 0 && len1 > 1 {
                            (b'1' + rng.random_range(0..9u8)) as char
                        } else {
                            (b'0' + rng.random_range(0..10u8)) as char
                        }
                    })
                    .collect();
                let num2: String = (0..len2)
                    .map(|i| {
                        if i == 0 && len2 > 1 {
                            (b'1' + rng.random_range(0..9u8)) as char
                        } else {
                            (b'0' + rng.random_range(0..10u8)) as char
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(MulStrTest { num1, num2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MulStrTest>().unwrap();
        let expected = ref_multiply_strings(&t.num1, &t.num2);
        let actual = solutions::multiply_strings(&t.num1, &t.num2);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("num1=\"{}\", num2=\"{}\"", t.num1, t.num2),
            expected: format!("\"{expected}\""),
            actual: format!("\"{actual}\""),
        }
    }
}

// ── Medium 3: Max Points on a Line ───────────────────────────────────

struct MathMaxPointsOnLine;

struct MaxPointsTest {
    points: Vec<(i32, i32)>,
}

impl Problem for MathMaxPointsOnLine {
    fn id(&self) -> &str {
        "math_max_points_on_line"
    }
    fn name(&self) -> &str {
        "Max Collinear Points"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Find the maximum number of points that lie on the same straight line.\n\n\
         Input: Vec<(i32, i32)>\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(2..=10);
                let points: Vec<(i32, i32)> = (0..n)
                    .map(|_| (rng.random_range(-20..=20), rng.random_range(-20..=20)))
                    .collect();
                TestCase {
                    data: Box::new(MaxPointsTest { points }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxPointsTest>().unwrap();
        let expected = ref_max_points_on_line(&t.points);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked: Vec<(Tracked<i32>, Tracked<i32>)> = t
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
        let actual = solutions::max_points_on_line(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("points={:?}", t.points),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 4: Ugly Number ────────────────────────────────────────────

struct MathUglyNumber;

struct UglyNumTest {
    n: i32,
}

impl Problem for MathUglyNumber {
    fn id(&self) -> &str {
        "math_ugly_number"
    }
    fn name(&self) -> &str {
        "Nth Ugly Number"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Find the nth ugly number. Ugly numbers have only prime factors 2, 3, and 5.\n\
         The sequence starts: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12, ...\n\n\
         Input: i32 (n, 1-indexed)\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=200);
                TestCase {
                    data: Box::new(UglyNumTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UglyNumTest>().unwrap();
        let expected = ref_ugly_number(t.n);
        let actual = solutions::ugly_number(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 5: Next Permutation ───────────────────────────────────────

struct MathNextPermutation;

struct NextPermTest {
    nums: Vec<i32>,
}

impl Problem for MathNextPermutation {
    fn id(&self) -> &str {
        "math_next_permutation"
    }
    fn name(&self) -> &str {
        "Next Lexicographic Permutation"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Rearrange numbers into the lexicographically next greater permutation.\n\
         If at the largest permutation, return the sorted (smallest) order.\n\n\
         Input: Vec<i32>\n\
         Output: Vec<i32>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=8);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=5)).collect();
                TestCase {
                    data: Box::new(NextPermTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NextPermTest>().unwrap();
        let expected = ref_next_permutation(&t.nums);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::next_permutation(&tracked_nums);
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

// ── Hard 1: Convex Hull ──────────────────────────────────────────────

struct MathConvexHull;

struct ConvexHullTest {
    points: Vec<(i64, i64)>,
}

impl Problem for MathConvexHull {
    fn id(&self) -> &str {
        "math_convex_hull"
    }
    fn name(&self) -> &str {
        "Convex Hull"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Compute the convex hull of a set of 2D points.\n\
         Return the hull vertices sorted by coordinates.\n\n\
         Input: Vec<(i64, i64)>\n\
         Output: Vec<(i64, i64)> (sorted)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=15);
                let points: Vec<(i64, i64)> = (0..n)
                    .map(|_| {
                        (
                            rng.random_range(-100..=100i64),
                            rng.random_range(-100..=100i64),
                        )
                    })
                    .collect();
                TestCase {
                    data: Box::new(ConvexHullTest { points }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ConvexHullTest>().unwrap();
        let expected = ref_convex_hull(&t.points);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked: Vec<(Tracked<i64>, Tracked<i64>)> = t
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
        let mut actual = solutions::convex_hull(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        actual.sort();
        actual.dedup();
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("points={:?}", t.points),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 2: Modular Inverse ──────────────────────────────────────────

struct MathModularInverse;

struct ModInvTest {
    a: i64,
    m: i64,
}

impl Problem for MathModularInverse {
    fn id(&self) -> &str {
        "math_modular_inverse"
    }
    fn name(&self) -> &str {
        "Modular Multiplicative Inverse"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find x such that (a * x) mod m = 1. Return -1 if no inverse exists.\n\n\
         Input: (a: i64, m: i64)\n\
         Output: i64"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let m = rng.random_range(2..=1000i64);
                let a = rng.random_range(1..=m - 1);
                TestCase {
                    data: Box::new(ModInvTest { a, m }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ModInvTest>().unwrap();
        let expected = ref_modular_inverse(t.a, t.m);
        let actual = solutions::modular_inverse(t.a, t.m);
        // Both -1 or both satisfy (a * x) % m == 1
        let is_correct = if expected == -1 {
            actual == -1
        } else {
            actual >= 0 && (t.a * actual) % t.m == 1
        };
        SolutionResult {
            is_correct,
            input_description: format!("a={}, m={}", t.a, t.m),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 3: Matrix Exponentiation ────────────────────────────────────

struct MathMatrixExponentiation;

struct MatExpTest {
    matrix: Vec<Vec<i64>>,
    exp: i64,
    m: i64,
}

impl Problem for MathMatrixExponentiation {
    fn id(&self) -> &str {
        "math_matrix_exponentiation"
    }
    fn name(&self) -> &str {
        "Matrix Exponentiation"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Compute matrix^exp mod m using fast exponentiation.\n\
         Matrix is 2x2.\n\n\
         Input: (matrix: Vec<Vec<i64>>, exp: i64, m: i64)\n\
         Output: Vec<Vec<i64>>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let matrix = vec![
                    vec![rng.random_range(0..=10i64), rng.random_range(0..=10i64)],
                    vec![rng.random_range(0..=10i64), rng.random_range(0..=10i64)],
                ];
                let exp = rng.random_range(0..=30i64);
                let m = rng.random_range(2..=1_000_000_007i64);
                TestCase {
                    data: Box::new(MatExpTest { matrix, exp, m }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MatExpTest>().unwrap();
        let expected = ref_matrix_exp(&t.matrix, t.exp, t.m);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked: Vec<Vec<Tracked<i64>>> = t
            .matrix
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, &v)| Tracked::new(v, r * row.len().max(1) + c, shared_log.clone()))
                    .collect()
            })
            .collect();
        let actual = solutions::matrix_exponentiation(&tracked, t.exp, t.m);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("matrix={:?}, exp={}, m={}", t.matrix, t.exp, t.m),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 4: Chinese Remainder Theorem ────────────────────────────────

struct MathChineseRemainder;

struct CrtTest {
    remainders: Vec<i64>,
    moduli: Vec<i64>,
}

impl Problem for MathChineseRemainder {
    fn id(&self) -> &str {
        "math_chinese_remainder"
    }
    fn name(&self) -> &str {
        "Chinese Remainder Theorem"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Solve the system of congruences:\n\
         x = remainders[i] (mod moduli[i]) for all i.\n\
         Moduli are pairwise coprime.\n\
         Return the smallest non-negative x, or -1 if no solution.\n\n\
         Input: (remainders: Vec<i64>, moduli: Vec<i64>)\n\
         Output: i64"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let coprime_sets: Vec<Vec<i64>> = vec![
            vec![3, 5, 7],
            vec![2, 3, 5],
            vec![3, 7, 11],
            vec![2, 5, 7],
            vec![3, 5, 11],
            vec![2, 3, 7],
            vec![5, 7, 11],
            vec![2, 3, 11],
            vec![3, 7, 13],
            vec![2, 5, 11],
        ];
        coprime_sets
            .into_iter()
            .map(|moduli| {
                let remainders: Vec<i64> = moduli.iter().map(|&m| rng.random_range(0..m)).collect();
                TestCase {
                    data: Box::new(CrtTest { remainders, moduli }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CrtTest>().unwrap();
        let expected = ref_chinese_remainder(&t.remainders, &t.moduli);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_remainders = track_slice(&t.remainders, shared_log.clone());
        let tracked_moduli = track_slice(&t.moduli, shared_log.clone());
        let actual = solutions::chinese_remainder(&tracked_remainders, &tracked_moduli);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        // Validate: actual satisfies all congruences
        let valid = if expected == -1 {
            actual == -1
        } else {
            actual >= 0
                && t.moduli
                    .iter()
                    .zip(t.remainders.iter())
                    .all(|(&m, &r)| actual % m == r % m)
        };
        SolutionResult {
            is_correct: valid,
            input_description: format!("remainders={:?}, moduli={:?}", t.remainders, t.moduli),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 5: Trapezoidal Integration ──────────────────────────────────

struct MathTrapezoidalIntegral;

struct TrapIntTest {
    ys: Vec<f64>,
    dx: f64,
}

impl Problem for MathTrapezoidalIntegral {
    fn id(&self) -> &str {
        "math_largest_rectangle_under_curve"
    }
    fn name(&self) -> &str {
        "Trapezoidal Integration"
    }
    fn topic(&self) -> &str {
        "math_geometry"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Approximate the integral using the trapezoidal rule.\n\
         Given y-values at equally spaced x-points with spacing dx.\n\n\
         Input: (ys: Vec<f64>, dx: f64)\n\
         Output: f64 (abs error < 1e-6)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(3..=20);
                let ys: Vec<f64> = (0..n)
                    .map(|_| rng.random_range(0..=100) as f64 / 10.0)
                    .collect();
                let dx = rng.random_range(1..=10) as f64 / 10.0;
                TestCase {
                    data: Box::new(TrapIntTest { ys, dx }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TrapIntTest>().unwrap();
        let expected = ref_trapezoidal(&t.ys, t.dx);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_ys = track_slice(&t.ys, shared_log.clone());
        let actual = solutions::trapezoidal_integral(&tracked_ys, t.dx);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: (expected - actual).abs() < 1e-6,
            input_description: format!("ys={:?}, dx={}", t.ys, t.dx),
            expected: format!("{expected:.6}"),
            actual: format!("{actual:.6}"),
        }
    }
}
