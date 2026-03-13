# Lesson 38: Math & Geometry

## Why This Lesson Exists

You can get remarkably far in coding interviews without remembering much math. But
every few months, an interviewer drops a problem that is secretly about GCD, modular
arithmetic, or rotating a matrix in-place. If you have never seen the trick, you will
spend 30 minutes reinventing what Euclid figured out in 300 BC.

This lesson arms you with the math and geometry primitives that appear in interviews.
None of it requires a math degree -- it requires knowing which tool to reach for and
how to implement it without overflow bugs in Rust.

We will cover:

1. Number theory: GCD, LCM, primes, modular arithmetic, fast exponentiation
2. Combinatorics: factorials, permutations, combinations, Pascal's Triangle
3. Geometry: points, distances, containment tests, matrix rotation, spiral traversal
4. Rust-specific concerns: integer overflow, checked arithmetic, type boundaries
5. A recap of Floyd's cycle detection applied to math problems

---

## 1. Number Theory

### 1.1 GCD (Greatest Common Divisor) -- The Euclidean Algorithm

**Real-world analogy.** You have two ropes, 18 meters and 12 meters. You want to cut
them into equal-length pieces with zero waste. What is the longest piece you can cut?
You need the GCD of 18 and 12, which is 6. Cut the first rope into 3 pieces, the
second into 2 pieces, each 6 meters long.

The Euclidean algorithm is ancient and elegant: repeatedly replace the larger number
with the remainder of dividing the larger by the smaller. When the remainder hits zero,
the other number is the GCD.

```
gcd(18, 12)
  18 % 12 = 6   -> gcd(12, 6)
  12 % 6  = 0   -> gcd(6, 0)
  answer: 6
```

```rust
/// Euclidean algorithm -- iterative version (no stack overflow risk).
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// LCM derived from GCD.  Divide first to avoid overflow.
/// lcm(a,b) = a / gcd(a,b) * b
fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b // divide before multiply to reduce overflow risk
}

fn main() {
    assert_eq!(gcd(18, 12), 6);
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(7, 0), 7);
    assert_eq!(lcm(4, 6), 12);
    assert_eq!(lcm(0, 10), 0);
    println!("GCD and LCM: all assertions passed");
}
```

**Complexity.** O(log(min(a, b))) time, O(1) space.

Why logarithmic? Each step, the remainder is at most half of the dividend. So the pair
of values shrinks by at least half every two steps.

---

### 1.2 Sieve of Eratosthenes (Prime Generation)

**Real-world analogy.** You have a class roster numbered 2 through 100. Start at 2: it
is prime, so circle it, then cross out every multiple of 2 (4, 6, 8, ...). Move to 3:
still uncrossed, so circle it and cross out 6, 9, 12, .... Move to 4: already crossed
out, skip. Move to 5: circle it, cross out 10, 15, 20, .... By the time you finish,
every circled number is prime.

```
Initial:  2  3  4  5  6  7  8  9  10 11 12 ...
After 2:  2  3  .  5  .  7  .  9  .  11  . ...
After 3:  2  3  .  5  .  7  .  .  .  11  . ...
After 5:  (nothing new to cross below 25)
Done for primes up to 12.
```

```rust
/// Returns a boolean sieve where sieve[i] == true means i is prime.
fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    if limit >= 1 {
        is_prime[0] = false;
    }
    if limit >= 1 {
        is_prime[1] = false;
    }

    let mut p = 2;
    while p * p <= limit {
        if is_prime[p] {
            // Cross out multiples starting from p*p (smaller multiples
            // were already handled by smaller primes).
            let mut multiple = p * p;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }
    is_prime
}

/// Collect the actual prime numbers from the sieve.
fn primes_up_to(limit: usize) -> Vec<usize> {
    let sieve = sieve_of_eratosthenes(limit);
    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &is_p)| if is_p { Some(i) } else { None })
        .collect()
}

fn main() {
    let primes = primes_up_to(30);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    println!("Primes up to 30: {:?}", primes);
}
```

**Complexity.** O(n log log n) time, O(n) space. This is nearly linear and the standard
approach for generating all primes up to n. If you only need to test a single number,
trial division up to sqrt(n) suffices (see next section).

---

### 1.3 Prime Factorization

**Real-world analogy.** Breaking a number into its prime factors is like disassembling
a LEGO structure into its individual bricks. 60 = 2 x 2 x 3 x 5. You cannot break
those bricks any further.

```rust
/// Returns the prime factorization as a list of (prime, exponent) pairs.
/// Example: 60 -> [(2, 2), (3, 1), (5, 1)]
fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut d = 2u64;

    while d * d <= n {
        if n % d == 0 {
            let mut count = 0u32;
            while n % d == 0 {
                n /= d;
                count += 1;
            }
            factors.push((d, count));
        }
        d += 1;
    }
    if n > 1 {
        factors.push((n, 1)); // n itself is prime
    }
    factors
}

fn main() {
    assert_eq!(prime_factors(60), vec![(2, 2), (3, 1), (5, 1)]);
    assert_eq!(prime_factors(13), vec![(13, 1)]);
    assert_eq!(prime_factors(1), vec![]);
    println!("Prime factorization: all assertions passed");
}
```

**Complexity.** O(sqrt(n)) time. For interview problems this is almost always fast
enough.

---

### 1.4 Modular Arithmetic

**Real-world analogy.** A clock. After 12 comes 1, not 13. Hours wrap around modulo
12. Modular arithmetic is just "clock math" for any modulus, not just 12.

**Key properties** you must internalize for interviews:

```
(a + b) mod m = ((a mod m) + (b mod m)) mod m
(a - b) mod m = ((a mod m) - (b mod m) + m) mod m    // +m to keep positive
(a * b) mod m = ((a mod m) * (b mod m)) mod m
```

Division does NOT distribute this cleanly. For `(a / b) mod m`, you need the modular
inverse of b, which exists only when gcd(b, m) = 1. The modular inverse of b is
`b^(m-2) mod m` when m is prime (by Fermat's little theorem).

Many interview problems ask you to "return the answer modulo 10^9 + 7." This modulus
(1_000_000_007) is prime, which guarantees modular inverses exist for any nonzero b.

```rust
const MOD: u64 = 1_000_000_007;

/// Modular addition -- safe from overflow for values < MOD.
fn mod_add(a: u64, b: u64) -> u64 {
    (a % MOD + b % MOD) % MOD
}

/// Modular subtraction -- avoids underflow.
fn mod_sub(a: u64, b: u64) -> u64 {
    (a % MOD + MOD - b % MOD) % MOD
}

/// Modular multiplication -- cast to u128 to avoid overflow.
fn mod_mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % MOD as u128) as u64
}

fn main() {
    assert_eq!(mod_add(500_000_004, 500_000_004), 1);
    assert_eq!(mod_sub(3, 5), MOD - 2); // 3 - 5 mod MOD = MOD - 2
    assert_eq!(mod_mul(MOD - 1, 2), MOD - 2);
    println!("Modular arithmetic: all assertions passed");
}
```

---

### 1.5 Fast Exponentiation (Binary Exponentiation)

**Real-world analogy.** Suppose you need to fold a piece of paper 30 times (each fold
doubles the thickness). You do not need 2^30 separate multiplications. You square the
thickness at each step: 2, 4, 16, 256, .... Ten squarings (and selective multiplications)
get you to 2^30. That is binary exponentiation: decompose the exponent into powers of 2
and multiply only the relevant squares together.

```
Computing 3^13:
  13 in binary = 1101
  3^13 = 3^8 * 3^4 * 3^1

Step-by-step (tracking base and result):
  exp=13 (odd)  -> result *= base    base: 3      result: 3
                   base *= base      base: 9
  exp=6  (even) -> skip multiply     base: 9      result: 3
                   base *= base      base: 81
  exp=3  (odd)  -> result *= base    base: 81     result: 243
                   base *= base      base: 6561
  exp=1  (odd)  -> result *= base    base: 6561   result: 1594323
  Done.  3^13 = 1594323
```

```rust
/// Modular exponentiation: computes (base^exp) % modulus.
/// Uses binary exponentiation: O(log exp) multiplications.
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp /= 2;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    result
}

/// Modular inverse using Fermat's little theorem (modulus must be prime).
fn mod_inverse(a: u64, modulus: u64) -> u64 {
    mod_pow(a, modulus - 2, modulus)
}

fn main() {
    const MOD: u64 = 1_000_000_007;

    assert_eq!(mod_pow(3, 13, MOD), 1594323);
    assert_eq!(mod_pow(2, 10, MOD), 1024);

    // Verify modular inverse: a * a^(-1) mod p = 1
    let a = 42u64;
    let inv = mod_inverse(a, MOD);
    assert_eq!(((a as u128 * inv as u128) % MOD as u128) as u64, 1);

    println!("Fast exponentiation: all assertions passed");
}
```

**Complexity.** O(log n) time, O(1) space. This is the workhorse behind modular
division and large-exponent computation.

---

## 2. Combinatorics

### 2.1 Factorials, Permutations, and Combinations

**Real-world analogy.** You have 5 books on a shelf.

- **Factorial (5!):** How many ways can you arrange all 5? 5! = 120.
- **Permutation P(5,3):** How many ways to pick 3 books and arrange them in order?
  5! / 2! = 60.
- **Combination C(5,3):** How many ways to pick 3 books ignoring order?
  5! / (3! * 2!) = 10.

```rust
const MOD: u64 = 1_000_000_007;

/// Precompute factorials and inverse factorials for nCr queries.
struct Combinatorics {
    fact: Vec<u64>,
    inv_fact: Vec<u64>,
}

impl Combinatorics {
    fn new(max_n: usize) -> Self {
        let mut fact = vec![1u64; max_n + 1];
        for i in 1..=max_n {
            fact[i] = (fact[i - 1] as u128 * i as u128 % MOD as u128) as u64;
        }

        let mut inv_fact = vec![1u64; max_n + 1];
        // Compute inverse of max_n!, then work backwards.
        inv_fact[max_n] = mod_pow(fact[max_n], MOD - 2, MOD);
        for i in (0..max_n).rev() {
            inv_fact[i] =
                (inv_fact[i + 1] as u128 * (i + 1) as u128 % MOD as u128) as u64;
        }

        Self { fact, inv_fact }
    }

    /// n choose r (combination), mod MOD.
    fn ncr(&self, n: usize, r: usize) -> u64 {
        if r > n {
            return 0;
        }
        let num = self.fact[n];
        let den = (self.inv_fact[r] as u128 * self.inv_fact[n - r] as u128
            % MOD as u128) as u64;
        (num as u128 * den as u128 % MOD as u128) as u64
    }

    /// n permute r, mod MOD.
    fn npr(&self, n: usize, r: usize) -> u64 {
        if r > n {
            return 0;
        }
        (self.fact[n] as u128 * self.inv_fact[n - r] as u128 % MOD as u128) as u64
    }
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        exp >>= 1;
        base = (base as u128 * base as u128 % modulus as u128) as u64;
    }
    result
}

fn main() {
    let comb = Combinatorics::new(100_000);

    assert_eq!(comb.ncr(5, 3), 10);
    assert_eq!(comb.ncr(10, 0), 1);
    assert_eq!(comb.ncr(10, 10), 1);
    assert_eq!(comb.npr(5, 3), 60);

    println!("Combinatorics: all assertions passed");
}
```

**Complexity.** O(n) precomputation, O(1) per query. This pattern appears in countless
DP + counting problems.

---

### 2.2 Pascal's Triangle

**Real-world analogy.** Pascal's Triangle is a cheat sheet for combinations. Each
entry is the sum of the two entries directly above it. The n-th row (0-indexed) gives
you C(n,0), C(n,1), ..., C(n,n).

```
Row 0:          1
Row 1:        1   1
Row 2:      1   2   1
Row 3:    1   3   3   1
Row 4:  1   4   6   4   1

Each number = sum of the two above it.
C(4,2) = 6 -- read straight from row 4, position 2.
```

```rust
/// Generate the first `num_rows` rows of Pascal's Triangle.
fn pascals_triangle(num_rows: usize) -> Vec<Vec<u64>> {
    let mut triangle: Vec<Vec<u64>> = Vec::with_capacity(num_rows);

    for row in 0..num_rows {
        let mut current = vec![1u64; row + 1];
        for j in 1..row {
            current[j] = triangle[row - 1][j - 1] + triangle[row - 1][j];
        }
        triangle.push(current);
    }
    triangle
}

fn main() {
    let tri = pascals_triangle(6);
    assert_eq!(tri[4], vec![1, 4, 6, 4, 1]);
    assert_eq!(tri[5], vec![1, 5, 10, 10, 5, 1]);
    for (i, row) in tri.iter().enumerate() {
        let padding = " ".repeat((5 - i) * 2);
        let nums: Vec<String> = row.iter().map(|x| format!("{:3}", x)).collect();
        println!("{}{}", padding, nums.join(" "));
    }
}
```

**Complexity.** O(n^2) time and space for n rows. Useful when you need small nCr
values without modular arithmetic overhead.

---

## 3. Geometry Problems

### 3.1 Points, Lines, and Distances

**Real-world analogy.** A GPS coordinate is a point. The straight-line distance between
two GPS pins is the Euclidean distance. Manhattan distance is like navigating a city
grid where you can only walk along streets (no cutting through buildings diagonally).

```rust
/// Euclidean distance (floating point).
fn euclidean_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// Squared Euclidean distance (integer-safe, avoids sqrt).
/// Use this when you only need to *compare* distances.
fn distance_squared(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x2 - x1).pow(2) + (y2 - y1).pow(2)
}

/// Manhattan distance -- movement along grid axes only.
fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn main() {
    let d = euclidean_distance(0.0, 0.0, 3.0, 4.0);
    assert!((d - 5.0).abs() < 1e-9);

    assert_eq!(distance_squared(0, 0, 3, 4), 25);
    assert_eq!(manhattan_distance(0, 0, 3, 4), 7);

    println!("Distance calculations: all assertions passed");
}
```

**Interview tip.** When the problem asks "which point is closest," compare squared
distances instead of computing square roots. This avoids floating-point imprecision
and is faster. Many interviewers specifically look for this optimization.

---

### 3.2 Containment Tests

**Point inside rectangle.** A rectangle aligned with the axes is defined by its
bottom-left corner (x1, y1) and top-right corner (x2, y2). A point (px, py) is inside
if and only if x1 <= px <= x2 AND y1 <= py <= y2.

```
  (x1,y2) +-----------+ (x2,y2)
           |           |
           |    * P    |     P is inside if x1 <= px <= x2
           |           |                 and y1 <= py <= y2
  (x1,y1) +-----------+ (x2,y1)
```

**Point inside circle.** A circle has center (cx, cy) and radius r. A point (px, py)
is inside if the distance from (px, py) to the center is <= r. Use squared distance
to avoid floating point.

```
             .  *  .
           *    |r   *
          *     |     *
          * ----+---- *   center (cx, cy)
          *           *
           *         *
             *  .  *

  Point P is inside if (px-cx)^2 + (py-cy)^2 <= r^2
```

```rust
fn point_in_rectangle(
    px: i64, py: i64,
    x1: i64, y1: i64,   // bottom-left
    x2: i64, y2: i64,   // top-right
) -> bool {
    px >= x1 && px <= x2 && py >= y1 && py <= y2
}

fn point_in_circle(
    px: i64, py: i64,
    cx: i64, cy: i64,
    r: i64,
) -> bool {
    let dx = px - cx;
    let dy = py - cy;
    dx * dx + dy * dy <= r * r
}

fn main() {
    assert!(point_in_rectangle(3, 4, 0, 0, 5, 5));
    assert!(!point_in_rectangle(6, 4, 0, 0, 5, 5));

    assert!(point_in_circle(1, 1, 0, 0, 2));
    assert!(!point_in_circle(2, 2, 0, 0, 2)); // 4+4=8 > 4

    println!("Containment tests: all assertions passed");
}
```

---

### 3.3 Rotate a Matrix 90 Degrees Clockwise (In-Place)

**Real-world analogy.** Think of a square photo printed on a tile. To rotate it 90
degrees clockwise, you first flip it upside down (reverse the rows), then flip it
along the main diagonal (transpose). Two simple operations compose into a rotation.

The trick: **Rotate = Transpose + Reverse each row.**

Wait -- which order? For 90 degrees clockwise:
- Step 1: Transpose (swap matrix[i][j] with matrix[j][i]).
- Step 2: Reverse each row.

For 90 degrees counter-clockwise:
- Step 1: Transpose.
- Step 2: Reverse each column (or equivalently: reverse each row, then transpose).

```
Original:          Transpose:         Reverse rows:
1  2  3            1  4  7            7  4  1
4  5  6    --->    2  5  8    --->    8  5  2
7  8  9            3  6  9            9  6  3

Result: 90-degree clockwise rotation.
```

```rust
/// Rotate an NxN matrix 90 degrees clockwise in-place.
fn rotate_matrix(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();

    // Step 1: Transpose (swap across the main diagonal).
    for i in 0..n {
        for j in (i + 1)..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    // Step 2: Reverse each row.
    for row in matrix.iter_mut() {
        row.reverse();
    }
}

fn main() {
    let mut m = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    rotate_matrix(&mut m);
    assert_eq!(m, vec![
        vec![7, 4, 1],
        vec![8, 5, 2],
        vec![9, 6, 3],
    ]);
    println!("Matrix rotation: assertion passed");
}
```

**Complexity.** O(n^2) time, O(1) extra space (in-place).

---

### 3.4 Spiral Matrix Traversal

**Real-world analogy.** Imagine mowing a rectangular lawn. You start at the top-left
corner, mow across the top edge, turn right and mow down the right edge, turn right
and mow across the bottom, turn right and mow up the left side, then move one row
inward and repeat. You spiral inward until every strip of grass is cut.

```
Traversal order for a 4x4 matrix:

 1  ->  2  ->  3  ->  4
                       |
                       v
 12 -> 13 -> 14        5
 ^                     |
 |                     v
 11        16 <- 15    6
 ^                     |
 |                     v
 10 <-  9  <-  8  <-  7

Direction cycle: RIGHT -> DOWN -> LEFT -> UP -> repeat.
Shrink boundaries after completing each edge.
```

```rust
/// Return elements of the matrix in spiral order.
fn spiral_order(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return vec![];
    }

    let mut result = Vec::new();
    let (mut top, mut bottom) = (0i32, matrix.len() as i32 - 1);
    let (mut left, mut right) = (0i32, matrix[0].len() as i32 - 1);

    while top <= bottom && left <= right {
        // Traverse right across the top row.
        for col in left..=right {
            result.push(matrix[top as usize][col as usize]);
        }
        top += 1;

        // Traverse down the right column.
        for row in top..=bottom {
            result.push(matrix[row as usize][right as usize]);
        }
        right -= 1;

        // Traverse left across the bottom row (if rows remain).
        if top <= bottom {
            for col in (left..=right).rev() {
                result.push(matrix[bottom as usize][col as usize]);
            }
            bottom -= 1;
        }

        // Traverse up the left column (if columns remain).
        if left <= right {
            for row in (top..=bottom).rev() {
                result.push(matrix[row as usize][left as usize]);
            }
            left += 1;
        }
    }
    result
}

fn main() {
    let matrix = vec![
        vec![ 1,  2,  3,  4],
        vec![ 5,  6,  7,  8],
        vec![ 9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    let spiral = spiral_order(&matrix);
    assert_eq!(
        spiral,
        vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
    );
    println!("Spiral traversal: assertion passed");
}
```

**Complexity.** O(m * n) time, O(1) extra space beyond the output vector. Every
element is visited exactly once.

---

## 4. Math Tricks for Interviews (Rust-Specific)

### 4.1 Integer Overflow Handling

Rust panics on integer overflow in debug mode and wraps silently in release mode (by
default). Neither behavior is what you want in an interview. You must be deliberate.

**The landmines:**

| Operation           | Example that overflows          | Risk              |
|---------------------|---------------------------------|--------------------|
| `a * b`             | `i32::MAX * 2`                  | Product too large  |
| `a + b`             | `i32::MAX + 1`                  | Sum too large      |
| `(a + b) / 2`       | When a + b overflows            | Midpoint calc      |
| `n!`                | `21!` exceeds `u64::MAX`        | Factorial growth   |
| `-i32::MIN`         | `-(-2_147_483_648)` has no i32  | Negation overflow  |

**Rust's toolkit for safe arithmetic:**

```rust
fn main() {
    // checked_* returns Option: None on overflow.
    assert_eq!(100i32.checked_mul(200), Some(20_000));
    assert_eq!(i32::MAX.checked_add(1), None);

    // saturating_* clamps at the type boundary.
    assert_eq!(i32::MAX.saturating_add(1), i32::MAX);

    // wrapping_* does modular arithmetic (like C).
    assert_eq!(i32::MAX.wrapping_add(1), i32::MIN);

    // overflowing_* returns (result, did_overflow).
    let (val, overflowed) = i32::MAX.overflowing_add(1);
    assert!(overflowed);
    assert_eq!(val, i32::MIN);

    // The safe midpoint calculation (avoids a + b overflow):
    let a: i32 = 1_000_000_000;
    let b: i32 = 2_000_000_000;
    let mid = a + (b - a) / 2; // safe: b - a does not overflow
    assert_eq!(mid, 1_500_000_000);

    // For multiplication in mod arithmetic, cast up to u128 / i128:
    let x: u64 = 1_000_000_006;
    let y: u64 = 1_000_000_006;
    let product_mod = (x as u128 * y as u128 % 1_000_000_007) as u64;
    assert_eq!(product_mod, 999_999_986);

    println!("Overflow handling: all assertions passed");
}
```

**Interview rule of thumb:** whenever you multiply two numbers that could each be up
to 10^9, cast to u128 (or i128) before multiplying. Rust makes this painless compared
to C/C++.

---

### 4.2 The Negation Trap with i32::MIN

This is subtle enough to warrant its own callout. In two's complement, the absolute
value of `i32::MIN` (-2,147,483,648) does not fit in an `i32` (max positive is
2,147,483,647). Rust will panic in debug mode if you write `(-i32::MIN)`.

The standard workaround: cast to `i64` before negating, or use `.unsigned_abs()`.

```rust
fn safe_abs(x: i32) -> u32 {
    x.unsigned_abs() // available since Rust 1.51, always safe
}

fn main() {
    assert_eq!(safe_abs(i32::MIN), 2_147_483_648u32);
    assert_eq!(safe_abs(-42), 42);
    assert_eq!(safe_abs(0), 0);
    println!("Safe abs: all assertions passed");
}
```

---

### 4.3 Floyd's Cycle Detection (Math Context)

We covered Floyd's tortoise-and-hare algorithm in the linked list lesson. It also
applies to pure math problems. Any time you have a function `f(x)` applied
repeatedly (x, f(x), f(f(x)), ...) where the domain is finite, there must be a
cycle. Floyd's algorithm finds it in O(1) space.

**Classic interview problem:** "Happy Number." Repeatedly sum the squares of the digits.
If you reach 1, the number is happy. If you enter a cycle, it is not.

```
19 -> 1^2 + 9^2 = 82 -> 64 + 4 = 68 -> 36 + 64 = 100 -> 1  (happy!)
 2 -> 4 -> 16 -> 37 -> 58 -> 89 -> 145 -> 42 -> 20 -> 4  (cycle, not happy)
```

```rust
fn digit_square_sum(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        let d = n % 10;
        sum += d * d;
        n /= 10;
    }
    sum
}

fn is_happy(n: u32) -> bool {
    let mut slow = n;
    let mut fast = digit_square_sum(n);

    while fast != 1 && slow != fast {
        slow = digit_square_sum(slow);          // one step
        fast = digit_square_sum(digit_square_sum(fast)); // two steps
    }
    fast == 1
}

fn main() {
    assert!(is_happy(19));
    assert!(!is_happy(2));
    assert!(is_happy(1));
    println!("Happy number (Floyd's): all assertions passed");
}
```

---

## 5. Complexity Summary

| Algorithm / Operation           | Time             | Space   |
|---------------------------------|------------------|---------|
| GCD (Euclidean)                 | O(log min(a,b))  | O(1)    |
| Sieve of Eratosthenes (to n)   | O(n log log n)   | O(n)    |
| Prime factorization of n        | O(sqrt(n))       | O(log n)|
| Binary exponentiation           | O(log exp)       | O(1)    |
| Factorial precomputation (to n) | O(n)             | O(n)    |
| nCr / nPr query (precomputed)   | O(1)             | --      |
| Pascal's Triangle (n rows)      | O(n^2)           | O(n^2)  |
| Euclidean distance              | O(1)             | O(1)    |
| Point-in-rect / point-in-circle | O(1)             | O(1)    |
| Rotate NxN matrix (in-place)    | O(n^2)           | O(1)    |
| Spiral traversal (m x n)        | O(m*n)           | O(1)*   |

\* O(1) extra space beyond the output.

---

## 6. Practice Problems

### Easy (Warm-Up)

| #  | Problem                          | Key Idea                                |
|----|----------------------------------|-----------------------------------------|
| 1  | Fizz Buzz                        | Modular arithmetic basics               |
| 2  | Count Primes (< n)               | Sieve of Eratosthenes                   |
| 3  | Power of Two                     | Bit trick or repeated division          |
| 4  | Happy Number                     | Floyd's cycle detection on a function   |
| 5  | Plus One (digit array)           | Carry propagation                       |

### Medium (Core Interview Problems)

| #  | Problem                          | Key Idea                                |
|----|----------------------------------|-----------------------------------------|
| 1  | Rotate Image (NxN matrix)        | Transpose + reverse rows                |
| 2  | Spiral Matrix                    | Boundary shrinking traversal            |
| 3  | Pow(x, n)                        | Binary exponentiation, handle negatives |
| 4  | Unique Paths (grid)              | nCr(m+n-2, m-1) or DP                  |
| 5  | Multiply Strings                 | Grade-school multiplication simulation  |

### Hard (Deep Dives)

| #  | Problem                          | Key Idea                                |
|----|----------------------------------|-----------------------------------------|
| 1  | Max Points on a Line             | GCD to normalize slope, HashMap         |
| 2  | Largest Rectangle in Histogram   | Stack-based geometry reasoning          |
| 3  | Count of Smaller Numbers (mergesort) | Inversion count via merge sort       |
| 4  | Super Pow (a^[huge array])       | Modular exponent decomposition          |
| 5  | Number of Digit One (1 to n)     | Digit DP with math pattern              |

---

## 7. Key Takeaways

1. **GCD is everywhere.** Simplifying fractions, normalizing slopes, reducing ratios --
   if a problem involves ratios of integers, GCD is likely the tool. The Euclidean
   algorithm is O(log n) and fits in three lines.

2. **Modular arithmetic follows addition and multiplication, but NOT division.** For
   division under a prime modulus, use Fermat's little theorem (modular inverse via
   binary exponentiation). When the problem says "return answer mod 10^9+7," apply
   mod after every addition and multiplication.

3. **Binary exponentiation is the universal power tool.** Computing `a^b mod m` in
   O(log b) shows up in combinatorics (modular inverse), number theory, and matrix
   exponentiation for recurrence speedups.

4. **Matrix rotation and spiral traversal are pattern-matching problems.** Once you
   memorize "transpose + reverse = 90-degree rotation" and the boundary-shrinking
   spiral loop, these become free points in interviews.

5. **In Rust, respect the type system.** Cast to `u128` before multiplying two `u64`
   values. Use `checked_*` when overflow is possible. Use `.unsigned_abs()` to avoid
   the `i32::MIN` negation trap. These are Rust-specific details that catch people in
   timed interviews.

6. **Avoid floating point when possible.** Compare squared distances instead of
   taking square roots. Use integer GCD to represent fractions exactly. Floating-point
   equality checks are a source of subtle bugs.

7. **Floyd's cycle detection applies beyond linked lists.** Any repeated function
   application on a finite domain creates a cycle. The happy number problem is the
   classic example, but it also appears in detecting duplicate values (Leetcode 287:
   Find the Duplicate Number).
