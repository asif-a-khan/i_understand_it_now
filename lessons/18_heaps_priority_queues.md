# Lesson 18: Heaps & Priority Queues

## Beyond Sorting -- The Heap as a Data Structure

In [Lesson 12](./12_heap_sort.md) we met the binary heap and used it for one specific
purpose: sorting. We introduced the array-based complete binary tree, the heap property,
and sift-down. That was the heap as a *tool for sorting*. This lesson is about the heap as
a *data structure in its own right* -- one that powers priority queues, graph algorithms,
streaming computations, and more.

If heap sort was about extracting elements from a fixed collection, this lesson is about a
*living* collection: one that grows, shrinks, and always keeps its highest-priority element
instantly accessible.

---

## Quick Recap: Heap Fundamentals

Since we covered this in Lesson 12, here is the short version.

A **binary heap** is a complete binary tree stored in a flat array. Two invariants hold:

1. **Shape property** -- the tree is complete: every level full except possibly the last,
   which is filled left to right.
2. **Heap property** -- every parent compares favorably to its children. In a max-heap,
   `parent >= children`. In a min-heap, `parent <= children`.

The array layout gives us parent-child relationships through arithmetic:

```
  For a node at index i (0-indexed):

    Parent:       (i - 1) / 2
    Left child:   2 * i + 1
    Right child:  2 * i + 2
```

```
  Example max-heap:

  Tree view:                           Array view:
           [90]                        index: 0  1  2  3  4  5  6
          /    \                             +--+--+--+--+--+--+--+
       [70]    [80]                          |90|70|80|30|50|60|40|
      /    \   /  \                          +--+--+--+--+--+--+--+
   [30]  [50][60] [40]
```

In Lesson 12 we focused on **sift-down** -- the operation that restores the heap property
by pushing a violating node toward the leaves. That is half the story. The other half is
**sift-up**, which pushes a node toward the root. Sift-down drives extraction. Sift-up
drives insertion. Together, they make the heap a dynamic data structure.

---

## The Priority Queue: An Abstract Data Type

Before diving into more heap mechanics, let's define what we are building.

A **priority queue** is an abstract data type (ADT) with these operations:

| Operation | Description | Typical Complexity |
|-----------|-------------|-------------------|
| `insert(item, priority)` | Add an item with a given priority | O(log n) |
| `extract_max` / `extract_min` | Remove and return the highest-priority item | O(log n) |
| `peek` | Look at the highest-priority item without removing it | O(1) |

Notice what is missing: there is no "get the 5th item" or "iterate in order." A priority
queue makes one promise: **the highest-priority element is always available in O(1), and
you can insert or remove in O(log n).**

### The Key Distinction

A **priority queue** is an *abstraction* -- a contract specifying which operations exist
and their semantics. A **binary heap** is a *concrete implementation* of that abstraction.
You could also implement a priority queue with a sorted array, an unsorted array, a
balanced BST, or a Fibonacci heap. Each trades off differently:

```
  Implementation       insert     extract_min    peek
  ─────────────────────────────────────────────────────
  Unsorted array       O(1)       O(n)           O(n)
  Sorted array         O(n)       O(1)           O(1)
  Binary heap          O(log n)   O(log n)       O(1)      <-- the sweet spot
  Balanced BST         O(log n)   O(log n)       O(log n)
  Fibonacci heap*      O(1)*      O(log n)*      O(1)
  ─────────────────────────────────────────────────────
  * amortized
```

The binary heap wins in practice for almost all use cases: its operations are all O(log n)
or better, the constant factors are tiny (just array indexing and swaps), and the memory
layout is cache-friendly. This is why every standard library uses it.

---

## Real-World Analogies

### Emergency Room Triage

Imagine a hospital emergency room. Patients arrive at various times with various severities.
A patient with a heart attack gets seen before someone with a sprained ankle, even if the
ankle patient arrived first. The triage nurse assigns each patient a priority level, and the
next patient seen is always the one with the highest urgency -- regardless of arrival order.

That is a priority queue. The "extract" operation pulls out the most critical patient. New
arrivals are "inserted" with their priority. The triage desk can always tell you who is next
("peek") without discharging anyone.

A regular queue (FIFO) would serve patients in arrival order. A priority queue serves them
in priority order. The binary heap is the mechanism that makes this efficient.

### Airline Boarding

When boarding a plane, passengers do not board in the order they arrived at the gate.
First class boards first, then business, then priority groups, then general boarding zones.
Within each priority level, you might maintain arrival order, but the priority level
dominates. The gate agent is running a priority queue.

### Operating System Task Scheduler

Your OS has many processes competing for CPU time. Each has a priority. The scheduler
always picks the highest-priority runnable process next. When a new process arrives or an
existing one changes priority, the scheduler adjusts. This is a priority queue in action,
and many real schedulers use heap-based structures internally.

---

## Sift-Up: The Insert Operation

In Lesson 12, we only needed sift-down because heap sort builds the heap bottom-up and then
extracts from the top. But when you are *inserting* a new element into an existing heap, you
need sift-up.

### The Idea

1. Place the new element at the **end** of the array (the next available leaf position).
   This preserves the shape property.
2. The heap property might now be violated: the new element could be larger than its parent
   (max-heap) or smaller than its parent (min-heap).
3. **Sift up**: compare the element with its parent. If it outranks the parent, swap them.
   Repeat until the element is in the right place or reaches the root.

### Step-by-Step: Insert 85 into a Max-Heap

```
  Starting max-heap:

           [90]
          /    \
       [70]    [80]
      /    \   /
   [30]  [50][60]

  Array: [90, 70, 80, 30, 50, 60]
  Size: 6

  Step 1: Append 85 at index 6 (next leaf position).

           [90]
          /    \
       [70]    [80]
      /    \   /  \
   [30]  [50][60] [85]

  Array: [90, 70, 80, 30, 50, 60, 85]

  Step 2: Compare 85 with its parent.
          Parent of index 6 = (6-1)/2 = index 2, value 80.
          85 > 80 --> swap.

           [90]
          /    \
       [70]    [85]        <-- 85 moved up
      /    \   /  \
   [30]  [50][60] [80]

  Array: [90, 70, 85, 30, 50, 60, 80]

  Step 3: Compare 85 with its new parent.
          Parent of index 2 = (2-1)/2 = index 0, value 90.
          85 < 90 --> stop. Heap property holds.

  DONE. Final max-heap:

           [90]
          /    \
       [70]    [85]
      /    \   /  \
   [30]  [50][60] [80]

  Array: [90, 70, 85, 30, 50, 60, 80]
```

### Another Insert: 95 into the Same Heap

```
  Step 1: Append 95 at index 7.

           [90]
          /    \
       [70]    [85]
      /    \   /  \
   [30]  [50][60] [80]
    /
  [95]

  Array: [90, 70, 85, 30, 50, 60, 80, 95]

  Step 2: Parent of index 7 = index 3, value 30.
          95 > 30 --> swap.

           [90]
          /    \
       [70]    [85]
      /    \   /  \
   [95]  [50][60] [80]
    /
  [30]

  Array: [90, 70, 85, 95, 50, 60, 80, 30]

  Step 3: Parent of index 3 = index 1, value 70.
          95 > 70 --> swap.

           [90]
          /    \
       [95]    [85]
      /    \   /  \
   [70]  [50][60] [80]
    /
  [30]

  Array: [90, 95, 85, 70, 50, 60, 80, 30]

  Step 4: Parent of index 1 = index 0, value 90.
          95 > 90 --> swap.

           [95]
          /    \
       [90]    [85]
      /    \   /  \
   [70]  [50][60] [80]
    /
  [30]

  Array: [95, 90, 85, 70, 50, 60, 80, 30]

  Step 5: Index 0 is the root. Stop.

  DONE. 95 bubbled all the way to the top.
```

The element "floats up" to its natural position -- like a balloon released underwater, it
rises until it reaches a level where it belongs.

### Sift-Up in Rust

```rust
fn sift_up(arr: &mut [i32], mut idx: usize) {
    while idx > 0 {
        let parent = (idx - 1) / 2;
        if arr[idx] > arr[parent] {
            arr.swap(idx, parent);
            idx = parent;
        } else {
            break;
        }
    }
}
```

**Time complexity:** O(log n). The element travels at most from a leaf to the root, which
is the height of the tree: floor(log2(n)).

---

## Extract-Max with Sift-Down (Revisited)

We covered sift-down in Lesson 12, but let's see it in the context of a priority queue
rather than sorting. The extract operation removes and returns the root (the maximum in a
max-heap).

### The Procedure

1. Save the root value (the max).
2. Move the **last** element to the root. This preserves the shape property.
3. Sift the new root down to restore the heap property.

### Step-by-Step: Extract-Max

```
  Starting max-heap:

           [95]
          /    \
       [90]    [85]
      /    \   /  \
   [70]  [50][60] [80]
    /
  [30]

  Array: [95, 90, 85, 70, 50, 60, 80, 30]

  Step 1: Save 95 (the max). Replace root with the last element (30).
          Reduce size by 1.

           [30]                  <-- former last element, now at root
          /    \
       [90]    [85]
      /    \   /  \
   [70]  [50][60] [80]

  Array: [30, 90, 85, 70, 50, 60, 80]

  Step 2: Sift down. Children of index 0: index 1 (90), index 2 (85).
          Largest child: 90.
          30 < 90 --> swap.

           [90]
          /    \
       [30]    [85]
      /    \   /  \
   [70]  [50][60] [80]

  Step 3: Children of index 1: index 3 (70), index 4 (50).
          Largest child: 70.
          30 < 70 --> swap.

           [90]
          /    \
       [70]    [85]
      /    \   /  \
   [30]  [50][60] [80]

  Step 4: Children of index 3: index 7. Out of bounds (size is 7).
          No children. Stop.

  DONE. Extracted 95. Remaining heap:

           [90]
          /    \
       [70]    [85]
      /    \   /  \
   [30]  [50][60] [80]

  Array: [90, 70, 85, 30, 50, 60, 80]
```

**Time complexity:** O(log n), same as sift-up. The element sinks at most from root to
leaf.

---

## Building a Min-Heap from Scratch

Let's put sift-up and sift-down together into a complete, working min-heap. We use a
min-heap here because it is the more common variant for priority queues (you typically
want the smallest/most-urgent item first).

```rust
/// A min-heap: the smallest element is always at the root.
struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    /// Build a heap from an existing vector in O(n).
    fn from_vec(mut data: Vec<i32>) -> Self {
        let n = data.len();
        // Sift down every non-leaf node, starting from the bottom.
        for i in (0..n / 2).rev() {
            Self::sift_down_slice(&mut data, i);
        }
        MinHeap { data }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Look at the minimum element without removing it. O(1).
    fn peek(&self) -> Option<&i32> {
        self.data.first()
    }

    /// Insert a new element. O(log n).
    fn push(&mut self, val: i32) {
        self.data.push(val);              // append to end (preserves shape)
        self.sift_up(self.data.len() - 1); // restore heap property
    }

    /// Remove and return the minimum element. O(log n).
    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);          // move min to end
        let min = self.data.pop();        // remove it
        if !self.data.is_empty() {
            Self::sift_down_slice(&mut self.data, 0); // restore heap property
        }
        min
    }

    /// Sift a node UP toward the root. Used after insertion.
    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            // Min-heap: child should be >= parent. If child < parent, swap.
            if self.data[idx] < self.data[parent] {
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    /// Sift a node DOWN toward the leaves. Used after extraction.
    fn sift_down_slice(data: &mut [i32], root: usize) {
        let n = data.len();
        let mut parent = root;

        loop {
            let left = 2 * parent + 1;
            let right = 2 * parent + 2;
            let mut smallest = parent;

            if left < n && data[left] < data[smallest] {
                smallest = left;
            }
            if right < n && data[right] < data[smallest] {
                smallest = right;
            }

            if smallest == parent {
                break;
            }

            data.swap(parent, smallest);
            parent = smallest;
        }
    }
}

fn main() {
    let mut heap = MinHeap::new();
    heap.push(40);
    heap.push(10);
    heap.push(30);
    heap.push(5);
    heap.push(20);

    // Elements come out in sorted order (ascending).
    while let Some(val) = heap.pop() {
        print!("{val} ");
    }
    // Output: 5 10 20 30 40
    println!();

    // Build from an existing vec in O(n):
    let heap2 = MinHeap::from_vec(vec![40, 10, 30, 5, 20]);
    assert_eq!(heap2.peek(), Some(&5));
}
```

Notice `from_vec` -- this uses the bottom-up heap construction from Lesson 12, which runs
in O(n) rather than the O(n log n) you would get from inserting elements one at a time.
When you have all your data upfront, always prefer the O(n) build.

---

## Rust's `BinaryHeap<T>` -- The Standard Library Solution

Rust provides a production-quality heap in `std::collections::BinaryHeap`. You should use
it instead of rolling your own in real code.

### The Catch: It's a Max-Heap

`BinaryHeap<T>` is a **max-heap**. The largest element is at the top. This is the opposite
convention from many algorithms textbooks and other languages (C++ `priority_queue` is also
max by default; Python's `heapq` is min by default; Java's `PriorityQueue` is min by
default).

### Basic API

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();

    // push: insert an element. O(log n).
    heap.push(30);
    heap.push(10);
    heap.push(50);
    heap.push(20);
    heap.push(40);

    // peek: look at the max without removing. O(1).
    assert_eq!(heap.peek(), Some(&50));

    // pop: remove and return the max. O(log n).
    assert_eq!(heap.pop(), Some(50));
    assert_eq!(heap.pop(), Some(40));
    assert_eq!(heap.pop(), Some(30));
    assert_eq!(heap.pop(), Some(20));
    assert_eq!(heap.pop(), Some(10));
    assert_eq!(heap.pop(), None);

    // Build from a Vec in O(n) using From/Into:
    let heap2 = BinaryHeap::from(vec![5, 3, 8, 1, 9]);
    // Internal heapification happens automatically.
    assert_eq!(heap2.peek(), Some(&9));

    // len, is_empty:
    let heap3: BinaryHeap<i32> = BinaryHeap::new();
    assert!(heap3.is_empty());
    assert_eq!(heap3.len(), 0);
}
```

### Building from an Iterator

```rust
use std::collections::BinaryHeap;

fn main() {
    // collect() into a BinaryHeap builds in O(n).
    let data = vec![4, 1, 7, 3, 9, 2];
    let heap: BinaryHeap<_> = data.into_iter().collect();
    assert_eq!(heap.peek(), Some(&9));

    // into_sorted_vec: consume the heap and return a sorted Vec.
    // This is essentially heap sort. O(n log n).
    let heap = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    let sorted = heap.into_sorted_vec();
    assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
}
```

### Converting to a Vec and Back

```rust
use std::collections::BinaryHeap;

fn main() {
    let heap = BinaryHeap::from(vec![10, 20, 30]);

    // into_vec: get the underlying array (NOT sorted, it's in heap order).
    let raw: Vec<i32> = heap.into_vec();
    // raw could be [30, 20, 10] or [30, 10, 20] -- valid heap order, not sorted.
    println!("{raw:?}");
}
```

---

## Getting a Min-Heap: `Reverse` and Custom Ordering

Since `BinaryHeap` is a max-heap, you need a strategy to get min-heap behavior. Rust gives
you two main approaches.

### Approach 1: `std::cmp::Reverse`

The `Reverse` wrapper flips the ordering of any `Ord` type. Wrapping your values in
`Reverse` turns a max-heap into a min-heap:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut min_heap = BinaryHeap::new();

    // Wrap every value in Reverse on the way in.
    min_heap.push(Reverse(30));
    min_heap.push(Reverse(10));
    min_heap.push(Reverse(50));
    min_heap.push(Reverse(20));

    // The "max" of Reverse values is the one with the smallest inner value.
    assert_eq!(min_heap.peek(), Some(&Reverse(10)));

    // Unwrap on the way out.
    assert_eq!(min_heap.pop(), Some(Reverse(10)));
    assert_eq!(min_heap.pop(), Some(Reverse(20)));
    assert_eq!(min_heap.pop(), Some(Reverse(30)));
    assert_eq!(min_heap.pop(), Some(Reverse(50)));
}
```

How does this work? `Reverse<T>` implements `Ord` by flipping the comparison:

```rust
// Simplified version of what std does:
impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)  // note: reversed
    }
}
```

So `Reverse(10) > Reverse(50)` because 10 < 50, and the comparison is flipped. The
max-heap now extracts the smallest original value first.

The `Reverse` wrapper is zero-cost -- it's a newtype, so it compiles down to the same
code as the inner type with the comparison flipped.

### Approach 2: Custom Ord on a Wrapper Struct

When you need more control -- for example, when priority depends on a field of a struct --
you implement `Ord` yourself:

```rust
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Task {
    priority: u32,  // lower number = higher urgency (like the ER)
    name: String,
}

// We want the task with the LOWEST priority number to come out first.
// Since BinaryHeap is a max-heap, we reverse the comparison.
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse: other.priority compared to self.priority
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut queue = BinaryHeap::new();

    queue.push(Task { priority: 3, name: "sprained ankle".to_string() });
    queue.push(Task { priority: 1, name: "heart attack".to_string() });
    queue.push(Task { priority: 2, name: "broken arm".to_string() });
    queue.push(Task { priority: 1, name: "stroke".to_string() });

    while let Some(task) = queue.pop() {
        println!("Treating: {} (priority {})", task.name, task.priority);
    }
    // Output:
    // Treating: heart attack (priority 1)
    // Treating: stroke (priority 1)
    // Treating: broken arm (priority 2)
    // Treating: sprained ankle (priority 3)
}
```

**Important Rust trait rule:** when you implement `Ord`, you must also implement `PartialOrd`,
`Eq`, and `PartialEq`. The derive for `Eq` and `PartialEq` handles the equality side; you
write `Ord` and `PartialOrd` manually to control the ordering.

### Approach 3: Reverse with Tuples (Quick and Practical)

For many algorithms, you don't need a full struct. You can use `Reverse` on tuples:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    // Min-heap of (distance, node_id) -- useful for Dijkstra's.
    let mut heap: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();

    heap.push(Reverse((5, 0)));   // node 0, distance 5
    heap.push(Reverse((2, 1)));   // node 1, distance 2
    heap.push(Reverse((8, 2)));   // node 2, distance 8

    // Extracts by smallest distance first:
    let Reverse((dist, node)) = heap.pop().unwrap();
    assert_eq!((dist, node), (2, 1));
}
```

Tuples implement `Ord` lexicographically, so `(2, 1) < (5, 0) < (8, 2)`. Wrapping in
`Reverse` gets you a min-heap ordered by the first tuple element (distance), with the
second element (node ID) as a tiebreaker. This pattern shows up constantly in graph
algorithms.

---

## Building a Heap in O(n) -- Why It Matters

We proved in Lesson 12 that bottom-up heap construction (Floyd's algorithm) runs in O(n),
not O(n log n). Here's why this matters beyond sorting.

If you have a batch of data and need a priority queue, you have two choices:

```
  Option A: Insert one at a time.
    n insertions * O(log n) each = O(n log n)

  Option B: Dump everything into an array, then heapify.
    Build heap = O(n)
```

Option B is strictly better. In Rust:

```rust
use std::collections::BinaryHeap;

fn main() {
    let data = vec![42, 17, 93, 5, 68, 31, 7, 84];

    // Option A: one at a time -- O(n log n)
    let mut heap_a = BinaryHeap::new();
    for val in &data {
        heap_a.push(*val);
    }

    // Option B: build from vec -- O(n)
    let heap_b = BinaryHeap::from(data);

    // Both produce valid max-heaps, but Option B is faster.
    assert_eq!(heap_a.peek(), heap_b.peek());
}
```

`BinaryHeap::from(vec)` and `.collect()` both use the O(n) build internally. Use them
whenever you have the data upfront.

---

## Common Applications

### 1. Top-K Elements

**Problem:** Given a stream of n elements, find the k largest.

**Naive approach:** Sort everything, take the last k. O(n log n).

**Heap approach:** Maintain a min-heap of size k. For each element: if the heap has fewer
than k elements, push. Otherwise, if the element is larger than the heap's minimum, pop the
minimum and push the new element.

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn top_k(stream: &[i32], k: usize) -> Vec<i32> {
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for &val in stream {
        if min_heap.len() < k {
            min_heap.push(Reverse(val));
        } else if let Some(&Reverse(min)) = min_heap.peek() {
            if val > min {
                min_heap.pop();
                min_heap.push(Reverse(val));
            }
        }
    }

    // Extract results (not necessarily sorted).
    min_heap.into_iter().map(|Reverse(v)| v).collect()
}

fn main() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut result = top_k(&data, 3);
    result.sort();
    assert_eq!(result, vec![5, 6, 9]);
}
```

**Why a min-heap for the largest-k?** Because you want to quickly discard the smallest
element in your candidate set. The min-heap keeps the gatekeeper (the smallest of the top-k)
at the root. If a new element can't beat the gatekeeper, it's immediately rejected.

**Time complexity:** O(n log k). Each of the n elements does at most one push and one pop
on a heap of size k. When k is much smaller than n (e.g., "find the top 10 from a million
items"), this is dramatically faster than sorting.

### 2. Merge K Sorted Lists

**Problem:** Given k sorted arrays, merge them into one sorted output.

**Approach:** Use a min-heap to always know which array has the smallest current element.
Push the first element of each array. Pop the minimum, and push the next element from that
same array.

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn merge_k_sorted(lists: &[Vec<i32>]) -> Vec<i32> {
    // (value, list_index, element_index)
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

    // Push the first element of each list.
    for (i, list) in lists.iter().enumerate() {
        if !list.is_empty() {
            heap.push(Reverse((list[0], i, 0)));
        }
    }

    let mut result = Vec::new();

    while let Some(Reverse((val, list_idx, elem_idx))) = heap.pop() {
        result.push(val);

        // Push the next element from the same list, if any.
        let next = elem_idx + 1;
        if next < lists[list_idx].len() {
            heap.push(Reverse((lists[list_idx][next], list_idx, next)));
        }
    }

    result
}

fn main() {
    let lists = vec![
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![3, 6, 9],
    ];
    let merged = merge_k_sorted(&lists);
    assert_eq!(merged, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
```

**Time complexity:** O(n log k), where n is the total number of elements. The heap never
has more than k items. Each of the n elements is pushed and popped once.

### 3. Median Maintenance with Two Heaps

**Problem:** Given a stream of numbers arriving one at a time, efficiently report the
median after each insertion.

**Approach:** Maintain two heaps:

- A **max-heap** for the lower half of the numbers.
- A **min-heap** for the upper half of the numbers.

The max-heap's root is the largest of the small numbers. The min-heap's root is the
smallest of the large numbers. The median is at the boundary.

```
  Numbers seen so far: [1, 5, 2, 8, 3]
  Sorted: [1, 2, 3, 5, 8]
  Median: 3

  max-heap (lower half):    min-heap (upper half):
        [3]                       [5]
       /   \                     /
     [1]   [2]                 [8]

  The median is the root of the max-heap (when sizes are equal or
  max-heap has one more element).
```

The invariant: `max_heap.len()` is equal to `min_heap.len()` or one greater.

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    lower: BinaryHeap<i32>,           // max-heap: largest of the small half
    upper: BinaryHeap<Reverse<i32>>,  // min-heap: smallest of the large half
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            lower: BinaryHeap::new(),
            upper: BinaryHeap::new(),
        }
    }

    fn insert(&mut self, num: i32) {
        // Always add to the lower (max-heap) first.
        self.lower.push(num);

        // Ensure the largest of the lower half is <= smallest of the upper half.
        if let (Some(&lo_max), Some(&Reverse(up_min))) =
            (self.lower.peek(), self.upper.peek())
        {
            if lo_max > up_min {
                self.upper.push(Reverse(self.lower.pop().unwrap()));
            }
        }

        // Rebalance: lower should have equal or one more element than upper.
        if self.lower.len() > self.upper.len() + 1 {
            self.upper.push(Reverse(self.lower.pop().unwrap()));
        } else if self.upper.len() > self.lower.len() {
            self.lower.push(self.upper.pop().unwrap().0);
        }
    }

    fn median(&self) -> f64 {
        if self.lower.len() > self.upper.len() {
            *self.lower.peek().unwrap() as f64
        } else {
            let lo = *self.lower.peek().unwrap() as f64;
            let up = self.upper.peek().unwrap().0 as f64;
            (lo + up) / 2.0
        }
    }
}

fn main() {
    let mut mf = MedianFinder::new();

    mf.insert(1);
    assert_eq!(mf.median(), 1.0);

    mf.insert(5);
    assert_eq!(mf.median(), 3.0);    // median of [1, 5] = 3.0

    mf.insert(2);
    assert_eq!(mf.median(), 2.0);    // median of [1, 2, 5] = 2.0

    mf.insert(8);
    assert_eq!(mf.median(), 3.5);    // median of [1, 2, 5, 8] = 3.5

    mf.insert(3);
    assert_eq!(mf.median(), 3.0);    // median of [1, 2, 3, 5, 8] = 3.0

    println!("All median tests passed.");
}
```

**Time complexity:** O(log n) per insertion, O(1) for median query. Without heaps, you
would need to sort after every insertion (O(n log n)) or maintain a sorted list with O(n)
insertion.

### 4. Dijkstra's Algorithm (Preview)

Dijkstra's shortest-path algorithm uses a priority queue to always process the closest
unvisited node next. We will cover this in full in the graph algorithms lesson, but here
is a sketch showing why heaps matter:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// Edges are (neighbor, weight).
fn dijkstra(adj: &[Vec<(usize, u32)>], start: usize) -> Vec<u32> {
    let n = adj.len();
    let mut dist = vec![u32::MAX; n];
    dist[start] = 0;

    // Min-heap of (distance, node).
    let mut heap: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        // If we've already found a shorter path, skip.
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            let new_dist = d + w;
            if new_dist < dist[v] {
                dist[v] = new_dist;
                heap.push(Reverse((new_dist, v)));
            }
        }
    }

    dist
}

fn main() {
    // Graph:
    //   0 --1--> 1 --3--> 3
    //   |        |        ^
    //   4        1        |
    //   |        v        1
    //   +------> 2 -------+
    let adj = vec![
        vec![(1, 1), (2, 4)],  // 0
        vec![(2, 1), (3, 3)],  // 1
        vec![(3, 1)],          // 2
        vec![],                // 3
    ];

    let distances = dijkstra(&adj, 0);
    assert_eq!(distances, vec![0, 1, 2, 3]);
    // 0->0: 0, 0->1: 1, 0->1->2: 2, 0->1->2->3: 3
}
```

Without a heap, you would scan all nodes to find the closest unvisited one -- O(V) per
step, O(V^2) total. With a binary heap, each relaxation is O(log V), giving O((V + E)
log V) overall. For sparse graphs, this is a massive improvement.

---

## The Operations at a Glance

Here is a visual summary of how the core operations map to sift-up and sift-down:

```
  INSERT (push)                          EXTRACT-MIN/MAX (pop)
  ─────────────                          ────────────────────

  1. Append to end of array.             1. Save root (the min/max).
  2. Sift UP to restore heap property.   2. Move last element to root.
                                         3. Sift DOWN to restore heap property.

      [10]                                    [50]
     /    \                                  /    \
   [30]   [20]                            [30]    [20]
   /  \                                   /
  [40] [*25*] <-- new element           [40]
                                              ^-- 50 was last, now at root
  25 < 30? Yes, swap up.
                                         50 > 20? Swap with smallest child.
      [10]
     /    \                                   [20]
   [*25*] [20]                               /    \
   /  \                                    [30]   [50]
  [40] [30]                                /
                                         [40]
  25 < 10? No, stop.                     50 > 30? Swap.  50 > 40? Swap.
  Done.                                  ...keeps sinking until it settles.


  PEEK                                   BUILD HEAP (from array)
  ────                                   ──────────────────────

  Just return arr[0].                    Sift down every non-leaf node,
  O(1). No modifications.               from the last non-leaf to the root.
                                         O(n) -- not O(n log n).
```

---

## Complexity Summary

| Operation | Time | Notes |
|-----------|------|-------|
| `peek` | O(1) | Root is always min/max |
| `push` (insert) | O(log n) | Sift up from leaf to root |
| `pop` (extract) | O(log n) | Sift down from root to leaf |
| `build` (heapify) | O(n) | Bottom-up, Floyd's algorithm |
| `push` then `pop` | O(log n) | Can be optimized to one sift-down |

**Space:** O(n) for the data. No overhead beyond the array itself -- no pointers, no
metadata per node. This is the beauty of the implicit tree representation.

---

## Common Pitfalls

**1. Forgetting that Rust's `BinaryHeap` is a max-heap.**
If you push raw `i32` values and `pop`, you get the largest first. For algorithms expecting
a min-heap (Dijkstra's, merge-k-sorted, etc.), you must use `Reverse` or a custom `Ord`.
This is the single most common mistake.

**2. Trying to update priorities in place.**
`BinaryHeap` does not support decrease-key or increase-key. You cannot change an element's
priority and have the heap adjust. The standard workaround is "lazy deletion": push a new
entry with the updated priority, and skip stale entries when you pop (check if the popped
value is outdated). This is exactly what the Dijkstra implementation above does with the
`if d > dist[u] { continue; }` check.

**3. Confusing sift-up and sift-down.**
Sift-up is for insertion (new element at the bottom, float it up). Sift-down is for
extraction (replacement element at the top, sink it down). Mixing them up will corrupt your
heap.

**4. Using O(n log n) build when O(n) is available.**
If you have all your data before building the heap, use `BinaryHeap::from(vec)` or
`.collect()`, not a loop of `push` calls. The difference is measurable on large inputs.

**5. Implementing `Ord` inconsistently with `PartialEq`.**
Rust requires that `a == b` implies `a.cmp(&b) == Equal`. If your `Ord` and `PartialEq`
disagree, the heap will exhibit undefined (but memory-safe) behavior -- elements may end up
in wrong positions. When in doubt, derive `PartialEq` and `Eq`, and only manually implement
`Ord` and `PartialOrd`.

---

## Key Takeaways

1. **A priority queue is an ADT; a binary heap is the standard implementation.** The heap
   gives you O(log n) insert and extract, O(1) peek, with excellent constants and cache
   behavior.

2. **Sift-up drives insertion; sift-down drives extraction.** Together they keep the heap
   property intact as the collection grows and shrinks.

3. **Rust's `BinaryHeap<T>` is a max-heap.** Use `Reverse<T>` for min-heap behavior, or
   implement custom `Ord` for struct-based priorities.

4. **Build heaps in O(n) when possible.** `BinaryHeap::from(vec)` uses Floyd's algorithm.
   Inserting one at a time costs O(n log n).

5. **Key applications:** top-k elements, merging k sorted sequences, running median, and
   graph algorithms like Dijkstra's. The heap is a workhorse data structure that shows up
   far more often in real systems than heap sort does.

6. **Lazy deletion** is the standard workaround for the lack of decrease-key. Push updated
   entries and skip stale ones on pop.

---

## Exercises

1. **Implement a generic min-heap.** Adapt the `MinHeap` shown above to work with any
   `T: Ord`. Test with `i32`, `String`, and a custom struct.

2. **Kth largest element.** Given an unsorted array and an integer k, find the kth largest
   element. Solve it in O(n log k) using a heap of size k. Compare with the O(n log n)
   sorting approach on large inputs.

3. **Sort a nearly-sorted array.** Each element is at most k positions from its sorted
   position. Sort the array in O(n log k) time using a heap of size k+1.

4. **Merge k sorted arrays.** Implement the merge-k-sorted solution above. Then test it
   with k = 1000 arrays of 1000 elements each. Verify the output is sorted.

5. **Running median.** Implement the two-heap `MedianFinder`. Feed it the sequence
   `[5, 15, 1, 3, 2, 8, 7, 9, 10, 6, 11, 4]` and verify the median after each insertion.

6. **Custom priority struct.** Create a `Job` struct with `name: String` and
   `deadline: u32`. Implement `Ord` so that the job with the *earliest* deadline has the
   highest priority. Use a `BinaryHeap<Job>` to process jobs in deadline order.

7. **Dijkstra's shortest path.** Implement the sketch above for a graph of your choice.
   Verify distances against a hand-traced BFS on an unweighted version of the same graph.

<details>
<summary>Hint for Exercise 3</summary>

Use a min-heap of size k+1. Push the first k+1 elements. Then for each remaining element,
pop the minimum (which must be the next element in sorted order), push the new element.
After the input is exhausted, drain the heap.

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn sort_nearly_sorted(arr: &[i32], k: usize) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut result = Vec::with_capacity(arr.len());

    for &val in arr {
        heap.push(Reverse(val));
        if heap.len() > k + 1 {
            result.push(heap.pop().unwrap().0);
        }
    }
    while let Some(Reverse(val)) = heap.pop() {
        result.push(val);
    }

    result
}
```

</details>

---

*Next up: graph fundamentals, where priority queues become central to shortest-path
algorithms, minimum spanning trees, and more.*
