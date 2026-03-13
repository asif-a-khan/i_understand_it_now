# Lesson 33: Segment Trees & Fenwick Trees (Binary Indexed Trees)

## The Problem: Dynamic Range Queries

In [Lesson 15](./15_prefix_sum.md) you learned that a prefix sum array lets you answer
"what is the sum from index `left` to `right`?" in O(1) after an O(n) build step. That
is spectacular -- until someone changes one of the original values.

If `nums[3]` changes from 5 to 8, your entire prefix sum array from index 4 onward is
stale. Rebuilding it costs O(n). If you are handling q updates interleaved with q
queries, the total cost is O(n * q). For n = 100,000 and q = 100,000, that is 10 billion
operations. We need a structure that handles both queries *and* updates efficiently.

This is the **dynamic range query problem**: given an array, support two operations:

1. **Query(left, right):** Return the sum (or min, or max) of elements in `nums[left..=right]`.
2. **Update(index, value):** Change `nums[index]` to a new value.

Both operations need to be fast -- ideally O(log n).

Two data structures solve this elegantly: the **segment tree** and the **Fenwick tree**
(also called Binary Indexed Tree, or BIT). This lesson covers both.

---

## When You Will See These

These are not obscure academic curiosities. They appear in:

- **Competitive programming:** Probably the single most common "advanced" data structure
  in contest problems. If you see "answer q range queries with updates," reach for these.
- **Coding interviews:** Rare in standard FAANG interviews, but they show up in companies
  that value algorithmic depth (quant firms, game engines, database internals roles).
- **Real systems:** Database query engines use segment-tree-like structures for range
  aggregation. Pixel shaders use them for prefix operations on GPU. Network routers use
  BIT-like structures for traffic counters.

---

## Part 1: Segment Trees

### The Analogy: Regional Sales Managers

Imagine a retail chain with 8 stores. The CEO wants to know total sales for any
contiguous range of stores at any time, and individual store sales get updated throughout
the day.

One approach: the CEO calls every store in the range and adds up the numbers. Slow.

Better approach: set up a hierarchy of regional managers.

```
                         [CEO: stores 0-7]
                        /                 \
           [West: 0-3]                    [East: 4-7]
           /          \                   /          \
     [NW: 0-1]    [SW: 2-3]       [NE: 4-5]    [SE: 6-7]
      /    \        /    \          /    \        /    \
  [S0]    [S1]  [S2]    [S3]    [S4]    [S5]  [S6]    [S7]
```

Each manager knows the total sales for their region. The NW manager tracks stores 0-1.
The West VP tracks stores 0-3 (sum of NW and SW). The CEO tracks all 8.

**Query "total sales for stores 2-5":** The CEO does not call all four stores. Instead:
the SW manager reports stores 2-3, the NE manager reports stores 4-5. Two lookups,
one addition. Done.

**Update "store 3 sold another $100":** Update store 3, then tell SW, then tell West,
then tell the CEO. Four updates total (the height of the tree), not eight.

That hierarchy *is* a segment tree. Each node stores a precomputed aggregate for a
contiguous segment of the array. Queries and updates follow paths through the tree,
touching O(log n) nodes.

### The Structure

A segment tree for an array of size n is a binary tree where:

- Each **leaf** corresponds to one element of the array.
- Each **internal node** stores the aggregate (sum, min, max, etc.) of its children's
  segments.
- The **root** covers the entire array `[0, n-1]`.

For the array `[2, 1, 5, 3, 4, 7, 2, 6]`:

```
  Array indices:    0   1   2   3   4   5   6   7
  Array values:    [2,  1,  5,  3,  4,  7,  2,  6]

  Segment tree (sum):

                            [30]                     <- sum of [0..7]
                          /      \
                    [11]              [19]            <- [0..3], [4..7]
                   /    \            /    \
               [3]       [8]     [11]     [8]        <- [0..1],[2..3],[4..5],[6..7]
              / \       / \      / \      / \
            [2] [1]   [5] [3] [4] [7]  [2] [6]      <- individual elements
```

The tree has at most 2n nodes (approximately 4n if we use a flat array with 1-indexing).
The height is O(log n).

### Array-Based Storage

We store the segment tree in a flat array, similar to how heaps work (Lesson 18). For a
1-indexed array:

- Node `i`'s left child is at `2*i`.
- Node `i`'s right child is at `2*i + 1`.
- Node `i`'s parent is at `i / 2`.
- The root is at index 1.

We allocate an array of size `4 * n` to be safe (the exact size depends on n, but 4n
always works).

```
  Index:   1    2    3    4    5    6    7    8    9   10   11   12   13   14   15
  Value: [30] [11] [19]  [3]  [8] [11]  [8]  [2]  [1]  [5]  [3]  [4]  [7]  [2]  [6]
          |    |    |     |    |    |    |     |    |    |    |    |    |    |    |
         root  --------internal--------      ---------------leaves---------------
```

### Visual: Query(2, 5) Walkthrough

```
  Query sum of arr[2..=5] on arr = [2, 1, 5, 3, 4, 7, 2, 6]
  Expected: 5 + 3 + 4 + 7 = 19

                            [30] node=1, [0,7]
                          /      \               partial overlap -> recurse
                    [11]              [19]
                  node=2,[0,3]      node=3,[4,7]
                   /    \            /    \
               [3]       [8]     [11]     [8]
             [0,1]      [2,3]   [4,5]    [6,7]

  node=1 [0,7]: partial overlap with [2,5] -> recurse both children
    node=2 [0,3]: partial overlap with [2,5] -> recurse both children
      node=4 [0,1]: no overlap with [2,5] -> return 0
      node=5 [2,3]: total overlap (2<=2 and 3<=5) -> return 8
    node=3 [4,7]: partial overlap with [2,5] -> recurse both children
      node=6 [4,5]: total overlap (2<=4 and 5<=5) -> return 11
      node=7 [6,7]: no overlap with [2,5] -> return 0

  Result: 0 + 8 + 11 + 0 = 19

  Nodes visited: 7 out of 15. In general, O(log n) nodes on each "boundary"
  side of the query range, so O(log n) total.
```

### Visual: Update(3, 10) Walkthrough

```
  Change arr[3] from 3 to 10.  Difference: +7.

  Before:                              After:
          [30]                                 [37]          +7
         /    \                               /    \
     [11]      [19]                       [18]      [19]    +7 on left
     /  \      /  \                       /  \      /  \
   [3]  [8] [11] [8]                   [3] [15] [11] [8]   +7 on node 5
   /\   /\   /\   /\                   /\   /\   /\   /\
  2  1 5  3 4  7 2  6                 2  1 5 10 4  7 2  6  leaf updated
              ^                                ^
              index 3                          index 3

  Only nodes on the path from leaf to root are touched: 4 nodes = O(log 8) = O(3).
```

### Full Rust Implementation: Segment Tree (Generic)

```rust
/// A segment tree supporting point updates and range queries.
/// Generic over any associative binary operation with an identity element.
pub struct SegTree<T, F> {
    n: usize,
    tree: Vec<T>,
    identity: T,
    combine: F,
}

impl<T, F> SegTree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    /// Build a segment tree from a slice.
    /// `identity` is the neutral element (0 for sum, i64::MAX for min, etc.).
    /// `combine` is the merge function (add for sum, min for min, etc.).
    pub fn new(data: &[T], identity: T, combine: F) -> Self {
        let n = data.len();
        let mut st = SegTree {
            n,
            tree: vec![identity; 4 * n],
            identity,
            combine,
        };
        if n > 0 {
            st.build(data, 1, 0, n - 1);
        }
        st
    }

    fn build(&mut self, data: &[T], node: usize, lo: usize, hi: usize) {
        if lo == hi {
            self.tree[node] = data[lo];
            return;
        }
        let mid = lo + (hi - lo) / 2;
        self.build(data, 2 * node, lo, mid);
        self.build(data, 2 * node + 1, mid + 1, hi);
        self.tree[node] = (self.combine)(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    /// Point update: set the value at `idx` to `val`.
    pub fn update(&mut self, idx: usize, val: T) {
        self.update_impl(1, 0, self.n - 1, idx, val);
    }

    fn update_impl(&mut self, node: usize, lo: usize, hi: usize, idx: usize, val: T) {
        if lo == hi {
            self.tree[node] = val;
            return;
        }
        let mid = lo + (hi - lo) / 2;
        if idx <= mid {
            self.update_impl(2 * node, lo, mid, idx, val);
        } else {
            self.update_impl(2 * node + 1, mid + 1, hi, idx, val);
        }
        self.tree[node] = (self.combine)(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    /// Query the aggregate over the range [ql..=qr].
    pub fn query(&self, ql: usize, qr: usize) -> T {
        self.query_impl(1, 0, self.n - 1, ql, qr)
    }

    fn query_impl(&self, node: usize, lo: usize, hi: usize, ql: usize, qr: usize) -> T {
        if qr < lo || hi < ql {
            return self.identity;
        }
        if ql <= lo && hi <= qr {
            return self.tree[node];
        }
        let mid = lo + (hi - lo) / 2;
        let left = self.query_impl(2 * node, lo, mid, ql, qr);
        let right = self.query_impl(2 * node + 1, mid + 1, hi, ql, qr);
        (self.combine)(left, right)
    }
}

fn main() {
    let data = vec![2i64, 1, 5, 3, 4, 7, 2, 6];

    // --- Range Sum ---
    let mut sum_tree = SegTree::new(&data, 0, |a, b| a + b);
    assert_eq!(sum_tree.query(2, 5), 19);  // 5+3+4+7
    sum_tree.update(3, 10);                 // arr[3] = 10
    assert_eq!(sum_tree.query(2, 5), 26);  // 5+10+4+7
    assert_eq!(sum_tree.query(0, 7), 37);  // total

    // --- Range Min ---
    let min_tree = SegTree::new(&data, i64::MAX, |a, b| a.min(b));
    assert_eq!(min_tree.query(2, 5), 3);   // min(5,3,4,7)
    assert_eq!(min_tree.query(0, 7), 1);   // min of all

    // --- Range Max ---
    let max_tree = SegTree::new(&data, i64::MIN, |a, b| a.max(b));
    assert_eq!(max_tree.query(2, 5), 7);   // max(5,3,4,7)

    println!("All segment tree assertions passed.");
}
```

---

## Lazy Propagation: Range Updates in O(log n)

So far, our segment tree supports **point updates** (change one element) and **range
queries**. But what if you need **range updates** -- "add 5 to every element from index 2
to index 6"?

With point updates, that would cost O(k log n) where k is the size of the range. Lazy
propagation brings it down to O(log n) per range update.

### The Idea

When you update a range, do not push the change all the way down to every leaf immediately.
Instead, mark the internal node with a **lazy** tag that says "everything below me has a
pending update I have not applied yet." Only push the update down when a future query or
update actually needs to look at those children.

Think of it like a manager who receives a policy change affecting all their stores. Instead
of immediately calling every store, they write a sticky note on their desk: "pending: add
$5 to all stores." If someone later asks about a specific store, *then* the manager
forwards the memo to the relevant sub-managers.

```
  Range update: add 5 to [2..5]

  Before push-down, node 5 [2..3] stores:
    value = 8 (sum of range), lazy = 5

  When we later query or update within [2..3], we push down:
    node 10 [2..2]: value += 5, becomes 10
    node 11 [3..3]: value += 5, becomes 8
    node 5 lazy reset to 0
```

### Rust Implementation: Lazy Segment Tree

```rust
/// Segment tree with lazy propagation for range-add, range-sum queries.
pub struct LazySegTree {
    n: usize,
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegTree {
    pub fn new(data: &[i64]) -> Self {
        let n = data.len();
        let mut st = LazySegTree {
            n,
            tree: vec![0; 4 * n],
            lazy: vec![0; 4 * n],
        };
        if n > 0 {
            st.build(data, 1, 0, n - 1);
        }
        st
    }

    fn build(&mut self, data: &[i64], node: usize, lo: usize, hi: usize) {
        if lo == hi {
            self.tree[node] = data[lo];
            return;
        }
        let mid = lo + (hi - lo) / 2;
        self.build(data, 2 * node, lo, mid);
        self.build(data, 2 * node + 1, mid + 1, hi);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    /// Push pending lazy value down to children.
    fn push_down(&mut self, node: usize, lo: usize, hi: usize) {
        if self.lazy[node] != 0 {
            let mid = lo + (hi - lo) / 2;
            self.apply(2 * node, lo, mid, self.lazy[node]);
            self.apply(2 * node + 1, mid + 1, hi, self.lazy[node]);
            self.lazy[node] = 0;
        }
    }

    /// Apply a pending addition to a node.
    fn apply(&mut self, node: usize, lo: usize, hi: usize, val: i64) {
        self.tree[node] += val * (hi - lo + 1) as i64;
        self.lazy[node] += val;
    }

    /// Add `val` to every element in [ql..=qr].
    pub fn range_update(&mut self, ql: usize, qr: usize, val: i64) {
        self.range_update_impl(1, 0, self.n - 1, ql, qr, val);
    }

    fn range_update_impl(
        &mut self, node: usize, lo: usize, hi: usize,
        ql: usize, qr: usize, val: i64,
    ) {
        if qr < lo || hi < ql {
            return;
        }
        if ql <= lo && hi <= qr {
            self.apply(node, lo, hi, val);
            return;
        }
        self.push_down(node, lo, hi);
        let mid = lo + (hi - lo) / 2;
        self.range_update_impl(2 * node, lo, mid, ql, qr, val);
        self.range_update_impl(2 * node + 1, mid + 1, hi, ql, qr, val);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    /// Query the sum of elements in [ql..=qr].
    pub fn range_query(&mut self, ql: usize, qr: usize) -> i64 {
        self.range_query_impl(1, 0, self.n - 1, ql, qr)
    }

    fn range_query_impl(
        &mut self, node: usize, lo: usize, hi: usize,
        ql: usize, qr: usize,
    ) -> i64 {
        if qr < lo || hi < ql {
            return 0;
        }
        if ql <= lo && hi <= qr {
            return self.tree[node];
        }
        self.push_down(node, lo, hi);
        let mid = lo + (hi - lo) / 2;
        self.range_query_impl(2 * node, lo, mid, ql, qr)
            + self.range_query_impl(2 * node + 1, mid + 1, hi, ql, qr)
    }
}

fn main() {
    let data = vec![1i64, 3, 5, 7, 9, 11];
    let mut st = LazySegTree::new(&data);

    assert_eq!(st.range_query(1, 3), 15);  // 3+5+7

    st.range_update(1, 4, 10);              // add 10 to indices 1..=4
    assert_eq!(st.range_query(1, 3), 45);  // 13+15+17
    assert_eq!(st.range_query(2, 2), 15);  // point query: arr[2] after update

    println!("All lazy segment tree assertions passed.");
}
```

Notice that `range_query` takes `&mut self` -- because it may need to push lazy values
down during the query. This is the same kind of "mutation during read" that path
compression causes in Union-Find ([Lesson 27](./27_union_find.md)).

---

## Part 2: Fenwick Tree (Binary Indexed Tree)

The segment tree is powerful but verbose. For one specific (and very common) use case --
**prefix sums with point updates** -- there is a simpler structure that uses less memory,
has smaller constant factors, and is easier to code from memory.

This is the Fenwick tree, invented by Peter Fenwick in 1994, also called a Binary Indexed
Tree (BIT).

### The Core Insight: Binary Decomposition of Indices

The Fenwick tree exploits the binary representation of indices to create an implicit tree.
Every positive integer decomposes into powers of 2. The Fenwick tree uses this to break
prefix sums into O(log n) precomputed partial sums.

For any index `i` (1-indexed), define `lowbit(i)` as the lowest set bit of `i`. Each
position `i` in the Fenwick array stores the sum of `lowbit(i)` elements ending at `i`.

```
  i     binary    lowbit(i)    bit[i] stores sum of range
  ----------------------------------------------------------------
  1     0001      1            [1..1]   (1 element)
  2     0010      2            [1..2]   (2 elements)
  3     0011      1            [3..3]   (1 element)
  4     0100      4            [1..4]   (4 elements)
  5     0101      1            [5..5]   (1 element)
  6     0110      2            [5..6]   (2 elements)
  7     0111      1            [7..7]   (1 element)
  8     1000      8            [1..8]   (8 elements)
```

### Computing lowbit: The `i & (-i)` Trick

In two's complement, `-i` is the bitwise complement of `i` plus 1. AND-ing `i` with `-i`
isolates the lowest set bit:

```
  i = 12 = 0b1100
 -i      = 0b0100  (two's complement)
  i & -i = 0b0100 = 4

  i = 6  = 0b0110
 -i      = 0b1010
  i & -i = 0b0010 = 2

  i = 7  = 0b0111
 -i      = 0b1001
  i & -i = 0b0001 = 1
```

In Rust with unsigned types: `i & i.wrapping_neg()`.

### Fenwick Tree Visual

```
  Array (1-indexed): [_, 2, 1, 5, 3, 4, 7, 2, 6]
                         1  2  3  4  5  6  7  8

  BIT array:
    bit[1] = 2          covers [1..1]  = arr[1]
    bit[2] = 3          covers [1..2]  = arr[1] + arr[2]
    bit[3] = 5          covers [3..3]  = arr[3]
    bit[4] = 11         covers [1..4]  = arr[1]+arr[2]+arr[3]+arr[4]
    bit[5] = 4          covers [5..5]  = arr[5]
    bit[6] = 11         covers [5..6]  = arr[5] + arr[6]
    bit[7] = 2          covers [7..7]  = arr[7]
    bit[8] = 30         covers [1..8]  = sum of all

  Ranges visualized:

  Index:  1    2    3    4    5    6    7    8
          |    |    |    |    |    |    |    |
  bit[1]: [1]  .    .    .    .    .    .    .
  bit[2]: [1---2]   .    .    .    .    .    .
  bit[3]:  .   .   [3]   .    .    .    .    .
  bit[4]: [1---2----3----4]   .    .    .    .
  bit[5]:  .   .    .    .   [5]   .    .    .
  bit[6]:  .   .    .    .   [5---6]   .    .
  bit[7]:  .   .    .    .    .    .   [7]   .
  bit[8]: [1---2----3----4----5----6----7----8]

  Implicit tree (update propagation, child -> parent):

  Level 3:                         bit[8]
                                  /  |   \   \
  Level 2:          bit[4]          bit[6]   |
                   /     |           |       |
  Level 1:   bit[2]      |        bit[5]     |
              |          |                   |
  Level 0: bit[1]     bit[3]              bit[7]
```

### Query Walkthrough: prefix_sum(7)

```
  prefix_sum(7):
    i = 7 (0b0111) -> add bit[7] = 2,   then i = 7 - 1 = 6
    i = 6 (0b0110) -> add bit[6] = 11,  then i = 6 - 2 = 4
    i = 4 (0b0100) -> add bit[4] = 11,  then i = 4 - 4 = 0
    i = 0 -> stop.

    Total = 2 + 11 + 11 = 24
    Check: 2 + 1 + 5 + 3 + 4 + 7 + 2 = 24
```

### Update Walkthrough: add 10 to position 3

```
  update(3, +10):
    i = 3 (0b0011) -> bit[3] += 10,  then i = 3 + 1 = 4
    i = 4 (0b0100) -> bit[4] += 10,  then i = 4 + 4 = 8
    i = 8 (0b1000) -> bit[8] += 10,  then i = 8 + 8 = 16
    i = 16 > n -> stop.

  Only 3 cells updated = O(log n).
```

### Full Rust Implementation: Fenwick Tree

```rust
/// A Fenwick tree (Binary Indexed Tree) for prefix sums.
/// Uses 1-based indexing internally, 0-based API externally.
pub struct FenwickTree {
    n: usize,
    bit: Vec<i64>,
}

impl FenwickTree {
    /// Create a Fenwick tree from a 0-indexed slice. O(n) build.
    pub fn new(data: &[i64]) -> Self {
        let n = data.len();
        let mut bit = vec![0i64; n + 1];
        // Copy data into 1-indexed positions.
        for i in 0..n {
            bit[i + 1] = data[i];
        }
        // Propagate each position's value to its parent.
        for i in 1..=n {
            let parent = i + (i & i.wrapping_neg());
            if parent <= n {
                bit[parent] += bit[i];
            }
        }
        FenwickTree { n, bit }
    }

    /// Add `delta` to the element at 0-indexed position `idx`.
    pub fn add(&mut self, idx: usize, delta: i64) {
        let mut i = idx + 1; // convert to 1-indexed
        while i <= self.n {
            self.bit[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    /// Prefix sum: arr[0] + arr[1] + ... + arr[idx] (0-indexed, inclusive).
    pub fn prefix_sum(&self, idx: usize) -> i64 {
        let mut sum = 0i64;
        let mut i = idx + 1; // convert to 1-indexed
        while i > 0 {
            sum += self.bit[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }

    /// Range sum: arr[left] + ... + arr[right] (0-indexed, inclusive).
    pub fn range_sum(&self, left: usize, right: usize) -> i64 {
        if left == 0 {
            self.prefix_sum(right)
        } else {
            self.prefix_sum(right) - self.prefix_sum(left - 1)
        }
    }
}

fn main() {
    let data = vec![2i64, 1, 5, 3, 4, 7, 2, 6];
    let mut ft = FenwickTree::new(&data);

    // Prefix sums (0-indexed)
    assert_eq!(ft.prefix_sum(3), 11);   // 2+1+5+3
    assert_eq!(ft.prefix_sum(7), 30);   // entire array

    // Range sum
    assert_eq!(ft.range_sum(2, 5), 19); // 5+3+4+7

    // Point update: add 7 to index 3 (changing arr[3] from 3 to 10)
    ft.add(3, 7);
    assert_eq!(ft.prefix_sum(3), 18);   // 2+1+5+10
    assert_eq!(ft.range_sum(2, 5), 26); // 5+10+4+7

    println!("All Fenwick tree assertions passed.");
}
```

---

## Segment Tree vs. Fenwick Tree: When to Use Which

| Criterion | Segment Tree | Fenwick Tree |
|-----------|-------------|-------------|
| **Code complexity** | ~50-70 lines | ~25 lines |
| **Memory** | ~4n | ~n |
| **Constant factor** | Larger (recursion overhead) | Smaller (tight bit loops) |
| **Supported operations** | Any associative op (sum, min, max, GCD, XOR) | Only invertible ops (sum, XOR) |
| **Point update** | O(log n) | O(log n) |
| **Range query** | O(log n) | O(log n) via prefix subtraction |
| **Range update** | O(log n) with lazy | O(log n) with two-BIT trick |
| **Range min/max** | Yes | No (min/max have no inverse) |
| **Lazy propagation** | Natural | Not standard |
| **Ease of memorization** | Harder | Easier (great for contests) |
| **Cache performance** | Moderate | Better (sequential access) |

**Rules of thumb:**

1. **Problem only needs prefix sums with point updates:** Fenwick tree. Simpler, faster,
   fewer bugs.

2. **Need range minimum or maximum:** Segment tree. Fenwick trees cannot do this.

3. **Need range updates AND range queries:** Segment tree with lazy propagation.

4. **In a contest under time pressure:** Fenwick tree if possible. You can code it in
   2 minutes from memory.

5. **Need persistent queries (old versions):** Persistent segment tree. Fenwick trees
   are very hard to make persistent.

---

## Complexity Summary

| Approach | Build | Point Update | Range Query | Range Update | Space |
|----------|:-----:|:------------:|:-----------:|:------------:|:-----:|
| Brute force array | O(1) | O(1) | O(n) | O(n) | O(n) |
| Prefix sum array | O(n) | O(n) rebuild | O(1) | O(n) rebuild | O(n) |
| Fenwick tree | O(n) | O(log n) | O(log n) | O(log n)* | O(n) |
| Segment tree | O(n) | O(log n) | O(log n) | O(log n)** | O(4n) |
| Segment tree + lazy | O(n) | O(log n) | O(log n) | O(log n) | O(4n) x 2 |

\* With the two-BIT trick for range-add + range-sum.
\** Naive: O(k log n) where k is range size. O(log n) only with lazy propagation.

### Why O(log n)?

- **Segment tree:** The tree has O(log n) levels. At each level, at most 2 nodes have
  partial overlap with the query range (the left and right boundaries). Total work per
  query or update: O(log n).

- **Fenwick tree:** Each query strips one bit per iteration. A number up to n has at
  most log2(n) bits. Each update adds one bit per iteration with the same bound.

---

## Common Use Cases

### 1. Range Sum / Range Min / Range Max with Updates
The textbook use case. Segment tree handles all three; Fenwick tree handles sums only.

### 2. Inversion Count
An inversion is a pair (i, j) where i < j but arr[i] > arr[j]. Process elements left to
right. For each element, query "how many previously seen elements are greater?" using a
Fenwick tree indexed by value. O(n log n) total.

### 3. Counting Elements in a Range
"How many elements in arr[l..r] fall within [lo, hi]?" Use a merge sort tree (segment
tree where each node stores a sorted list) or offline with a Fenwick tree + coordinate
compression.

### 4. Dynamic Order Statistics
"What is the k-th smallest element in the current set?" Use a segment tree over the
value domain. Insert/delete by updating leaves. Walk the tree to find the k-th element.

### 5. 2D Range Queries
Nest a Fenwick tree inside another for 2D prefix sums with point updates. Both update and
query cost O(log^2 n).

### 6. Interval Overlap Counting
Track the maximum number of overlapping intervals at any point using a segment tree with
range-add (lazy propagation) for interval insertion/deletion.

### 7. Offline Query Processing
Sort queries by right endpoint and sweep left-to-right, maintaining a Fenwick tree. This
pattern solves many "count distinct elements in range" problems.

---

## Common Pitfalls

1. **Off-by-one: 0-indexed vs 1-indexed.** Fenwick trees must be 1-indexed because
   `lowbit(0) = 0` causes an infinite loop. Expose a 0-indexed API but convert internally.

2. **Wrong identity element.** Sum uses 0. Min uses `i64::MAX`. Max uses `i64::MIN`.
   Using the wrong identity silently corrupts results.

3. **Integer overflow.** Elements up to 10^9 with 10^5 of them can sum to 10^14, which
   overflows `i32`. Always use `i64`.

4. **Segment tree array too small.** Allocating `2*n` is not safe for non-power-of-2 n.
   Always allocate `4*n`.

5. **Forgetting push_down in lazy propagation.** If you skip `push_down` before recursing
   on a partial-overlap node, you read stale values. This is the number one lazy seg tree
   bug.

---

## Practice Problems

### Easy (5 problems -- direct application)

| # | Problem | Key Idea |
|---|---------|----------|
| 1 | **Range Sum Query - Mutable** (LeetCode 307) | Classic seg tree or BIT. Point update + range sum. |
| 2 | **Range Sum Query 2D - Mutable** (LeetCode 308) | 2D BIT. Point update + rectangle sum. |
| 3 | **Implement a BIT from scratch** | Code the 25-line BIT, test on random arrays vs brute force. |
| 4 | **Single-element queries** | Build a seg tree, do point updates, verify with brute force. |
| 5 | **Prefix sum equivalence** | Implement both prefix sum array and BIT. Compare outputs on 10^5 queries. |

### Medium (5 problems -- algorithmic combinations)

| # | Problem | Key Idea |
|---|---------|----------|
| 1 | **Count of Smaller Numbers After Self** (LeetCode 315) | Process right-to-left, BIT indexed by value. |
| 2 | **Reverse Pairs** (LeetCode 493) | Count (i,j) with i<j, arr[i]>2*arr[j]. BIT + coordinate compression. |
| 3 | **Count of Range Sum** (LeetCode 327) | Prefix sums + BIT to count pairs in [lower, upper]. |
| 4 | **My Calendar III** (LeetCode 732) | Seg tree for max overlapping intervals (lazy range-add). |
| 5 | **Longest Increasing Subsequence** (LeetCode 300 variant) | BIT indexed by value for O(n log n) LIS. |

### Hard (5 problems -- deep mastery required)

| # | Problem | Key Idea |
|---|---------|----------|
| 1 | **Count Good Triplets in an Array** (LeetCode 2179) | Two BITs: prefix count + suffix count. |
| 2 | **Range Module** (LeetCode 715) | Lazy seg tree for range set/clear operations. |
| 3 | **Falling Squares** (LeetCode 699) | Seg tree + coordinate compression + range-max. |
| 4 | **Create Sorted Array through Instructions** (LeetCode 1649) | BIT for counting less-than and greater-than at each insert. |
| 5 | **Minimum Number of Operations to Make Array Continuous** (LeetCode 2009) | Sliding window + BIT for distinct element counting. |

---

## Quick Reference Card

```
SEGMENT TREE                           FENWICK TREE (BIT)
============                           ==================
Build:    O(n)                         Build:    O(n)
Update:   O(log n) point               Update:   O(log n) point
          O(log n) range (lazy)
Query:    O(log n) any range op        Query:    O(log n) prefix sum
Space:    O(4n)                        Space:    O(n)

Supports: sum, min, max, gcd,         Supports: sum, xor (invertible ops only)
          any associative operation

Key code (BIT update):                Key code (BIT query):
  let mut i = idx + 1;                  let mut i = idx + 1;
  while i <= n {                        while i > 0 {
      bit[i] += delta;                      sum += bit[i];
      i += i & i.wrapping_neg();            i -= i & i.wrapping_neg();
  }                                     }
```

---

## Key Takeaways

1. **Segment trees and Fenwick trees solve the dynamic range query problem:** efficient
   queries *and* updates on an array, both in O(log n).

2. **A segment tree is a balanced binary tree where each node stores the aggregate for a
   contiguous segment.** It supports any associative operation (sum, min, max, GCD, XOR).

3. **Lazy propagation extends segment trees to handle range updates in O(log n).** The
   idea: defer computation. Mark a node with a pending update, resolve only when needed.

4. **A Fenwick tree stores partial sums based on binary decomposition of indices.** It
   uses `i & i.wrapping_neg()` to navigate: subtract for queries, add for updates.
   Simpler and faster than a segment tree, but limited to invertible operations.

5. **Use a Fenwick tree for sum queries with point updates.** Use a segment tree for
   everything else.

6. **Both are O(n) space, O(log n) per operation.** The practical difference is in
   constant factors and code complexity. Fenwick has ~half the code and ~half the memory.

7. **These bridge the gap between prefix sums (Lesson 15) and balanced BSTs (Lesson 19).**
   More powerful than prefix sums, simpler than general-purpose trees, fast enough for
   nearly any range query problem.

---

*Segment trees and Fenwick trees feel like overkill until you need one -- and then nothing
else will do. The ideas are not complicated: precompute aggregates over ranges, exploit
tree structure to touch only O(log n) of them per operation. Once you have implemented
each one a few times, they become as natural as the prefix sum that started it all.*
