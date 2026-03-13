# Lesson 11: Quick Sort

## The Sort That Conquered the Real World

Merge sort is elegant. It has a clean O(n log n) guarantee and it is conceptually simple:
split, recurse, merge. But in practice, most standard libraries reach for a different
algorithm: **quick sort**. Rust's own `slice::sort_unstable` is based on a quicksort variant
called pdqsort (pattern-defeating quicksort). C's `qsort`, Java's `Arrays.sort` for
primitives, Go's `sort.Slice` -- all quicksort descendants.

Why? Because quicksort sorts **in place** (no auxiliary array), has excellent cache behavior
(sequential access patterns on contiguous memory), and its average case is O(n log n) with
small constant factors. It does have a worst case of O(n^2), but modern implementations make
that almost impossible to trigger in practice.

This lesson takes quicksort apart, piece by piece:

1. The partition operation -- the heart of the algorithm
2. Full implementation with the Lomuto partition scheme
3. Pivot selection strategies and why they matter
4. Why average case is O(n log n) and worst case is O(n^2)
5. Space complexity and the call stack
6. Handling duplicates with 3-way partitioning (Dutch National Flag)
7. Quicksort vs. merge sort -- when to choose each
8. What Rust's standard library actually does

---

## The Core Idea

Forget algorithms for a moment. Imagine you are a gym teacher and you need to line up
30 students by height.

Here is one approach:

1. **Pick one student** -- call them the **pivot**. Say they are 170 cm.
2. Tell everyone shorter than 170 cm to stand on the **left**.
3. Tell everyone taller than 170 cm to stand on the **right**.
4. The pivot stands in the gap between the two groups.

Now the pivot is in their **final correct position**. Everyone to their left is shorter,
everyone to their right is taller. You do not know the exact order within each group yet,
but you have made progress: one person is placed, and the problem has been split into two
smaller problems.

5. Repeat steps 1-4 for the left group and the right group.
6. When a group has zero or one person, it is already sorted. Stop.

That is quicksort. The entire algorithm is: **partition, then recurse on each side**.

---

## The Partition Operation

Partition is where all the real work happens. Everything else is just recursion management.

Given a slice and a pivot value, partition rearranges the elements so that:

- All elements less than or equal to the pivot are on the left
- All elements greater than the pivot are on the right
- The pivot is at its final sorted position

There are two classic partition schemes: **Lomuto** and **Hoare**. We will implement Lomuto
first because it is easier to understand and reason about, then discuss Hoare.

### Lomuto Partition Scheme

The idea: use the **last element** as the pivot. Walk through the array with one pointer.
Maintain a boundary `i` that tracks where the "less than or equal to pivot" region ends.
Every time you find an element that belongs on the left side, swap it into position and
advance the boundary.

Let's trace through an example. We want to partition `[8, 3, 7, 1, 5, 2, 6, 4]` with
pivot = 4 (the last element).

```
Initial state:
  pivot = 4 (last element)
  i = 0 (boundary of "small" region -- nothing in it yet)

  [8, 3, 7, 1, 5, 2, 6, 4]
   ^                     ^
   j (scanning)          pivot

j=0: arr[0]=8, 8 > 4?  Yes. Do nothing.
  i=0
  [8, 3, 7, 1, 5, 2, 6, 4]

j=1: arr[1]=3, 3 <= 4?  Yes! Swap arr[i] and arr[j], then i++.
  swap arr[0] and arr[1]
  i=1
  [3, 8, 7, 1, 5, 2, 6, 4]
   ^
   small region: [3]

j=2: arr[2]=7, 7 > 4?  Yes. Do nothing.
  i=1
  [3, 8, 7, 1, 5, 2, 6, 4]

j=3: arr[3]=1, 1 <= 4?  Yes! Swap arr[1] and arr[3], then i++.
  swap arr[1] and arr[3]
  i=2
  [3, 1, 7, 8, 5, 2, 6, 4]
   ^^^^
   small region: [3, 1]

j=4: arr[4]=5, 5 > 4?  Yes. Do nothing.
  i=2
  [3, 1, 7, 8, 5, 2, 6, 4]

j=5: arr[5]=2, 2 <= 4?  Yes! Swap arr[2] and arr[5], then i++.
  swap arr[2] and arr[5]
  i=3
  [3, 1, 2, 8, 5, 7, 6, 4]
   ^^^^^^^
   small region: [3, 1, 2]

j=6: arr[6]=6, 6 > 4?  Yes. Do nothing.
  i=3
  [3, 1, 2, 8, 5, 7, 6, 4]

Done scanning. Now place the pivot at position i:
  swap arr[3] and arr[7] (swap pivot into boundary position)
  [3, 1, 2, 4, 5, 7, 6, 8]
   ^^^^^^^  ^  ^^^^^^^^^^
   <= 4     |    > 4
          pivot (final position = 3)
```

The pivot `4` is now in index 3 -- exactly where it belongs in the sorted array. Everything
to its left is smaller, everything to its right is larger. We return index 3 so the caller
knows where to split for recursion.

### Visualizing the Invariant

At any point during Lomuto partition, the array looks like this:

```
  [  <= pivot  |  > pivot  |  unseen  | pivot ]
   0          i-1    i         j       last

  - arr[0..i]     : elements <= pivot (the "small" region)
  - arr[i..j]     : elements > pivot  (the "big" region)
  - arr[j..last]  : not yet examined
  - arr[last]     : the pivot
```

---

## Full Implementation: Lomuto Quicksort

```rust
/// Partition the slice arr[lo..=hi] using arr[hi] as the pivot.
/// Returns the final index of the pivot.
fn lomuto_partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi];
    let mut i = lo;

    for j in lo..hi {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, hi); // Place pivot at its final position
    i
}

/// Quicksort using Lomuto partition.
fn quicksort(arr: &mut [i32], lo: usize, hi: usize) {
    if lo >= hi {
        return; // Base case: zero or one element
    }

    let pivot_index = lomuto_partition(arr, lo, hi);

    // Recurse on left partition (elements < pivot)
    if pivot_index > 0 {
        quicksort(arr, lo, pivot_index - 1);
    }
    // Recurse on right partition (elements > pivot)
    quicksort(arr, pivot_index + 1, hi);
}

fn main() {
    let mut data = vec![8, 3, 7, 1, 5, 2, 6, 4];
    let hi = data.len() - 1;
    quicksort(&mut data, 0, hi);
    println!("{data:?}"); // [1, 2, 3, 4, 5, 6, 7, 8]
}
```

A note on the signature: using `lo` and `hi` indices is the classic textbook style. In
idiomatic Rust, you would more likely work with slices directly. Here is a cleaner version:

```rust
fn quicksort_slice(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_idx = partition(arr);

    let (left, right) = arr.split_at_mut(pivot_idx);
    quicksort_slice(left);
    quicksort_slice(&mut right[1..]); // Skip the pivot itself
}

fn partition(arr: &mut [i32]) -> usize {
    let hi = arr.len() - 1;
    let pivot = arr[hi];
    let mut i = 0;

    for j in 0..hi {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, hi);
    i
}
```

`split_at_mut` is the Rust way to get two non-overlapping mutable slices -- the borrow
checker is satisfied because the slices do not alias.

---

## Recursive Breakdown: Full Example

Let's trace quicksort on `[8, 3, 7, 1, 5, 2, 6, 4]`:

```
quicksort([8, 3, 7, 1, 5, 2, 6, 4])
  partition with pivot=4 --> [3, 1, 2, |4|, 5, 7, 6, 8]
  |
  +-- quicksort([3, 1, 2])
  |     partition with pivot=2 --> [1, |2|, 3]
  |     |
  |     +-- quicksort([1])         -- base case, done
  |     +-- quicksort([3])         -- base case, done
  |
  +-- quicksort([5, 7, 6, 8])
        partition with pivot=8 --> [5, 7, 6, |8|]
        |
        +-- quicksort([5, 7, 6])
        |     partition with pivot=6 --> [5, |6|, 7]
        |     |
        |     +-- quicksort([5])   -- base case, done
        |     +-- quicksort([7])   -- base case, done
        |
        +-- quicksort([])         -- base case, done


Result: [1, 2, 3, 4, 5, 6, 7, 8]
```

Each partition call places one element in its final position and produces two subproblems.
At each level of recursion, the total work across all partitions is O(n), and there are
O(log n) levels on average. Hence O(n log n) average time.

---

## Hoare Partition Scheme

Tony Hoare -- the inventor of quicksort itself -- proposed a different partition approach.
Instead of one pointer scanning left to right, Hoare uses **two pointers** starting from
opposite ends, walking toward each other and swapping misplaced elements.

```
  [  7,  2,  1,  8,  6,  3,  5,  4  ]
     L-->                       <--R

  L scans right looking for an element > pivot
  R scans left  looking for an element < pivot
  When both find one, swap and continue
```

Hoare partition is typically faster in practice because it does about three times fewer
swaps on average compared to Lomuto. However, it is trickier to implement correctly -- off-
by-one errors are easy to introduce, and the pivot does not necessarily end up at the
boundary index returned, which complicates the recursion slightly.

For learning purposes, Lomuto is the one to internalize first. Once you are comfortable
with partitioning as a concept, Hoare is a natural optimization to study.

---

## Pivot Selection Strategies

The choice of pivot determines whether quicksort is fast or catastrophically slow. The
ideal pivot would split the array into two equal halves every time. The worst pivot would
leave one side empty and the other with n-1 elements.

### Strategy 1: Always Pick the Last Element

This is what our Lomuto implementation does. Simple, but dangerous:

```
Already sorted input: [1, 2, 3, 4, 5, 6, 7, 8]
  pivot = 8 --> partition: [1, 2, 3, 4, 5, 6, 7] | 8 | []
  pivot = 7 --> partition: [1, 2, 3, 4, 5, 6] | 7 | []
  pivot = 6 --> partition: [1, 2, 3, 4, 5] | 6 | []
  ...

  Each partition removes only ONE element. That is n partitions, each doing
  O(n) work. Total: O(n^2).
```

Sorted input (or reverse-sorted) with a fixed-position pivot is the classic worst case.

### Strategy 2: Always Pick the First Element

Same problem, just triggered by reverse-sorted input instead.

### Strategy 3: Random Pivot

Pick a random index and swap that element to the pivot position before partitioning. This
makes the worst case **extremely unlikely** -- an adversary cannot construct a malicious
input because the pivot is unpredictable.

```rust
use rand::Rng;

fn randomized_partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let mut rng = rand::thread_rng();
    let pivot_idx = rng.gen_range(lo..=hi);
    arr.swap(pivot_idx, hi); // Move random pivot to the end
    lomuto_partition(arr, lo, hi) // Then partition as usual
}
```

Random pivot selection gives O(n log n) **expected** time regardless of input order.

### Strategy 4: Median-of-Three

Pick three elements (commonly the first, middle, and last), find their median, and use that
as the pivot. This is cheap (just three comparisons) and avoids the worst case for sorted
and reverse-sorted inputs.

```rust
fn median_of_three(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let mid = lo + (hi - lo) / 2;

    // Sort the three candidates so we can pick the middle one
    if arr[lo] > arr[mid] {
        arr.swap(lo, mid);
    }
    if arr[lo] > arr[hi] {
        arr.swap(lo, hi);
    }
    if arr[mid] > arr[hi] {
        arr.swap(mid, hi);
    }

    // The median is now at arr[mid]. Move it to arr[hi] for Lomuto.
    arr.swap(mid, hi);
    lomuto_partition(arr, lo, hi)
}
```

Most production implementations use median-of-three or a related heuristic. It is cheap,
deterministic, and handles the common worst-case triggers.

---

## Time Complexity: Why O(n log n) on Average, O(n^2) at Worst

### The Good Case: Balanced Partitions

When the pivot consistently lands near the middle, each partition splits the array roughly
in half:

```
Level 0:   [..............n..............]         n work
Level 1:   [......n/2......][......n/2......]     n work
Level 2:   [..n/4..][..n/4..][..n/4..][..n/4..]  n work
Level 3:   [n/8][n/8][n/8][n/8][n/8][n/8][n/8][n/8]  n work
...
Level log(n):  [1][1][1][1]...[1][1][1][1]        n work

Total: n work per level * log(n) levels = O(n log n)
```

This looks just like merge sort's recursion tree -- and it is, for the same reason.

### The Bad Case: Maximally Unbalanced Partitions

When the pivot is always the smallest or largest element:

```
Level 0:   [..............n..............]         n work
Level 1:   []  [............n-1............]       n-1 work
Level 2:       []  [..........n-2..........]      n-2 work
Level 3:           []  [........n-3........]      n-3 work
...
Level n-1:                              [1]       1 work

Total: n + (n-1) + (n-2) + ... + 1 = n(n+1)/2 = O(n^2)
```

Instead of log(n) levels, you get n levels. The recursion tree degenerates into a straight
line -- essentially selection sort in disguise.

### What Triggers Worst Case

The combination of:
- **Already sorted (or reverse-sorted) input**, AND
- **A pivot strategy that always picks an extreme element** (first or last)

This is not a theoretical curiosity. Early naive quicksort implementations in standard
libraries were vulnerable to this. Someone could pass a sorted array and watch the program
grind to a halt. Modern implementations use randomization or median-of-three to prevent it.

### Average Case Analysis (Intuition)

On random input with random pivot selection, the probability of getting a "good" split
(better than 75/25) is about 50%. Even if half the partitions are somewhat unbalanced, the
balanced ones dominate the recursion depth. The math works out to O(n log n) expected time,
and the constant factor is small because partition is a tight, cache-friendly loop.

---

## Space Complexity: O(log n)

Quicksort is in-place -- it does not allocate an auxiliary array like merge sort does. But
it is not O(1) space. Each recursive call adds a frame to the call stack, and the recursion
depth determines the space usage.

- **Average case:** recursion depth is O(log n), so stack space is O(log n).
- **Worst case:** recursion depth is O(n), so stack space is O(n).

You can mitigate the worst-case stack depth with **tail call optimization** (recurse on the
smaller partition first, then use a loop for the larger one):

```rust
fn quicksort_tail_optimized(arr: &mut [i32], mut lo: usize, mut hi: usize) {
    while lo < hi {
        let pivot_index = lomuto_partition(arr, lo, hi);

        // Recurse on the smaller partition
        // Loop on the larger partition (simulates tail recursion)
        if pivot_index.saturating_sub(lo) < hi.saturating_sub(pivot_index) {
            if pivot_index > 0 {
                quicksort_tail_optimized(arr, lo, pivot_index - 1);
            }
            lo = pivot_index + 1;
        } else {
            quicksort_tail_optimized(arr, pivot_index + 1, hi);
            hi = pivot_index.saturating_sub(1);
        }
    }
}
```

This guarantees O(log n) stack depth even in the worst case, because you always recurse on
the half that is at most n/2 elements, and iterate on the other half.

---

## Quicksort Is Unstable

A sorting algorithm is **stable** if it preserves the relative order of elements with equal
keys. Quicksort is **not** stable.

Consider:

```
  Input:   [(A,3), (B,1), (C,3), (D,2)]
  Sort by the number.

  Stable sort:   [(B,1), (D,2), (A,3), (C,3)]
                                  ^^^^  ^^^^
                                  A before C -- original order preserved

  Quicksort:     [(B,1), (D,2), (C,3), (A,3)]   <-- possible
                                  ^^^^  ^^^^
                                  C before A -- original order NOT preserved
```

The swaps during partitioning can reorder equal elements. If you need stability, use merge
sort (or Rust's `slice::sort`, which is a stable sort based on TimSort).

---

## Handling Duplicates: 3-Way Partition (Dutch National Flag)

Standard Lomuto and Hoare partitioning produce two regions: `<= pivot` and `> pivot`. When
the input has many duplicate values, this can be inefficient. Consider an array where every
element is the same value:

```
  [5, 5, 5, 5, 5, 5, 5, 5]
  pivot = 5

  Lomuto partition: [5, 5, 5, 5, 5, 5, 5] | 5 | []
  All elements go to one side. O(n^2) behavior on uniform input!
```

The fix is **3-way partitioning**, also known as the **Dutch National Flag** problem (named
by Dijkstra). Instead of two regions, we create three:

```
  [  < pivot  |  == pivot  |  unseen  |  > pivot  ]
   0         lt            i          gt         n-1
```

Elements equal to the pivot are grouped in the middle. After partitioning, the middle
region is already sorted (they are all the same value), so we only recurse on the `< pivot`
and `> pivot` regions.

```rust
/// 3-way partition: rearranges arr so that
///   arr[0..lt]     < pivot
///   arr[lt..gt+1]  == pivot
///   arr[gt+1..n]   > pivot
/// Returns (lt, gt).
fn three_way_partition(arr: &mut [i32]) -> (usize, usize) {
    let pivot = arr[0]; // Or use median-of-three, random, etc.
    let mut lt = 0;     // arr[0..lt] < pivot
    let mut i = 0;      // arr[lt..i] == pivot
    let mut gt = arr.len() - 1; // arr[gt+1..] > pivot

    while i <= gt {
        if arr[i] < pivot {
            arr.swap(lt, i);
            lt += 1;
            i += 1;
        } else if arr[i] > pivot {
            arr.swap(i, gt);
            if gt == 0 {
                break;
            }
            gt -= 1;
            // Don't increment i -- the swapped element needs to be examined
        } else {
            i += 1; // arr[i] == pivot, it's in the right place
        }
    }

    (lt, gt)
}

fn quicksort_3way(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let (lt, gt) = three_way_partition(arr);

    quicksort_3way(&mut arr[..lt]);       // Elements < pivot
    if gt + 1 < arr.len() {
        quicksort_3way(&mut arr[gt + 1..]); // Elements > pivot
    }
    // Elements == pivot are already in their final positions
}
```

Let's trace 3-way partition on `[4, 9, 4, 2, 4, 7, 1, 4]` with pivot = 4:

```
Initial:  [4, 9, 4, 2, 4, 7, 1, 4]
           ^                    ^
          lt,i                  gt

i=0: arr[0]=4 == pivot. i++.
          [4, 9, 4, 2, 4, 7, 1, 4]
           lt  i                 gt

i=1: arr[1]=9 > pivot. Swap arr[1] and arr[7]. gt--.
          [4, 4, 4, 2, 4, 7, 1, 9]
           lt  i              gt

i=1: arr[1]=4 == pivot. i++.
          [4, 4, 4, 2, 4, 7, 1, 9]
           lt     i           gt

i=2: arr[2]=4 == pivot. i++.
          [4, 4, 4, 2, 4, 7, 1, 9]
           lt        i        gt

i=3: arr[3]=2 < pivot. Swap arr[0] and arr[3]. lt++. i++.
          [2, 4, 4, 4, 4, 7, 1, 9]
              lt        i     gt

i=4: arr[4]=4 == pivot. i++.
          [2, 4, 4, 4, 4, 7, 1, 9]
              lt           i  gt

i=5: arr[5]=7 > pivot. Swap arr[5] and arr[6]. gt--.
          [2, 4, 4, 4, 4, 1, 7, 9]
              lt           i gt

i=5: arr[5]=1 < pivot. Swap arr[1] and arr[5]. lt++. i++.
          [2, 1, 4, 4, 4, 4, 7, 9]
                 lt           i
                              gt

i > gt, stop.

Result:  [2, 1, | 4, 4, 4, 4, | 7, 9]
          < 4      == 4          > 4
         lt=2     (lt..gt+1)    gt=5

Now recurse only on [2, 1] and [7, 9]. The four 4s are done.
```

3-way partitioning is O(n) for inputs with many duplicates where standard quicksort would
be O(n^2). It is the idea behind Bentley-McIlroy's "fat partition" optimization used in
many production sorts.

---

## Quicksort vs. Merge Sort

These are the two heavyweight O(n log n) comparison sorts. Here's how they stack up:

```
  Property              Quicksort                   Merge Sort
  ──────────────────    ────────────────────────    ──────────────────────
  Time (average)        O(n log n)                  O(n log n)
  Time (worst)          O(n^2)*                     O(n log n)
  Space                 O(log n) stack              O(n) auxiliary
  In-place?             Yes                         No (needs temp array)
  Stable?               No                          Yes
  Cache behavior        Excellent                   Good
  Constant factors      Small                       Larger

  * O(n^2) worst case is avoidable with good pivot selection
```

### When to prefer quicksort:
- Memory is tight and you cannot afford an O(n) auxiliary buffer
- You do not need stability
- You are sorting primitives or simple types where stability is irrelevant
- You want the fastest practical performance on random data

### When to prefer merge sort:
- You need a **guaranteed** O(n log n) worst case with no exceptions
- You need a **stable** sort
- You are sorting linked lists (merge sort needs no random access; quicksort does)
- External sorting (data on disk) -- merge sort's sequential access patterns work well
  with disk I/O

### The Constant Factor Argument

Quicksort's inner loop is extremely tight: compare an element to the pivot, maybe do a
swap, increment a counter. Merge sort's inner loop involves maintaining two read pointers,
one write pointer, and writing to an auxiliary buffer. Even though both are O(n log n),
quicksort's constant is smaller. On modern CPUs with large caches, the difference can be
20-30% in quicksort's favor for in-memory sorts.

---

## What Rust's Standard Library Actually Does

Rust gives you two sorting methods on slices:

- **`slice::sort()`** -- Stable sort. Uses a modified TimSort (merge sort variant). O(n)
  auxiliary space. Preserves the order of equal elements. Use this when you need stability.

- **`slice::sort_unstable()`** -- Unstable sort. Based on **pdqsort** (pattern-defeating
  quicksort) by Orson Peters. This is a hybrid algorithm:
  - Uses quicksort as the main driver
  - Falls back to **heapsort** if it detects that the recursion depth is too deep (which
    would indicate an approaching O(n^2) case) -- this guarantees O(n log n) worst case
  - Uses **insertion sort** for small subarrays (below ~20 elements) where the overhead
    of recursion is not worth it
  - Uses clever pivot selection (median-of-three, and more)
  - Detects already-sorted runs and exploits them

```rust
fn main() {
    let mut data = vec![8, 3, 7, 1, 5, 2, 6, 4];

    // Stable sort (TimSort) -- preserves relative order of equal elements
    data.sort();

    // Unstable sort (pdqsort) -- faster, no stability guarantee
    data.sort_unstable();

    // Custom comparator
    data.sort_unstable_by(|a, b| b.cmp(a)); // Descending

    // Sort by key
    let mut pairs = vec![(3, "c"), (1, "a"), (2, "b")];
    pairs.sort_unstable_by_key(|&(k, _)| k);
}
```

When to use which:
- Default to `sort_unstable()` unless you need stability. It is faster and uses less memory.
- Use `sort()` when the relative order of equal elements matters (e.g., sorting records by
  one field while preserving the order from a previous sort on another field).

---

## Summary: The Properties of Quicksort

| Property              | Value                                              |
|-----------------------|----------------------------------------------------|
| Time (best)           | O(n log n)                                         |
| Time (average)        | O(n log n)                                         |
| Time (worst)          | O(n^2) -- with bad pivot on adversarial input      |
| Space                 | O(log n) average, O(n) worst (call stack)          |
| In-place              | Yes -- no auxiliary array needed                   |
| Stable                | No                                                 |
| Comparison-based      | Yes                                                |
| Adaptive              | No (but pdqsort variant is)                        |
| Worst-case trigger    | Already sorted + always picking min/max as pivot   |
| Mitigation            | Random pivot, median-of-three, or hybrid (pdqsort) |

---

## Exercises

Try these in `dsa-forge` before moving on:

1. **Implement Lomuto quicksort from scratch.** Do not look at the code above. Write the
   partition function, then the recursive driver. Test it on: an empty array, a single
   element, already sorted, reverse sorted, all duplicates, random data.

2. **Count comparisons.** Modify your quicksort to count how many comparisons the partition
   function makes. Run it on a sorted array of 1,000 elements with "last element" pivot.
   Then run it with random pivot selection. Compare the counts. You should see the sorted
   case go from ~500,000 comparisons (O(n^2)) down to ~10,000 (O(n log n)).

3. **Implement 3-way quicksort.** Test it on `[2, 2, 2, 1, 1, 3, 3, 3, 2, 1]` and verify
   the equal-to-pivot elements are not recursed into.

4. **Quickselect.** Quicksort's partition can find the k-th smallest element in O(n)
   average time without fully sorting. Implement `fn kth_smallest(arr: &mut [i32], k: usize) -> i32`
   that partitions, then recurses only on the side that contains index `k`. This is the
   algorithm behind `slice::select_nth_unstable` in Rust's standard library.

5. **Benchmark.** Use `std::time::Instant` to compare your quicksort, Rust's
   `sort_unstable()`, and your merge sort from the previous lesson on arrays of 100,000
   random integers. How do they compare? Try it on already-sorted input too.

---

*Next up: [Lesson 12 -- Heaps & Priority Queues](12_heaps.md) -- a tree structure that
lives inside an array, gives you O(1) access to the maximum, and powers heapsort (the
algorithm that quicksort falls back to when things go wrong).*
