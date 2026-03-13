# Lesson 09: Basic Sorts -- Bubble, Selection, and Insertion

Sorting is the most studied problem in computer science. Not because sorting itself is
glamorous, but because it is the gateway to understanding algorithm design, tradeoffs,
and analysis. Before you can appreciate *why* merge sort or quicksort are fast, you need
to understand *why* the simpler approaches are slow -- and when they are not.

This lesson covers the three **quadratic sorts**: Bubble Sort, Selection Sort, and
Insertion Sort. They are called "quadratic" because their worst-case time complexity
is O(n^2). They are slow on large inputs. Production sort implementations (including
Rust's `slice::sort`) use far more sophisticated algorithms. But these three are where
your intuition about sorting begins, and each one teaches a different lesson about
how algorithms move data.

---

## Before We Start: What Does "Sorting" Mean?

Sorting takes a collection of elements and rearranges them into a defined order --
typically ascending. For a slice `[5, 3, 8, 1]`, sorted ascending is `[1, 3, 5, 8]`.

Two properties matter when evaluating a sort:

1. **Correctness**: Does it produce the right output for every input?
2. **Efficiency**: How much time and space does it use?

And one subtle property that separates good sorts from great ones:

3. **Stability**: If two elements are "equal" according to the comparison, do they
   retain their original relative order? This matters when you sort by one field
   and want a previous sort by another field to be preserved. More on this later.

---

## 1. Bubble Sort

### The Analogy

Imagine a column of water with bubbles of different sizes. The largest bubbles float
to the top first. Each pass through the array, the largest unsorted element "bubbles up"
to its correct position at the end, like a big bubble rising through liquid.

Or think of it this way: you walk along a row of people sorted by height. You compare
each adjacent pair. If the left person is taller, they swap. After one complete pass,
the tallest person has "bubbled" to the far right. Repeat for the remaining people.

### How It Works

1. Start at the beginning of the array.
2. Compare each pair of adjacent elements.
3. If they are in the wrong order, swap them.
4. After one full pass, the largest element is guaranteed to be at the end.
5. Repeat for the remaining unsorted portion.
6. If a pass completes with zero swaps, the array is sorted -- stop early.

### Step-by-Step Walkthrough

Let's sort `[5, 3, 8, 1, 4]`:

```
Initial array: [5, 3, 8, 1, 4]

--- Pass 1 (bubble the largest to position 4) ---

  Compare [5, 3]: 5 > 3 -> swap
  [3, 5, 8, 1, 4]
       ^  ^

  Compare [5, 8]: 5 < 8 -> no swap
  [3, 5, 8, 1, 4]
          ^  ^

  Compare [8, 1]: 8 > 1 -> swap
  [3, 5, 1, 8, 4]
             ^  ^

  Compare [8, 4]: 8 > 4 -> swap
  [3, 5, 1, 4, 8]    <-- 8 is now in its final position
                 =

--- Pass 2 (bubble the next largest to position 3) ---

  Compare [3, 5]: 3 < 5 -> no swap
  [3, 5, 1, 4, 8]
   ^  ^

  Compare [5, 1]: 5 > 1 -> swap
  [3, 1, 5, 4, 8]
      ^  ^

  Compare [5, 4]: 5 > 4 -> swap
  [3, 1, 4, 5, 8]    <-- 5 is now in its final position
            ^  =

--- Pass 3 (bubble to position 2) ---

  Compare [3, 1]: 3 > 1 -> swap
  [1, 3, 4, 5, 8]
   ^  ^

  Compare [3, 4]: 3 < 4 -> no swap
  [1, 3, 4, 5, 8]    <-- 4 is now in its final position
      ^  =

--- Pass 4 (check position 1) ---

  Compare [1, 3]: 1 < 3 -> no swap
  [1, 3, 4, 5, 8]    <-- No swaps happened! Array is sorted. Stop early.

Result: [1, 3, 4, 5, 8]
```

### Rust Implementation

```rust
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        // Each pass, the last i elements are already sorted.
        for j in 0..n.saturating_sub(i + 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        // If no swaps occurred, the array is sorted.
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut data = vec![5, 3, 8, 1, 4];
    bubble_sort(&mut data);
    assert_eq!(data, vec![1, 3, 4, 5, 8]);

    // Already sorted -- triggers early exit after one pass with no swaps.
    let mut sorted = vec![1, 2, 3, 4, 5];
    bubble_sort(&mut sorted);
    assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
}
```

A few notes on the Rust:

- `T: Ord` means the elements must be totally orderable. This covers `i32`, `String`,
  and anything else that implements `Ord`.
- `arr.swap(j, j + 1)` is a safe, bounds-checked swap provided by slice. No need to
  juggle a temporary variable.
- `saturating_sub` prevents underflow when `n` is 0 or `i + 1 > n`. In Rust, unsigned
  subtraction that would go negative panics in debug mode, so `saturating_sub` is the
  defensive choice.

### Complexity

| Case    | Time   | Why                                             |
|---------|--------|-------------------------------------------------|
| Best    | O(n)   | Already sorted: one pass, no swaps, early exit  |
| Average | O(n^2) | Roughly n^2/4 comparisons and swaps on average  |
| Worst   | O(n^2) | Reverse sorted: every pair swaps, every pass     |

**Space**: O(1) -- only a few extra variables, swaps happen in place.

**Stable**: Yes. Bubble sort only swaps adjacent elements when the left is *strictly
greater*. Equal elements are never swapped past each other, so their relative order
is preserved.

### The Verdict

Bubble sort is the simplest sort to understand but essentially never the right choice
in practice. Its only redeeming quality is the O(n) best case on already-sorted data,
but insertion sort achieves the same thing while being faster on average. Bubble sort
exists primarily as a teaching tool.

---

## 2. Selection Sort

### The Analogy

You have a hand of unsorted playing cards spread face-up on a table. To sort them, you
scan all the cards, find the smallest one, and place it at the far left. Then scan the
remaining cards, find the next smallest, and place it next to the first. Repeat until
no cards remain. Each time, you are *selecting* the minimum from the unsorted portion.

### How It Works

1. Find the minimum element in the entire array.
2. Swap it with the element at position 0.
3. Find the minimum element in positions 1..n.
4. Swap it with the element at position 1.
5. Continue: on iteration `i`, find the minimum in `i..n` and swap it to position `i`.
6. After n-1 iterations, the array is sorted.

### Step-by-Step Walkthrough

Let's sort `[4, 2, 7, 1, 3]`:

```
Initial array: [4, 2, 7, 1, 3]

--- Pass 1: Find min in [0..5], swap to index 0 ---

  Scan: 4, 2, 7, 1, 3
                ^
                min = 1 at index 3

  Swap arr[0] and arr[3]:
  [1, 2, 7, 4, 3]
   =              <-- 1 is in its final position

--- Pass 2: Find min in [1..5], swap to index 1 ---

  Scan: 2, 7, 4, 3
        ^
        min = 2 at index 1 (already in place)

  No swap needed:
  [1, 2, 7, 4, 3]
      =           <-- 2 is in its final position

--- Pass 3: Find min in [2..5], swap to index 2 ---

  Scan: 7, 4, 3
              ^
              min = 3 at index 4

  Swap arr[2] and arr[4]:
  [1, 2, 3, 4, 7]
         =        <-- 3 is in its final position

--- Pass 4: Find min in [3..5], swap to index 3 ---

  Scan: 4, 7
        ^
        min = 4 at index 3 (already in place)

  No swap needed:
  [1, 2, 3, 4, 7]
            =  =  <-- 4 and 7 are in final positions

Result: [1, 2, 3, 4, 7]
```

### Rust Implementation

```rust
fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        // Find the index of the minimum element in arr[i..n].
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        // Swap the minimum into position i.
        arr.swap(i, min_idx);
    }
}

fn main() {
    let mut data = vec![4, 2, 7, 1, 3];
    selection_sort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 7]);

    let mut single = vec![42];
    selection_sort(&mut single);
    assert_eq!(single, vec![42]);

    let mut empty: Vec<i32> = vec![];
    selection_sort(&mut empty);
    assert_eq!(empty, Vec::<i32>::new());
}
```

The implementation is remarkably clean. The inner loop is a plain linear scan for the
minimum -- no branching, no early exits, no special cases. This simplicity is both its
strength and its weakness.

### Complexity

| Case    | Time   | Why                                                |
|---------|--------|----------------------------------------------------|
| Best    | O(n^2) | Always scans the full remaining unsorted portion   |
| Average | O(n^2) | Same -- input order does not affect comparison count |
| Worst   | O(n^2) | Same -- always does n(n-1)/2 comparisons           |

**Space**: O(1) -- in-place, only a few index variables.

**Stable**: No. Selection sort is **not stable** by default. Here is why:

```
Consider: [(2,a), (1,b), (2,c)]  -- sort by first element

Pass 1: min is (1,b) at index 1. Swap with index 0:
  [(1,b), (2,a), (2,c)]
                  ^^^
                  (2,a) was before (2,c) originally -- still true. But...

Now consider: [(2,a), (2,b), (1,c)]

Pass 1: min is (1,c) at index 2. Swap with index 0:
  [(1,c), (2,b), (2,a)]
                  ^^^
                  (2,a) was BEFORE (2,b) originally, but now it is AFTER.
                  Stability violated.
```

The long-distance swap is the culprit. It can jump an element over others with equal
keys. You *can* make selection sort stable by shifting instead of swapping (inserting
the minimum), but that adds O(n) work per pass and nobody does it.

### The Verdict

Selection sort always does exactly n(n-1)/2 comparisons, regardless of input. It does
not benefit from partially sorted data. However, it performs at most n-1 *swaps* --
the fewest swaps of any comparison sort. If swaps are expensive (e.g., each element is
a massive struct and comparison is cheap), selection sort minimizes data movement. This
is a niche advantage but a real one.

---

## 3. Insertion Sort

### The Analogy

Think about how you sort a hand of cards as you pick them up one by one. You draw the
first card -- it is trivially sorted. You draw the second card and insert it either
before or after the first. You draw the third and slide it into the correct position
among the first two. Each new card goes into its proper place within the already-sorted
portion of your hand.

This is exactly insertion sort. You maintain a sorted prefix of the array and repeatedly
insert the next unsorted element into its correct position within that prefix.

### How It Works

1. Consider the first element as a sorted subarray of length 1.
2. Take the next element (the "key").
3. Shift elements in the sorted portion to the right until you find the correct spot.
4. Insert the key.
5. Repeat for all remaining elements.

### Step-by-Step Walkthrough

Let's sort `[5, 2, 4, 1, 3]`:

```
Initial array: [5, 2, 4, 1, 3]
                 ^
                 sorted portion (length 1)

--- Insert arr[1] = 2 into sorted portion [5] ---

  Key = 2
  Compare with 5: 2 < 5 -> shift 5 right
  [_, 5, 4, 1, 3]
   ^
   Insert 2 here
  [2, 5, 4, 1, 3]
   ====           sorted portion: [2, 5]

--- Insert arr[2] = 4 into sorted portion [2, 5] ---

  Key = 4
  Compare with 5: 4 < 5 -> shift 5 right
  [2, _, 5, 1, 3]
      ^
  Compare with 2: 4 > 2 -> stop
  Insert 4 here
  [2, 4, 5, 1, 3]
   =======        sorted portion: [2, 4, 5]

--- Insert arr[3] = 1 into sorted portion [2, 4, 5] ---

  Key = 1
  Compare with 5: 1 < 5 -> shift 5 right
  Compare with 4: 1 < 4 -> shift 4 right
  Compare with 2: 1 < 2 -> shift 2 right
  [_, 2, 4, 5, 3]
   ^
   Reached the beginning -- insert 1 here
  [1, 2, 4, 5, 3]
   ==========     sorted portion: [1, 2, 4, 5]

--- Insert arr[4] = 3 into sorted portion [1, 2, 4, 5] ---

  Key = 3
  Compare with 5: 3 < 5 -> shift 5 right
  Compare with 4: 3 < 4 -> shift 4 right
  Compare with 2: 3 > 2 -> stop
  [1, 2, _, 4, 5]
         ^
         Insert 3 here
  [1, 2, 3, 4, 5]
   ============== sorted!

Result: [1, 2, 3, 4, 5]
```

### Rust Implementation

```rust
fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        // Walk arr[i] leftward until it is in the right position.
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn main() {
    let mut data = vec![5, 2, 4, 1, 3];
    insertion_sort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5]);

    // Nearly sorted data -- insertion sort's best case.
    let mut nearly = vec![1, 2, 4, 3, 5];
    insertion_sort(&mut nearly);
    assert_eq!(nearly, vec![1, 2, 3, 4, 5]);
}
```

A note on this implementation: the textbook version saves the key and shifts elements
with assignment rather than repeated swaps. Both are correct. The swap-based version
is more idiomatic in Rust because `arr.swap()` is safe and avoids `unsafe` code that
would be needed to do the "save key, shift, place key" approach efficiently (you would
need to move values without going through Rust's ownership rules). The compiler is
also smart enough to optimize repeated swaps into shifts in many cases.

Here is the shift-based version for completeness, using an index-based approach that
stays within safe Rust:

```rust
fn insertion_sort_shift<T: Ord + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = key;
    }
}
```

This requires `Clone`, which is a tradeoff. The swap-based version works with any `Ord`
type and is the version you should prefer in Rust.

### Complexity

| Case    | Time   | Why                                                  |
|---------|--------|------------------------------------------------------|
| Best    | O(n)   | Already sorted: inner loop never executes            |
| Average | O(n^2) | Roughly n^2/4 comparisons and shifts                 |
| Worst   | O(n^2) | Reverse sorted: every element shifts to the front    |

**Space**: O(1) -- in-place.

**Stable**: Yes. Insertion sort only moves an element leftward past elements that are
*strictly greater*. Equal elements never swap past each other, so relative order is
preserved.

### When Insertion Sort Wins

This is the important part. Despite being O(n^2), insertion sort has properties that
make it genuinely useful in practice:

**1. Nearly sorted data (few inversions)**

An "inversion" is a pair (i, j) where i < j but arr[i] > arr[j]. Insertion sort's
running time is O(n + k), where k is the number of inversions. If the data has very
few inversions (almost sorted), insertion sort is essentially linear.

```
Nearly sorted:  [1, 2, 3, 5, 4, 6, 7, 8, 10, 9]
                              ^^^^          ^^^^^
                         Only 2 inversions: (5,4) and (10,9)

Insertion sort: O(n + 2) = O(n). Only two elements need to shift.
```

**2. Small arrays**

For small n (say, n < 20), the constant factors and overhead of advanced sorts like
merge sort (function calls, allocations, recursion) outweigh their asymptotic advantage.
Insertion sort, with its tight inner loop and no overhead, is faster in practice.

This is why production sort implementations are **hybrid**: Rust's `slice::sort` (based
on a pattern-defeating quicksort variant) and Python's Timsort both switch to insertion
sort for small subarrays. It is not a toy -- it is a critical component of real sort
implementations.

**3. Online sorting**

Insertion sort can sort data as it arrives. You do not need all elements up front. If
you are reading elements from a stream and need to maintain a sorted collection, you
can insert each one in O(n) time. (For large n, a balanced BST or BTreeSet is better,
but for small n, insertion sort into a Vec is simpler and faster.)

---

## Stability: Why It Matters

We said bubble sort and insertion sort are stable, and selection sort is not. But why
does stability matter?

Consider a list of employees sorted by name:

```
Name           Department
---            ----------
Alice          Engineering
Bob            Sales
Charlie        Engineering
Diana          Sales
```

Now you sort by department. A **stable** sort preserves the name ordering within each
department:

```
Stable sort by department:
Alice          Engineering    <-- Alice before Charlie (original order)
Charlie        Engineering
Bob            Sales          <-- Bob before Diana (original order)
Diana          Sales
```

An **unstable** sort might produce:

```
Unstable sort by department:
Charlie        Engineering    <-- Charlie before Alice -- name order lost
Alice          Engineering
Diana          Sales
Bob            Sales
```

Both are correctly sorted by department, but the stable sort gives you a more
predictable, useful result. This is especially important when sorting by multiple
keys: sort by secondary key first, then stable-sort by primary key.

Rust's standard library reflects this distinction:
- `slice::sort()` -- stable, O(n log n), allocates O(n) temporary memory
- `slice::sort_unstable()` -- unstable, O(n log n), in-place (no extra allocation)

---

## Why "Quadratic" Sorts?

All three algorithms have O(n^2) worst-case time. That means doubling the input size
quadruples the running time. Let's see what this means concretely:

```
 n          n^2           Approximate time at 10^8 ops/sec
 --------   -----------   --------------------------------
 100        10,000        < 1 ms       (instant)
 1,000      1,000,000     ~10 ms       (fine)
 10,000     100,000,000   ~1 second    (noticeable)
 100,000    10^10         ~100 seconds (painful)
 1,000,000  10^12         ~2.8 hours   (unacceptable)
```

At n = 1,000, quadratic sorts are perfectly fine. At n = 100,000, they are already
uncomfortable. At n = 1,000,000, they are useless. This is why O(n log n) sorts like
merge sort and quicksort exist -- they handle large inputs gracefully:

```
 n            n^2             n log n
 ---------   ------------    --------
 1,000,000   10^12           ~20,000,000

 Quadratic: 2.8 hours
 n log n:   0.2 seconds
```

That is a factor of ~50,000. No amount of hardware makes up for the wrong algorithm.

---

## The Comparison Table

Here is everything side by side:

```
+------------------+--------+---------+---------+--------+---------+-----------+
|                  | Best   | Average | Worst   | Space  | Stable? | Swaps     |
|                  | Time   | Time    | Time    |        |         | (worst)   |
+------------------+--------+---------+---------+--------+---------+-----------+
| Bubble Sort      | O(n)   | O(n^2)  | O(n^2)  | O(1)   | Yes     | O(n^2)    |
| Selection Sort   | O(n^2) | O(n^2)  | O(n^2)  | O(1)   | No      | O(n)      |
| Insertion Sort   | O(n)   | O(n^2)  | O(n^2)  | O(1)   | Yes     | O(n^2)    |
+------------------+--------+---------+---------+--------+---------+-----------+
```

Key observations:

- **Bubble and insertion** both achieve O(n) best case on sorted input. Selection
  sort does not -- it always scans the full unsorted remainder.
- **Selection sort** does the fewest swaps (at most n-1). If swaps are expensive and
  comparisons are cheap, selection sort has an edge.
- **Insertion sort** is the practical winner among these three. It is adaptive (fast on
  nearly sorted data), stable, has low overhead, and is used as a building block in
  production sort implementations.
- **Bubble sort** has no practical advantage over insertion sort. It does more work
  to achieve the same result. It is taught because it is simple to understand.

---

## Putting It Into Perspective

These three sorts are **building blocks for understanding**, not tools you will reach
for when sorting a million records. Production code uses:

- **Merge sort** -- O(n log n) guaranteed, stable, but uses O(n) extra space.
- **Quicksort** -- O(n log n) average, in-place, but O(n^2) worst case (mitigated
  by good pivot selection).
- **Timsort** (Python, Java) -- hybrid merge sort + insertion sort, optimized for
  real-world data that often has pre-existing order.
- **pdqsort** (Rust's `sort_unstable`) -- pattern-defeating quicksort, a hybrid
  that combines quicksort, heap sort, and insertion sort.

Notice that insertion sort appears *inside* the advanced algorithms. Understanding it
is not just academic -- it is foundational knowledge that directly explains why
production sorts work the way they do.

---

## One More Visual: How Each Sort Thinks

```
BUBBLE SORT -- "swap adjacent mismatches until clean"

  Pass 1:  [5  3  8  1  4]   compare neighbors, swap if wrong
            3  5  8  1  4    (5,3 swapped)
            3  5  8  1  4    (5,8 ok)
            3  5  1  8  4    (8,1 swapped)
            3  5  1  4 [8]   (8,4 swapped) -- 8 settled
  Pass 2:  [3  5  1  4] 8    repeat on unsorted portion
  ...


SELECTION SORT -- "find the min, place it next"

  Pass 1:  [4  2  7  1  3]   scan all, min=1 at index 3
           [1] 2  7  4  3    swap 1 to front -- 1 settled
  Pass 2:   1 [2  7  4  3]   scan remaining, min=2 at index 1
            1 [2] 7  4  3    already in place -- 2 settled
  Pass 3:   1  2 [7  4  3]   scan remaining, min=3 at index 4
            1  2 [3] 4  7    swap 3 to position 2 -- 3 settled
  ...


INSERTION SORT -- "grow a sorted prefix, insert each new element"

            [5] 2  4  1  3   sorted prefix = [5]
  Insert 2: [2  5] 4  1  3   2 slides left past 5
  Insert 4: [2  4  5] 1  3   4 slides left past 5, stops at 2
  Insert 1: [1  2  4  5] 3   1 slides all the way left
  Insert 3: [1  2  3  4  5]  3 slides left past 5, past 4, stops at 2
```

---

## Generic Implementations: A Complete Module

Here are all three sorts as generic functions, ready to use:

```rust
/// Bubble sort: repeatedly swap adjacent out-of-order pairs.
/// Stable. Best O(n), worst O(n^2), space O(1).
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n.saturating_sub(i + 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

/// Selection sort: find the minimum, swap it to the front, repeat.
/// NOT stable. All cases O(n^2), space O(1).
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
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

/// Insertion sort: grow a sorted prefix by inserting each element.
/// Stable. Best O(n), worst O(n^2), space O(1).
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T: Ord>(arr: &[T]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![5, 3, 8, 1, 4, 2, 7, 6];
        bubble_sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn test_selection_sort() {
        let mut v = vec![5, 3, 8, 1, 4, 2, 7, 6];
        selection_sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![5, 3, 8, 1, 4, 2, 7, 6];
        insertion_sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn test_empty_and_single() {
        for sort_fn in [bubble_sort, selection_sort, insertion_sort] {
            let mut empty: Vec<i32> = vec![];
            sort_fn(&mut empty);
            assert_eq!(empty, vec![]);

            let mut single = vec![42];
            sort_fn(&mut single);
            assert_eq!(single, vec![42]);
        }
    }

    #[test]
    fn test_already_sorted() {
        for sort_fn in [bubble_sort, selection_sort, insertion_sort] {
            let mut v = vec![1, 2, 3, 4, 5];
            sort_fn(&mut v);
            assert_eq!(v, vec![1, 2, 3, 4, 5]);
        }
    }

    #[test]
    fn test_reverse_sorted() {
        for sort_fn in [bubble_sort, selection_sort, insertion_sort] {
            let mut v = vec![5, 4, 3, 2, 1];
            sort_fn(&mut v);
            assert_eq!(v, vec![1, 2, 3, 4, 5]);
        }
    }

    #[test]
    fn test_duplicates() {
        for sort_fn in [bubble_sort, selection_sort, insertion_sort] {
            let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
            sort_fn(&mut v);
            assert!(is_sorted(&v));
        }
    }
}
```

---

## Key Takeaways

1. **All three are O(n^2) worst case.** This makes them impractical for large datasets
   but perfectly fine for small ones (n < a few hundred).

2. **Insertion sort is the practical winner** among the three. It is adaptive (O(n) on
   nearly sorted data), stable, has minimal overhead, and is used inside production
   sort algorithms for small subarrays.

3. **Selection sort minimizes swaps.** If comparisons are cheap but moving data is
   expensive, selection sort does at most n-1 swaps.

4. **Bubble sort has no practical advantage** over insertion sort. It does more total
   work for the same result. Learn it for understanding, then forget it for practice.

5. **Stability matters** when sorting by multiple keys or when you need deterministic
   ordering of equal elements. Bubble sort and insertion sort are stable; selection
   sort is not.

6. **These are building blocks.** Understanding how they fail at scale is exactly what
   motivates merge sort, quicksort, and the hybrid algorithms that production code
   actually uses. We will cover those next.

---

## Exercises

1. **Trace by hand**: Sort `[6, 4, 3, 7, 1, 5, 2]` using each of the three algorithms.
   Count the number of comparisons and swaps for each. Which did the least total work?

2. **Nearly sorted performance**: Create a nearly-sorted array of 1000 elements (only
   10 elements out of place). Time bubble sort vs insertion sort on it. Why is one
   faster?

3. **Stability test**: Create a `Vec<(i32, char)>` like `[(2,'a'), (1,'b'), (2,'c')]`.
   Sort by the first element using selection sort and insertion sort. Verify that
   insertion sort preserves the relative order of the two `2`s but selection sort may not.

4. **Descending order**: Modify each sort to accept a custom comparator (use `Fn(&T, &T)
   -> std::cmp::Ordering`) instead of relying on `Ord`. Sort a vector in descending
   order.

5. **Hybrid sort**: Write a function that uses insertion sort for arrays of length <= 16
   and falls back to `slice::sort()` for larger arrays. Benchmark it against plain
   `slice::sort()` on random data of various sizes. At what size does the hybrid
   approach stop helping?
