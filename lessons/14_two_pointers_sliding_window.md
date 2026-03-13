# Lesson 14: Two Pointers & Sliding Window

## Two Techniques, One Core Insight

Here's a pattern you've probably written dozens of times without thinking twice:

```rust
for i in 0..n {
    for j in (i + 1)..n {
        // check some condition on (i, j)
    }
}
```

That's O(n^2). It checks every pair. And in a shocking number of cases, it's doing
redundant work -- re-examining pairs you could have skipped if you'd been smarter about
how you moved through the data.

Two pointers and sliding window are techniques for **eliminating that redundancy**. They
exploit structure in the problem (sorted order, contiguity, monotonicity) to visit each
element a constant number of times, collapsing O(n^2) down to O(n).

They're not obscure tricks. They're fundamental patterns that show up constantly in
real codebases -- parsers, network protocol handlers, streaming analytics, audio
processing, anything that walks through sequential data.

---

## Part 1: Two Pointers

### The Idea

You maintain two indices (pointers) into a sequence and move them according to some
rule. Instead of checking all pairs with nested loops, you use information from the
current pair to decide which pointer to advance.

### The Analogy

Imagine a long hallway with numbered tiles on the floor: 1, 2, 3, ..., 100. Two
people start at opposite ends. They're trying to find two tiles whose numbers add up
to a target. Person A steps forward, Person B steps backward, and after each step
they check their sum. If the sum is too small, Person A steps forward (increasing the
sum). If the sum is too big, Person B steps backward (decreasing it). They never
backtrack, and they never need to check every combination -- the sorted order of the
tiles tells them which direction to move.

That's opposite-direction two pointers. There are several variants:

```
Pattern 1: Opposite ends, converging
  L -->                          <-- R

Pattern 2: Same direction, different speeds (fast/slow)
  slow ->    fast ---->

Pattern 3: Partitioning (multiple pointers carving regions)
  |  region A  |  unprocessed  |  region B  |
```

---

## 1.1 Opposite-Direction: Two Sum on a Sorted Array

### The Problem

Given a **sorted** array and a target sum, find two elements that add up to the target.

### Brute Force: O(n^2)

Check every pair:

```rust
fn two_sum_brute(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let n = nums.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if nums[i] + nums[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}
```

This works, but it's doing far more work than necessary. When `nums[i] + nums[j]`
overshoots the target, every `j' > j` will also overshoot (because the array is
sorted). That inner loop keeps going anyway.

### Optimized: O(n)

Two pointers, one at each end:

```rust
fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;   // need a bigger sum, move left pointer right
        } else {
            right -= 1;  // need a smaller sum, move right pointer left
        }
    }
    None
}
```

Let's trace it:

```
nums = [1, 3, 5, 7, 9, 11]    target = 12

Step 1:  [1, 3, 5, 7, 9, 11]
          L                 R     sum = 1 + 11 = 12  --> found!

Another example, target = 10:

Step 1:  [1, 3, 5, 7, 9, 11]
          L                 R     sum = 1 + 11 = 12 > 10  --> R--

Step 2:  [1, 3, 5, 7, 9, 11]
          L              R        sum = 1 + 9 = 10  --> found!

Another example, target = 8:

Step 1:  [1, 3, 5, 7, 9, 11]
          L                 R     sum = 1 + 11 = 12 > 8  --> R--

Step 2:  [1, 3, 5, 7, 9, 11]
          L              R        sum = 1 + 9 = 10 > 8   --> R--

Step 3:  [1, 3, 5, 7, 9, 11]
          L           R           sum = 1 + 7 = 8   --> found!
```

### Why It's Correct

The key insight: when `nums[left] + nums[right] < target`, you know that
`nums[left] + nums[k]` for all `k < right` is also too small (since the array is
sorted and `nums[k] <= nums[right]`). So moving `left` forward is the only productive
move. Symmetrically, when the sum is too large, moving `right` backward is the only
productive move. This means every step eliminates an entire row or column of the
pair-space, and you converge in at most `n` steps.

### Complexity

- **Time**: O(n). Each iteration advances at least one pointer. The pointers start
  `n` apart and converge, so at most `n` iterations.
- **Space**: O(1). Just two indices.

---

## 1.2 Opposite-Direction: Container With Most Water

### The Problem

Given an array `heights` where `heights[i]` is the height of a vertical line at
position `i`, find two lines that, together with the x-axis, form a container that
holds the most water.

```
Height:  |
    8    |              |
    7    |              |     |
    6    |  |           |     |
    5    |  |     |     |     |
    4    |  |     |  |  |     |
    3    |  |     |  |  |  |  |
    2    |  |  |  |  |  |  |  |
    1 |  |  |  |  |  |  |  |  |
    0 +--+--+--+--+--+--+--+--+--
      0  1  2  3  4  5  6  7  8
```

The area between lines at positions `i` and `j` is:

    area = min(heights[i], heights[j]) * (j - i)

### Brute Force: O(n^2)

```rust
fn max_water_brute(heights: &[i32]) -> i32 {
    let n = heights.len();
    let mut max_area = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let area = heights[i].min(heights[j]) * (j - i) as i32;
            max_area = max_area.max(area);
        }
    }
    max_area
}
```

### Optimized: O(n)

```rust
fn max_water(heights: &[i32]) -> i32 {
    let mut left = 0;
    let mut right = heights.len() - 1;
    let mut max_area = 0;

    while left < right {
        let width = (right - left) as i32;
        let height = heights[left].min(heights[right]);
        max_area = max_area.max(width * height);

        // Move the pointer at the shorter line inward.
        // Moving the taller line can never increase the area
        // because the height is limited by the shorter line
        // and the width is shrinking.
        if heights[left] < heights[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_area
}
```

### Why Move the Shorter Side?

This is the part that trips people up. Think of it this way:

- The area is `min(h_left, h_right) * width`.
- As we move pointers inward, `width` always **decreases**.
- The only way to find a larger area is if `min(h_left, h_right)` **increases enough**
  to compensate.
- If `heights[left] < heights[right]`, then `heights[left]` is the bottleneck. Moving
  `right` inward would shrink the width while keeping the height at most `heights[left]`
  (since the min is already constrained by the left). That can never improve things.
- Moving `left` inward *might* find a taller left wall, which *could* increase the area.

So moving the shorter pointer is the only move with any chance of improvement.

---

## 1.3 Same-Direction: Remove Duplicates In-Place

### The Problem

Given a sorted array, remove duplicates **in-place** and return the count of unique
elements. The first `k` elements of the array should contain the unique values.

### The Analogy

Picture two people walking through a library shelf in the same direction. The "slow"
person is the librarian placing books onto a clean shelf. The "fast" person is scanning
ahead through the original shelf. Every time the scanner finds a book with a new title,
they hand it to the librarian, who places it next. Duplicates get skipped.

### Brute Force: O(n^2)

Without the two-pointer insight, you might create a new vector (O(n) space) or shift
elements on each duplicate removal (O(n^2) time in the worst case):

```rust
fn remove_duplicates_brute(nums: &mut Vec<i32>) -> usize {
    let mut i = 0;
    while i < nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            nums.remove(i + 1); // O(n) shift on every removal
        } else {
            i += 1;
        }
    }
    nums.len()
}
```

Each `remove` shifts all elements after the removed position. Worst case (all
duplicates): O(n^2).

### Optimized: O(n), O(1) Space

```rust
fn remove_duplicates(nums: &mut [i32]) -> usize {
    if nums.is_empty() {
        return 0;
    }

    let mut slow = 0; // points to the last unique element placed

    for fast in 1..nums.len() {
        if nums[fast] != nums[slow] {
            slow += 1;
            nums[slow] = nums[fast];
        }
    }

    slow + 1 // count of unique elements
}
```

Trace it:

```
nums = [1, 1, 2, 2, 2, 3, 4, 4]

fast=1:  nums[1]=1 == nums[0]=1  --> skip
         [1, 1, 2, 2, 2, 3, 4, 4]
          S  F

fast=2:  nums[2]=2 != nums[0]=1  --> slow=1, nums[1]=2
         [1, 2, 2, 2, 2, 3, 4, 4]
             S  F

fast=3:  nums[3]=2 == nums[1]=2  --> skip
fast=4:  nums[4]=2 == nums[1]=2  --> skip

fast=5:  nums[5]=3 != nums[1]=2  --> slow=2, nums[2]=3
         [1, 2, 3, 2, 2, 3, 4, 4]
                S           F

fast=6:  nums[6]=4 != nums[2]=3  --> slow=3, nums[3]=4
         [1, 2, 3, 4, 2, 3, 4, 4]
                   S           F

fast=7:  nums[7]=4 == nums[3]=4  --> skip

Result: first 4 elements = [1, 2, 3, 4], return 4
```

`slow` only advances when we find a new unique value. `fast` scans every element
exactly once. Total work: O(n).

---

## 1.4 Same-Direction: Linked List Cycle Detection (Floyd's Algorithm)

This was covered in Lesson 04 on linked lists, but it's worth revisiting as a
two-pointer pattern.

The fast pointer moves 2 steps per iteration, the slow pointer moves 1 step. If there's
a cycle, the fast pointer will eventually "lap" the slow pointer and they'll meet. If
there's no cycle, the fast pointer reaches the end.

```
  No cycle:
    slow ->        fast --------> NULL
    1 -> 2 -> 3 -> 4 -> 5 -> NULL

  Cycle:
    slow ->     fast -->
    1 -> 2 -> 3 -> 4 -> 5
              ^              |
              +--------------+
    Eventually they meet inside the loop.
```

```rust
type Link = Option<Box<ListNode>>;

fn has_cycle(head: &Link) -> bool {
    // In safe Rust with owned nodes, true cycle detection requires
    // raw pointers or Rc<RefCell<>>. Here's the conceptual algorithm
    // using raw pointers for clarity:
    //
    // let mut slow = head;
    // let mut fast = head;
    // while fast is not null and fast.next is not null {
    //     slow = slow.next;
    //     fast = fast.next.next;
    //     if slow == fast { return true; }
    // }
    // return false;

    // Time:  O(n)
    // Space: O(1)
    unimplemented!("see Lesson 04 for full Rc/RefCell implementation")
}
```

The reason this works: in a cycle, the fast pointer closes the gap by 1 node each
iteration (fast gains +2, slow gains +1, net relative gain is +1). So if the cycle has
length `c`, they must meet within `c` iterations of the slow pointer entering the cycle.

---

## 1.5 Partitioning: Dutch National Flag

### The Problem

Given an array containing only 0s, 1s, and 2s, sort it in-place in a single pass.
This is Dijkstra's Dutch National Flag problem.

### Why Not Just Sort?

You could call `.sort()` in O(n log n). But the constraint that there are only three
distinct values means you can do it in O(n) with O(1) space, in a single pass.

### The Approach: Three Pointers

Maintain three regions:

```
  +----------+-----------+--------------+----------+
  |   0s     |    1s     |  unprocessed |    2s    |
  +----------+-----------+--------------+----------+
  0         low         mid            high       n-1

  Everything before `low`  is 0.
  Everything from `low` to `mid-1` is 1.
  Everything from `mid` to `high` is unprocessed.
  Everything after `high` is 2.
```

### The Algorithm

```rust
fn dutch_national_flag(nums: &mut [i32]) {
    let mut low = 0usize;
    let mut mid = 0usize;
    let mut high = nums.len().wrapping_sub(1); // handle empty array

    if nums.is_empty() {
        return;
    }

    // Use i32 for high to handle the underflow case cleanly
    let mut hi = nums.len() as i32 - 1;

    let mut lo = 0i32;
    let mut md = 0i32;

    while md <= hi {
        match nums[md as usize] {
            0 => {
                nums.swap(lo as usize, md as usize);
                lo += 1;
                md += 1;
            }
            1 => {
                md += 1;
            }
            2 => {
                nums.swap(md as usize, hi as usize);
                hi -= 1;
                // don't advance mid -- the swapped element is unexamined
            }
            _ => unreachable!(),
        }
    }
}
```

Trace:

```
nums = [2, 0, 1, 2, 1, 0]
        lo/md            hi

Step 1: nums[md]=2 --> swap(md, hi), hi--
  [0, 0, 1, 2, 1, 2]       (swapped 2 and 0)
   lo/md         hi

Step 2: nums[md]=0 --> swap(lo, md), lo++, md++
  [0, 0, 1, 2, 1, 2]       (swapped with itself)
      lo/md      hi

Step 3: nums[md]=0 --> swap(lo, md), lo++, md++
  [0, 0, 1, 2, 1, 2]
         lo/md   hi

Step 4: nums[md]=1 --> md++
  [0, 0, 1, 2, 1, 2]
         lo  md  hi

Step 5: nums[md]=2 --> swap(md, hi), hi--
  [0, 0, 1, 1, 2, 2]
         lo  md
             hi

Step 6: nums[md]=1 --> md++
  [0, 0, 1, 1, 2, 2]
         lo    md
             hi

md > hi --> done. Result: [0, 0, 1, 1, 2, 2]
```

**Time**: O(n) -- each element is examined at most twice.
**Space**: O(1).

---

## Part 2: Sliding Window

### The Idea

You maintain a **window** -- a contiguous subarray defined by a `start` and `end`
index -- and slide it across the array. Instead of recomputing everything about the
subarray from scratch each time you move, you **incrementally update** the window's
state by adding the new element entering the window and removing the element leaving it.

### The Analogy

You're on a train, looking through a window that's exactly 4 seats wide. As the train
moves forward, the scenery at the trailing edge disappears and new scenery appears at
the leading edge. You don't re-examine the entire visible scene at each moment -- you
just note what left and what appeared.

```
  Train:   [a, b, c, d, e, f, g, h, i]

  Window at position 0:    [a, b, c, d] e  f  g  h  i
  Window at position 1:     a [b, c, d, e] f  g  h  i
  Window at position 2:     a  b [c, d, e, f] g  h  i
  Window at position 3:     a  b  c [d, e, f, g] h  i
                                        ^           ^
                              d enters --+   a left earlier
```

Each slide: remove the effect of the element falling off the left, add the effect of
the element appearing on the right. This turns an O(k) per-position recalculation into
O(1), and an O(nk) total into O(n).

There are two major variants:

| Variant | Window Size | When to Shrink |
|---------|-------------|----------------|
| Fixed-size | Always `k` | Every step (after window is full) |
| Variable-size | Changes | When a constraint is violated or satisfied |

---

## 2.1 Fixed-Size Window: Maximum Sum of K Consecutive Elements

### The Problem

Given an array of integers and a number `k`, find the maximum sum of any `k`
consecutive elements.

### Brute Force: O(nk)

```rust
fn max_sum_k_brute(nums: &[i32], k: usize) -> Option<i32> {
    if nums.len() < k || k == 0 {
        return None;
    }
    let mut max_sum = i32::MIN;

    for i in 0..=(nums.len() - k) {
        let window_sum: i32 = nums[i..i + k].iter().sum(); // O(k) each time
        max_sum = max_sum.max(window_sum);
    }
    Some(max_sum)
}
```

For each starting position, we sum `k` elements. That's `(n - k + 1) * k` operations,
which is O(nk).

The waste is obvious: when sliding from position `i` to `i+1`, the windows overlap
in `k-1` elements. We're re-adding all of them.

```
  Position i:    [ a  b  c  d ]  e
  Position i+1:     a  [ b  c  d  e ]

  Overlap: b, c, d are summed both times.
  Only change: subtract a, add e.
```

### Optimized: O(n)

```rust
fn max_sum_k(nums: &[i32], k: usize) -> Option<i32> {
    if nums.len() < k || k == 0 {
        return None;
    }

    // Compute the sum of the first window
    let mut window_sum: i32 = nums[..k].iter().sum();
    let mut max_sum = window_sum;

    // Slide the window: subtract the leaving element, add the entering one
    for i in k..nums.len() {
        window_sum += nums[i] - nums[i - k];
        max_sum = max_sum.max(window_sum);
    }

    Some(max_sum)
}
```

Trace:

```
nums = [2, 1, 5, 1, 3, 2]    k = 3

Initial window: [2, 1, 5] = 8     max = 8
                 ^     ^
                start  end

Slide 1: remove 2, add 1:  8 - 2 + 1 = 7    [1, 5, 1]    max = 8
Slide 2: remove 1, add 3:  7 - 1 + 3 = 9    [5, 1, 3]    max = 9
Slide 3: remove 5, add 2:  9 - 5 + 2 = 6    [1, 3, 2]    max = 9

Answer: 9
```

**Time**: O(n). One pass.
**Space**: O(1).

---

## 2.2 Variable-Size Window: Smallest Subarray With Sum >= Target

### The Problem

Given an array of positive integers and a target, find the length of the shortest
contiguous subarray whose sum is greater than or equal to the target. If none exists,
return 0.

### Brute Force: O(n^2)

```rust
fn min_subarray_len_brute(nums: &[i32], target: i32) -> usize {
    let n = nums.len();
    let mut min_len = usize::MAX;

    for i in 0..n {
        let mut sum = 0;
        for j in i..n {
            sum += nums[j];
            if sum >= target {
                min_len = min_len.min(j - i + 1);
                break; // found shortest starting at i, no need to continue
            }
        }
    }

    if min_len == usize::MAX { 0 } else { min_len }
}
```

### Optimized: O(n) Variable-Size Sliding Window

The key insight: all values are **positive**, so expanding the window (moving `end`
right) can only increase the sum, and shrinking it (moving `start` right) can only
decrease it. This monotonicity is what makes the sliding window work.

```rust
fn min_subarray_len(nums: &[i32], target: i32) -> usize {
    let mut start = 0;
    let mut sum = 0;
    let mut min_len = usize::MAX;

    for end in 0..nums.len() {
        sum += nums[end]; // expand window

        // Shrink window from the left as long as the constraint is met
        while sum >= target {
            min_len = min_len.min(end - start + 1);
            sum -= nums[start];
            start += 1;
        }
    }

    if min_len == usize::MAX { 0 } else { min_len }
}
```

Trace:

```
nums = [2, 3, 1, 2, 4, 3]    target = 7

end=0: sum=2                          window: [2]
end=1: sum=5                          window: [2,3]
end=2: sum=6                          window: [2,3,1]
end=3: sum=8 >= 7 --> min_len=4       window: [2,3,1,2]
       shrink: sum=8-2=6, start=1     window:   [3,1,2]       6 < 7, stop

end=4: sum=6+4=10 >= 7 --> min_len=4  window:   [3,1,2,4]
       shrink: sum=10-3=7, start=2    window:     [1,2,4]     min_len=3
       shrink: sum=7-1=6, start=3     window:       [2,4]     6 < 7, stop

end=5: sum=6+3=9 >= 7 --> min_len=3   window:       [2,4,3]
       shrink: sum=9-2=7, start=4     window:         [4,3]   min_len=2
       shrink: sum=7-4=3, start=5     window:           [3]   3 < 7, stop

Answer: 2  (the subarray [4, 3])
```

### Why Is This O(n) and Not O(n^2)?

It looks like there's a `while` loop inside a `for` loop, which usually screams O(n^2).
But look at what `start` does: it only ever moves forward, and it can advance at most
`n` times total across the entire execution. Each element is added to the window exactly
once (`end` visits it) and removed at most once (`start` passes it). So the total work
is at most `2n`, which is O(n).

This is an important pattern to internalize: **amortized analysis**. The inner loop
isn't bounded by `n` on each outer iteration -- it's bounded by `n` across all outer
iterations combined.

---

## 2.3 Variable-Size Window: Longest Substring Without Repeating Characters

### The Problem

Given a string, find the length of the longest substring that contains no repeating
characters.

### Brute Force: O(n^2) (or O(n^3) naively)

The truly naive approach checks all substrings and validates each:

```rust
fn longest_unique_brute(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut max_len = 0;

    for i in 0..n {
        let mut seen = std::collections::HashSet::new();
        for j in i..n {
            if seen.contains(&chars[j]) {
                break;
            }
            seen.insert(chars[j]);
            max_len = max_len.max(j - i + 1);
        }
    }
    max_len
}
```

This is O(n^2) because for each starting position we may scan up to `n` characters.

### Optimized: O(n) Sliding Window + HashSet

When we hit a duplicate, instead of starting over from `i + 1`, we shrink the window
from the left until the duplicate is removed.

```rust
use std::collections::HashSet;

fn longest_unique_substring(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut set = HashSet::new();
    let mut start = 0;
    let mut max_len = 0;

    for end in 0..chars.len() {
        // Shrink window until chars[end] is no longer in the set
        while set.contains(&chars[end]) {
            set.remove(&chars[start]);
            start += 1;
        }
        set.insert(chars[end]);
        max_len = max_len.max(end - start + 1);
    }

    max_len
}
```

Trace:

```
s = "abcabcbb"

end=0: 'a'  set={a}          window="a"       max=1
end=1: 'b'  set={a,b}        window="ab"      max=2
end=2: 'c'  set={a,b,c}      window="abc"     max=3
end=3: 'a'  conflict! remove 'a', start=1
             set={b,c,a}     window="bca"     max=3
end=4: 'b'  conflict! remove 'b', start=2
             set={c,a,b}     window="cab"     max=3
end=5: 'c'  conflict! remove 'c', start=3
             set={a,b,c}     window="abc"     max=3
end=6: 'b'  conflict! remove 'a', start=4
             still conflict! remove 'b', start=5
             set={c,b}       window="cb"      max=3
end=7: 'b'  conflict! remove 'c', start=6
             still conflict! remove 'b', start=7
             set={b}         window="b"       max=3

Answer: 3  ("abc")
```

### Faster Variant: HashMap for Jump-Ahead

Instead of shrinking one element at a time, store the *last index* of each character.
When you see a duplicate, jump `start` directly past the previous occurrence:

```rust
use std::collections::HashMap;

fn longest_unique_fast(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    let mut start = 0;
    let mut max_len = 0;

    for end in 0..chars.len() {
        if let Some(&prev_idx) = last_seen.get(&chars[end]) {
            // Only move start forward, never backward
            start = start.max(prev_idx + 1);
        }
        last_seen.insert(chars[end], end);
        max_len = max_len.max(end - start + 1);
    }

    max_len
}
```

Same O(n) time complexity, but avoids the inner `while` loop entirely. Each character
is processed in O(1) amortized time (HashMap operations).

---

## 2.4 Window With HashMap: Frequency Tracking

Many sliding window problems need to track element frequencies inside the window.
The pattern: maintain a HashMap that counts occurrences, and update it as the window
slides.

### Problem: Find All Anagrams

Given a string `s` and a pattern `p`, find all start indices where an anagram of `p`
begins in `s`.

### Approach

An anagram has the same character frequencies. So maintain a frequency map of the
window and compare it to the pattern's frequency map.

```rust
use std::collections::HashMap;

fn find_anagrams(s: &str, p: &str) -> Vec<usize> {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();

    if s.len() < p.len() {
        return vec![];
    }

    // Build frequency map for the pattern
    let mut p_freq: HashMap<char, i32> = HashMap::new();
    for &ch in &p {
        *p_freq.entry(ch).or_insert(0) += 1;
    }

    // Track how many characters still need to be matched
    let mut remaining = p_freq.len(); // distinct chars to match
    let mut w_freq: HashMap<char, i32> = HashMap::new();

    let mut result = Vec::new();
    let k = p.len();

    for end in 0..s.len() {
        // Add s[end] to window
        let ch = s[end];
        *w_freq.entry(ch).or_insert(0) += 1;

        // If this char is in the pattern and its count now matches, decrement remaining
        if let Some(&needed) = p_freq.get(&ch) {
            if w_freq[&ch] == needed {
                remaining -= 1;
            }
        }

        // If window is larger than pattern, remove the leftmost element
        if end >= k {
            let left_ch = s[end - k];
            if let Some(&needed) = p_freq.get(&left_ch) {
                if w_freq[&left_ch] == needed {
                    remaining += 1; // we're about to break a match
                }
            }
            *w_freq.get_mut(&left_ch).unwrap() -= 1;
        }

        // If all characters match, record the start index
        if remaining == 0 {
            result.push(end + 1 - k);
        }
    }

    result
}
```

Trace:

```
s = "cbaebabacd"    p = "abc"    p_freq = {a:1, b:1, c:1}

Window positions:
  [c,b,a]  --> freq={c:1,b:1,a:1} matches!  --> index 0
  [b,a,e]  --> 'e' not in pattern, no match
  [a,e,b]  --> no match
  [e,b,a]  --> no match
  [b,a,b]  --> b:2, doesn't match b:1
  [a,b,a]  --> a:2, doesn't match
  [b,a,c]  --> freq={b:1,a:1,c:1} matches!  --> index 6
  [a,c,d]  --> no match

Answer: [0, 6]
```

**Time**: O(n) -- each character is added and removed from the window exactly once.
**Space**: O(k) where k is the pattern length (for the frequency maps).

The `remaining` counter is a common optimization: instead of comparing two HashMaps
on every step (which would be O(distinct_chars)), you maintain a single counter that
tracks how many distinct characters are fully matched. When it hits 0, you have an
anagram. This keeps each step O(1).

---

## Recognizing When to Use These Techniques

This is the real skill. Knowing the patterns is only useful if you can spot when they
apply.

### Two Pointers: Look for These Signals

| Signal | Pattern |
|--------|---------|
| **Sorted array** + find pair with some property | Opposite-direction |
| **In-place** modification, remove/overwrite elements | Fast/slow same-direction |
| **Partition** into categories (2 or 3 groups) | Multi-pointer partitioning |
| **Linked list** cycle, middle element, or merge | Fast/slow |
| Problem says "two elements" or "pair" | Two pointers |

### Sliding Window: Look for These Signals

| Signal | Pattern |
|--------|---------|
| "Contiguous subarray" or "substring" | Sliding window |
| "Of size k" or "k consecutive" | Fixed-size window |
| "Minimum/maximum length subarray with property X" | Variable-size window |
| Sum/product/count constraint on a contiguous range | Variable-size window |
| "All anagrams" or "permutation in string" | Fixed-size window + frequency map |
| Elements are all positive (sums only increase) | Variable-size window is valid |

### When They Don't Apply

- **Unsorted data** where order doesn't carry meaning: two pointers from opposite
  ends won't help unless you sort first.
- **Non-contiguous** subsequences: sliding window requires contiguity.
- **Negative numbers** in sum-based sliding window: the monotonicity breaks. If adding
  an element can decrease the sum, shrinking the window doesn't reliably help. You may
  need a deque-based approach or prefix sums instead.

---

## The Template

Here are reusable skeletons you can adapt.

### Opposite-Direction Two Pointers

```rust
fn two_pointer_opposite(nums: &[i32]) -> SomeResult {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        // Compute something with nums[left] and nums[right]
        // Based on the result, move left forward or right backward
        if condition_to_advance_left {
            left += 1;
        } else {
            right -= 1;
        }
    }
}
```

### Fast/Slow Two Pointers

```rust
fn two_pointer_fast_slow(nums: &mut [i32]) -> usize {
    let mut slow = 0;

    for fast in 0..nums.len() {
        if should_keep(nums[fast]) {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }
    slow // number of kept elements
}
```

### Fixed-Size Sliding Window

```rust
fn fixed_window(nums: &[i32], k: usize) -> SomeResult {
    // Initialize window state with first k elements
    let mut state = compute_initial(&nums[..k]);
    let mut best = state;

    for i in k..nums.len() {
        state = state + nums[i] - nums[i - k]; // add entering, remove leaving
        best = update(best, state);
    }
    best
}
```

### Variable-Size Sliding Window

```rust
fn variable_window(nums: &[i32], target: SomeTarget) -> SomeResult {
    let mut start = 0;
    let mut state = initial_state();
    let mut best = initial_best();

    for end in 0..nums.len() {
        expand_state(&mut state, nums[end]); // add nums[end] to window

        while window_should_shrink(&state, target) {
            shrink_state(&mut state, nums[start]); // remove nums[start]
            start += 1;
        }

        best = update_best(best, end - start + 1);
    }
    best
}
```

---

## Common Mistakes

**1. Off-by-one errors with window boundaries.**
Fixed-size windows have `end - start + 1 == k`. Carefully track whether your `start`
is inclusive or exclusive. Drawing the window on paper before coding saves grief.

**2. Forgetting to handle the empty/short input.**
Always check `nums.len() < k` for fixed-size windows. Check `nums.is_empty()` for
two-pointer problems where you'd subtract from `len()`.

**3. Moving the wrong pointer.**
In opposite-direction two pointers, the decision of which pointer to move is the crux
of correctness. If you move the wrong one, you might skip the optimal pair. Always
prove (even informally) why your movement rule doesn't miss valid solutions.

**4. Applying sliding window when values can be negative.**
If the problem involves sums and the array can contain negative numbers, expanding the
window might decrease the sum and shrinking it might increase it. The monotonicity
assumption breaks. Consider prefix sums + binary search or deque-based approaches.

**5. Not using `start.max(...)` in the jump-ahead HashMap variant.**
When you store last-seen indices, a character's stored index might be from *before*
the current window. Always ensure `start` only moves forward:
`start = start.max(prev_idx + 1)`, not `start = prev_idx + 1`.

---

## Complexity Summary

| Technique | Time | Space | Key Requirement |
|-----------|------|-------|-----------------|
| Two Sum (sorted) | O(n) | O(1) | Sorted input |
| Container With Most Water | O(n) | O(1) | Greedy narrowing |
| Remove Duplicates | O(n) | O(1) | Sorted input |
| Dutch National Flag | O(n) | O(1) | Finite categories |
| Max Sum of K Elements | O(n) | O(1) | Fixed window |
| Min Subarray >= Target | O(n) | O(1) | Positive values |
| Longest Unique Substring | O(n) | O(min(n,charset)) | HashSet/HashMap |
| Find All Anagrams | O(n) | O(k) | Fixed window + freq map |

Every single one replaces an O(n^2) or O(nk) brute force with an O(n) single-pass
(or two-pass) algorithm. The space is almost always O(1) or bounded by the alphabet
or pattern size. That's the power of these techniques: they don't trade space for time.
They trade *insight* for time.

---

## Exercises

1. **Three Sum**: Given an array, find all unique triplets that sum to zero. (Hint:
   sort, then for each element, run two-sum on the remainder.)

2. **Move Zeroes**: Given an array, move all zeroes to the end while maintaining the
   relative order of non-zero elements. Do it in-place. (Fast/slow pointer.)

3. **Maximum Average Subarray**: Find the contiguous subarray of length `k` with the
   maximum average value. (Fixed-size window -- the average is just sum/k.)

4. **Minimum Window Substring**: Given strings `s` and `t`, find the smallest window
   in `s` that contains all characters of `t`. (Variable window + frequency map.
   This is the hardest classic sliding window problem.)

5. **Trapping Rain Water**: Given an elevation map, compute how much water it can
   trap after rain. (Two pointers from opposite ends, tracking max heights.)
