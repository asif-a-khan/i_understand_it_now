# Lesson 34: Sparse Tables

## The Problem: Static Range Queries, Answered Instantly

You have an array of numbers. It never changes. You will be asked, thousands or millions
of times: "What is the minimum value between index `left` and index `right`?"

Brute force scans every element in the range: O(n) per query. For q queries, O(n * q).

In [Lesson 15](./15_prefix_sum.md) we solved a similar problem for **sums** using prefix
sums: precompute in O(n), answer any range sum in O(1). But prefix sums only work for
operations with an inverse. Addition has subtraction: `sum(l..r) = prefix[r] - prefix[l]`.
Minimum has no inverse. You cannot "un-min" a value.

The sparse table gives us O(1) range minimum queries (and max, GCD, and similar) after
O(n log n) preprocessing. The catch: the data must be **static**. No updates. If you need
updates, use a segment tree. If your data is fixed, a sparse table is simpler and faster.

---

## The Core Idea: Precomputing Power-of-Two Ranges

### A Real-World Analogy

You manage daily sales data for 365 days. The CEO constantly asks: "What was the worst
sales day between day X and day Y?"

You pre-build summary reports at power-of-2 granularities:

- **1-day reports:** Each day's value (trivially, itself).
- **2-day reports:** The minimum for every pair of consecutive days.
- **4-day, 8-day, 16-day, ..., 256-day reports.**

When asked about days 50-83 (34 days), you grab two 32-day reports:

- 32-day report starting at day 50: covers days 50-81.
- 32-day report starting at day 52: covers days 52-83.

They overlap (days 52-81 counted twice), but for minimum that is fine:
`min(report_1, report_2)` gives the correct answer. Two lookups instead of 34 scans.

### Why "Sparse"?

We skip most range lengths. Only powers of 2 are precomputed: O(n log n) entries total,
versus O(n^2) for all possible ranges.

---

## The Sparse Table Grid

Given `arr = [3, 1, 4, 1, 5, 9, 2, 6]` (n = 8), cell `table[k][i]` stores the minimum
of `arr[i .. i + 2^k]`:

```
  Array:  index  0  1  2  3  4  5  6  7
                [3, 1, 4, 1, 5, 9, 2, 6]

  k=0 (len 1):  [3] [1] [4] [1] [5] [9] [2] [6]   each element itself
  k=1 (len 2):  [1] [1] [1] [1] [5] [2] [2]  -     min of every pair
  k=2 (len 4):  [1] [1] [1] [1] [2] [2]  -   -     min of every quad
  k=3 (len 8):  [1]  -   -   -   -   -   -   -     min of entire array
```

- `table[1][0] = 1`: min of `[3, 1]`
- `table[2][3] = 1`: min of `[1, 5, 9, 2]`
- Row `k` has `n - 2^k + 1` entries. Total rows: `floor(log2(n)) + 1`.

---

## Construction: O(n log n)

Dynamic programming. Base case: each element is its own 1-element range. Longer ranges
split into two halves:

```
table[k][i] = min(table[k-1][i], table[k-1][i + 2^(k-1)])
```

```
  Building table[2][1]  (min of arr[1..5], length 4):

  arr:  3  [1   4   1   5]  9   2   6
            |--half 1--|--half 2--|
            table[1][1]  table[1][3]
            = min(1,4)   = min(1,5)
            = 1          = 1

  table[2][1] = min(1, 1) = 1
```

Each cell: O(1). Total cells: O(n log n). Build time: **O(n log n)**.

---

## Answering a Query in O(1)

For query `[left, right]`, length `len = right - left + 1`, find `k = floor(log2(len))`.
Place two windows of size `2^k` -- one anchored at `left`, one anchored at `right`:

```
  Query: min of arr[2..=6]     len = 5, k = 2 (2^2 = 4)

  Window 1: arr[2..=5]  = [4, 1, 5, 9]   table[2][2] = 1
  Window 2: arr[3..=6]  = [1, 5, 9, 2]   table[2][3] = 1

  Array:    3   1  [4   1   5   9   2]  6
                    |--- window 1 ---|
                        |--- window 2 ---|
                    |---- overlap ----|

  Answer: min(1, 1) = 1
```

Two table lookups, one `min`. **O(1)** regardless of range size.

---

## Why Overlapping Is Fine (Idempotent Operations)

The overlap trick works because `min` is **idempotent**: `min(a, a) = a`. Double-counting
does not change the result. The same holds for:

- **max:** `max(a, a) = a`
- **gcd:** `gcd(a, a) = a`
- **bitwise AND / OR:** `a & a = a`, `a | a = a`

But **sum** is NOT idempotent: `a + a != a`. Overlapping windows would double-count.

An operation works with the O(1) overlap trick if it is:
1. **Associative:** `f(f(a, b), c) = f(a, f(b, c))`
2. **Idempotent:** `f(a, a) = a`

---

## The Rust Implementation

```rust
struct SparseTable {
    table: Vec<Vec<i32>>,
    log: Vec<usize>,
}

impl SparseTable {
    fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        assert!(n > 0, "array must be non-empty");

        // Precompute floor(log2(i)) for i in 0..=n
        let mut log = vec![0usize; n + 1];
        for i in 2..=n {
            log[i] = log[i / 2] + 1;
        }

        let max_k = log[n] + 1;
        let mut table = vec![vec![0i32; n]; max_k];

        // Base case: k = 0
        for i in 0..n {
            table[0][i] = arr[i];
        }

        // Fill rows k = 1, 2, ...
        for k in 1..max_k {
            let half = 1 << (k - 1);
            let limit = n + 1 - (1 << k);
            for i in 0..limit {
                table[k][i] = table[k - 1][i].min(table[k - 1][i + half]);
            }
        }

        SparseTable { table, log }
    }

    /// Returns the minimum of arr[left..=right] in O(1).
    fn query(&self, left: usize, right: usize) -> i32 {
        let len = right - left + 1;
        let k = self.log[len];
        self.table[k][left].min(self.table[k][right + 1 - (1 << k)])
    }
}

fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6];
    let st = SparseTable::new(&arr);

    println!("{}", st.query(2, 6)); // 1  (min of [4,1,5,9,2])
    println!("{}", st.query(4, 7)); // 2  (min of [5,9,2,6])
    println!("{}", st.query(0, 0)); // 3  (single element)
    println!("{}", st.query(0, 7)); // 1  (entire array)
}
```

**The `log` array:** `floor(log2(len))` via the recurrence `log[i] = log[i/2] + 1`.
Avoids floating-point precision traps.

**Memory:** O(n log n). For n = 1M with i32: ~80 MB. For a flat cache-friendly layout:

```rust
// flat_table[k * n + i] instead of table[k][i]
let flat_table: Vec<i32> = vec![0; n * max_k];
```

---

## Making It Generic (Min, Max, GCD, AND, OR)

```rust
struct SparseTable<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    table: Vec<Vec<T>>,
    log: Vec<usize>,
    combine: F,
}

impl<T, F> SparseTable<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    fn new(arr: &[T], combine: F) -> Self {
        let n = arr.len();
        assert!(n > 0);

        let mut log = vec![0usize; n + 1];
        for i in 2..=n {
            log[i] = log[i / 2] + 1;
        }
        let max_k = log[n] + 1;

        let mut table = vec![vec![arr[0]; n]; max_k];
        for i in 0..n {
            table[0][i] = arr[i];
        }
        for k in 1..max_k {
            let half = 1 << (k - 1);
            let limit = n + 1 - (1 << k);
            for i in 0..limit {
                table[k][i] = (combine)(table[k - 1][i], table[k - 1][i + half]);
            }
        }

        SparseTable { table, log, combine }
    }

    fn query(&self, left: usize, right: usize) -> T {
        let len = right - left + 1;
        let k = self.log[len];
        (self.combine)(self.table[k][left], self.table[k][right + 1 - (1 << k)])
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6];

    // Range minimum
    let rmq = SparseTable::new(&arr, |a: i32, b: i32| a.min(b));
    println!("min [2..=6]: {}", rmq.query(2, 6)); // 1

    // Range maximum
    let rmx = SparseTable::new(&arr, |a: i32, b: i32| a.max(b));
    println!("max [2..=6]: {}", rmx.query(2, 6)); // 9

    // Range GCD
    let rgcd = SparseTable::new(&arr, gcd);
    println!("gcd [0..=3]: {}", rgcd.query(0, 3)); // gcd(3,1,4,1) = 1

    // Range bitwise OR
    let ror = SparseTable::new(&arr, |a: i32, b: i32| a | b);
    println!("or [0..=7]: {}", ror.query(0, 7));  // 3|1|4|1|5|9|2|6 = 15
}
```

---

## Sparse Table vs. Segment Tree vs. Fenwick Tree

```
  +-----------------------+-------------------+-------------------+-------------------+
  |                       |  Sparse Table     |  Segment Tree     |  Fenwick Tree     |
  +-----------------------+-------------------+-------------------+-------------------+
  | Build time            |  O(n log n)       |  O(n)             |  O(n log n)       |
  | Query time            |  O(1)             |  O(log n)         |  O(log n)         |
  | Point update          |  not supported    |  O(log n)         |  O(log n)         |
  | Range update          |  not supported    |  O(log n) w/lazy  |  O(log n)*        |
  | Memory                |  O(n log n)       |  O(n)             |  O(n)             |
  | Implementation        |  simplest         |  moderate          |  simple           |
  | Supported operations  |  idempotent only  |  any associative  |  invertible only  |
  |   (for O(1) query)    |  (min,max,gcd)    |  (sum,min,etc.)   |  (sum,xor,etc.)   |
  +-----------------------+-------------------+-------------------+-------------------+

  * Fenwick range updates work only for invertible functions (sum, xor -- not min/max).
```

**Sparse table:** Static data, O(1) queries, idempotent ops. Best constant-time RMQ.

**Segment tree:** Dynamic data, O(log n) everything. Most versatile. Use when updates
are needed or the operation is not idempotent.

**Fenwick tree:** Dynamic prefix sums/XOR with point updates. Simplest code for
sum-with-updates. Cannot do min/max efficiently.

---

## Limitations: Static Arrays Only

Once built, the sparse table **cannot handle updates**. If `arr[5]` changes, every cell
whose range includes index 5 is stale, and there is no way to patch selectively.

Options when data changes:
1. Rebuild from scratch: O(n log n) per modification. Only if updates are very rare.
2. Switch to a segment tree: O(log n) update, O(log n) query.
3. Hybrid: rebuild between batches of updates.

If an interviewer adds updates to the problem, pivot to a segment tree immediately.

---

## Disjoint Sparse Tables (Brief Overview)

Standard sparse tables give O(1) queries **only** for idempotent operations. For
non-idempotent ops (sum, product), you must decompose ranges into disjoint power-of-2
blocks, yielding O(log n) per query.

**Disjoint Sparse Tables** are a variant that achieves **O(1) queries for any associative
operation** on static arrays -- including sum and product. The key difference: instead of
overlapping windows, they organize the array into a hierarchy of non-overlapping blocks
and store prefix/suffix aggregates within each block.

```
  Standard (overlapping -- idempotent only):
    [---window 1---]
         [---window 2---]
         ^^^overlap^^^

  Disjoint (no overlap -- any associative op):
    [---- block A ----][---- block B ----]
    suffixes -->  <-- prefixes
    Query [l, r] = suffix_A(l) OP prefix_B(r)    O(1)
```

A query finds the correct hierarchy level using `level = floor(log2(left XOR right))`,
then combines one suffix lookup with one prefix lookup. Same O(n log n) build/space.

**When to use:** You need O(1) range queries for a non-invertible, non-idempotent
operation (e.g., product with zeros, matrix multiplication over ranges). For sum, prefer
plain prefix sums. For most interview problems, standard sparse tables or segment trees
suffice.

---

## LCA Reduction to RMQ

A classic application: Lowest Common Ancestor queries on trees reduce to RMQ.

1. Euler tour the tree, recording depth at each step.
2. LCA(u, v) = the node with minimum depth between first occurrences of u and v.
3. That is a range minimum query -- O(1) with a sparse table.

Result: O(n log n) preprocessing, O(1) per LCA query.

---

## Complexity Summary

```
  +---------------------+----------------+
  | Operation           | Time           |
  +---------------------+----------------+
  | Build               | O(n log n)     |
  | Query (idempotent)  | O(1)           |
  | Query (general)     | O(log n)       |
  | Update              | not supported  |
  | Space               | O(n log n)     |
  +---------------------+----------------+
```

---

## Common Pitfalls

1. **Off-by-one in the query.** `right + 1 - (1 << k)` is the second window's start.
   That window covers `[right + 1 - 2^k, right]` -- exactly `2^k` elements.

2. **Using it for sum.** The O(1) overlap trick does NOT work for sum. You will
   double-count the overlap region.

3. **Forgetting it is static.** No efficient patching exists. Rebuild or use a segment
   tree.

4. **Integer overflow in `1 << k`.** Use `1usize << k`, not `1i32 << k` (overflows at
   k = 31).

5. **Log table edge cases.** `log[1] = 0` (since 2^0 = 1). `log[0]` is never used in a
   valid query but initialize it to 0 to avoid surprises.

---

## Key Takeaways

1. Sparse tables precompute answers for all power-of-2 ranges: O(n log n) cells.
2. Any range is covered by two overlapping power-of-2 windows.
3. For idempotent operations (min, max, gcd, AND, OR), overlap is fine -- O(1) queries.
4. Construction is DP: each row builds from the previous row.
5. No updates. Static data only.
6. Precomputed integer log table avoids floating-point math.
7. Disjoint sparse tables extend O(1) to non-idempotent ops, but are rarely needed.

---

## Practice Problems

### Easy

1. **Static Range Minimum Queries** (CSES 1647) -- Direct application of the lesson.
   Build a sparse table, answer each query in O(1).

2. **Range Minimum Query** (SPOJ RMQSQ) -- Classic RMQ. Given array and Q queries,
   output the min for each range.

3. **Sliding Window Maximum** (LeetCode 239) -- Build a max sparse table. Query each
   window `[i, i+k-1]` in O(1). (The deque approach is O(n), but this is good practice.)

4. **Range GCD Queries** (CF Educational Rounds / custom) -- Build a sparse table with
   `gcd`. Validates the generic implementation.

5. **Static Range Queries** (Kattis variants) -- Large n, large Q. Confirms your O(1)
   query does not TLE.

### Medium

6. **LCA via Euler Tour + Sparse Table** (SPOJ LCA / CF 1328E) -- Flatten tree, build
   RMQ on depth array, answer LCA in O(1).

7. **CGCDSSQ** (CF 475D) -- Count pairs (l, r) with a given range GCD. Combine sparse
   table with two-pointer / binary search.

8. **Maximum of Minimums over Segments** (CF 872B variant) -- Divide array into k
   contiguous segments, maximize the minimum. Binary search + sparse table validation.

9. **Range AND Queries with Counting** -- Build AND sparse table. For each query, count
   how many subarrays have AND equal to a target. Sparse table + binary search on
   monotone AND values.

10. **Longest Subarray With GCD = 1** (CF 891A -- Pride) -- Range GCD sparse table. For
    each start, binary-search for the first position where GCD drops to 1.

### Hard

11. **Cartesian Tree via RMQ** -- Build a Cartesian tree by recursively finding range
    minimums. With sparse table, each step is O(1), total O(n log n).

12. **Suffix Array LCP + Sparse Table** -- Build suffix array, compute LCP array, build
    sparse table on LCP. Answer "longest common prefix of two suffixes" in O(1). Key
    subroutine in string matching problems.

13. **2D Sparse Table for Submatrix Min** (CF 713D) -- Extend to 2D:
    `table[kx][ky][i][j]` = min of `2^kx x 2^ky` submatrix. Query with four overlapping
    blocks. Build: O(nm log(n) log(m)).

14. **Minimum Stack-Sortable Partitions** (CF 1175F) -- Advanced problem where RMQ is a
    subroutine. Partition array into minimum subsequences sortable by a stack.

15. **Offline RMQ with Coordinate Compression** -- Indices up to 10^9 but only N values.
    Compress, build sparse table on compressed array, answer queries offline.

---

*Sparse tables occupy a precise niche: static range queries for idempotent operations,
answered in O(1). Memorize the build loop, the query formula, and the log table trick.
That is all you need for O(1) RMQ in any interview or contest.*
