# Lesson 02: Arrays & Slices

## The Most Important Data Structure You Already Know

You've used arrays for six years. You've pushed to vectors, iterated over slices,
indexed into buffers. But have you ever stopped to think about *why* `v[3]` is
instant, or *why* inserting at the front of a `Vec` makes you wince? This lesson
tears the lid off.

We're going to cover:

1. What contiguous memory actually means and why it matters
2. Rust's three array types: `[T; N]`, `Vec<T>`, `&[T]`
3. The real costs: indexing, insertion, deletion, growing
4. Why CPUs love arrays (cache locality)
5. Classic patterns: two pointers, prefix sums, Kadane's algorithm

---

## 1. Contiguous Memory: The Foundation

An array is a block of elements laid out **one after another** in memory, with no
gaps. That's it. That's the entire idea. Everything else follows from this.

```
  address:   0x7000  0x7004  0x7008  0x700C  0x7010
           +-------+-------+-------+-------+-------+
           |  10   |  20   |  30   |  40   |  50   |
           +-------+-------+-------+-------+-------+
  index:      [0]     [1]     [2]     [3]     [4]

  Each element is 4 bytes (i32). Element [i] lives at:
      base_address + i * size_of::<T>()
      0x7000       + 3 * 4  =  0x700C  -->  40
```

This is why indexing is **O(1)**. The CPU doesn't walk through elements one by one
like a linked list. It does one multiplication and one addition, then jumps
directly to the right memory address. No searching. No pointer chasing.

### A Real-World Analogy

Think of a row of post office boxes. They're numbered sequentially, they're all the
same size, and they're physically next to each other. If someone says "go to box
247", you don't start at box 1 and count forward -- you compute the position and
walk straight there. That's array indexing.

---

## 2. Rust's Array Types

Rust gives you three flavors, each serving a different purpose. Understanding the
differences is essential because Rust is explicit about ownership and borrowing, and
these three types sit at different points on that spectrum.

### 2.1 Fixed-Size Arrays: `[T; N]`

```rust
let temps: [f64; 7] = [18.3, 19.1, 22.0, 21.5, 20.8, 23.1, 22.7];
```

The size `N` is baked into the **type**. `[i32; 3]` and `[i32; 5]` are different
types entirely. The compiler knows the size at compile time, so it can place the
whole thing on the stack.

```
  Stack frame:
  +-------+-------+-------+-------+-------+-------+-------+
  | 18.3  | 19.1  | 22.0  | 21.5  | 20.8  | 23.1  | 22.7  |
  +-------+-------+-------+-------+-------+-------+-------+
  temps (56 bytes on the stack -- 7 x 8 bytes per f64)
```

**When to use:** Fixed, small collections where the size is known at compile time.
Lookup tables, RGB color values, a week of temperatures. The compiler can optimize
aggressively because nothing about the size is uncertain.

```rust
// All elements initialized to the same value:
let zeroes: [u8; 1024] = [0; 1024];

// Pattern matching works:
let [first, second, ..] = temps;
println!("Monday: {first}, Tuesday: {second}");
```

### 2.2 Growable Arrays: `Vec<T>`

This is your workhorse. A `Vec` is a heap-allocated, dynamically-sized array.

```rust
let mut scores: Vec<i32> = Vec::new();
scores.push(95);
scores.push(87);
scores.push(92);
```

Under the hood, a `Vec<T>` is three values stored on the stack:

```
  Stack:                          Heap:
  +----------+                    +------+------+------+------+------+------+
  | ptr  ----+------------------->|  95  |  87  |  92  |      |      |      |
  +----------+                    +------+------+------+------+------+------+
  | len: 3   |                     [0]    [1]    [2]    (unused capacity)
  +----------+
  | cap: 6   |
  +----------+

  ptr  = pointer to heap allocation
  len  = number of elements currently stored
  cap  = number of elements the allocation can hold before resizing
```

**Key insight:** `Vec` over-allocates. When you push beyond capacity, it doesn't
allocate space for just one more element. It typically **doubles** the capacity.
This is what makes `push` amortized O(1) instead of O(n).

### How Vec Grows

Let's trace what happens:

```
  Operation          len   cap   Heap contents
  ─────────────────  ───   ───   ─────────────────────────
  Vec::new()           0     0   (no allocation yet)
  push(1)              1     4   [1, _, _, _]
  push(2)              2     4   [1, 2, _, _]
  push(3)              3     4   [1, 2, 3, _]
  push(4)              4     4   [1, 2, 3, 4]
  push(5)              5     8   [1, 2, 3, 4, 5, _, _, _]   <-- REALLOC!
  push(6)              6     8   [1, 2, 3, 4, 5, 6, _, _]
  push(7)              7     8   [1, 2, 3, 4, 5, 6, 7, _]
  push(8)              8     8   [1, 2, 3, 4, 5, 6, 7, 8]
  push(9)              9    16   [1, 2, 3, 4, 5, 6, 7, 8, 9, _, ...]  <-- REALLOC!
```

When a reallocation happens:
1. A new, larger block is allocated on the heap
2. All existing elements are copied from the old block to the new one
3. The old block is freed
4. The internal pointer is updated

That copy is O(n). But because capacity doubles each time, the total cost of n
pushes is still O(n) overall -- roughly 2n copies total, which averages to O(1) per
push. This is the classic **amortized analysis** argument.

If you know the final size up front, skip the reallocations entirely:

```rust
let mut data = Vec::with_capacity(10_000);
// No reallocations until you exceed 10,000 elements.
```

### 2.3 Slices: `&[T]`

A slice is a **view** into a contiguous sequence. It doesn't own the data. It's a
borrow -- a reference to some range of elements that live somewhere else (in an
array, a Vec, or another slice).

```rust
let nums = vec![10, 20, 30, 40, 50];

let all: &[i32] = &nums;          // Slice of the whole Vec
let middle: &[i32] = &nums[1..4]; // Slice of elements [20, 30, 40]
```

A slice is a **fat pointer** -- two words instead of one:

```
  &[i32] on the stack:
  +----------+
  | ptr  ----+--->  points to the first element of the viewed range
  +----------+
  | len: 3   |     number of elements in this view
  +----------+

  Example: middle = &nums[1..4]

  nums heap:  [10, 20, 30, 40, 50]
                    ^
                    |
  middle.ptr ------+
  middle.len = 3

  middle "sees": [20, 30, 40]
```

**Why slices matter:** They let you write functions that work with *any* contiguous
data, regardless of whether it came from an array, a Vec, or another slice.

```rust
fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

let array = [1, 2, 3];
let vector = vec![4, 5, 6];

// Both work. The function doesn't care about the source.
println!("{}", sum(&array));
println!("{}", sum(&vector));
```

This is idiomatic Rust: take `&[T]` as a parameter when you only need to read a
sequence. It's the most general type for that job.

### How They Relate

```
   [T; N]  ──coerce──>  &[T]
                           ^
   Vec<T>  ──deref───────╯

   Ownership:
   [T; N]  = owns data, fixed size, usually stack
   Vec<T>  = owns data, growable, heap
   &[T]    = borrows data, fixed view, no ownership
```

A fixed array `[T; N]` can be implicitly coerced to a slice `&[T]`. A `Vec<T>`
implements `Deref<Target = [T]>`, so it also coerces to `&[T]`. This is why you can
pass `&my_vec` or `&my_array` to any function expecting `&[T]`.

---

## 3. Operation Costs

Here's the truth table. Memorize the *why*, not the chart -- then you can rederive
it any time.

```
  Operation                [T; N]      Vec<T>         Why
  ─────────────────────    ────────    ────────       ──────────────────────────
  Index (v[i])             O(1)       O(1)           base + i * size
  Search (unsorted)        O(n)       O(n)           must check every element
  Search (sorted)          O(log n)   O(log n)       binary search
  Push back                  --       O(1) amort.    occasional realloc
  Pop back                   --       O(1)           just decrement len
  Insert at index i          --       O(n)           shift elements right
  Remove at index i          --       O(n)           shift elements left
  Insert at front            --       O(n)           shift ALL elements right
```

### Why Insert/Remove at Index Is O(n)

Contiguous memory is a double-edged sword. To insert at position 2, every element
from index 2 onward must shift right by one slot to make room:

```
  Before insert(2, 99):
  +----+----+----+----+----+
  | 10 | 20 | 30 | 40 | 50 |
  +----+----+----+----+----+
    [0]  [1]  [2]  [3]  [4]

  Step 1 -- shift elements [2..] right:
  +----+----+----+----+----+----+
  | 10 | 20 |    | 30 | 40 | 50 |
  +----+----+----+----+----+----+
                ^
                gap at [2]

  Step 2 -- write the new value:
  +----+----+----+----+----+----+
  | 10 | 20 | 99 | 30 | 40 | 50 |
  +----+----+----+----+----+----+
    [0]  [1]  [2]  [3]  [4]  [5]
```

In the worst case (insert at front), you shift all n elements. That's O(n). Same
logic for removal -- elements shift left to fill the gap.

In Rust:

```rust
let mut v = vec![10, 20, 30, 40, 50];
v.insert(2, 99);   // O(n) -- shifts 30, 40, 50 right
v.remove(0);        // O(n) -- shifts everything left
```

**Tip:** If you don't care about order, `Vec::swap_remove` is O(1). It swaps the
target element with the last element, then pops the last:

```rust
let mut v = vec![10, 20, 30, 40, 50];
v.swap_remove(1);  // Swaps v[1] with v[4], then pops
// v is now [10, 50, 30, 40]  -- order changed, but O(1)
```

---

## 4. Why CPUs Love Arrays: Cache Locality

This is the part most DSA courses skip, but it matters enormously in practice.

### The Memory Hierarchy

Your CPU doesn't talk to RAM directly for every read. There's a hierarchy of
progressively larger, slower storage:

```
  ┌─────────────────────────────────────────────────────┐
  │  CPU Registers    ~0.3 ns    ~1 KB                  │
  │  L1 Cache         ~1 ns      ~64 KB                 │
  │  L2 Cache         ~4 ns      ~256 KB                │
  │  L3 Cache         ~12 ns     ~8 MB                  │
  │  Main Memory      ~100 ns    ~16 GB                 │
  │  SSD              ~100,000 ns                       │
  └─────────────────────────────────────────────────────┘
       (approximate, varies by hardware)
```

When the CPU reads `v[0]`, it doesn't fetch just that one i32. It loads an entire
**cache line** (typically 64 bytes) from memory. For an array of `i32` values,
that's 16 elements loaded at once.

```
  Cache line (64 bytes):
  +----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
  | v0 | v1 | v2 | v3 | v4 | v5 | v6 | v7 | v8 | v9 |v10 |v11 |v12 |v13 |v14 |v15 |
  +----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  All loaded in one ~100ns memory fetch
```

So when you iterate over an array sequentially, elements `[1]` through `[15]` are
already in cache when you need them. The CPU's prefetcher sees the sequential access
pattern and starts loading the *next* cache line before you even ask for it.

Compare this to a linked list, where each node could be anywhere in memory. Each
access is a potential cache miss -- a ~100ns penalty instead of ~1ns. For large
datasets, this can be a 100x difference in practice.

**This is why `Vec` is almost always faster than `LinkedList` in Rust, even for
operations where the linked list has better Big-O complexity.** The constant factors
from cache behavior dominate until your dataset is truly huge.

---

## 5. Common Patterns

These patterns come up constantly in interviews and real-world code. They all
exploit properties of contiguous arrays.

### 5.1 Two Pointers

The idea: use two indices that move through the array, usually from opposite ends or
at different speeds, to solve problems in O(n) time instead of O(n^2).

**Example: Two Sum on a Sorted Array**

Given a sorted array and a target sum, find two elements that add up to the target.

```rust
/// Returns indices (i, j) such that nums[i] + nums[j] == target,
/// assuming nums is sorted in ascending order.
fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len().checked_sub(1)?;

    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;    // Need a bigger sum, move left pointer right
        } else {
            right -= 1;   // Need a smaller sum, move right pointer left
        }
    }
    None
}

fn main() {
    let nums = [1, 3, 5, 7, 11, 15];
    assert_eq!(two_sum_sorted(&nums, 8), Some((0, 3)));   // 1 + 7  = 8
    assert_eq!(two_sum_sorted(&nums, 14), Some((1, 4)));  // 3 + 11 = 14
}
```

Why it works: because the array is sorted, we know which direction to move each
pointer to adjust the sum. We never need to consider pairs we've already ruled out.
Each pointer moves at most n times, so it's O(n) total.

```
  [1,  3,  5,  7,  11,  15]    target = 8
   L                    R       1 + 15 = 16 > 8  -->  R--
   L               R            1 + 11 = 12 > 8  -->  R--
   L          R                 1 + 7  = 8  == 8  -->  found (0, 3)
```

### 5.2 Prefix Sums

A prefix sum array stores cumulative sums so you can compute the sum of *any*
subarray in O(1) after O(n) preprocessing.

```rust
/// Build a prefix sum array. prefix[i] = sum of nums[0..i].
fn prefix_sums(nums: &[i32]) -> Vec<i32> {
    let mut prefix = Vec::with_capacity(nums.len() + 1);
    prefix.push(0); // prefix[0] = 0 (sum of empty range)
    for &num in nums {
        prefix.push(prefix.last().unwrap() + num);
    }
    prefix
}

/// Sum of nums[left..right] (inclusive left, exclusive right).
fn range_sum(prefix: &[i32], left: usize, right: usize) -> i32 {
    prefix[right] - prefix[left]
}

fn main() {
    let nums = [3, 1, 4, 1, 5, 9, 2, 6];
    let prefix = prefix_sums(&nums);
    // prefix = [0, 3, 4, 8, 9, 14, 23, 25, 31]

    // Sum of nums[2..5] = 4 + 1 + 5 = 10
    assert_eq!(range_sum(&prefix, 2, 5), 10);
}
```

Visually:

```
  nums:      [3,  1,  4,  1,  5,  9,  2,  6]
  index:      0   1   2   3   4   5   6   7

  prefix:  [0,  3,  4,  8,  9, 14, 23, 25, 31]
  index:    0   1   2   3   4   5   6   7   8

  range_sum(2, 5) = prefix[5] - prefix[2] = 14 - 4 = 10
                                               ^^   ^^
                                   sum(0..5)  sum(0..2)
```

The prefix sum trick comes up everywhere: range queries, running averages, counting
occurrences in a range, 2D grid problems. Once you see it, you see it constantly.

### 5.3 Kadane's Algorithm: Maximum Subarray Sum

**Problem:** Given an array of integers (possibly negative), find the contiguous
subarray with the largest sum.

Brute force is O(n^3) -- check every pair (i, j) and sum the range. With prefix
sums you can get O(n^2). Kadane's does it in **O(n)** with O(1) extra space.

The insight: at each position, you either extend the current subarray or start a new
one. If the running sum drops below zero, any future subarray is better off starting
fresh.

```rust
fn max_subarray_sum(nums: &[i32]) -> i32 {
    assert!(!nums.is_empty(), "array must be non-empty");

    let mut max_ending_here = nums[0]; // Best sum ending at current position
    let mut max_so_far = nums[0];      // Best sum seen anywhere

    for &num in &nums[1..] {
        // Either extend the previous subarray, or start fresh at num
        max_ending_here = num.max(max_ending_here + num);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}

fn main() {
    let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(max_subarray_sum(&nums), 6); // subarray [4, -1, 2, 1]
}
```

Trace through to build intuition:

```
  nums:             [-2,  1, -3,  4, -1,  2,  1, -5,  4]

  max_ending_here:   -2   1  -2   4   3   5   6   1   5
  max_so_far:        -2   1   1   4   4   5   6   6   6
                                                   ^
                                              answer = 6

  At index 3 (value 4):
    max_ending_here = max(4, -2 + 4) = max(4, 2) = 4
    Starting fresh at 4 is better than extending [-2, 1, -3, 4]
```

Kadane's is a gateway to dynamic programming thinking: "What's the best answer
ending at position i, given the best answer ending at position i-1?" We'll build on
this idea heavily in later lessons.

---

## 6. Practical Rust Patterns

A few things worth keeping in your back pocket.

### Iteration Over Indices vs. Values

```rust
let data = vec![10, 20, 30];

// Idiomatic: iterate by reference
for val in &data {
    println!("{val}");
}

// When you need the index too
for (i, val) in data.iter().enumerate() {
    println!("[{i}] = {val}");
}

// Avoid this unless you have a reason -- indexing on each iteration
// is slightly less idiomatic and the bounds check is redundant work:
for i in 0..data.len() {
    println!("[{i}] = {}", data[i]);
}
```

### Windows and Chunks

The standard library gives you sliding windows and fixed-size chunks for free:

```rust
let data = [1, 2, 3, 4, 5, 6];

// Sliding window of size 3:
for window in data.windows(3) {
    println!("{window:?}");
}
// [1, 2, 3]
// [2, 3, 4]
// [3, 4, 5]
// [4, 5, 6]

// Non-overlapping chunks of size 2:
for chunk in data.chunks(2) {
    println!("{chunk:?}");
}
// [1, 2]
// [3, 4]
// [5, 6]
```

These are slices under the hood -- no allocations, just pointer + length views into
the original array.

### Sorting and Binary Search

```rust
let mut data = vec![5, 2, 8, 1, 9, 3];
data.sort();
// data = [1, 2, 3, 5, 8, 9]

// Binary search: O(log n) -- returns Result<index, insertion_point>
match data.binary_search(&5) {
    Ok(index) => println!("Found 5 at index {index}"),
    Err(pos) => println!("5 not found, would insert at {pos}"),
}
```

### Collecting Iterators Into Vecs

Rust's iterator combinators produce lazy chains. `.collect()` materializes them:

```rust
let squares: Vec<i32> = (1..=10).map(|x| x * x).collect();
// [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]

let evens: Vec<&i32> = squares.iter().filter(|x| *x % 2 == 0).collect();
// [4, 16, 36, 64, 100]
```

---

## 7. When NOT to Use Arrays

Arrays aren't always the answer. Reach for something else when:

- **Frequent insertion/deletion in the middle** -- Consider a `VecDeque` (double-
  ended queue backed by a ring buffer) or a `LinkedList` (rare in Rust, but it
  exists).
- **Frequent insertion/deletion at the front** -- `VecDeque` gives you O(1) for
  push/pop at both ends.
- **You need fast membership testing** -- A `HashSet` gives O(1) average lookup
  vs. O(n) for an unsorted array.
- **You need key-value lookup** -- `HashMap` rather than a sorted array with binary
  search (though the latter can be better for small, static datasets due to cache
  effects -- see lesson on hashing).
- **Sparse data** -- If your "array" is 99% zeros, a `HashMap<usize, T>` uses far
  less memory.

But when in doubt, start with `Vec`. It's the right default. Profile before you
switch to something exotic.

---

## 8. Summary

| Concept | Key Takeaway |
|---|---|
| Contiguous layout | Elements sit side by side in memory -- enables O(1) indexing |
| `[T; N]` | Fixed-size, stack-allocated, size is part of the type |
| `Vec<T>` | Growable, heap-allocated, amortized O(1) push |
| `&[T]` | Fat pointer (ptr + len), borrows a contiguous range |
| Cache locality | Sequential access is fast because the CPU prefetches cache lines |
| Insert/remove | O(n) because elements must shift to stay contiguous |
| Two pointers | O(n) technique for sorted-array problems |
| Prefix sums | O(n) preprocess, then O(1) range queries |
| Kadane's | O(n) max subarray sum via local/global max tracking |

---

## Exercises

Try these in `dsa-forge` before moving on:

1. **`arrays_two_sum`** -- Given an unsorted array and a target, return indices of
   two elements that sum to the target. (Hint: there's an O(n) solution using a
   HashMap -- but first try the O(n^2) brute force to appreciate the difference.)

2. **Implement `max_subarray_sum`** from scratch. Then extend it to also return the
   start and end indices of the subarray.

3. **Range sum queries** -- Build a prefix sum array, then answer 1000 random range
   queries. Compare the time against naively summing each range.

4. **Rotation** -- Rotate an array left by `k` positions in O(n) time and O(1)
   extra space. (Hint: three reverses.)

---

*Next up: [Lesson 03 -- Linked Lists](03_linked_lists.md) -- where we trade cache
locality for O(1) insertion, and learn why that trade is usually not worth it.*
