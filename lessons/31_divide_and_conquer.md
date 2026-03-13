# Lesson 31: Divide and Conquer

## The Pattern Behind the Patterns

You have already used divide and conquer twice without pausing to name it. In
[Lesson 10](./10_merge_sort.md), you split an array in half, sorted each half, and merged.
In [Lesson 11](./11_quick_sort.md), you partitioned around a pivot, then sorted each side.
Both times, the structure was the same: break a big problem into smaller pieces, solve each
piece, combine the answers. That structure is divide and conquer.

This lesson steps back to see the general pattern. Once you recognize it, you start seeing
it everywhere -- not just in sorting, but in searching, counting, geometry, and selection
problems. You also learn when it is the right tool and when dynamic programming or a greedy
approach would serve you better.

---

## The Three Steps

Every divide and conquer algorithm follows the same skeleton:

```
┌─────────────────────────────────────────────────────────┐
│                     PROBLEM (size n)                    │
└──────────────────────┬──────────────────────────────────┘
                       │  1. DIVIDE
                       v
        ┌──────────────┴──────────────┐
        │                             │
   ┌────v────┐                  ┌─────v────┐
   │ sub-    │                  │ sub-     │
   │ problem │                  │ problem  │
   │ (n/b)   │                  │ (n/b)    │
   └────┬────┘                  └─────┬────┘
        │  2. CONQUER (recurse)       │
        v                             v
   ┌─────────┐                  ┌──────────┐
   │ sub-    │                  │ sub-     │
   │ solution│                  │ solution │
   └────┬────┘                  └─────┬────┘
        │                             │
        └──────────────┬──────────────┘
                       │  3. COMBINE
                       v
        ┌──────────────────────────────┐
        │       SOLUTION (size n)      │
        └──────────────────────────────┘
```

1. **Divide** -- Split the problem into smaller subproblems of the same type.
2. **Conquer** -- Solve each subproblem recursively. When a subproblem is small enough (the
   base case), solve it directly.
3. **Combine** -- Merge the subproblem solutions into a solution for the original problem.

The critical insight: **each subproblem is structurally identical to the original**, just
smaller. That self-similarity is what makes recursion the natural implementation tool (as
you saw in [Lesson 07](./07_recursion.md)).

### A Real-World Analogy

You are the lead engineer on a team tasked with auditing a codebase of 10,000 files for
security vulnerabilities.

- **Divide**: Split the files into four roughly equal groups of 2,500. Assign each group
  to a team member.
- **Conquer**: Each team member audits their group independently. If their group is still
  too large to handle alone, they can sub-divide further (maybe by module or directory).
  Eventually someone is looking at a single file -- that is the base case.
- **Combine**: Collect all the findings into a single report, deduplicating and prioritizing.

The key properties: (a) the sub-tasks are the same kind of work as the original, just
smaller; (b) the sub-tasks are independent -- nobody needs to wait for someone else's
results to do their own work; (c) there is a combining step at the end.

That independence property is what distinguishes D&C from dynamic programming, where
subproblems overlap and share state. We will come back to this distinction later.

---

## Relationship to Recursion

Divide and conquer is a *strategy*. Recursion is the *implementation technique* you use to
express it. The generic recursive template from Lesson 07 maps directly:

```rust
fn solve(input: &[T]) -> Answer {
    // Base case
    if input.len() <= THRESHOLD {
        return solve_directly(input);
    }

    // 1. Divide
    let (left, right) = split(input);

    // 2. Conquer
    let left_answer = solve(left);
    let right_answer = solve(right);

    // 3. Combine
    combine(left_answer, right_answer)
}
```

Not all recursion is divide and conquer. Computing `factorial(n)` is recursive but not D&C
-- you make one recursive call, not multiple. D&C specifically involves splitting into
*multiple* subproblems. And not all D&C implementations use recursion -- you can sometimes
unroll the recursion into iteration (as we saw with bottom-up merge sort). But recursion is
the natural first expression of any D&C algorithm.

---

## The Recursion Tree: Visualizing Work Per Level

Understanding *why* D&C algorithms have the time complexity they do comes down to drawing
the recursion tree and summing the work at each level.

For a typical D&C algorithm that splits into **a** subproblems of size **n/b** and does
**O(n^d)** work to divide and combine:

```
Level 0:  1 problem of size n                          Work: n^d
             /          \
Level 1:  a problems of size n/b                       Work: a * (n/b)^d
           /  \        /  \
Level 2:  a^2 problems of size n/b^2                   Work: a^2 * (n/b^2)^d
          ...
Level k:  a^k problems of size n/b^k                   Work: a^k * (n/b^k)^d

The tree has log_b(n) levels (we stop when subproblem size = 1).
```

For merge sort (a=2, b=2, d=1):

```
Level 0:  1 array of size n           merge work: n
             /          \
Level 1:  2 arrays of size n/2        merge work: 2*(n/2) = n
           /  \        /  \
Level 2:  4 arrays of size n/4        merge work: 4*(n/4) = n
           ...
Level k:  2^k arrays of size n/2^k   merge work: 2^k * (n/2^k) = n
           ...
Level log n: n arrays of size 1       merge work: n * 1 = n

Total: n work per level * log(n) levels = O(n log n)
```

Every level does exactly O(n) total work. The levels just distribute it across more,
smaller merges. This "equal work per level" pattern is the signature of the
O(n^d * log n) case of the Master Theorem.

---

## The Master Theorem

You have seen recurrences like T(n) = 2T(n/2) + O(n) pop up repeatedly. The **Master
Theorem** gives you a formula for solving them without having to draw the recursion tree
every time.

For recurrences of the form:

```
T(n) = a * T(n/b) + O(n^d)

where:
  a = number of subproblems at each level
  b = factor by which the problem shrinks
  d = exponent in the work done outside the recursive calls
```

There are three cases. The intuition: we compare the rate at which subproblems multiply
(governed by a and b) to the rate at which the per-subproblem work shrinks (governed by d).

```
Let c = log_b(a).  This is the "critical exponent."

Case 1:  d > c     (combine work dominates)
         T(n) = O(n^d)

Case 2:  d = c     (work is balanced across levels)
         T(n) = O(n^d * log n)

Case 3:  d < c     (recursive subproblem work dominates)
         T(n) = O(n^c) = O(n^(log_b(a)))
```

### An Intuitive Reading

Think of the recursion tree again:

- **Case 1** (d > c): The root of the tree does the most work. Each level below does
  geometrically less. Total work is dominated by the top level: O(n^d).
- **Case 2** (d = c): Every level of the tree does roughly the same amount of work. There
  are O(log n) levels, each doing O(n^d). Total: O(n^d * log n).
- **Case 3** (d < c): The leaves do the most work. The tree "fans out" faster than the
  per-subproblem work shrinks. Total is dominated by the number of leaves: O(n^(log_b a)).

```
Case 1: Top-heavy          Case 2: Balanced         Case 3: Bottom-heavy

  ================           ================         ====
  ========                   ================         ========
  ====                       ================         ============
  ==                         ================         ================

  Work dominated             Work spread evenly       Work dominated
  by root level              across all levels        by leaf level
```

### Applying It to Algorithms You Know

| Algorithm             | Recurrence              | a | b | d | c=log_b(a) | Case | Result       |
|-----------------------|-------------------------|---|---|---|------------|------|--------------|
| Binary search         | T(n) = T(n/2) + O(1)   | 1 | 2 | 0 | 0          | 2    | O(log n)     |
| Merge sort            | T(n) = 2T(n/2) + O(n)  | 2 | 2 | 1 | 1          | 2    | O(n log n)   |
| Quickselect (avg)     | T(n) = T(n/2) + O(n)   | 1 | 2 | 1 | 0          | 1    | O(n)         |
| Max subarray (D&C)    | T(n) = 2T(n/2) + O(n)  | 2 | 2 | 1 | 1          | 2    | O(n log n)   |
| Karatsuba             | T(n) = 3T(n/2) + O(n)  | 3 | 2 | 1 | 1.58       | 3    | O(n^1.58)    |
| Strassen's multiply   | T(n) = 7T(n/2) + O(n^2)| 7 | 2 | 2 | ~2.81      | 3    | O(n^2.81)    |
| Naive matrix multiply | T(n) = 8T(n/2) + O(n^2)| 8 | 2 | 2 | 3          | 3    | O(n^3)       |

### When the Master Theorem Does Not Apply

The theorem requires:
- Subproblems of **equal size** (all n/b). If you split into n/3 and 2n/3, it does not
  directly apply (though you can often still analyze by other means).
- The non-recursive work is a **polynomial** in n. If the extra work is something like
  O(n log n), you need the more general Akra-Bazzi theorem or a direct recursion tree
  analysis.

For this course, the three cases above will cover every D&C recurrence you encounter.

---

## Classic Problem 1: Merge Sort (Brief Recap)

Merge sort is the canonical D&C algorithm. We covered it in detail in
[Lesson 10](./10_merge_sort.md). The pattern:

| Step    | What happens                                       |
|---------|----------------------------------------------------|
| Divide  | Split the array in half at the midpoint             |
| Conquer | Recursively sort each half                          |
| Combine | Merge two sorted halves into one sorted array (O(n))|

The divide step is trivial (just compute the midpoint). The combine step does the real work.
Quicksort is the mirror image: expensive divide (partition), trivial combine (nothing).

Recurrence: `T(n) = 2T(n/2) + O(n)` => O(n log n) by the Master Theorem (Case 2).

If you have not read Lesson 10, go there first. Everything below builds on that foundation.

---

## Classic Problem 2: Quick Select (Finding the Kth Element)

### The Problem

Given an unsorted array and an integer k, find the kth smallest element. Sorting first
would cost O(n log n). Quick select does it in **O(n) average time** by using quicksort's
partitioning but only recursing into one side.

### How It Works

After partitioning, the pivot lands at its final sorted position p:

- If p == k, the pivot is the answer.
- If k < p, the answer is in the left partition. Recurse left only.
- If k > p, the answer is in the right partition. Recurse right only.

```
Find 3rd smallest (k=2, zero-indexed) in [7, 2, 5, 1, 8, 3, 6]:

Partition with pivot=6: [2, 5, 1, 3, |6|, 8, 7]
                         0  1  2  3   4   5  6
Pivot landed at index 4. We want index 2. k < p, so recurse left.

Partition [2, 5, 1, 3] with pivot=3: [2, 1, |3|, 5]
                                       0  1   2   3
Pivot landed at index 2. k == 2. Answer is 3.
```

### Implementation

```rust
use std::cmp::Ordering;

/// Lomuto partition: choose arr[hi] as pivot.
fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi];
    let mut i = lo;
    for j in lo..hi {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, hi);
    i
}

/// Find the kth smallest element (0-indexed).
/// Modifies arr in place (partial sorting).
fn quick_select(arr: &mut [i32], k: usize) -> i32 {
    let mut lo = 0;
    let mut hi = arr.len() - 1;

    loop {
        if lo == hi {
            return arr[lo];
        }

        let pivot_idx = partition(arr, lo, hi);

        match k.cmp(&pivot_idx) {
            Ordering::Equal => return arr[pivot_idx],
            Ordering::Less => hi = pivot_idx - 1,
            Ordering::Greater => lo = pivot_idx + 1,
        }
    }
}

fn main() {
    let mut data = vec![7, 2, 5, 1, 8, 3, 6];
    let third = quick_select(&mut data, 2);
    println!("3rd smallest: {}", third); // Output: 3

    let mut data2 = vec![9, 1, 5, 3, 7];
    let median = quick_select(&mut data2, data2.len() / 2);
    println!("Median: {}", median); // Output: 5
}
```

**Time**: O(n) average. Each partition halves the problem (on average), so total work is
n + n/2 + n/4 + ... = 2n = O(n). Worst case is O(n^2) with bad pivots, same as quicksort.
Randomized pivot selection makes this vanishingly unlikely.

**Space**: O(1) extra for this iterative version.

This is the algorithm behind Rust's `select_nth_unstable()`. When an interviewer asks
"find the kth largest," quick select is the answer that beats sorting.

---

## Classic Problem 3: Maximum Subarray (D&C Approach)

### The Problem

Given an array of integers (some negative), find the contiguous subarray with the largest
sum.

```
Input:  [-2, 1, -3, 4, -1, 2, 1, -5, 4]
Answer: [4, -1, 2, 1] with sum 6
```

### The D&C Strategy

The maximum subarray either lies entirely in the left half, entirely in the right half, or
it **crosses the midpoint**. Handle all three:

```
         [-2, 1, -3, 4, -1, 2, 1, -5, 4]
                      |
          left half   |   right half
        [-2, 1, -3, 4]|[-1, 2, 1, -5, 4]
                      |
max subarray is one of:
  1. Entirely in left  -->  recurse
  2. Entirely in right -->  recurse
  3. Crossing the mid  -->  find directly in O(n)
```

To find the crossing subarray: expand outward from the midpoint in both directions, tracking
the best sum.

### Implementation

```rust
fn max_crossing_sum(arr: &[i32], mid: usize) -> i64 {
    // Expand left from mid (inclusive).
    let mut left_best: i64 = i64::MIN;
    let mut running: i64 = 0;
    for i in (0..=mid).rev() {
        running += arr[i] as i64;
        left_best = left_best.max(running);
    }

    // Expand right from mid+1.
    let mut right_best: i64 = i64::MIN;
    running = 0;
    for i in (mid + 1)..arr.len() {
        running += arr[i] as i64;
        right_best = right_best.max(running);
    }

    if mid + 1 >= arr.len() {
        return left_best; // no right half
    }
    left_best + right_best
}

fn max_subarray_dc(arr: &[i32]) -> i64 {
    if arr.is_empty() {
        return 0;
    }
    if arr.len() == 1 {
        return arr[0] as i64;
    }

    let mid = arr.len() / 2;
    let left_max = max_subarray_dc(&arr[..mid]);
    let right_max = max_subarray_dc(&arr[mid..]);
    let cross_max = max_crossing_sum(arr, mid - 1);

    left_max.max(right_max).max(cross_max)
}

fn main() {
    let data = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Max subarray sum (D&C): {}", max_subarray_dc(&data));
    // Output: 6
}
```

**Recurrence**: T(n) = 2T(n/2) + O(n) => **O(n log n)** by Master Theorem Case 2.

Note: Kadane's algorithm solves this in O(n) using a DP-like scan. The D&C version is slower
but illustrates the paradigm clearly, and it generalizes to variants (e.g., 2D max subarray)
where Kadane's does not directly apply. In interviews, you might be asked to solve it with
D&C explicitly as a follow-up.

---

## Classic Problem 4: Count Inversions

### The Problem

An **inversion** is a pair (i, j) where i < j but arr[i] > arr[j]. The inversion count
measures how "far from sorted" an array is: 0 for sorted, n*(n-1)/2 for reverse-sorted.

Brute force checks all pairs in O(n^2). D&C does it in O(n log n) by piggybacking on merge
sort.

### The Insight

During the merge step, every time we pick an element from the right half (because it is
smaller), that element is "jumping over" all remaining elements in the left half. Each such
jump represents an inversion.

```
Merging [3, 8, 12] and [2, 5, 10]:

  left:  [3, 8, 12]    right: [2, 5, 10]    output: []
          ^                     ^
  Take 2 from right. It jumps over 3, 8, 12 (3 remaining in left).
  Inversions += 3.        output: [2]

  left:  [3, 8, 12]    right: [2, 5, 10]    output: [2]
          ^                        ^
  Take 3 from left. No inversions counted.
                          output: [2, 3]

  left:  [3, 8, 12]    right: [2, 5, 10]    output: [2, 3]
              ^                    ^
  Take 5 from right. It jumps over 8, 12 (2 remaining).
  Inversions += 2.        output: [2, 3, 5]

  left:  [3, 8, 12]    right: [2, 5, 10]    output: [2, 3, 5]
              ^                        ^
  Take 8 from left. No inversions.
                          output: [2, 3, 5, 8]

  left:  [3, 8, 12]    right: [2, 5, 10]    output: [2, 3, 5, 8]
                  ^                    ^
  Take 10 from right. It jumps over 12 (1 remaining).
  Inversions += 1.        output: [2, 3, 5, 8, 10]

  Take 12 from left.     output: [2, 3, 5, 8, 10, 12]

  Total inversions from this merge: 3 + 2 + 1 = 6
```

### Implementation

```rust
/// Returns a sorted copy and the number of inversions.
fn count_inversions(arr: &[i32]) -> (Vec<i32>, u64) {
    let n = arr.len();
    if n <= 1 {
        return (arr.to_vec(), 0);
    }

    let mid = n / 2;
    let (left_sorted, left_inv) = count_inversions(&arr[..mid]);
    let (right_sorted, right_inv) = count_inversions(&arr[mid..]);

    // Merge and count split inversions.
    let mut merged = Vec::with_capacity(n);
    let mut split_inv: u64 = 0;
    let mut i = 0;
    let mut j = 0;

    while i < left_sorted.len() && j < right_sorted.len() {
        if left_sorted[i] <= right_sorted[j] {
            merged.push(left_sorted[i]);
            i += 1;
        } else {
            merged.push(right_sorted[j]);
            // Everything remaining in left_sorted[i..] forms an inversion
            // with right_sorted[j].
            split_inv += (left_sorted.len() - i) as u64;
            j += 1;
        }
    }

    merged.extend_from_slice(&left_sorted[i..]);
    merged.extend_from_slice(&right_sorted[j..]);

    (merged, left_inv + right_inv + split_inv)
}

fn main() {
    let data = [2, 4, 1, 3, 5];
    let (sorted, inversions) = count_inversions(&data);
    println!("Sorted: {:?}", sorted);
    println!("Inversions: {}", inversions);
    // Inversions: 3  -- the pairs (2,1), (4,1), (4,3)

    let reversed = [5, 4, 3, 2, 1];
    let (_, inv) = count_inversions(&reversed);
    println!("Reversed array inversions: {}", inv);
    // Output: 10 = 5*4/2
}
```

**Time**: O(n log n). It *is* merge sort with a counter bolted on.
**Space**: O(n) for the merge buffer.

This is a top interview question. The key recognition: "count pairs with an ordering
relationship" often maps to augmented merge sort.

---

## Classic Problem 5: Closest Pair of Points

### The Problem

Given n points in 2D, find the pair with the smallest Euclidean distance. Brute force is
O(n^2). D&C achieves O(n log n).

### The Algorithm

1. Sort points by x-coordinate (once, upfront).
2. Split at the median x into left and right halves.
3. Recursively find the closest pair in each half. Let d = min(d_left, d_right).
4. **Combine**: check whether any cross-half pair is closer than d. Only points within a
   vertical strip of width 2d need checking.

```
       d_left = 2.5     |      d_right = 3.0
                          |
     .    .               |         .
              .           |    .
          .               |       .      .
                          |
<---- left half --------> | <---- right half ---->
                          |
            |<--- d ----->|<--- d --->|
            +-------------+-----------+
             strip of candidates (2d wide)
```

The crucial geometric fact: within the strip, each point needs comparison against at most 7
others (sorted by y). You cannot pack more points in a d-by-2d rectangle with all pairwise
distances >= d.

### Implementation

```rust
#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn dist(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn closest_pair(points: &mut [Point]) -> f64 {
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    closest_pair_rec(points)
}

fn closest_pair_rec(points: &[Point]) -> f64 {
    let n = points.len();
    if n <= 3 {
        let mut best = f64::INFINITY;
        for i in 0..n {
            for j in (i + 1)..n {
                best = best.min(dist(&points[i], &points[j]));
            }
        }
        return best;
    }

    let mid = n / 2;
    let mid_x = points[mid].x;

    let d_left = closest_pair_rec(&points[..mid]);
    let d_right = closest_pair_rec(&points[mid..]);
    let mut d = d_left.min(d_right);

    // Build the strip.
    let mut strip: Vec<Point> = points
        .iter()
        .filter(|p| (p.x - mid_x).abs() < d)
        .copied()
        .collect();
    strip.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    // Check each point against the next few in the strip.
    for i in 0..strip.len() {
        let mut j = i + 1;
        while j < strip.len() && (strip[j].y - strip[i].y) < d {
            d = d.min(dist(&strip[i], &strip[j]));
            j += 1;
        }
    }

    d
}

fn main() {
    let mut pts = vec![
        Point { x: 2.0, y: 3.0 },
        Point { x: 12.0, y: 30.0 },
        Point { x: 40.0, y: 50.0 },
        Point { x: 5.0, y: 1.0 },
        Point { x: 12.0, y: 10.0 },
        Point { x: 3.0, y: 4.0 },
    ];

    let d = closest_pair(&mut pts);
    println!("Closest pair distance: {:.4}", d);
    // Output: 1.4142 (points (2,3) and (3,4), distance = sqrt(2))
}
```

**Recurrence**: T(n) = 2T(n/2) + O(n log n) for this version (strip sort). With a
pre-maintained y-sorted list, the combine step becomes O(n), giving T(n) = 2T(n/2) + O(n)
= **O(n log n)**.

---

## Classic Problem 6: Karatsuba Multiplication (Conceptual)

### The Problem

Multiply two n-digit numbers. The grade-school algorithm is O(n^2) digit multiplications.
Karatsuba (1960) showed you can do it in O(n^1.585) using D&C.

### The Idea

Split each number into high and low halves:

```
x = a * 10^n + b       (a = high digits, b = low digits)
y = c * 10^n + d

Naive expansion:
  x * y = ac * 10^(2n) + (ad + bc) * 10^n + bd
          4 half-size multiplications => T(n) = 4T(n/2) + O(n) => O(n^2)
          No improvement!

Karatsuba's trick:
  (a + b)(c + d) = ac + ad + bc + bd
  So: ad + bc = (a + b)(c + d) - ac - bd

  We already need ac and bd, so we get ad + bc with just ONE extra
  multiplication instead of two!

  Three multiplications total:
    p1 = a * c
    p2 = b * d
    p3 = (a + b) * (c + d)

  Result = p1 * 10^(2n) + (p3 - p1 - p2) * 10^n + p2
```

**Recurrence**: T(n) = 3T(n/2) + O(n). By Master Theorem: log_2(3) ~ 1.585 > d = 1,
so Case 3 applies: **O(n^1.585)**.

This seems like a modest improvement, but for 10,000-digit numbers it is significant.
Karatsuba multiplication is used in big-integer libraries and is the first step toward
even faster algorithms (Toom-Cook, FFT-based).

Here is a simplified demonstration using i64 (not a practical big-integer implementation,
but it shows the three-multiplication trick):

```rust
fn karatsuba(x: i64, y: i64) -> i64 {
    // Base case: small enough for direct multiplication.
    if x < 10 || y < 10 {
        return x * y;
    }

    let n = x.to_string().len().max(y.to_string().len());
    let half = n / 2;
    let power = 10_i64.pow(half as u32);

    let a = x / power;  // high digits of x
    let b = x % power;  // low digits of x
    let c = y / power;  // high digits of y
    let d = y % power;  // low digits of y

    // Three recursive multiplications instead of four.
    let p1 = karatsuba(a, c);
    let p2 = karatsuba(b, d);
    let p3 = karatsuba(a + b, c + d);

    p1 * power * power + (p3 - p1 - p2) * power + p2
}

fn main() {
    let x = 1234;
    let y = 5678;
    println!("{} * {} = {}", x, y, karatsuba(x, y));
    // Output: 1234 * 5678 = 7006652
    assert_eq!(karatsuba(x, y), x * y);
}
```

---

## D&C vs Dynamic Programming: The Key Differences

These two paradigms both break problems into subproblems, but they differ in a fundamental
way:

```
┌─────────────────────────────┬──────────────────────────────────┐
│     Divide and Conquer      │     Dynamic Programming          │
├─────────────────────────────┼──────────────────────────────────┤
│ Subproblems are INDEPENDENT.│ Subproblems OVERLAP -- the same  │
│ The left half and right     │ subproblem is needed by multiple │
│ half of merge sort share    │ parent problems.                 │
│ nothing.                    │                                  │
├─────────────────────────────┼──────────────────────────────────┤
│ Each subproblem is solved   │ Subproblems are solved once and  │
│ exactly once.               │ results are CACHED (memoized or  │
│                             │ stored in a table).              │
├─────────────────────────────┼──────────────────────────────────┤
│ Top-down recursion is the   │ Can be top-down (memoization)    │
│ natural structure.          │ or bottom-up (tabulation).       │
├─────────────────────────────┼──────────────────────────────────┤
│ No memoization needed       │ Memoization is essential -- it   │
│ because subproblems never   │ is what makes DP efficient.      │
│ repeat.                     │                                  │
├─────────────────────────────┼──────────────────────────────────┤
│ Examples: merge sort,       │ Examples: Fibonacci, knapsack,   │
│ quicksort, binary search,   │ longest common subsequence,      │
│ closest pair, Karatsuba     │ edit distance, coin change       │
└─────────────────────────────┴──────────────────────────────────┘
```

A useful litmus test: if you draw the recursion tree and the same subproblem appears in
multiple branches, you need DP (or memoization). If every subproblem is unique, D&C is
appropriate.

Consider Fibonacci vs merge sort:

```
Fibonacci -- subproblems OVERLAP:

  fib(5)
  ├── fib(4)
  │   ├── fib(3)       <-- appears here...
  │   │   ├── fib(2)   <-- and here...
  │   │   └── fib(1)
  │   └── fib(2)       <-- and here again
  └── fib(3)           <-- ...and here too
      ├── fib(2)
      └── fib(1)

  Same subproblems recomputed => use DP.
```

```
Merge sort -- subproblems are UNIQUE:

  sort([5,3,1,4,2])
  ├── sort([5,3,1])
  │   ├── sort([5,3])
  │   │   ├── sort([5])
  │   │   └── sort([3])
  │   └── sort([1])
  └── sort([4,2])
      ├── sort([4])
      └── sort([2])

  Every subproblem is a different slice. No overlap => use D&C.
```

The maximum subarray problem illustrates the boundary. D&C gives O(n log n), but the problem
has DP structure (the best subarray ending at position i depends on position i-1), so Kadane's
algorithm achieves O(n). When you spot overlap, switch from D&C to DP.

---

## Time and Space Complexity Summary

### Common Recurrences

```
Recurrence                  Solution           Algorithm Example
────────────────────────────────────────────────────────────────────
T(n) = T(n/2) + O(1)       O(log n)           Binary search
T(n) = T(n/2) + O(n)       O(n)               Quickselect (average)
T(n) = 2T(n/2) + O(1)      O(n)               Tree traversal
T(n) = 2T(n/2) + O(n)      O(n log n)         Merge sort, inversions
T(n) = 3T(n/2) + O(n)      O(n^1.585)         Karatsuba multiplication
T(n) = 7T(n/2) + O(n^2)    O(n^2.807)         Strassen matrix multiply
T(n) = 2T(n/2) + O(n^2)    O(n^2)             Some geometry problems
```

### Space in D&C Algorithms

Space comes from two sources:

1. **Recursion stack depth**: O(log n) for balanced splits, O(n) for degenerate cases.
2. **Auxiliary storage in the combine step**: O(n) for merge sort's buffer, O(n) for the
   closest-pair strip, etc.

Total space is the max of these. For balanced D&C, it is typically O(n) due to the combine
step, with O(log n) stack depth dominated.

---

## Common Interview Patterns

### Pattern 1: Augmented Merge Sort

Modify the merge step to count or track something. Signal: "count pairs with an ordering
relationship."

- Count inversions (covered above)
- Count of smaller numbers after self (LeetCode 315)
- Reverse pairs (LeetCode 493)

### Pattern 2: Recurse on Halves, Combine at Boundary

The answer is in the left, the right, or crosses the middle. Handle all three.

- Maximum subarray (covered above)
- The skyline problem (LeetCode 218)
- Different ways to add parentheses (LeetCode 241)

### Pattern 3: Recurse on One Side Only

Eliminate half the search space each step. Gets you O(n) or O(log n).

- Quick select (covered above)
- Median of two sorted arrays (LeetCode 4)
- Search in rotated sorted array

### Pattern 4: Binary Search on Answer + Verify

Binary search the answer space, verify each candidate. D&C spirit with iteration.

- Koko eating bananas (LeetCode 875)
- Split array largest sum (LeetCode 410)

### Pattern 5: Tree-Shaped Recursion

The input itself has recursive structure. Follow it.

- Construct quad-tree (LeetCode 427)
- Build balanced BST from sorted array
- Different ways to add parentheses (LeetCode 241)

---

## Practice Problems

### Easy (5 problems)

1. **Pow(x, n)** (LeetCode 50) -- fast exponentiation by squaring. x^n = (x^(n/2))^2.
2. **Maximum Subarray** (LeetCode 53) -- solve with D&C, then compare to Kadane's O(n).
3. **Majority Element** (LeetCode 169) -- D&C: the majority of the whole must be the
   majority of at least one half.
4. **Merge Sorted Array** (LeetCode 88) -- the combine step in isolation.
5. **Search a 2D Matrix** (LeetCode 74) -- binary search on a virtually flattened matrix.

### Medium (5 problems)

1. **Kth Largest Element** (LeetCode 215) -- quick select.
2. **Sort an Array** (LeetCode 912) -- implement merge sort from scratch.
3. **Count of Smaller Numbers After Self** (LeetCode 315) -- augmented merge sort.
4. **Different Ways to Add Parentheses** (LeetCode 241) -- split at each operator, recurse.
5. **The Skyline Problem** (LeetCode 218) -- merge two skylines in the combine step.

### Hard (5 problems)

1. **Reverse Pairs** (LeetCode 493) -- count pairs where arr[i] > 2*arr[j] for i < j.
2. **Median of Two Sorted Arrays** (LeetCode 4) -- O(log(min(m,n))) binary elimination.
3. **Count of Range Sum** (LeetCode 327) -- merge sort on prefix sums.
4. **Closest Pair of Points** -- implement the full O(n log n) algorithm.
5. **Create Sorted Array through Instructions** (LeetCode 1649) -- inversion counting
   variant using augmented merge sort.

---

## Exercises

**Exercise 1: Counting inversions from scratch.** Implement `count_inversions` without
looking at the code above. Test on: `[1, 2, 3, 4, 5]` (expect 0), `[5, 4, 3, 2, 1]`
(expect 10), `[2, 4, 1, 3, 5]` (expect 3).

<details>
<summary>Hint</summary>

The only change from merge sort: when you take from the right half during the merge, add
`left.len() - i` to your counter (where i is the current left index).

</details>

**Exercise 2: Quickselect with random pivot.** Implement quickselect with randomized pivot
selection. Verify that `quickselect(&mut [7, 2, 5, 1, 8, 3, 6], 0)` returns 1 and
`quickselect(&mut [7, 2, 5, 1, 8, 3, 6], 6)` returns 8.

<details>
<summary>Hint</summary>

Before partitioning, pick a random index in the range and swap it with the last element.
Then use Lomuto partition as usual. Adjust k when recursing right: the new k is
`k - pivot_idx - 1`.

</details>

**Exercise 3: Maximum subarray (D&C).** Implement the D&C solution and verify it handles
all-negative arrays correctly. Test on `[-2, 1, -3, 4, -1, 2, 1, -5, 4]` (expect 6) and
`[-1, -2, -3]` (expect -1).

<details>
<summary>Hint</summary>

The crossing-sum function scans left from mid and right from mid+1. Initialize running sum
at 0 and best at i64::MIN for each direction.

</details>

**Exercise 4: Master Theorem practice.** Determine the time complexity:

1. T(n) = 4T(n/2) + O(n)
2. T(n) = 4T(n/2) + O(n^2)
3. T(n) = 4T(n/2) + O(n^3)
4. T(n) = 3T(n/3) + O(n)

<details>
<summary>Answers</summary>

1. a=4, b=2, d=1, c=log_2(4)=2. d < c => Case 3: **O(n^2)**.
2. a=4, b=2, d=2, c=log_2(4)=2. d = c => Case 2: **O(n^2 log n)**.
3. a=4, b=2, d=3, c=log_2(4)=2. d > c => Case 1: **O(n^3)**.
4. a=3, b=3, d=1, c=log_3(3)=1. d = c => Case 2: **O(n log n)**.

</details>

**Exercise 5: Closest pair in 1D.** Implement closest pair for points on a line. Sort the
points, then the closest pair must be adjacent. That is O(n log n) from the sort plus O(n)
for the scan. Then think about why the 2D case needs the strip argument.

<details>
<summary>Hint</summary>

In 1D, after sorting, just scan adjacent pairs and track the minimum difference. The 2D case
is harder because "close in x" does not mean "close overall" -- you also need to check the
y-coordinate, which is why the strip sorted by y is needed.

</details>

---

## Key Takeaways

1. **Divide and conquer is a paradigm, not a single algorithm.** It is a way of thinking:
   break the problem down, solve the pieces, combine the results.

2. **The Master Theorem is your best friend for analysis.** T(n) = aT(n/b) + O(n^d). Compare
   d to log_b(a). Three cases. Memorize them.

3. **The combine step is where the magic happens.** Dividing is usually trivial. The creative
   work is in how you merge solutions back together.

4. **D&C is NOT the same as DP.** D&C has independent subproblems; DP has overlapping ones.
   If the same subproblem appears multiple times, you need memoization.

5. **Augmented merge sort is a powerful interview pattern.** Many counting problems reduce to
   modifying the merge step. Signal: "count pairs with an ordering constraint."

6. **Quick select gives O(n) average for selection problems.** When asked "find the kth
   largest," this beats sorting.

7. **Do not force D&C where a simpler solution exists.** Maximum subarray has an O(n) DP
   solution. Use D&C when it provides a genuine advantage or when explicitly required.

---

Next lesson: [32 - Intervals](./32_intervals.md)
