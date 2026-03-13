# Lesson 12: Heap Sort

## The Sorting Algorithm Nobody's Favorite -- But Everyone Should Know

Heap sort sits in an unusual place. It is not the fastest in practice (quicksort usually
wins). It does not preserve the order of equal elements (merge sort does). But it has a
combination of properties that no other comparison sort can match: **O(n log n) worst-case
time, O(1) extra space, and no pathological inputs**. When you need a guaranteed time bound
with no auxiliary allocation, heap sort is the only game in town.

To understand heap sort, you first need to understand the binary heap. That is where we
start.

---

## What Is a Binary Heap?

A binary heap is a complete binary tree stored in an array, where every node satisfies the
**heap property**: each parent is greater than or equal to its children (max-heap) or less
than or equal to its children (min-heap).

Two rules define a binary heap:

1. **Shape property** -- It is a *complete* binary tree: every level is fully filled except
   possibly the last, which is filled left to right. No gaps.
2. **Heap property** -- Every parent node compares favorably to its children. In a max-heap,
   every parent is >= both children. In a min-heap, every parent is <= both children.

### A Real-World Analogy: The Tournament Bracket

Think of a single-elimination tournament. After every match, the winner advances to the next
round. The champion -- the overall best -- ends up at the top. If you look at the bracket
from the top down, the winner of any sub-bracket is always better than (or equal to) both
opponents they beat.

```
  Tournament bracket (max-heap):

                    [90]              <-- champion (max element)
                   /    \
               [85]      [70]
              /    \    /    \
           [40]  [80] [60]  [50]
           / \   /
        [10][35][75]

  Every parent beat both children.
  90 >= 85, 90 >= 70
  85 >= 40, 85 >= 80
  70 >= 60, 70 >= 50
  40 >= 10, 40 >= 35
  80 >= 75
```

The champion always "bubbles to the top." That is the heap property. Heap sort exploits
this: repeatedly extract the champion, and you get a sorted sequence.

---

## The Array Representation

Here is the key insight that makes heaps practical: **you do not need pointers or nodes.
A complete binary tree maps perfectly onto a flat array.**

Because the tree is complete (no gaps), you can lay out the nodes level by level, left to
right, into an array. The parent-child relationships are then pure arithmetic:

```
  Tree:
                     [90]                    index 0
                    /    \
                [85]      [70]               indices 1, 2
               /    \    /    \
            [40]  [80] [60]  [50]            indices 3, 4, 5, 6
            / \   /
         [10][35][75]                        indices 7, 8, 9

  Array (same data, level-order):
  index:   0    1    2    3    4    5    6    7    8    9
        +----+----+----+----+----+----+----+----+----+----+
        | 90 | 85 | 70 | 40 | 80 | 60 | 50 | 10 | 35 | 75 |
        +----+----+----+----+----+----+----+----+----+----+
```

### Index Formulas (0-Indexed)

For a node at index `i`:

```
  Parent:       (i - 1) / 2      (integer division)
  Left child:   2 * i + 1
  Right child:  2 * i + 2
```

Let's verify with index 1 (value 85):
- Parent: (1 - 1) / 2 = 0 --> arr[0] = 90.  Correct.
- Left child: 2*1 + 1 = 3 --> arr[3] = 40.  Correct.
- Right child: 2*1 + 2 = 4 --> arr[4] = 80.  Correct.

And index 4 (value 80):
- Parent: (4 - 1) / 2 = 1 --> arr[1] = 85.  Correct.
- Left child: 2*4 + 1 = 9 --> arr[9] = 75.  Correct.
- Right child: 2*4 + 2 = 10 --> out of bounds (no right child).

**This is why heaps are O(1) space for the data structure itself -- the array IS the tree.**
No pointers, no `Box<Node>`, no heap allocation beyond the data you are already sorting. The
tree structure is implicit in the indices.

---

## Max-Heap vs. Min-Heap

The only difference is the direction of the comparison:

| Property | Max-Heap | Min-Heap |
|----------|----------|----------|
| Root contains | Maximum element | Minimum element |
| Parent vs. children | parent >= children | parent <= children |
| Used in heap sort? | Yes (for ascending order) | Yes (for descending order) |
| Used in priority queues? | When you need the max first | When you need the min first |

Rust's standard library `BinaryHeap<T>` is a **max-heap**. For heap sort in ascending order,
we also use a max-heap -- you'll see why shortly.

---

## The Sift-Down Operation (aka Heapify-Down)

This is the core building block. Given a node that might violate the heap property (it is
smaller than one or both of its children), sift-down restores the property by swapping the
node downward until it finds its correct position.

### Step-by-Step Example

Suppose index 0 has value 10 and its children are 85 and 70. The heap property is violated
at the root:

```
  BEFORE sift_down(0):

          [10]            <-- violates heap property (10 < 85 and 10 < 70)
         /    \
      [85]    [70]
     /    \   /  \
   [40] [80][60] [50]

  Step 1: Compare 10 with children 85 and 70.
          Largest child is 85 (index 1).
          10 < 85, so swap.

          [85]
         /    \
      [10]    [70]       <-- 10 is now at index 1
     /    \   /  \
   [40] [80][60] [50]

  Step 2: Compare 10 with its new children 40 and 80.
          Largest child is 80 (index 4).
          10 < 80, so swap.

          [85]
         /    \
      [80]    [70]
     /    \   /  \
   [40] [10][60] [50]   <-- 10 is now at index 4

  Step 3: Compare 10 with its new children.
          Index 4's children would be at indices 9 and 10.
          If those are out of bounds (no children), stop.

  DONE. Heap property restored.
```

The element sinks to its natural level -- like a lightweight object dropped into water,
it floats down until it reaches a layer where it is heavier than everything below it.

### Sift-Down in Rust

```rust
/// Sift the element at `root` down to restore the max-heap property
/// within the subarray arr[0..heap_size].
fn sift_down(arr: &mut [i32], heap_size: usize, root: usize) {
    let mut parent = root;

    loop {
        let left = 2 * parent + 1;
        let right = 2 * parent + 2;
        let mut largest = parent;

        // Check if left child exists and is larger than current largest
        if left < heap_size && arr[left] > arr[largest] {
            largest = left;
        }

        // Check if right child exists and is larger than current largest
        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        // If parent is already the largest, heap property holds
        if largest == parent {
            break;
        }

        // Swap parent with the largest child and continue sifting
        arr.swap(parent, largest);
        parent = largest;
    }
}
```

**Time complexity of sift_down:** O(log n). In the worst case, the element sinks from the
root all the way to a leaf. The height of a complete binary tree with n nodes is
floor(log2(n)), so sift_down does at most log2(n) swaps and comparisons.

---

## Building a Heap: The Bottom-Up Approach

Given an unsorted array, we need to turn it into a valid max-heap. There are two approaches:

1. **Top-down (insert one by one):** Insert elements one at a time, sifting each up. This is
   O(n log n).
2. **Bottom-up (Floyd's algorithm):** Start from the last non-leaf node and sift down every
   node back to the root. This is **O(n)**.

Wait -- O(n)? Building an entire heap in linear time? Yes. This is not obvious, so let's
walk through it.

### Why Bottom-Up Is O(n)

The insight is that most nodes are near the bottom of the tree, where sift-down barely moves
them:

```
  Level 0 (root):       1 node,   sift down up to h levels
  Level 1:              2 nodes,  sift down up to h-1 levels
  Level 2:              4 nodes,  sift down up to h-2 levels
  ...
  Level h-1:         n/4 nodes,   sift down up to 1 level
  Level h (leaves):  n/2 nodes,   sift down 0 levels (SKIP THESE)
```

Half the nodes are leaves -- they're already trivial heaps. A quarter of the nodes need at
most one swap. An eighth need at most two swaps. The total work sums to:

```
  Sum = (n/4)*1 + (n/8)*2 + (n/16)*3 + ... + 1*h
      = n * (1/4 + 2/8 + 3/16 + ...)
      = n * sum_{k=1}^{h} k / 2^(k+1)
      <= n * 1                              (the series converges to ~1)
      = O(n)
```

This is one of those beautiful results: the bottom-heavy shape of the tree means the
expensive operations (deep sift-downs) happen on very few nodes, and the cheap operations
(shallow sift-downs) happen on many nodes.

### Build Max-Heap: Walkthrough

Starting array: `[4, 10, 3, 5, 1, 8, 7, 2, 9, 6]`

```
  Initial tree (NOT a heap):

              [4]
            /     \
         [10]      [3]
        /    \    /    \
      [5]   [1] [8]   [7]
     / \    /
   [2] [9][6]

  n = 10, last non-leaf = (10 / 2) - 1 = 4 (value 1)
  Process indices: 4, 3, 2, 1, 0

  --- sift_down(4): node [1] at index 4 ---
  Left child = index 9 (value 6). Right child = out of bounds.
  6 > 1, swap.
              [4]
            /     \
         [10]      [3]
        /    \    /    \
      [5]   [6] [8]   [7]
     / \    /
   [2] [9][1]

  --- sift_down(3): node [5] at index 3 ---
  Children: index 7 (value 2), index 8 (value 9). Largest child: 9.
  9 > 5, swap.
              [4]
            /     \
         [10]      [3]
        /    \    /    \
      [9]   [6] [8]   [7]
     / \    /
   [2] [5][1]

  --- sift_down(2): node [3] at index 2 ---
  Children: index 5 (value 8), index 6 (value 7). Largest child: 8.
  8 > 3, swap.
              [4]
            /     \
         [10]      [8]
        /    \    /    \
      [9]   [6] [3]   [7]
     / \    /
   [2] [5][1]
  Now at index 5, children would be 11, 12 -- out of bounds. Done.

  --- sift_down(1): node [10] at index 1 ---
  Children: index 3 (value 9), index 4 (value 6). Largest child: 9.
  10 > 9, no swap needed. Done.

  --- sift_down(0): node [4] at index 0 ---
  Children: index 1 (value 10), index 2 (value 8). Largest child: 10.
  10 > 4, swap.
              [10]
            /      \
          [4]      [8]
        /    \    /    \
      [9]   [6] [3]   [7]
     / \    /
   [2] [5][1]
  Now at index 1. Children: index 3 (value 9), index 4 (value 6). Largest: 9.
  9 > 4, swap.
              [10]
            /      \
          [9]      [8]
        /    \    /    \
      [4]   [6] [3]   [7]
     / \    /
   [2] [5][1]
  Now at index 3. Children: index 7 (value 2), index 8 (value 5). Largest: 5.
  5 > 4, swap.
              [10]
            /      \
          [9]      [8]
        /    \    /    \
      [5]   [6] [3]   [7]
     / \    /
   [2] [4][1]
  Now at index 8. No children. Done.

  RESULT -- valid max-heap:
  Array: [10, 9, 8, 5, 6, 3, 7, 2, 4, 1]

  Verify: 10 >= 9, 10 >= 8, 9 >= 5, 9 >= 6, 8 >= 3, 8 >= 7, 5 >= 2, 5 >= 4, 6 >= 1.
  All good.
```

### Build Heap in Rust

```rust
fn build_max_heap(arr: &mut [i32]) {
    let n = arr.len();
    // Start from the last non-leaf node and work backward to the root.
    // The last non-leaf is at index (n / 2) - 1.
    for i in (0..n / 2).rev() {
        sift_down(arr, n, i);
    }
}
```

---

## The Sort: Extract, Swap, Shrink, Repeat

Now the actual sorting algorithm. The idea is simple once you have a max-heap:

1. The root (index 0) is the maximum element.
2. Swap it with the last element in the heap.
3. Shrink the heap size by 1 (the max is now in its final sorted position).
4. The new root probably violates the heap property -- sift it down.
5. Repeat until the heap has one element left.

### Full Walkthrough

Starting from our max-heap: `[10, 9, 8, 5, 6, 3, 7, 2, 4, 1]`

```
  heap_size = 10

  --- Iteration 1 ---
  Swap arr[0] and arr[9]: swap 10 and 1
  [1, 9, 8, 5, 6, 3, 7, 2, 4, | 10]
                                  ^^ sorted region
  heap_size = 9
  sift_down(0): 1 sinks down
  [9, 6, 8, 5, 1, 3, 7, 2, 4, | 10]

  --- Iteration 2 ---
  Swap arr[0] and arr[8]: swap 9 and 4
  [4, 6, 8, 5, 1, 3, 7, 2, | 9, 10]
  heap_size = 8
  sift_down(0): 4 sinks down
  [8, 6, 7, 5, 1, 3, 4, 2, | 9, 10]

  --- Iteration 3 ---
  Swap arr[0] and arr[7]: swap 8 and 2
  [2, 6, 7, 5, 1, 3, 4, | 8, 9, 10]
  heap_size = 7
  sift_down(0): 2 sinks down
  [7, 6, 4, 5, 1, 3, 2, | 8, 9, 10]

  --- Iteration 4 ---
  Swap arr[0] and arr[6]: swap 7 and 2
  [2, 6, 4, 5, 1, 3, | 7, 8, 9, 10]
  heap_size = 6
  sift_down(0):
  [6, 5, 4, 2, 1, 3, | 7, 8, 9, 10]

  --- Iteration 5 ---
  Swap arr[0] and arr[5]: swap 6 and 3
  [3, 5, 4, 2, 1, | 6, 7, 8, 9, 10]
  heap_size = 5
  sift_down(0):
  [5, 3, 4, 2, 1, | 6, 7, 8, 9, 10]

  --- Iteration 6 ---
  Swap arr[0] and arr[4]: swap 5 and 1
  [1, 3, 4, 2, | 5, 6, 7, 8, 9, 10]
  heap_size = 4
  sift_down(0):
  [4, 3, 1, 2, | 5, 6, 7, 8, 9, 10]

  --- Iteration 7 ---
  Swap arr[0] and arr[3]: swap 4 and 2
  [2, 3, 1, | 4, 5, 6, 7, 8, 9, 10]
  heap_size = 3
  sift_down(0):
  [3, 2, 1, | 4, 5, 6, 7, 8, 9, 10]

  --- Iteration 8 ---
  Swap arr[0] and arr[2]: swap 3 and 1
  [1, 2, | 3, 4, 5, 6, 7, 8, 9, 10]
  heap_size = 2
  sift_down(0):
  [2, 1, | 3, 4, 5, 6, 7, 8, 9, 10]

  --- Iteration 9 ---
  Swap arr[0] and arr[1]: swap 2 and 1
  [1, | 2, 3, 4, 5, 6, 7, 8, 9, 10]
  heap_size = 1
  Done. One element remaining = trivially sorted.

  RESULT: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
```

Notice the pattern: the sorted region grows from right to left, and the heap region shrinks
from right to left. At every step, the largest remaining element is pulled out of the heap
and placed in its correct final position. This is why we use a max-heap for ascending sort.

---

## The Complete Implementation

Putting it all together:

```rust
/// Sift the element at `root` down to restore the max-heap property
/// within arr[0..heap_size].
fn sift_down(arr: &mut [i32], heap_size: usize, root: usize) {
    let mut parent = root;

    loop {
        let left = 2 * parent + 1;
        let right = 2 * parent + 2;
        let mut largest = parent;

        if left < heap_size && arr[left] > arr[largest] {
            largest = left;
        }
        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        if largest == parent {
            break;
        }

        arr.swap(parent, largest);
        parent = largest;
    }
}

/// Sort `arr` in ascending order using heap sort.
fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Phase 1: Build a max-heap. O(n).
    for i in (0..n / 2).rev() {
        sift_down(arr, n, i);
    }

    // Phase 2: Repeatedly extract the max. O(n log n).
    for end in (1..n).rev() {
        // The max element is at arr[0]. Move it to its final position.
        arr.swap(0, end);
        // Shrink the heap and restore the heap property.
        sift_down(arr, end, 0);
    }
}

fn main() {
    let mut data = vec![4, 10, 3, 5, 1, 8, 7, 2, 9, 6];
    println!("Before: {:?}", data);
    heap_sort(&mut data);
    println!("After:  {:?}", data);
    // After:  [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
}
```

### A Generic Version

The version above works only for `i32`. Here is a generic version that sorts anything
implementing `Ord`:

```rust
fn sift_down<T: Ord>(arr: &mut [T], heap_size: usize, root: usize) {
    let mut parent = root;

    loop {
        let left = 2 * parent + 1;
        let right = 2 * parent + 2;
        let mut largest = parent;

        if left < heap_size && arr[left] > arr[largest] {
            largest = left;
        }
        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        if largest == parent {
            break;
        }

        arr.swap(parent, largest);
        parent = largest;
    }
}

fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    for i in (0..n / 2).rev() {
        sift_down(arr, n, i);
    }

    for end in (1..n).rev() {
        arr.swap(0, end);
        sift_down(arr, end, 0);
    }
}
```

Note: Rust's `Ord` trait requires total ordering. For types with partial ordering (like
`f64` which has `NaN`), you would use a custom comparator or `total_cmp()`.

---

## Complexity Analysis

### Time Complexity

| Phase | Work | Why |
|-------|------|-----|
| Build max-heap | O(n) | Bottom-up heapify (Floyd's algorithm) |
| Extract max (n-1 times) | O(n log n) | Each extraction does one swap + one sift_down of O(log n) |
| **Total** | **O(n log n)** | The extraction phase dominates |

Critically, this is O(n log n) in **all cases**. There is no "bad input" for heap sort the
way that a naive quicksort degrades to O(n^2) on already-sorted data. The structure of the
heap guarantees that sift_down always takes at most log(n) steps, regardless of the input
distribution.

### Space Complexity

**O(1) auxiliary space.** The sort is performed entirely within the input array. The heap is
the array. The sorted region grows inside the same array. No temporary buffers, no recursive
calls on the stack (the sift_down above is iterative). This makes heap sort one of the very
few O(n log n) sorts that is truly in-place.

### Stability

Heap sort is **NOT stable**. Equal elements may be reordered. Consider two elements with the
same value at different positions in the array. The heap-building and extraction process
does not preserve their original relative order.

Example: if you sort `[(3, 'a'), (1, 'b'), (3, 'c')]` by the first field, a stable sort
guarantees `(3, 'a')` comes before `(3, 'c')` in the output. Heap sort does not make that
guarantee.

If stability matters, use merge sort or Rust's built-in `sort_stable()`.

---

## Heap Sort vs. The Competition

Here is how heap sort compares to the other O(n log n) sorts:

```
                    Time                    Space    Stable?   In-place?
              Best     Avg      Worst
  ──────────────────────────────────────────────────────────────────────
  Heap sort   O(n log n) O(n log n) O(n log n)  O(1)     No       Yes
  Merge sort  O(n log n) O(n log n) O(n log n)  O(n)     Yes      No*
  Quick sort  O(n log n) O(n log n) O(n^2)      O(log n) No       Yes**
  ──────────────────────────────────────────────────────────────────────

  *  In-place merge sort exists but is complex and slow in practice.
  ** Quicksort's O(log n) space is for the recursion stack.
```

### When to choose each:

**Quicksort** is the default choice for general-purpose sorting. It has excellent cache
locality (sequential array scans), small constant factors, and the O(n^2) worst case can
be avoided with randomized pivots or median-of-three. Rust's `sort_unstable()` uses a
pattern-defeating quicksort variant (pdqsort) for exactly these reasons.

**Merge sort** is the choice when stability matters or when you are sorting linked structures
(where random access is expensive). Rust's `sort()` (the stable one) uses a merge sort
variant (TimSort). It needs O(n) extra memory for the merge buffer.

**Heap sort** is the choice when:
- You need a **hard O(n log n) worst-case guarantee** (e.g., real-time systems, adversarial
  inputs, security-sensitive contexts where an attacker might craft worst-case data).
- You need **O(1) extra memory** (e.g., embedded systems with tight memory constraints).
- You are implementing a **hybrid sort** -- some real-world sort implementations fall back to
  heap sort when quicksort's recursion depth exceeds a threshold (this is called introsort).
- You only need the **top k elements** from a large dataset. Building a heap and extracting
  k times is O(n + k log n), which beats full sorting when k << n.

### Why Heap Sort Loses in Practice

Despite its theoretical guarantees, heap sort is typically 2-3x slower than quicksort on
random data. The reason is cache behavior. Quicksort scans elements sequentially (great for
the CPU prefetcher). Heap sort's sift_down operation jumps from parent to child -- indices
like 0, 1, 3, 7, 15 -- which leaps across the array in exponentially growing strides. At
the deeper levels of a large heap, parent-to-child traversals cause frequent cache misses.

This is a concrete example of why Big-O alone does not determine practical performance.
Constants matter. Cache effects matter. But Big-O still tells you the fundamental scaling
story, and heap sort's scaling story is the most robust of the three.

---

## A Visual Summary of the Full Algorithm

```
  Input:  [4, 10, 3, 5, 1, 8, 7, 2, 9, 6]

  ┌──────────────────────────────────────────────┐
  │  Phase 1: BUILD MAX-HEAP (bottom-up)  O(n)   │
  │                                              │
  │  Process non-leaf nodes from right to left:  │
  │  indices 4, 3, 2, 1, 0                       │
  │  Each call to sift_down pushes a small       │
  │  value down to its correct level.            │
  └──────────────────────────────────────────────┘

  Max-heap: [10, 9, 8, 5, 6, 3, 7, 2, 4, 1]

              [10]
            /      \
          [9]      [8]
        /    \    /    \
      [5]   [6] [3]   [7]
     / \    /
   [2] [4][1]

  ┌──────────────────────────────────────────────┐
  │  Phase 2: EXTRACT MAX (n-1 times)  O(n log n)│
  │                                              │
  │  Repeat:                                     │
  │    1. Swap root (max) with last heap element │
  │    2. Shrink heap boundary by 1              │
  │    3. Sift down the new root                 │
  └──────────────────────────────────────────────┘

  After each extraction:

  [9, 6, 8, 5, 1, 3, 7, 2, 4 | 10]
  [8, 6, 7, 5, 1, 3, 4, 2 | 9, 10]
  [7, 6, 4, 5, 1, 3, 2 | 8, 9, 10]
  [6, 5, 4, 2, 1, 3 | 7, 8, 9, 10]
  [5, 3, 4, 2, 1 | 6, 7, 8, 9, 10]
  [4, 3, 1, 2 | 5, 6, 7, 8, 9, 10]
  [3, 2, 1 | 4, 5, 6, 7, 8, 9, 10]
  [2, 1 | 3, 4, 5, 6, 7, 8, 9, 10]
  [1 | 2, 3, 4, 5, 6, 7, 8, 9, 10]
        ^^^^^^^^^^^^^^^^^^^^^^^^^^
        sorted region grows right-to-left

  Result: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
```

---

## Common Mistakes and Pitfalls

**Off-by-one in the last non-leaf index.** The last non-leaf is at `(n / 2) - 1`. Starting
sift_down from `n - 1` (a leaf) wastes time but does not break correctness. Starting from
`n / 2` (without the `-1`) misses one node in some cases.

**Forgetting to shrink the heap size.** After swapping the root to the end, sift_down must
operate on `end` elements, not `n`. If you always pass `n`, you will corrupt the sorted
region.

**Confusing sift-down with sift-up.** Sift-down pushes a node *toward the leaves*; sift-up
pushes a node *toward the root*. Heap sort uses sift-down exclusively. Sift-up is used
when inserting into a heap (relevant for priority queues -- covered in Part 3).

**Trying to sort descending with a max-heap.** A max-heap naturally produces ascending order
via the extract-max process. For descending order, use a min-heap (flip the comparisons in
sift_down).

---

## What Comes Next: Heaps Beyond Sorting

We have barely scratched the surface of heaps. Heap sort is one application, but the heap
data structure is far more versatile. In **Part 3: Heaps & Priority Queues**, we will cover:

- Priority queues and how `BinaryHeap<T>` works in Rust's standard library
- The insert operation and sift-up
- Decrease-key / increase-key for algorithms like Dijkstra's shortest path
- The heap as a building block for median-finding, k-way merging, and scheduling
- d-ary heaps and when they outperform binary heaps

For now, the important takeaway is: a heap is a complete binary tree in an array, and the
sift_down operation is its fundamental primitive. Everything else builds on that.

---

## Key Takeaways

1. **A binary heap is a complete binary tree stored in a flat array.** Parent-child
   relationships are computed with arithmetic: `left = 2i + 1`, `right = 2i + 2`,
   `parent = (i - 1) / 2`.

2. **Sift-down is the core operation.** It restores the heap property by pushing a violating
   node downward. It runs in O(log n) time.

3. **Building a heap bottom-up is O(n)**, not O(n log n). Most nodes are near the bottom
   where sift-down is cheap.

4. **Heap sort: build a max-heap, then extract the max n-1 times.** Each extraction swaps
   the root to the sorted region and sift-downs the new root.

5. **O(n log n) worst case, O(1) space, but not stable.** This is heap sort's unique selling
   point: guaranteed performance with no extra memory.

6. **In practice, quicksort is faster** due to cache locality. Heap sort is the fallback for
   worst-case guarantees, memory constraints, or partial sorting (top-k problems).

---

## Exercises

1. **Implement heap sort from scratch** without looking at the code above. Verify it on
   inputs: already sorted, reverse sorted, all equal elements, single element, empty array.

2. **Trace build_max_heap by hand** on the array `[1, 2, 3, 4, 5, 6, 7]`. Draw the tree
   at each step. What does the final max-heap array look like?

3. **Count comparisons.** Modify `sift_down` to count and return the number of comparisons
   made. Run heap sort on arrays of size 100, 1000, and 10000. Verify that the total
   comparisons are roughly proportional to n * log2(n).

4. **Partial sort.** Modify heap sort to find only the top-k largest elements. What is the
   time complexity? (Hint: build the heap in O(n), then extract only k times.)

5. **Generic heap sort in Rust.** Implement heap sort that works on `&mut [T]` where
   `T: Ord`. Test it with `String`, `(i32, &str)` (sort by first element), and a custom
   struct.

<details>
<summary>Hint for Exercise 2</summary>

Start from the tree representation of `[1, 2, 3, 4, 5, 6, 7]`:

```
          [1]
        /     \
      [2]      [3]
     /   \    /   \
   [4]  [5] [6]  [7]
```

The last non-leaf is index 2 (value 3). Process indices 2, 1, 0.

Index 2: children are 6 and 7. 7 > 3, swap.
Index 1: children are 4 and 5. 5 > 2, swap.
Index 0: children are now 5 and 7. 7 > 1, swap. Continue sifting: children of new position
are 6 and 3. 6 > 1, swap.

Final: `[7, 5, 6, 4, 2, 1, 3]`

</details>

---

*Next up: [Lesson 13](./13_counting_radix_sort.md) -- Counting Sort & Radix Sort, where we
break the O(n log n) comparison-sort barrier by not comparing at all.*
