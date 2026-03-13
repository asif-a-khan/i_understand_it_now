# Lesson 01: Big-O Notation & Complexity Analysis

## Why Should You Care?

You have six years of shipping software. You know that "slow code" is bad and "fast code"
is good. You have probably profiled a hot path, slapped a cache on something, or swapped
a nested loop for a hash map lookup based on gut instinct. Big-O gives you the *vocabulary
and framework* to reason about those decisions before you write a single line of code.

It answers one question: **as the input grows, how does the cost grow?**

Not the exact runtime in milliseconds. Not the number of CPU cycles. The *shape* of the
growth curve. That shape determines whether your solution works on 100 items, 10,000 items,
or 10 million items.

---

## What Big-O Actually Means

Big-O is a mathematical notation borrowed from analysis. In CS, we use it informally to
describe an **upper bound** on how a function's resource usage (time or space) scales with
input size `n`.

When we write:

    f(n) is O(g(n))

we mean: there exist constants `c > 0` and `n0 >= 0` such that for all `n >= n0`:

    f(n) <= c * g(n)

In plain English: past some threshold, `f(n)` never grows faster than `g(n)` (up to a
constant multiplier). We drop constants and lower-order terms because we care about the
*trend*, not the exact count.

### A Real-World Analogy

Imagine you are shipping packages.

- **O(1)**: You have a magic teleporter. Whether you ship 1 package or 1 million, it takes
  the same effort. The number of packages is irrelevant.
- **O(n)**: You load packages onto a truck one by one. Twice as many packages means roughly
  twice as long.
- **O(n^2)**: Before loading each package, you compare it against every other package to
  check for duplicates by hand. Double the packages and you quadruple the comparisons.

The teleporter is not literally instant -- it might take 5 seconds each time. But that 5
seconds stays 5 seconds whether `n` is 10 or 10 billion. That is the point: we care about
scaling behavior, not the constant.

---

## The Common Complexity Classes

Here they are from best to worst, with the growth curve visualized:

```
cost
 ^
 |                                                        . 2^n
 |                                                      .
 |                                                    .
 |                                                  .
 |                                               .
 |                                            .
 |                                        ..
 |                                    ...          n^2
 |                                ...         ....
 |                           ....        ....
 |                       ....        ....
 |                   ....        ....
 |               ...         ...
 |           ....        ....                  n log n
 |       ....       ....               ........
 |   ....      ....            ........
 |....     ....        ........                      n
 |..   ....     ........               ..............
 |. ...  .......              ..........
 |......            ..........                   log n
 |...    ...........
 |.......                                        1
 |_______________________________________________........____> n
```

And here is the table:

| Notation     | Name             | Example Operation                         |
|--------------|------------------|-------------------------------------------|
| O(1)         | Constant         | Array index lookup, HashMap get            |
| O(log n)     | Logarithmic      | Binary search                              |
| O(n)         | Linear           | Scanning an array once                     |
| O(n log n)   | Linearithmic     | Merge sort, good general-purpose sorts     |
| O(n^2)       | Quadratic        | Nested loops over same collection          |
| O(2^n)       | Exponential      | Brute-force subsets, naive recursive fib   |
| O(n!)        | Factorial         | Brute-force permutations                  |

### How They Feel at Scale

| n       | O(1) | O(log n) | O(n)     | O(n log n) | O(n^2)       | O(2^n)          |
|---------|------|----------|----------|------------|--------------|-----------------|
| 10      | 1    | ~3       | 10       | ~33        | 100          | 1,024           |
| 100     | 1    | ~7       | 100      | ~664       | 10,000       | 1.27 x 10^30   |
| 1,000   | 1    | ~10      | 1,000    | ~9,966     | 1,000,000    | heat death      |
| 1M      | 1    | ~20      | 1M       | ~20M       | 10^12        | do not ask      |

At n = 1,000, an O(n^2) algorithm does a million operations. An O(2^n) algorithm cannot
finish before the universe ends. This is why Big-O matters in practice.

---

## Analyzing Time Complexity

The core skill is looking at code and determining its Big-O. Let's build this up.

### Rule 1: Sequential Statements Add

```rust
fn example(data: &[i32]) {
    // O(n)
    let sum: i32 = data.iter().sum();

    // O(n)
    let max = data.iter().max();

    // Total: O(n) + O(n) = O(2n) = O(n)
    // We drop the constant.
    println!("sum={sum}, max={max:?}");
}
```

### Rule 2: Nested Loops Multiply

```rust
fn has_duplicate_pair(data: &[i32]) -> bool {
    let n = data.len();
    for i in 0..n {              // outer: n iterations
        for j in (i + 1)..n {    // inner: up to n iterations
            if data[i] == data[j] {
                return true;
            }
        }
    }
    false
}
// Iterations: n*(n-1)/2 => O(n^2)
```

The inner loop does not always run `n` times, but in the worst case it is proportional to
`n` for each outer iteration. We always analyze the **worst case** unless stated otherwise.

### Rule 3: Drop Non-Dominant Terms

If your function does O(n^2) work and then O(n) work:

    O(n^2 + n) = O(n^2)

The n^2 term dominates as n grows. The linear part becomes a rounding error.

### Rule 4: Different Inputs Get Different Variables

```rust
fn print_pairs(a: &[i32], b: &[i32]) {
    for x in a {           // O(a)
        for y in b {       // O(b) for each x
            println!("{x}, {y}");
        }
    }
}
// Total: O(a * b), NOT O(n^2)
// Only O(n^2) if a and b are the same collection.
```

This is a common interview trick and a common real-world mistake. If you are iterating over
users and for each user iterating over their orders, that is O(users * orders), not
O(users^2) -- unless every user has O(users) orders.

---

## Analyzing Loops: Worked Examples in Rust

### Linear: O(n)

```rust
fn linear_search(haystack: &[i32], needle: i32) -> Option<usize> {
    for (i, &val) in haystack.iter().enumerate() {
        if val == needle {
            return Some(i);
        }
    }
    None
}
// Best case: O(1) -- found at index 0
// Worst case: O(n) -- not found or found at last index
// Big-O (without qualifier) means worst case: O(n)
```

### Logarithmic: O(log n)

```rust
fn binary_search(sorted: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = sorted.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match sorted[mid].cmp(&target) {
            std::cmp::Ordering::Equal   => return Some(mid),
            std::cmp::Ordering::Less    => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}
// Each iteration cuts the search space in half.
// Starting with n elements: n -> n/2 -> n/4 -> ... -> 1
// Number of halvings to reach 1: log2(n)
// Therefore: O(log n)
```

**The halving pattern is the signature of O(log n).** Whenever you see the problem size
being cut in half (or thirds, or any constant fraction) each step, think logarithmic.

```
n = 16:  [________________]
          [________]          step 1: 16 -> 8
          [____]              step 2: 8 -> 4
          [__]                step 3: 4 -> 2
          [_]                 step 4: 2 -> 1
                              4 steps = log2(16)
```

### Quadratic: O(n^2)

```rust
/// Selection sort: find the minimum, swap it to the front, repeat.
fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}
// Outer loop: n iterations
// Inner loop: (n-1) + (n-2) + ... + 1 + 0 = n(n-1)/2
// O(n^2/2) = O(n^2)
```

### O(n log n): The "Good Sort" Complexity

```rust
fn merge_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;

    // Split and recurse -- each half is n/2
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);

    // Merge -- linear scan of both halves: O(n)
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
// Recursion depth: log n (halving each time)
// Work at each level: O(n) (merge touches every element once)
// Total: O(n) * O(log n) = O(n log n)
```

Visualized:

```
Level 0 (1 merge of n):     [........] = n work
Level 1 (2 merges of n/2):  [....][....] = n work
Level 2 (4 merges of n/4):  [..][..][..][..] = n work
Level 3 (8 merges of n/8):  [.][.][.][.][.][.][.][.] = n work
                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
                             log(n) levels, each doing n work
                             => O(n log n) total
```

---

## Analyzing Recursion

Recursive functions require a different mental model. The key question:
**how many times does the function call itself, and how much work does each call do?**

### Example: Fibonacci (The Cautionary Tale)

```rust
fn fib_naive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib_naive(n - 1) + fib_naive(n - 2)
}
// Each call spawns TWO more calls.
// Call tree for fib(5):
//
//                         fib(5)
//                       /        \
//                  fib(4)          fib(3)
//                 /      \        /      \
//            fib(3)    fib(2)  fib(2)  fib(1)
//           /    \     /    \   /   \
//       fib(2) fib(1) f(1) f(0) f(1) f(0)
//       /   \
//    fib(1) fib(0)
//
// The tree has roughly 2^n nodes.
// Time: O(2^n)   -- catastrophically slow
// Space: O(n)    -- max recursion depth is n
```

The fix: memoize or iterate.

```rust
fn fib_linear(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
// Time: O(n), Space: O(1). Night and day.
```

### The Master Theorem (Simplified)

For recurrences of the form `T(n) = a * T(n/b) + O(n^d)`:

- If `d > log_b(a)`: T(n) = O(n^d)
- If `d = log_b(a)`: T(n) = O(n^d * log n)
- If `d < log_b(a)`: T(n) = O(n^(log_b(a)))

You do not need to memorize this. But recognizing the pattern helps:

| Algorithm     | Recurrence           | a | b | d | Result      |
|---------------|----------------------|---|---|---|-------------|
| Binary search | T(n) = T(n/2) + O(1)| 1 | 2 | 0 | O(log n)    |
| Merge sort    | T(n) = 2T(n/2) + O(n)| 2| 2 | 1 | O(n log n)  |

---

## Space Complexity

Time is not the only resource. Memory matters too, especially in Rust where you think about
allocation constantly.

Space complexity measures **additional memory used** as a function of input size. The input
itself does not count (unless you are transforming it in place).

```rust
// O(1) space -- only a few variables regardless of input size
fn sum(data: &[i32]) -> i64 {
    let mut total: i64 = 0;
    for &val in data {
        total += val as i64;
    }
    total
}

// O(n) space -- we allocate a new Vec proportional to input
fn doubled(data: &[i32]) -> Vec<i32> {
    data.iter().map(|&x| x * 2).collect()
}

// O(n) space from recursion -- the call stack grows with input
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
// Each recursive call adds a stack frame.
// Max depth = n => O(n) space on the stack.
```

### The Rust Angle: Stack vs Heap

In Rust, stack space is limited (typically 8 MB default on Linux). A recursive function
with O(n) stack depth will overflow for large n. This is one reason iterative solutions
are preferred in Rust -- and why you might reach for `Vec` (heap) over deep recursion
(stack).

```rust
// This will stack overflow for large n:
fn sum_recursive(data: &[i32]) -> i64 {
    match data {
        [] => 0,
        [first, rest @ ..] => *first as i64 + sum_recursive(rest),
    }
}

// This will not:
fn sum_iterative(data: &[i32]) -> i64 {
    data.iter().map(|&x| x as i64).sum()
}
```

---

## Amortized Analysis: When Worst Case Lies

Sometimes the worst case for a *single operation* is misleading because it happens rarely.
Amortized analysis spreads the cost of expensive operations over a sequence of cheap ones.

The classic example: `Vec::push` in Rust.

```
Vec with capacity 4, length 4:  [a][b][c][d]

push(e):
  - Vec is full! Must reallocate.
  - Allocate new buffer of capacity 8 (double).
  - Copy 4 elements to new buffer: O(n) work.
  - Insert e.
  Result: [a][b][c][d][e][ ][ ][ ]   capacity=8

push(f): O(1) -- space available
push(g): O(1) -- space available
push(h): O(1) -- space available

push(i):
  - Full again! Reallocate to capacity 16.
  - Copy 8 elements: O(n) work.
  ...
```

Visualized:

```
push #:  1  2  3  4  5  6  7  8  9  10  11 ...
cost:    1  1  1  1  5  1  1  1  9   1   1 ...
                     ^              ^
                     |              |
              realloc: copy 4  realloc: copy 8
              + 1 insert       + 1 insert
```

The expensive reallocation at push 5 cost 5 (copy 4 + insert 1). But it "paid for" the
next 4 cheap pushes. If you spread the cost evenly:

- n pushes total
- Reallocations happen at sizes 1, 2, 4, 8, 16, ... up to n
- Total copy work: 1 + 2 + 4 + ... + n = roughly 2n
- Total cost for n pushes: about 3n
- **Amortized cost per push: O(1)**

This is why `Vec::push` is documented as "amortized O(1)" in Rust's standard library. Any
single push *might* be O(n), but averaged over a sequence of pushes, each one costs O(1).

### When Amortized Analysis Matters

- **Vec / dynamic arrays**: push is amortized O(1)
- **HashMap**: insertion is amortized O(1) (resizing is O(n) but rare)
- **Splay trees**: individual operations can be O(n), amortized O(log n)

If you are doing real-time work (audio, game ticks, network packet processing), amortized
guarantees may not be enough -- that one O(n) spike could blow your deadline. In those cases
you might pre-allocate with `Vec::with_capacity` or use data structures with strict
worst-case bounds.

---

## Best, Worst, and Average Case

Big-O usually describes **worst case** unless stated otherwise. But the distinction matters.

```rust
fn contains(data: &[i32], target: i32) -> bool {
    for &val in data {
        if val == target {
            return true;
        }
    }
    false
}
```

- **Best case**: target is at index 0. O(1).
- **Worst case**: target is not in the array. O(n).
- **Average case**: assuming target is equally likely at any position, expected comparisons
  = n/2. Still O(n) -- we drop the 1/2 constant.

There are also sibling notations:
- **Big-Omega** (lower bound): the function grows *at least* this fast.
- **Big-Theta** (tight bound): the function grows *exactly* this fast (within constants).

In practice, when someone says "this algorithm is O(n log n)," they usually mean Theta --
it is both the upper and lower bound. Being precise about this in conversation is good;
being pedantic about it in interviews is not.

---

## Common Patterns Cheat Sheet

| Pattern You See                          | Likely Complexity |
|------------------------------------------|-------------------|
| No loops, no recursion                   | O(1)              |
| Single loop over n elements              | O(n)              |
| Loop that halves/doubles each step       | O(log n)          |
| Two nested loops over same data          | O(n^2)            |
| Three nested loops                       | O(n^3)            |
| Sorting then scanning                    | O(n log n)        |
| Iterating over all subsets               | O(2^n)            |
| Iterating over all permutations          | O(n!)             |
| HashMap lookup inside a loop             | O(n) total        |
| Binary search inside a loop              | O(n log n) total  |
| Divide-and-conquer with linear merge     | O(n log n)        |

---

## Practical Intuition: What Is "Fast Enough"?

A modern machine does roughly 10^8 to 10^9 simple operations per second. Use this to
sanity-check your algorithm:

| n         | O(n)   | O(n log n) | O(n^2)   | O(2^n)       |
|-----------|--------|------------|----------|--------------|
| 10^3      | instant| instant    | ~1ms     | impossible   |
| 10^5      | instant| instant    | ~10s     | impossible   |
| 10^6      | ~1ms   | ~20ms      | ~17 min  | impossible   |
| 10^8      | ~0.1s  | ~2.7s      | ~3 years | impossible   |

If your input is n = 10^6 and your algorithm is O(n^2), it will not finish in a reasonable
time. You need O(n log n) or better.

---

## Putting It All Together: A Realistic Example

Problem: given a list of timestamps, find if any two are within `k` seconds of each other.

```rust
use std::collections::HashSet;

/// Naive approach: compare every pair. O(n^2) time, O(1) space.
fn has_nearby_timestamp_naive(times: &[i64], k: i64) -> bool {
    let n = times.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if (times[i] - times[j]).abs() <= k {
                return true;
            }
        }
    }
    false
}

/// Better: sort first, then scan neighbors. O(n log n) time, O(n) space.
fn has_nearby_timestamp_sort(times: &[i64], k: i64) -> bool {
    let mut sorted = times.to_vec();  // O(n) space
    sorted.sort();                     // O(n log n)
    for window in sorted.windows(2) {  // O(n)
        if (window[1] - window[0]).abs() <= k {
            return true;
        }
    }
    false
}

/// If timestamps are integers and k is small: bucket approach.
/// O(n) time, O(n) space.
fn has_nearby_timestamp_bucket(times: &[i64], k: i64) -> bool {
    if k < 0 { return false; }
    let bucket_size = k + 1; // avoid division by zero and ensure correctness
    let mut buckets: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();

    for &t in times {
        let b = if t >= 0 {
            t / bucket_size
        } else {
            (t + 1) / bucket_size - 1
        };

        // Same bucket => difference is at most k
        if buckets.contains_key(&b) {
            return true;
        }
        // Adjacent buckets => check actual difference
        if let Some(&prev) = buckets.get(&(b - 1)) {
            if (t - prev).abs() <= k { return true; }
        }
        if let Some(&prev) = buckets.get(&(b + 1)) {
            if (t - prev).abs() <= k { return true; }
        }
        buckets.insert(b, t);
    }
    false
}
```

Three solutions to the same problem:

```
Approach         Time        Space     Works at n=10^6?
---------------------------------------------------------
Naive            O(n^2)      O(1)      No (~17 minutes)
Sort + scan      O(n log n)  O(n)      Yes (~20ms)
Bucket           O(n)        O(n)      Yes (~1ms)
```

Choosing between them is an engineering decision. If n is always small (say, under 1000),
the naive version is fine and simpler to maintain. If n can be large, you need the better
algorithm -- no amount of hardware will rescue O(n^2) at scale.

---

## Key Takeaways

1. **Big-O describes growth rate, not absolute speed.** An O(n) algorithm with a large
   constant can be slower than O(n^2) for small inputs.

2. **Analyze the worst case by default.** Mention best/average when relevant.

3. **Nested loops multiply.** Two loops over the same data = O(n^2). A loop with a binary
   search inside = O(n log n).

4. **Drop constants and lower-order terms.** O(3n + 500) = O(n).

5. **Space counts too.** Especially in Rust, where you think about allocation. Recursive
   calls consume stack space proportional to recursion depth.

6. **Amortized != worst case.** Vec::push is amortized O(1), but a single push can be O(n).
   Know when this distinction matters for your use case.

7. **Use the ~10^8 operations/second rule** to estimate whether your algorithm is fast
   enough before writing it.

---

## Exercises

Try these before moving on. Analyze the time and space complexity of each.

**Exercise 1:** What is the time complexity?
```rust
fn mystery(n: usize) {
    let mut i = 1;
    while i < n {
        println!("{i}");
        i *= 2;
    }
}
```

<details>
<summary>Answer</summary>

O(log n). The variable `i` doubles each iteration: 1, 2, 4, 8, ... until it reaches n.
That is log2(n) iterations.

</details>

**Exercise 2:** What is the time complexity?
```rust
fn mystery2(n: usize) {
    for i in 0..n {
        let mut j = 1;
        while j < n {
            println!("{i}, {j}");
            j *= 2;
        }
    }
}
```

<details>
<summary>Answer</summary>

O(n log n). The outer loop runs n times. The inner loop runs log(n) times (doubling
pattern). Multiply: n * log(n).

</details>

**Exercise 3:** What is the time and space complexity?
```rust
fn mystery3(data: &[i32]) -> Vec<Vec<i32>> {
    let n = data.len();
    let mut result = Vec::new();
    for i in 0..n {
        let mut inner = Vec::new();
        for j in i..n {
            inner.push(data[j]);
        }
        result.push(inner);
    }
    result
}
```

<details>
<summary>Answer</summary>

Time: O(n^2). The total iterations are n + (n-1) + (n-2) + ... + 1 = n(n+1)/2.

Space: O(n^2). The result contains n vectors whose total element count is n(n+1)/2.

</details>

---

Next lesson: [02 - Arrays, Vectors, and Slices](./02_arrays_vectors_slices.md)
