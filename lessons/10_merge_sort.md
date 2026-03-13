# Lesson 10: Merge Sort

## The Big Idea

You have a huge unsorted pile of exams to arrange alphabetically. Here is a strategy that
actually works:

1. Split the pile in half.
2. Split each half in half again.
3. Keep splitting until you have piles of one exam each (a single exam is trivially "sorted").
4. Now merge pairs of sorted piles back together, always picking the alphabetically first exam
   from the top of either pile.

By the time you have merged everything back into one pile, it is fully sorted.

That is merge sort. It is the textbook example of **divide and conquer**: break a problem into
smaller subproblems, solve each one, then combine the solutions. The "conquer" step is trivial
(a single element is already sorted). The "combine" step -- the merge -- is where the real work
happens.

Merge sort was invented by John von Neumann in 1945. It was one of the first algorithms
designed specifically for computers, and eight decades later it is still one of the best
general-purpose sorting algorithms. There is a reason for that longevity: its O(n log n)
worst-case guarantee is something most other sorting algorithms cannot match.

---

## Divide and Conquer, Step by Step

The algorithm has three phases:

1. **Divide**: Split the array in half.
2. **Recurse**: Sort each half (by calling merge sort on it).
3. **Merge**: Combine two sorted halves into one sorted array.

The recursion bottoms out when a subarray has 0 or 1 elements -- nothing to sort.

### The Card Deck Analogy

Imagine you have a shuffled deck of 8 playing cards and you want to sort them by rank.

1. Split the deck into two piles of 4.
2. Split each pile of 4 into two piles of 2.
3. Split each pile of 2 into two piles of 1.
4. Now pick up two single-card piles. Compare them and stack them in order. You now have a
   sorted pile of 2.
5. Take two sorted piles of 2. Compare the top cards, take the smaller one. Compare again.
   Keep going. You now have a sorted pile of 4.
6. Take two sorted piles of 4 and merge them the same way. Done -- sorted deck of 8.

The key insight: **merging two sorted sequences is easy.** You only ever compare the front
elements. You never have to dig into the middle of a pile. This is what makes merge sort
efficient.

---

## Visual Trace: Sorting [38, 27, 43, 3, 9, 82, 10]

Let's trace the full algorithm on a 7-element array. First the divide phase (splitting),
then the merge phase (combining).

### Divide Phase

```
                       [38, 27, 43, 3, 9, 82, 10]
                      /                            \
              [38, 27, 43, 3]                [9, 82, 10]
              /              \               /          \
         [38, 27]        [43, 3]        [9, 82]        [10]
         /      \        /     \        /      \         |
       [38]    [27]    [43]    [3]    [9]     [82]     [10]
```

Each leaf is a single element -- already sorted by definition.

### Merge Phase

Now we merge bottom-up, combining sorted subarrays at each level:

```
Level 0 (leaves -- already sorted):
       [38]    [27]    [43]    [3]    [9]     [82]     [10]

Level 1 (merge pairs):
       [27, 38]        [3, 43]        [9, 82]         [10]
        ^                ^               ^               ^
    merge(38,27)    merge(43,3)    merge(9,82)       (single)

Level 2 (merge pairs):
        [3, 27, 38, 43]                [9, 10, 82]
               ^                            ^
    merge([27,38],[3,43])         merge([9,82],[10])

Level 3 (final merge):
              [3, 9, 10, 27, 38, 43, 82]
                         ^
             merge([3,27,38,43],[9,10,82])
```

Done. Seven elements, sorted.

### Detailed Merge Trace: merge([3, 27, 38, 43], [9, 10, 82])

Let's zoom into that final merge to see exactly what happens. We have two sorted halves and
an output buffer. At each step, we compare the front elements of both halves and take the
smaller one.

```
left:   [3, 27, 38, 43]    right: [9, 10, 82]    output: []
         ^                          ^
Compare 3 vs 9 => take 3

left:   [3, 27, 38, 43]    right: [9, 10, 82]    output: [3]
             ^                      ^
Compare 27 vs 9 => take 9

left:   [3, 27, 38, 43]    right: [9, 10, 82]    output: [3, 9]
             ^                          ^
Compare 27 vs 10 => take 10

left:   [3, 27, 38, 43]    right: [9, 10, 82]    output: [3, 9, 10]
             ^                              ^
Compare 27 vs 82 => take 27

left:   [3, 27, 38, 43]    right: [9, 10, 82]    output: [3, 9, 10, 27]
                 ^                          ^
Compare 38 vs 82 => take 38

left:   [3, 27, 38, 43]    right: [9, 10, 82]    output: [3, 9, 10, 27, 38]
                     ^                      ^
Compare 43 vs 82 => take 43

left:   [3, 27, 38, 43]    right: [9, 10, 82]    output: [3, 9, 10, 27, 38, 43]
                 (exhausted)                ^
Left is empty. Copy remaining right.

output: [3, 9, 10, 27, 38, 43, 82]    -- done
```

Every element was looked at exactly once during this merge. That is the key: merging two
sorted sequences of combined length n takes exactly n comparisons (at most).

---

## Implementation in Rust

### The Merge Function

This is the workhorse. Given a mutable slice and a midpoint, it merges `arr[..mid]` and
`arr[mid..]` (both assumed sorted) into a single sorted sequence. We need a temporary buffer
because we cannot merge in place without significant complexity.

```rust
fn merge(arr: &mut [i32], mid: usize) {
    // Copy both halves into temporary vectors.
    // This is where the O(n) auxiliary space comes from.
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut i = 0; // index into left
    let mut j = 0; // index into right
    let mut k = 0; // index into arr (output)

    // Compare front elements of both halves, take the smaller one.
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

    // One half is exhausted. Copy the remainder of the other.
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
```

**Why `<=` and not `<`?** Using `<=` in the comparison (`left[i] <= right[j]`) means that
when two elements are equal, we take from the left half first. This preserves the original
relative order of equal elements, which is what makes merge sort **stable**. More on stability
later.

### Top-Down Recursive Merge Sort

```rust
fn merge_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return; // Base case: 0 or 1 elements are already sorted.
    }

    let mid = n / 2;

    // Recursively sort both halves.
    // Rust lets us split a mutable slice into two non-overlapping mutable slices.
    // But we cannot do that across the recursive call and the merge easily,
    // so we sort the halves in place and then merge.
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    // Merge the two sorted halves.
    merge(arr, mid);
}
```

Let's verify it works:

```rust
fn main() {
    let mut data = [38, 27, 43, 3, 9, 82, 10];
    merge_sort(&mut data);
    println!("{:?}", data);
    // Output: [3, 9, 10, 27, 38, 43, 82]
}
```

### A Generic Version

The implementation above works for `i32` only. Let's make it generic over any type that
implements `Ord` (total ordering) and `Clone` (so we can copy into the temp buffer):

```rust
fn merge_sort_generic<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;
    merge_sort_generic(&mut arr[..mid]);
    merge_sort_generic(&mut arr[mid..]);

    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
```

If you want to be more flexible (support partial ordering or custom comparators), you could
take a `FnMut(&T, &T) -> Ordering` closure instead of requiring `Ord`. But `Ord + Clone` is
the practical starting point.

---

## Why O(n log n)? The Complexity Argument

This is one of the cleanest complexity analyses in all of algorithms.

### Time Complexity

Think about what happens at each level of the recursion tree:

```
Level 0:  [................]                        1 array of size n
Level 1:  [........][........]                      2 arrays of size n/2
Level 2:  [....][....][....][....]                  4 arrays of size n/4
Level 3:  [..][..][..][..][..][..][..][..]          8 arrays of size n/8
  ...
Level k:  n/2^k arrays of size 2^k
  ...
Level log n: [.][.][.][.][.][.][.][.][.][.]...      n arrays of size 1
```

**How many levels?** We start with n elements and halve at each level. We stop when subarrays
have size 1. The number of halvings to go from n to 1 is log2(n). So there are **log n levels**.

**How much work at each level?** At every level, we merge all the subarrays at that level. The
merges at level k process every one of the n elements exactly once (each element participates
in exactly one merge at each level). So the total merge work at each level is **O(n)**.

**Total:** O(n) work per level times log(n) levels = **O(n log n)**.

```
Work per level:
  Level 0:  merge n elements                 = n
  Level 1:  merge n/2 + n/2 elements         = n
  Level 2:  merge n/4 + n/4 + n/4 + n/4      = n
  Level 3:  merge n/8 * 8                     = n
  ...
  Level log n: merge 1 * n                    = n
                                               -----
  Total:  n * log(n)                           = O(n log n)
```

This holds for **all cases**: best, worst, and average. Unlike quicksort, merge sort does not
care about the initial ordering of the data. It always splits in half, and the merge always
does linear work. The O(n log n) guarantee is unconditional.

### Space Complexity

The main space cost is the temporary buffer used during merging. At each merge step, we
allocate a temporary copy of the subarray being merged. The largest merge is the final one,
which copies all n elements. So the auxiliary space is **O(n)**.

The recursion stack adds O(log n) frames (one per level), but that is dominated by the O(n)
buffer.

**Total space: O(n).**

This is merge sort's main trade-off compared to quicksort: it needs O(n) extra memory.
Quicksort sorts in place with O(log n) stack space. If memory is tight (embedded systems,
huge arrays), this matters.

---

## Stability

A sort is **stable** if elements with equal keys maintain their original relative order after
sorting.

Example: sorting people by age.

```
Input:   [("Alice", 30), ("Bob", 25), ("Carol", 30), ("Dave", 25)]
Sorted by age:
  Stable:   [("Bob", 25), ("Dave", 25), ("Alice", 30), ("Carol", 30)]
             Bob before Dave (original order).  Alice before Carol (original order).
  Unstable: [("Dave", 25), ("Bob", 25), ("Carol", 30), ("Alice", 30)]
             Dave and Bob swapped. Doesn't preserve original order.
```

**Merge sort is stable.** The key is in the merge step: when `left[i] == right[j]`, we take
from the left. Since left elements came before right elements in the original array, equal
elements maintain their original order.

Why does this matter? When you sort a spreadsheet by column A and then by column B, you want
rows with the same column B value to remain sorted by column A. Stability gives you that for
free. Without it, you need more complex multi-key comparison logic.

---

## Merge Sort Variants

### Bottom-Up Merge Sort

The recursive version is called **top-down** because it starts with the full array and splits
downward. The **bottom-up** variant flips this: start by treating every element as a sorted
run of length 1, merge adjacent pairs into sorted runs of length 2, then merge those into runs
of length 4, and so on.

```
Original:  [38] [27] [43] [3] [9] [82] [10]

Pass 1 (merge runs of 1 into runs of 2):
           [27, 38] [3, 43] [9, 82] [10]

Pass 2 (merge runs of 2 into runs of 4):
           [3, 27, 38, 43] [9, 10, 82]

Pass 3 (merge runs of 4 into one):
           [3, 9, 10, 27, 38, 43, 82]
```

Bottom-up merge sort has the same O(n log n) time and O(n) space complexity. Its advantage:
**no recursion.** This eliminates the O(log n) stack overhead and avoids potential stack
overflow on very large arrays. It is also simpler to implement iteratively.

A sketch in Rust:

```rust
fn merge_sort_bottom_up(arr: &mut [i32]) {
    let n = arr.len();
    let mut width = 1;

    while width < n {
        let mut start = 0;
        while start < n {
            let mid = (start + width).min(n);
            let end = (start + 2 * width).min(n);

            if mid < end {
                merge(&mut arr[start..end], mid - start);
            }
            start += 2 * width;
        }
        width *= 2;
    }
}
```

This reuses the same `merge` function from before. The outer loop doubles the run width each
pass (log n passes), and the inner loop merges all adjacent pairs at the current width.

### Natural Merge Sort

Real-world data is rarely fully random. It often has existing "runs" -- subsequences that are
already sorted. **Natural merge sort** exploits this by scanning for existing sorted runs
instead of blindly splitting at the midpoint.

```
Input:  [3, 27, 38, 43, 9, 82, 10]

Detect runs:
  Run 1: [3, 27, 38, 43]   (already ascending)
  Run 2: [9, 82]            (already ascending)
  Run 3: [10]               (trivially ascending)

Merge run 1 + run 2: [3, 9, 27, 38, 43, 82]
Merge with run 3:    [3, 9, 10, 27, 38, 43, 82]
```

If the input is already sorted, natural merge sort detects one big run and finishes in O(n).
If the input is reverse-sorted, it detects n runs of length 1 and behaves like standard merge
sort.

Python's Timsort (used in Python, Java, and Android) is essentially a highly optimized natural
merge sort. It identifies runs, extends short ones with insertion sort, and merges them with
sophisticated rules to minimize comparisons.

---

## When Merge Sort Wins

Merge sort is not always the best choice, but there are scenarios where it is clearly superior.

### 1. Linked Lists

Sorting a linked list with quicksort is painful. Quicksort needs random access to pick pivots
and partition efficiently. Linked lists only support sequential access.

Merge sort, on the other hand, is a natural fit for linked lists:

- **Splitting** is O(n) using the fast/slow pointer technique (find the middle, cut the list).
- **Merging** two sorted linked lists is O(n) and requires **zero extra space** -- you just
  re-link the existing nodes.

This is a big deal: on linked lists, merge sort achieves O(n log n) time with O(log n) space
(just the recursion stack). No auxiliary array needed. This makes it the standard choice for
sorting linked lists.

### 2. External Sorting

When you have more data than fits in RAM (e.g., sorting a 100 GB file), you cannot load it
all into memory. **External merge sort** works like this:

1. Read chunks that fit in memory, sort each chunk (using any in-memory sort), write sorted
   chunks to disk.
2. Merge sorted chunks using a k-way merge. You only need to keep the front element of each
   chunk in memory.

This is how databases sort large result sets, how `sort` on Unix works for huge files, and
how MapReduce's shuffle phase operates. The merge-based approach naturally extends to data
that lives on disk or across a network.

### 3. Guaranteed O(n log n)

Quicksort is O(n log n) on average but O(n^2) in the worst case. You can mitigate this with
randomized pivot selection, but you cannot eliminate it. Merge sort is O(n log n) always.

If you need a hard guarantee (real-time systems, adversarial input, SLA-bound code), merge
sort delivers where quicksort cannot.

### 4. Stability Required

As discussed above, merge sort is naturally stable. Quicksort is not (the Lomuto and Hoare
partition schemes can rearrange equal elements). If you need a stable sort and cannot use
extra comparisons for a tiebreaker, merge sort is the standard answer.

---

## Merge Sort vs Quick Sort

This comparison comes up constantly. Here is the honest breakdown:

| Property           | Merge Sort           | Quick Sort               |
|--------------------|----------------------|--------------------------|
| Worst-case time    | O(n log n)           | O(n^2)                   |
| Average time       | O(n log n)           | O(n log n)               |
| Best-case time     | O(n log n)           | O(n log n)               |
| Space              | O(n)                 | O(log n) in place        |
| Stable             | Yes                  | No (typically)           |
| Cache behavior     | Worse (copying)      | Better (in-place swaps)  |
| Linked lists       | Excellent            | Poor                     |
| External sorting   | Excellent            | Poor                     |
| Parallelism        | Excellent            | Good                     |
| Adaptive           | No (without natural) | No (without intro)       |

**In practice, quicksort is usually faster for in-memory array sorting** despite having the
same big-O average case. The constant factors are smaller because it works in place (better
cache locality, no allocation overhead). This is why Rust's `slice::sort_unstable()` uses a
quicksort variant (pdqsort), not merge sort.

However, Rust's `slice::sort()` -- the stable sort -- uses a merge sort variant (Timsort-like)
precisely because stability requires merge sort's approach.

So the answer to "which is better?" depends on the constraints:

- Need stability? Merge sort.
- Need guaranteed worst case? Merge sort.
- Sorting linked lists or external data? Merge sort.
- Sorting arrays in memory, no stability needed? Quicksort is likely faster.

---

## The Lower Bound: Why O(n log n) Is Optimal

Here is a satisfying theoretical result. Any **comparison-based** sorting algorithm must make
at least O(n log n) comparisons in the worst case. This is a provable lower bound.

The argument: n elements can be arranged in n! (n factorial) possible orders. Each comparison
eliminates at most half of the remaining possibilities (it gives a yes/no answer). To
distinguish between n! orderings, you need at least log2(n!) comparisons. By Stirling's
approximation:

    log2(n!) ~ n * log2(n)

So any comparison-based sort needs at least O(n log n) comparisons. Merge sort achieves this
bound. It is **asymptotically optimal** among comparison sorts.

Non-comparison sorts (radix sort, counting sort, bucket sort) can beat this bound by
exploiting structure in the keys, but they have their own constraints (keys must be integers
or fixed-length strings, extra space proportional to the key range, etc.).

---

## Common Pitfalls

### 1. Allocating in Every Recursive Call

The naive implementation allocates new vectors for `left` and `right` at every recursive call.
For n elements with log n levels, that is O(n log n) total allocation work. A more efficient
approach allocates a single auxiliary buffer of size n upfront and reuses it at every level:

```rust
fn merge_sort_efficient(arr: &mut [i32]) {
    let mut buf = arr.to_vec();
    merge_sort_with_buf(arr, &mut buf);
}

fn merge_sort_with_buf(arr: &mut [i32], buf: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;
    merge_sort_with_buf(&mut arr[..mid], &mut buf[..mid]);
    merge_sort_with_buf(&mut arr[mid..], &mut buf[mid..]);

    // Copy arr into buf, then merge from buf back into arr.
    buf[..n].copy_from_slice(&arr[..n]);
    let (left, right) = buf[..n].split_at(mid);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

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
```

This keeps the space at O(n) total rather than O(n log n) from repeated allocations.

### 2. Off-by-One in the Split

Always split at `mid = n / 2`. The left half is `arr[..mid]` (indices 0 to mid-1) and the
right half is `arr[mid..]` (indices mid to n-1). Rust's slice syntax makes this hard to get
wrong, but in languages with manual index arithmetic it is a classic source of bugs.

### 3. Forgetting the Base Case

If you forget `if n <= 1 { return; }`, the recursion never terminates. In Rust, you will get
a stack overflow. Always check for the trivial case.

---

## The Recurrence Relation

If you want the formal way to derive the time complexity:

```
T(n) = 2 * T(n/2) + O(n)
       ^^^^^^^^^^^^^   ^^^
     two recursive    merge step
     calls on half    is linear
     the input
```

Applying the Master Theorem (from lesson 01):
- a = 2, b = 2, d = 1
- log_b(a) = log_2(2) = 1 = d

We are in the case where d = log_b(a), so:

    T(n) = O(n^d * log n) = O(n log n)

---

## Exercises

**Exercise 1:** Trace merge sort on `[5, 1, 4, 2, 8]`. Draw the full split tree and then the
merge steps. Write out every comparison made during the final merge.

<details>
<summary>Hint</summary>

The split is: [5, 1, 4] and [2, 8]. Then [5, 1, 4] splits into [5, 1] and [4]. And
[5, 1] splits into [5] and [1]. Build back up from there.

</details>

**Exercise 2:** Modify the generic merge sort to accept a custom comparator
`FnMut(&T, &T) -> std::cmp::Ordering` so you can sort in descending order or by a specific
field of a struct.

<details>
<summary>Hint</summary>

Replace the `Ord` bound with a closure parameter. The merge comparison becomes
`if cmp(&left[i], &right[j]) != Ordering::Greater` to maintain stability.

</details>

**Exercise 3:** Implement bottom-up merge sort for a singly linked list. You do not need
an auxiliary buffer -- just re-link nodes. What is the space complexity?

<details>
<summary>Answer</summary>

O(1) auxiliary space if iterative (no recursion stack), or O(log n) if you use recursion for
the splitting. The merge itself is O(1) extra space because you re-link existing nodes rather
than copying.

</details>

**Exercise 4:** Merge sort's merge step can count **inversions** -- pairs (i, j) where i < j
but arr[i] > arr[j]. Modify merge sort to return the total inversion count. The number of
inversions is a measure of how "unsorted" an array is (0 for sorted, n*(n-1)/2 for
reverse-sorted).

<details>
<summary>Hint</summary>

Every time you take an element from the right half during a merge, it is "jumping over" all
remaining elements in the left half. That count of remaining left elements equals the number
of inversions contributed by that element.

</details>

**Exercise 5:** What happens if you split the array into thirds instead of halves (3-way
merge sort)? What is the time complexity? Is it faster in practice?

<details>
<summary>Answer</summary>

T(n) = 3 * T(n/3) + O(n). By the Master Theorem, this is still O(n log n) -- specifically
O(n * log_3(n)). Since log_3(n) = log_2(n) / log_2(3), the constant factor is smaller. But
the merge step becomes more complex (3-way merge), and in practice the extra comparisons and
branch mispredictions usually negate the theoretical savings. 2-way merge sort is simpler and
typically faster.

</details>

---

## Key Takeaways

1. **Merge sort is divide and conquer**: split in half, sort each half, merge the sorted
   halves. The merge step does all the real work.

2. **O(n log n) always**: best, worst, and average case. No pathological inputs. This
   guarantee is unique among practical comparison sorts.

3. **O(n) extra space**: the auxiliary buffer is the trade-off. Quicksort wins on space.

4. **Stable**: equal elements keep their original relative order. This is why Rust's
   `slice::sort()` uses a merge-sort-based algorithm.

5. **Ideal for linked lists and external sorting**: merge sort naturally adapts to sequential
   access patterns and data too large for memory.

6. **Bottom-up variant eliminates recursion**: same complexity, no stack overhead. Natural
   merge sort exploits existing order in the input.

7. **Comparison sorts cannot beat O(n log n)**: merge sort is asymptotically optimal. Faster
   sorts exist but require non-comparison techniques (radix, counting).

8. **In practice, quicksort is often faster for in-memory arrays** due to cache locality and
   lower constant factors. Choose based on your constraints: stability, worst-case guarantees,
   data structure, and memory budget.

---

Next lesson: [11 - Quick Sort](./11_quick_sort.md)
