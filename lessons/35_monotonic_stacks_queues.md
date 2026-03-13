# Lesson 35: Monotonic Stacks & Queues

In [Lesson 05](./05_stacks_queues_deques.md) we introduced stacks and queues as
general-purpose data structures, and briefly touched on the idea of a "monotonic stack"
for finding the next greater element. This lesson goes deep. Monotonic stacks and
monotonic deques are not new data structures -- they are *techniques* built on top of
`Vec` and `VecDeque` that maintain a sorted invariant to answer an entire class of
problems in O(n) where brute force takes O(n^2).

Once you internalize the pattern, you will start recognizing it everywhere: stock prices,
temperature forecasts, histogram analysis, rain water, sliding window queries. The common
thread is always the same: "for each element, I need to know something about the nearest
element satisfying some comparison."

---

## What Does "Monotonic" Mean?

A sequence is **monotonic** if its elements never break a particular ordering:

- **Monotonically increasing**: each element is >= the previous. `[1, 2, 2, 5, 9]`
- **Monotonically decreasing**: each element is <= the previous. `[9, 5, 2, 2, 1]`
- **Strictly increasing/decreasing**: same, but without equality.

A **monotonic stack** is a regular stack (`Vec<T>` in Rust) where, before pushing a new
element, you pop everything that would violate the ordering. The stack's contents, read
bottom-to-top, are always sorted.

A **monotonic deque** (or monotonic queue) is the same idea applied to a `VecDeque`,
where you can remove stale entries from the front and maintain the ordering from the back.
This is the key tool for sliding window min/max problems.

---

## The Real-World Analogy

### The Skyline View (Monotonic Decreasing Stack)

Stand at one end of a city street and look toward the horizon. You see a row of buildings
of varying heights. Which buildings are actually visible? Only the ones that are taller
than every building in front of them. A short building hidden behind a tall one disappears
from view. As you scan left to right, you mentally maintain a decreasing sequence of
visible heights -- each new visible building must be taller than everything before it.

```
  Heights:  3  1  4  1  5  9  2  6

            9
            |
       5    |
       |    |        6
  3    |    |        |
  | 4  |    |     |  |
  | |  |    |  2  |  |
  | | 1| 1  |  |  |  |
  +-+--+--+-+--+--+--+--
  3  1  4  1  5  9  2  6

  Visible from left: 3, 4, 5, 9   (each taller than the last)
  Hidden:            1, 1, 2, 6   (blocked by something taller to their left)
```

### The Temperature Tracker (Next Greater Element)

You are tracking daily temperatures and each day you ask: "How many days until it is
warmer than today?" You keep a stack of days whose "next warmer day" you have not found
yet. When you see a new temperature that is higher than the top of the stack, those
waiting days just found their answer. Days with high temperatures sit on the stack longer
because it takes an even hotter day to "resolve" them.

---

## Monotonic Increasing vs. Decreasing Stack

The naming can be confusing because different sources use opposite conventions. Here is the
definition this lesson uses:

| Stack Type | Invariant (bottom to top) | Pop when | Used to find |
|---|---|---|---|
| **Monotonic increasing** | Values increase from bottom to top | New element is **smaller** than top | Next smaller element |
| **Monotonic decreasing** | Values decrease from bottom to top | New element is **greater** than top | Next greater element |

The key insight: you pop elements **when the new element breaks the invariant**, and the
act of popping is what answers the question for the popped element.

---

## The Classic: Next Greater Element

Given an array, for each element find the first element to its right that is strictly
greater. If none exists, the answer is -1.

```
Input:  [2, 1, 4, 7, 3, 5, 8, 6]
Output: [4, 4, 7, 8, 5, 8, -1, -1]
```

### Brute Force: O(n^2)

For each element, scan right until you find something bigger:

```rust
fn next_greater_brute(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![-1i32; n];
    for i in 0..n {
        for j in (i + 1)..n {
            if nums[j] > nums[i] {
                result[i] = nums[j];
                break;
            }
        }
    }
    result
}
```

### Monotonic Stack: O(n)

Maintain a stack of *indices* whose corresponding values are in decreasing order (a
monotonic decreasing stack). When a new value is larger than the stack's top value, that
new value is the "next greater element" for the top. Pop, record, repeat.

```rust
fn next_greater_element(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![-1i32; n];
    let mut stack: Vec<usize> = Vec::new(); // indices

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if nums[i] > nums[top] {
                result[top] = nums[i];
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    result
}

fn main() {
    let nums = [2, 1, 4, 7, 3, 5, 8, 6];
    let result = next_greater_element(&nums);
    assert_eq!(result, vec![4, 4, 7, 8, 5, 8, -1, -1]);
    println!("Next greater element tests passed.");
}
```

### Step-by-Step Walkthrough

Let's trace through `[2, 1, 4, 7, 3, 5, 8, 6]` and watch the stack at each step.
The stack stores indices; values shown in parentheses for clarity.

```
  i=0, val=2:  stack=[]
    nothing to pop
    push 0
    stack=[0(2)]                         result=[_, _, _, _, _, _, _, _]

  i=1, val=1:  stack=[0(2)]
    1 < 2? yes, don't pop
    push 1
    stack=[0(2), 1(1)]                   result=[_, _, _, _, _, _, _, _]

  i=2, val=4:  stack=[0(2), 1(1)]
    4 > 1? yes -> result[1]=4, pop 1
    4 > 2? yes -> result[0]=4, pop 0
    stack empty, stop popping
    push 2
    stack=[2(4)]                         result=[4, 4, _, _, _, _, _, _]

  i=3, val=7:  stack=[2(4)]
    7 > 4? yes -> result[2]=7, pop 2
    stack empty
    push 3
    stack=[3(7)]                         result=[4, 4, 7, _, _, _, _, _]

  i=4, val=3:  stack=[3(7)]
    3 > 7? no
    push 4
    stack=[3(7), 4(3)]                   result=[4, 4, 7, _, _, _, _, _]

  i=5, val=5:  stack=[3(7), 4(3)]
    5 > 3? yes -> result[4]=5, pop 4
    5 > 7? no
    push 5
    stack=[3(7), 5(5)]                   result=[4, 4, 7, _, 5, _, _, _]

  i=6, val=8:  stack=[3(7), 5(5)]
    8 > 5? yes -> result[5]=8, pop 5
    8 > 7? yes -> result[3]=8, pop 3
    stack empty
    push 6
    stack=[6(8)]                         result=[4, 4, 7, 8, 5, 8, _, _]

  i=7, val=6:  stack=[6(8)]
    6 > 8? no
    push 7
    stack=[6(8), 7(6)]                   result=[4, 4, 7, 8, 5, 8, _, _]

  Done. Remaining indices 6,7 have no next greater -> stay as -1.
  Final result: [4, 4, 7, 8, 5, 8, -1, -1]
```

### Visualizing the Stack State

Here is the stack drawn as a vertical bar chart at each step. The `>` marks the current
element being processed. The stack holds a monotonically decreasing subsequence at all
times.

```
  Step:   0    1    2    3    4    5    6    7

         2>   1>   4>   7>   3>   5>   8>   6>
  8|                                   #
  7|                    #              #    #
  6|                    #              #    #
  5|              #     #         #    #    #
  4|              #     #         #    #    #
  3|              #     #    #    #    #    #
  2|    #    #    #     #    #    #    #    #
  1|    #    #    #     #    #    #    #    #
       ---  ---  ---   ---  ---  ---  ---  ---
  stk: [2] [2,1] [4]  [7] [7,3][7,5] [8] [8,6]
```

### Why O(n)?

Each index is pushed onto the stack exactly once and popped at most once. Push operations:
n total. Pop operations: at most n total. Everything else is O(1) per operation. Total
work: O(2n) = O(n).

---

## Next Smaller Element

The mirror problem: for each element, find the first element to its right that is strictly
smaller. This uses a **monotonic increasing stack** -- values increase from bottom to top,
and you pop when the new element is smaller.

```rust
fn next_smaller_element(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![-1i32; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if nums[i] < nums[top] {
                result[top] = nums[i];
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    result
}

fn main() {
    let nums = [4, 2, 1, 5, 3, 6];
    let result = next_smaller_element(&nums);
    assert_eq!(result, vec![2, 1, -1, 3, -1, -1]);
    println!("Next smaller element tests passed.");
}
```

The only change from next-greater is the comparison direction: `nums[i] < nums[top]`
instead of `nums[i] > nums[top]`. The stack invariant flips from decreasing to increasing.

### The Four Variants

By combining the scan direction (left-to-right vs. right-to-left) and the comparison
(greater vs. smaller), you get four related problems:

| Problem | Scan Direction | Stack Order | Comparison |
|---|---|---|---|
| Next greater element | Left to right | Decreasing | `>` |
| Next smaller element | Left to right | Increasing | `<` |
| Previous greater element | Right to left | Decreasing | `>` |
| Previous smaller element | Right to left | Increasing | `<` |

For "previous" variants, iterate from right to left and the logic is identical.

---

## Daily Temperatures

A classic LeetCode problem (739) that is next-greater-element in disguise.

**Problem**: Given an array of daily temperatures, return an array where each element tells
you how many days you have to wait for a warmer temperature. If there is no future warmer
day, the answer is 0.

```
Input:  [73, 74, 75, 71, 69, 72, 76, 73]
Output: [ 1,  1,  4,  2,  1,  1,  0,  0]
```

This is "next greater element," but instead of returning the value, you return the
distance (index difference).

```rust
fn daily_temperatures(temperatures: &[i32]) -> Vec<i32> {
    let n = temperatures.len();
    let mut result = vec![0i32; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if temperatures[i] > temperatures[top] {
                result[top] = (i - top) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    result
}

fn main() {
    let temps = [73, 74, 75, 71, 69, 72, 76, 73];
    let result = daily_temperatures(&temps);
    assert_eq!(result, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    println!("Daily temperatures tests passed.");
}
```

### Trace

```
  i=0 (73):  push 0.                         Stack: [0]
  i=1 (74):  74>73 -> pop 0, result[0]=1.    Stack: [1]
  i=2 (75):  75>74 -> pop 1, result[1]=1.    Stack: [2]
  i=3 (71):  71<75, push.                    Stack: [2,3]
  i=4 (69):  69<71, push.                    Stack: [2,3,4]
  i=5 (72):  72>69 -> pop 4, result[4]=1.
             72>71 -> pop 3, result[3]=2.
             72<75, push.                    Stack: [2,5]
  i=6 (76):  76>72 -> pop 5, result[5]=1.
             76>75 -> pop 2, result[2]=4.    Stack: [6]
  i=7 (73):  73<76, push.                   Stack: [6,7]

  Remaining on stack: indices 6,7 -> result stays 0.
  Result: [1, 1, 4, 2, 1, 1, 0, 0]
```

The difference from the generic next-greater-element is one line: `result[top] = (i - top)`
instead of `result[top] = nums[i]`. Same stack, same pattern, different answer recorded.

---

## Largest Rectangle in Histogram

This is the problem where monotonic stacks truly shine. It is one of the hardest "medium"
problems and one of the most satisfying to understand.

**Problem**: Given an array of non-negative integers representing bar heights in a
histogram (each bar has width 1), find the area of the largest rectangle that fits entirely
within the histogram.

```
  Heights: [2, 1, 5, 6, 2, 3]

  6 |         #
  5 |      #  #
  4 |      #  #
  3 |      #  #     #
  2 |  #   #  #  #  #
  1 |  #  #   #  #  #  #
  0 +--+--+--+--+--+--+--
     0  1  2  3  4  5

  The largest rectangle has area 10 (height=5, spanning indices 2-3).
```

### The Key Insight

For any bar of height `h`, the widest rectangle using that exact height extends left and
right as far as possible while all bars are >= h. That means you need the **previous
smaller element** (left boundary) and the **next smaller element** (right boundary) for
each bar.

A monotonic increasing stack finds both boundaries in a single pass.

### How It Works

Walk left to right, maintaining a stack of indices with increasing heights. When you
encounter a bar shorter than the stack's top, that shorter bar is the right boundary for
the top bar. The element below the top in the stack is the left boundary. Pop, compute
the area, track the maximum.

```rust
fn largest_rectangle_area(heights: &[i32]) -> i32 {
    let n = heights.len();
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0i32;

    for i in 0..=n {
        // Use 0 as a sentinel for the "bar" after the last real bar.
        // This ensures everything in the stack gets processed.
        let current_height = if i < n { heights[i] } else { 0 };

        while let Some(&top) = stack.last() {
            if current_height < heights[top] {
                stack.pop();
                let height = heights[top];
                let width = match stack.last() {
                    Some(&left) => (i - left - 1) as i32,
                    None => i as i32,
                };
                max_area = max_area.max(height * width);
            } else {
                break;
            }
        }
        stack.push(i);
    }

    max_area
}

fn main() {
    assert_eq!(largest_rectangle_area(&[2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(largest_rectangle_area(&[2, 4]), 4);
    assert_eq!(largest_rectangle_area(&[1, 1, 1, 1]), 4);
    assert_eq!(largest_rectangle_area(&[6, 2, 5, 4, 5, 1, 6]), 12);
    println!("Largest rectangle tests passed.");
}
```

### Step-by-Step Walkthrough

```
  Heights: [2, 1, 5, 6, 2, 3]    (plus sentinel 0 at position 6)

  i=0, h=2:  stack=[]
    push 0.                             stack=[0]

  i=1, h=1:  stack=[0]
    1 < heights[0]=2 -> pop 0
      height=2, stack empty -> width=1.  area=2*1=2.   max=2
    push 1.                             stack=[1]

  i=2, h=5:  stack=[1]
    5 >= 1, no pop. push 2.            stack=[1, 2]

  i=3, h=6:  stack=[1, 2]
    6 >= 5, no pop. push 3.            stack=[1, 2, 3]

  i=4, h=2:  stack=[1, 2, 3]
    2 < heights[3]=6 -> pop 3
      height=6, top=2 -> width=4-2-1=1. area=6.        max=6
    2 < heights[2]=5 -> pop 2
      height=5, top=1 -> width=4-1-1=2. area=10.       max=10
    2 >= heights[1]=1, stop. push 4.   stack=[1, 4]

  i=5, h=3:  stack=[1, 4]
    3 >= 2, no pop. push 5.            stack=[1, 4, 5]

  i=6, h=0 (sentinel):  stack=[1, 4, 5]
    0 < heights[5]=3 -> pop 5
      height=3, top=4 -> width=6-4-1=1. area=3.        max=10
    0 < heights[4]=2 -> pop 4
      height=2, top=1 -> width=6-1-1=4. area=8.        max=10
    0 < heights[1]=1 -> pop 1
      height=1, stack empty -> width=6.  area=6.        max=10

  Answer: 10
```

### Visualizing the Largest Rectangle

```
  6 |         +--+
  5 |      +--+##+  <--- height=5, width=2, area=10
  4 |      |##|##|
  3 |      |##|##|     #
  2 |  #   |##|##| #   #
  1 |  #  #|##|##| #   #  #
  0 +--+--+--+--+--+--+--
     0  1  2  3  4  5
```

### Why the Sentinel?

The sentinel value of 0 at position `n` forces all remaining bars in the stack to be
processed. Without it, you would need a separate loop after the main iteration to handle
bars left in the stack. The sentinel is a common trick in monotonic stack problems.

---

## Trapping Rain Water

Another classic problem (LeetCode 42) that can be solved multiple ways, including with
a monotonic stack.

**Problem**: Given an elevation map where each bar has width 1, compute how much water
can be trapped after raining.

```
  Heights: [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]

  3 |                        #
  2 |         #  ~ ~ ~ ~ ~ ~ #  #     #
  1 |   #  ~  #  #  ~  #  ~  #  #  #  #  #
  0 +--+--+--+--+--+--+--+--+--+--+--+--+--
     0  1  2  3  4  5  6  7  8  9  10 11

  Water (~): 6 units
```

### Approach 1: Two-Pointer (O(n) time, O(1) space)

The most space-efficient approach. For any position, the water level is determined by the
minimum of (max height to the left, max height to the right) minus the bar's own height.
Two pointers from opposite ends track these maxima.

```rust
fn trap_two_pointer(height: &[i32]) -> i32 {
    if height.len() < 3 {
        return 0;
    }

    let mut left = 0;
    let mut right = height.len() - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut water = 0;

    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                water += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                water += right_max - height[right];
            }
            right -= 1;
        }
    }

    water
}
```

### Approach 2: Monotonic Stack (O(n) time, O(n) space)

The stack-based approach processes water in horizontal layers rather than vertical columns.
Maintain a monotonic decreasing stack. When you encounter a bar taller than the stack's
top, you have found a "bowl" -- a valley between the current bar and the element below
the top. Calculate the water trapped in that layer.

```rust
fn trap_monotonic_stack(height: &[i32]) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut water = 0;

    for i in 0..height.len() {
        while let Some(&top) = stack.last() {
            if height[i] > height[top] {
                stack.pop();
                // The popped bar is the bottom of a potential bowl
                if let Some(&left) = stack.last() {
                    // Water height is bounded by the shorter of the two walls
                    let bounded_height = height[left].min(height[i]) - height[top];
                    let width = (i - left - 1) as i32;
                    water += bounded_height * width;
                }
            } else {
                break;
            }
        }
        stack.push(i);
    }

    water
}

fn main() {
    let h = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(trap_two_pointer(&h), 6);
    assert_eq!(trap_monotonic_stack(&h), 6);

    let h2 = [4, 2, 0, 3, 2, 5];
    assert_eq!(trap_two_pointer(&h2), 9);
    assert_eq!(trap_monotonic_stack(&h2), 9);
    println!("Trapping rain water tests passed.");
}
```

### How the Stack Approach Works

Think of it as filling the water layer by layer from the bottom up. Each time you pop
an element, you compute the rectangular layer of water sitting on top of it, bounded by
the walls on either side.

```
  At i=3 (h=2), stack=[1(h=1), 2(h=0)]:
    Pop index 2 (h=0): bottom=0, left wall=h[1]=1, right wall=h[3]=2
      bounded = min(1,2) - 0 = 1, width = 3-1-1 = 1
      water += 1

    Pop index 1 (h=1): stack empty, no left wall. Skip.
    Push 3. stack=[3(h=2)]

  At i=7 (h=3), after processing several steps, the stack
  has accumulated valleys that get peeled off layer by layer.
```

---

## Sliding Window Maximum (Monotonic Deque)

**Problem**: Given an array and a window size `k`, find the maximum value in each window
as it slides from left to right (LeetCode 239).

```
Input:  nums = [1, 3, -1, -3, 5, 3, 6, 7],  k = 3
Output:        [3, 3, 5, 5, 6, 7]

Windows:
  [1, 3, -1] -> 3
  [3, -1, -3] -> 3
  [-1, -3, 5] -> 5
  [-3, 5, 3] -> 5
  [5, 3, 6] -> 6
  [3, 6, 7] -> 7
```

### Why a Deque, Not a Stack?

A stack only lets you remove from one end. In a sliding window, elements leave from the
**front** (they fall out of the window as it slides) and new elements enter from the
**back**. You need both ends accessible -- that is a deque. Rust's `VecDeque` from
`std::collections` provides O(1) operations on both ends.

### The Idea

Maintain a deque of indices whose values are in **decreasing** order (front to back).
The front of the deque is always the index of the current window's maximum.

Two rules:
1. **Back removal**: Before pushing a new index, pop from the back any indices whose
   values are less than or equal to the new value. They can never be a window maximum
   because the new element is both larger and newer (it will stay in the window longer).
2. **Front removal**: If the front index has fallen out of the window, pop it.

### The Implementation

```rust
use std::collections::VecDeque;

fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    assert!(k > 0 && k <= nums.len(), "k must be in [1, nums.len()]");

    let mut deque: VecDeque<usize> = VecDeque::new(); // indices
    let mut result = Vec::with_capacity(nums.len() - k + 1);

    for i in 0..nums.len() {
        // 1. Remove indices that have left the window
        while let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        }

        // 2. Remove from back all indices with values <= nums[i]
        while let Some(&back) = deque.back() {
            if nums[back] <= nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        // 3. Add current index
        deque.push_back(i);

        // 4. Record the maximum for this window (once we have a full window)
        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }

    result
}

fn main() {
    let nums = [1, 3, -1, -3, 5, 3, 6, 7];
    let result = max_sliding_window(&nums, 3);
    assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
    println!("Sliding window maximum tests passed.");
}
```

### Step-by-Step Trace

```
  nums = [1, 3, -1, -3, 5, 3, 6, 7],  k=3
  deque stores indices; values shown in parentheses.

  i=0, val=1:
    deque empty, push 0
    deque=[0(1)]                      window not full yet

  i=1, val=3:
    back=0(1), 1 <= 3 -> pop_back
    deque=[], push 1
    deque=[1(3)]                      window not full yet

  i=2, val=-1:
    back=1(3), 3 > -1 -> stop
    push 2
    deque=[1(3), 2(-1)]              window [1,3,-1] -> max = 3

  i=3, val=-3:
    front=1, 1+3=4 > 3 -> stays
    back=2(-1), -1 > -3 -> stop
    push 3
    deque=[1(3), 2(-1), 3(-3)]      window [3,-1,-3] -> max = 3

  i=4, val=5:
    front=1, 1+3=4 <= 4 -> pop_front
    deque=[2(-1), 3(-3)]
    back=3(-3), -3 <= 5 -> pop_back
    back=2(-1), -1 <= 5 -> pop_back
    deque=[], push 4
    deque=[4(5)]                     window [-1,-3,5] -> max = 5

  i=5, val=3:
    front=4, 4+3=7 > 5 -> stays
    back=4(5), 5 > 3 -> stop
    push 5
    deque=[4(5), 5(3)]              window [-3,5,3] -> max = 5

  i=6, val=6:
    front=4, 4+3=7 > 6 -> stays
    back=5(3), 3 <= 6 -> pop_back
    back=4(5), 5 <= 6 -> pop_back
    deque=[], push 6
    deque=[6(6)]                     window [5,3,6] -> max = 6

  i=7, val=7:
    front=6, 6+3=9 > 7 -> stays
    back=6(6), 6 <= 7 -> pop_back
    deque=[], push 7
    deque=[7(7)]                     window [3,6,7] -> max = 7

  Result: [3, 3, 5, 5, 6, 7]
```

### Sliding Window Minimum

The same approach, but maintain a monotonic **increasing** deque. Remove from the back
any indices whose values are >= the new value. The front holds the window minimum.

```rust
use std::collections::VecDeque;

fn min_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut result = Vec::with_capacity(nums.len() - k + 1);

    for i in 0..nums.len() {
        while let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        }

        // Flip: remove from back values >= nums[i] (for minimum)
        while let Some(&back) = deque.back() {
            if nums[back] >= nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }

    result
}
```

### Visualizing the Deque

```
  Window slides right -->

  Array: [ 1   3  -1  -3   5   3   6   7 ]
           |-------|
               |-------|
                   |-------|
                       |-------|
                           |-------|
                               |-------|

  Deque always holds a decreasing subsequence of the current window:

  Window [1,3,-1]:   Deque: [3, -1]     max = 3
  Window [3,-1,-3]:  Deque: [3, -1, -3] max = 3
  Window [-1,-3,5]:  Deque: [5]         max = 5
  Window [-3,5,3]:   Deque: [5, 3]      max = 5
  Window [5,3,6]:    Deque: [6]         max = 6
  Window [3,6,7]:    Deque: [7]         max = 7
```

**Time**: O(n). Each index is added and removed at most once.
**Space**: O(k) for the deque contents.

---

## The General Pattern

All monotonic stack/deque problems share a common skeleton. Here is the template:

```rust
fn monotonic_stack_template(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![DEFAULT_VALUE; n];
    let mut stack: Vec<usize> = Vec::new(); // always store indices

    for i in 0..n {
        // Pop while the invariant is violated
        while let Some(&top) = stack.last() {
            if should_pop(nums[i], nums[top]) {
                // The answer for `top` has been found: it is related to `i`
                result[top] = compute_answer(i, top, nums);
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    // Remaining indices in the stack have no answer (stay as DEFAULT_VALUE)
    result
}
```

What changes between problems:

| What changes | Next Greater | Next Smaller | Histogram | Rain Water |
|---|---|---|---|---|
| `should_pop` | `new > top` | `new < top` | `new < top` | `new > top` |
| `compute_answer` | `nums[i]` | `nums[i]` | `height * width` | `bounded_h * width` |
| Stack order | Decreasing | Increasing | Increasing | Decreasing |
| Sentinel needed | No | No | Yes (0 at end) | No |

---

## How to Recognize Monotonic Stack/Deque Problems

Here are the signals to watch for:

**1. "For each element, find the next/previous greater/smaller element."**
This is the textbook signal. Direct monotonic stack application.

**2. "Largest rectangle" or "maximum area" involving bars or boundaries.**
The histogram problem and its variants (maximal rectangle in a binary matrix). These
require finding the nearest smaller element on both sides.

**3. "Sliding window minimum/maximum" or "min/max in every subarray of size k."**
This is the monotonic deque pattern.

**4. "How much water / how many units can be trapped."**
Trapping rain water and its variants.

**5. Stock span problems: "how many consecutive days had price <= today's?"**
This is "previous greater element" -- the span extends back to the last day with a
higher price.

**6. Any O(n^2) brute force that scans left/right looking for a comparison match.**
If you are scanning from each element to find "the first element that..." based on a
comparison, a monotonic stack can probably collapse O(n^2) to O(n).

### When It Does NOT Apply

- When the property is not comparison-based (e.g., "next prime element").
- When you need relationships between non-adjacent elements not captured by "nearest."
- When the dataset changes dynamically with insertions/deletions after the initial scan.

---

## Complexity Summary

| Problem | Brute Force | Monotonic Stack/Deque |
|---|---|---|
| Next greater element | O(n^2) | O(n) time, O(n) space |
| Next smaller element | O(n^2) | O(n) time, O(n) space |
| Daily temperatures | O(n^2) | O(n) time, O(n) space |
| Largest rectangle in histogram | O(n^2) | O(n) time, O(n) space |
| Trapping rain water | O(n^2) | O(n) time, O(n) or O(1) space |
| Sliding window maximum | O(nk) | O(n) time, O(k) space |

In every case, the improvement comes from the same source: each element is pushed and
popped at most once, giving amortized O(1) per element.

---

## Common Mistakes

**1. Storing values instead of indices.**
Almost always store indices, not values. You need indices to compute distances (daily
temperatures, stock span, histogram width) and to check whether an element has left the
window (sliding window maximum). You can always look up the value via `nums[index]`.

**2. Getting the comparison direction backwards.**
For "next greater," you pop when the new element is *greater*. For "next smaller," you
pop when the new element is *smaller*. Drawing the stack contents on paper for the first
3-4 elements is the fastest way to catch this.

**3. Forgetting the sentinel in histogram problems.**
Without the sentinel (pushing a 0-height bar at the end), bars remaining in the stack
after the main loop go unprocessed. Either add the sentinel or add a cleanup loop.

**4. Off-by-one in width calculations for the histogram.**
When computing width after a pop, the left boundary is the new stack top (the element
*below* the popped one), not the popped element itself. If the stack is empty after
popping, the width extends all the way to the left edge (width = `i`).

**5. Using `<` when you need `<=` (or vice versa).**
In the sliding window maximum, you pop elements that are `<=` the new value, not just `<`.
An equal element should be popped because the newer one will outlast it in the window.
But in next-greater-element, you typically want strict `>`. Get this wrong and you will
get incorrect results on inputs with duplicate values.

---

## Practice Problems

### Easy (5 problems)

1. **Next Greater Element I** (LeetCode 496) -- Subset of array, find next greater from
   another array. Direct monotonic stack.
2. **Daily Temperatures** (LeetCode 739) -- Covered above. Implement it yourself.
3. **Final Prices With Discount** (LeetCode 1475) -- Next smaller element gives the
   discount amount. Straightforward application.
4. **Stock Span Problem** (LeetCode 901) -- Online algorithm using previous greater
   element. Implement as a struct with a `next(price)` method.
5. **Remove All Adjacent Duplicates In String** (LeetCode 1047) -- Stack-based, good
   warm-up for stack thinking.

### Medium (5 problems)

1. **Next Greater Element II** (LeetCode 503) -- Circular array variant. Process the
   array twice using `i % n` for indexing.
2. **Largest Rectangle in Histogram** (LeetCode 84) -- Covered above. The canonical
   monotonic stack problem.
3. **Sliding Window Maximum** (LeetCode 239) -- Covered above. Monotonic deque.
4. **Remove K Digits** (LeetCode 402) -- Greedy + monotonic increasing stack to build
   the smallest possible number.
5. **Sum of Subarray Minimums** (LeetCode 907) -- For each element, count how many
   subarrays it is the minimum of. Uses previous/next smaller element.

### Hard (5 problems)

1. **Trapping Rain Water** (LeetCode 42) -- Covered above. Try both the stack and
   two-pointer approaches.
2. **Maximal Rectangle** (LeetCode 85) -- 2D version of largest rectangle. Build a
   histogram per row, then apply the histogram algorithm to each.
3. **Largest Rectangle in Histogram** (LeetCode 84) done without a sentinel -- Handle
   the end-of-array flush manually. Good for understanding edge cases deeply.
4. **Shortest Subarray with Sum at Least K** (LeetCode 862) -- Monotonic deque on
   prefix sums. Handles negative numbers, unlike simple sliding window.
5. **Sum of Subarray Ranges** (LeetCode 2104) -- Combine sum of subarray maximums and
   sum of subarray minimums. Two monotonic stack passes.

---

## Key Takeaways

1. **A monotonic stack is not a new data structure.** It is a `Vec` with a discipline:
   before pushing, pop anything that violates the ordering invariant.

2. **Every pop produces an answer.** The moment an element gets popped, the current
   element tells you something about the popped element (its next greater, next smaller,
   right boundary, etc.).

3. **It is always O(n).** Each element is pushed once and popped at most once. The inner
   `while` loop does not make it quadratic -- total pops across all iterations is at most n.

4. **Sliding window max/min needs a deque, not a stack.** Elements must be removable from
   the front (expired elements) and from the back (dominated elements). `VecDeque` from
   `std::collections` gives O(1) on both ends.

5. **Store indices, not values.** This gives you both positional information (for distances
   and result placement) and value information (via `nums[index]` lookup).

6. **When in doubt, trace it.** Draw the array, draw the stack state at each step, and
   annotate what each pop means. The pattern becomes obvious after 2-3 traces.
