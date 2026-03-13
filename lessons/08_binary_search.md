# Lesson 08: Binary Search & Its Many Variations

## The Core Intuition

You have done this a thousand times without calling it "binary search."

Think about looking up the word "mnemonic" in a physical dictionary. You do not start at
page one and read every word. You open the book roughly in the middle, see you are at "M"
-- good, close. You flip forward a bit, land on "mo" -- too far. You flip back slightly,
find "mn" -- there it is. Each decision cuts the remaining pages roughly in half.

Or think about the number guessing game. I am thinking of a number between 1 and 100. You
guess 50, I say "higher." You guess 75, I say "lower." You guess 63, I say "higher." Each
guess eliminates half the remaining possibilities. In at most 7 guesses you will nail it
every time, because log2(100) < 7.

That is binary search: **repeatedly halving the search space** until you find what you are
looking for (or prove it does not exist).

---

## Binary Search on a Sorted Array

The most classic form. You have a sorted slice, you want to find a target value.

```rust
fn binary_search(sorted: &[i32], target: i32) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = sorted.len(); // exclusive upper bound

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match sorted[mid].cmp(&target) {
            std::cmp::Ordering::Equal   => return Some(mid),
            std::cmp::Ordering::Less    => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}
```

Let's trace through an example to see the halving in action:

```
  sorted = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91]
  target = 23
  indices: 0  1  2   3   4   5   6   7   8   9

  Iteration 1:  lo=0  hi=10  mid=5
  [2, 5, 8, 12, 16, 23, 38, 56, 72, 91]
   lo                 mid                hi
  sorted[5] = 23 == 23 --> found! return Some(5)
```

That was lucky. Let's search for 8 instead:

```
  sorted = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91]
  target = 8

  Iteration 1:  lo=0  hi=10  mid=5
  [ 2,  5,  8, 12, 16, 23, 38, 56, 72, 91]
    lo                 mid                hi
  sorted[5]=23 > 8  -->  hi = 5

  Iteration 2:  lo=0  hi=5  mid=2
  [ 2,  5,  8, 12, 16]
    lo      mid        hi
  sorted[2]=8 == 8  -->  found! return Some(2)
```

And searching for 10 (not in the array):

```
  sorted = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91]
  target = 10

  Iteration 1:  lo=0  hi=10  mid=5
  sorted[5]=23 > 10  -->  hi = 5

  Iteration 2:  lo=0  hi=5  mid=2
  sorted[2]=8 < 10   -->  lo = 3

  Iteration 3:  lo=3  hi=5  mid=4
  sorted[4]=16 > 10  -->  hi = 4

  Iteration 4:  lo=3  hi=4  mid=3
  sorted[3]=12 > 10  -->  hi = 3

  lo=3  hi=3  -->  lo == hi, loop ends. return None.

  The search space narrowed like this:
  [2, 5, 8, 12, 16, 23, 38, 56, 72, 91]   10 elements
  [2, 5, 8, 12, 16]                          5 elements
  [12, 16]                                    2 elements
  [12]                                        1 element
  []                                          0 -- not found
```

---

## Why It Is O(log n)

Each iteration cuts the search space in half. If you start with `n` elements:

```
  Step 0:  n elements
  Step 1:  n/2
  Step 2:  n/4
  Step 3:  n/8
  ...
  Step k:  n / 2^k

  We stop when n / 2^k = 1, i.e., k = log2(n).
```

For concrete numbers:

```
  n = 1,000       --> ~10 steps    (2^10 = 1,024)
  n = 1,000,000   --> ~20 steps    (2^20 = 1,048,576)
  n = 1,000,000,000 --> ~30 steps  (2^30 = 1,073,741,824)
```

A billion-element sorted array, and you find your answer in 30 comparisons. Compare that
to linear search at a billion comparisons. This is the power of logarithmic time.

---

## The Off-by-One Minefield: Inclusive vs. Exclusive Bounds

Binary search has a well-earned reputation for off-by-one bugs. The root cause is
ambiguity about what `lo` and `hi` represent. There are two common conventions, and
mixing them up is how you get infinite loops or missed elements.

### Convention 1: Half-Open Interval `[lo, hi)`

This is the convention I used above, and it is the one I recommend. It matches how Rust
ranges work (`0..n` is `[0, n)`).

```
  lo = 0            (first valid index)
  hi = sorted.len() (one past the last valid index)

  Search space: indices lo, lo+1, ..., hi-1
  Empty when: lo == hi
  Mid: lo + (hi - lo) / 2

  On miss high:  hi = mid       (mid is too big, exclude it)
  On miss low:   lo = mid + 1   (mid is too small, exclude it)
```

### Convention 2: Closed Interval `[lo, hi]`

```
  lo = 0
  hi = sorted.len() - 1   (last valid index -- careful with empty slices!)

  Search space: indices lo, lo+1, ..., hi
  Empty when: lo > hi
  Mid: lo + (hi - lo) / 2

  On miss high:  hi = mid - 1   (mid is too big, exclude it)
  On miss low:   lo = mid + 1   (mid is too small, exclude it)
```

Both work. But the half-open convention is less error-prone because:
- You never need to subtract 1 from `sorted.len()` (which panics on an empty slice with
  unsigned types).
- The "exclude mid" logic is asymmetric (`hi = mid` vs `lo = mid + 1`), which makes it
  impossible to accidentally write `lo = mid` (a common source of infinite loops).

**The golden rule:** pick one convention, internalize it, and never deviate.

---

## Common Mistake: Integer Overflow in Mid Calculation

In languages with fixed-width integers (C, C++, Java), the naive `mid = (lo + hi) / 2`
can overflow if `lo + hi > MAX_INT`. The safe version is:

```
  mid = lo + (hi - lo) / 2
```

In Rust with `usize`, you will not hit overflow on a 64-bit machine (you would need an
array with more than 2^63 elements, which is impossible). But the subtraction form is
still preferred because:
1. It is a good habit that transfers to other languages.
2. It works correctly even with the closed-interval convention where `hi` could be
   `usize::MAX` in edge cases.

```rust
// Dangerous in C/Java (not really in Rust, but still bad habit):
let mid = (lo + hi) / 2;

// Safe everywhere:
let mid = lo + (hi - lo) / 2;
```

---

## Left-Bound Binary Search: Finding the First Occurrence

Standard binary search finds *some* index of the target. But what if the array has
duplicates and you need the *first* (leftmost) occurrence?

```
  sorted = [1, 3, 5, 5, 5, 5, 8, 9]
  target = 5

  Standard binary search might return index 2, 3, 4, or 5 -- any of them.
  Left-bound search always returns index 2 (the first 5).
```

The trick: when you find the target, do not stop. Instead, **keep searching left** by
setting `hi = mid`.

```rust
/// Returns the index of the first element equal to `target`,
/// or None if `target` is not in the slice.
fn left_bound(sorted: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0;
    let mut hi = sorted.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if sorted[mid] < target {
            lo = mid + 1;
        } else {
            // sorted[mid] >= target: could be the answer, keep searching left
            hi = mid;
        }
    }

    // lo is now the index of the first element >= target.
    // Check if it actually equals target.
    if lo < sorted.len() && sorted[lo] == target {
        Some(lo)
    } else {
        None
    }
}
```

Trace:

```
  sorted = [1, 3, 5, 5, 5, 5, 8, 9]    target = 5
            0  1  2  3  4  5  6  7

  Iter 1:  lo=0  hi=8  mid=4   sorted[4]=5 >= 5  -->  hi=4
  Iter 2:  lo=0  hi=4  mid=2   sorted[2]=5 >= 5  -->  hi=2
  Iter 3:  lo=0  hi=2  mid=1   sorted[1]=3 < 5   -->  lo=2
  lo=2  hi=2  -->  loop ends.  sorted[2]=5 == target --> Some(2)
```

Notice how the search did not stop when it first saw a 5 at mid=4. It kept narrowing
leftward until it pinpointed the very first 5.

---

## Right-Bound Binary Search: Finding the Last Occurrence

Mirror image: find the *last* (rightmost) occurrence.

```rust
/// Returns the index of the last element equal to `target`,
/// or None if `target` is not in the slice.
fn right_bound(sorted: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0;
    let mut hi = sorted.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if sorted[mid] <= target {
            // sorted[mid] could be the answer, keep searching right
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    // lo is now the index of the first element > target.
    // The last element == target is at lo - 1 (if it exists).
    if lo > 0 && sorted[lo - 1] == target {
        Some(lo - 1)
    } else {
        None
    }
}
```

Trace:

```
  sorted = [1, 3, 5, 5, 5, 5, 8, 9]    target = 5
            0  1  2  3  4  5  6  7

  Iter 1:  lo=0  hi=8  mid=4   sorted[4]=5 <= 5  -->  lo=5
  Iter 2:  lo=5  hi=8  mid=6   sorted[6]=8 > 5   -->  hi=6
  Iter 3:  lo=5  hi=6  mid=5   sorted[5]=5 <= 5  -->  lo=6
  lo=6  hi=6  -->  loop ends.  sorted[6-1]=sorted[5]=5 == target --> Some(5)
```

With left-bound and right-bound, you can count occurrences of a value in O(log n):

```rust
fn count_occurrences(sorted: &[i32], target: i32) -> usize {
    match (left_bound(sorted, target), right_bound(sorted, target)) {
        (Some(l), Some(r)) => r - l + 1,
        _ => 0,
    }
}
```

---

## Search Insert Position

A variation often seen in practice and interviews: given a sorted array and a target,
return the index where the target is found, or the index where it *would* be inserted to
keep the array sorted.

```rust
/// Returns the index at which `target` should be inserted to maintain sort order.
/// If `target` already exists, returns its index.
fn search_insert(sorted: &[i32], target: i32) -> usize {
    let mut lo = 0;
    let mut hi = sorted.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if sorted[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}
```

This is exactly the left-bound search, but without the final "is it actually equal?"
check. The value of `lo` at the end is the correct insertion point regardless.

```
  sorted = [1, 3, 5, 7, 9]

  search_insert(sorted, 5)  --> 2  (found at index 2)
  search_insert(sorted, 6)  --> 3  (would go between 5 and 7)
  search_insert(sorted, 0)  --> 0  (would go at the front)
  search_insert(sorted, 10) --> 5  (would go at the end)
```

---

## Rust's Built-in Binary Search Tools

Rust's standard library gives you two excellent tools. Know them well -- they cover most
use cases and save you from writing the loop yourself.

### `slice.binary_search()`

Returns `Result<usize, usize>`:
- `Ok(index)` -- the target was found at `index`.
- `Err(index)` -- the target was not found; `index` is where it would be inserted.

```rust
fn main() {
    let data = [1, 3, 5, 7, 9, 11];

    // Found:
    assert_eq!(data.binary_search(&7), Ok(3));

    // Not found: 6 would be inserted at index 3
    assert_eq!(data.binary_search(&6), Err(3));

    // Edge: 0 would go at the front
    assert_eq!(data.binary_search(&0), Err(0));

    // Edge: 100 would go at the end
    assert_eq!(data.binary_search(&100), Err(6));
}
```

**Important caveat:** if there are duplicates, `binary_search` returns *an arbitrary*
matching index, not necessarily the first or last. For that you need `partition_point`.

There is also `binary_search_by` for custom comparators:

```rust
let data = [(1, "one"), (3, "three"), (5, "five"), (7, "seven")];

// Search by the first element of each tuple:
let result = data.binary_search_by(|probe| probe.0.cmp(&5));
assert_eq!(result, Ok(2));

// Or use binary_search_by_key:
let result = data.binary_search_by_key(&5, |&(k, _)| k);
assert_eq!(result, Ok(2));
```

### `slice.partition_point()`

This is the Swiss Army knife. It takes a predicate and returns the index of the first
element for which the predicate returns `false`. The slice must be partitioned such that
all `true` elements come before all `false` elements (which a sorted slice naturally is,
for monotonic predicates).

```rust
fn main() {
    let data = [1, 3, 5, 5, 5, 7, 9, 11];

    // Index of first element >= 5  (left bound):
    let left = data.partition_point(|&x| x < 5);
    assert_eq!(left, 2);

    // Index of first element > 5  (one past right bound):
    let right = data.partition_point(|&x| x <= 5);
    assert_eq!(right, 5);

    // Count of 5s:
    assert_eq!(right - left, 3);

    // Search insert position for 6:
    let insert = data.partition_point(|&x| x < 6);
    assert_eq!(insert, 5); // goes between the 5s and 7
}
```

`partition_point` is the most general binary search primitive. Once you understand it,
you can express left-bound, right-bound, search-insert, and more, all as one-liners.

Think of it this way:

```
  [true, true, true, true, false, false, false, false]
                            ^
                            partition_point returns this index

  The predicate divides the slice into two regions:
  [  satisfies predicate  |  does not satisfy  ]
                           ^
                           returned index
```

---

## Binary Search on Answer (Search Space Over Values, Not Indices)

This is where binary search goes from "data structure technique" to "algorithmic
paradigm." The idea: instead of searching for a target in an array, you binary search
over the space of possible *answers* to an optimization problem.

The pattern applies whenever:
1. The answer is a number in some range `[lo, hi]`.
2. There is a monotonic feasibility function: all answers below some threshold are
   infeasible, and all answers at or above it are feasible (or vice versa).
3. You can check feasibility for a given answer efficiently.

### Example: Minimum Ship Capacity (LeetCode 1011)

**Problem:** You have packages with weights `[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]` and you
must ship them in order within `days = 5` days. What is the minimum ship capacity needed?

The answer must be between `max(weights)` (can carry at least the heaviest package) and
`sum(weights)` (could carry everything in one day). We binary search this range.

```rust
/// Can we ship all packages within `days` if the ship has capacity `cap`?
fn can_ship(weights: &[i32], days: usize, cap: i32) -> bool {
    let mut current_load = 0;
    let mut days_needed = 1;

    for &w in weights {
        if current_load + w > cap {
            days_needed += 1;
            current_load = 0;
        }
        current_load += w;
    }
    days_needed <= days
}

fn min_ship_capacity(weights: &[i32], days: usize) -> i32 {
    let mut lo = *weights.iter().max().unwrap();       // minimum possible
    let mut hi = weights.iter().sum::<i32>();           // maximum possible

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if can_ship(weights, days, mid) {
            hi = mid;       // mid works, but maybe something smaller does too
        } else {
            lo = mid + 1;   // mid is too small
        }
    }
    lo
}

fn main() {
    let weights = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let days = 5;
    let result = min_ship_capacity(&weights, days);
    println!("Minimum capacity: {result}"); // 15
}
```

The search space visualization:

```
  capacity:  10  11  12  13  14  15  16  17  ...  55
  feasible?   N   N   N   N   N   Y   Y   Y  ...   Y
                                  ^
                                  answer = 15

  [  infeasible  |  feasible  ]
                  ^
                  binary search finds this boundary
```

This is exactly `partition_point` logic, applied to an abstract value range instead of an
array. The feasibility function `can_ship` is O(n), and we call it O(log(sum - max))
times, so the total is O(n * log(sum)).

### The General Template

```rust
fn binary_search_on_answer(lo: i64, hi: i64, feasible: impl Fn(i64) -> bool) -> i64 {
    let mut lo = lo;
    let mut hi = hi;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if feasible(mid) {
            hi = mid;       // minimize: look for smaller feasible answer
        } else {
            lo = mid + 1;
        }
    }
    lo
}
```

For maximization problems (find the largest feasible answer), flip the logic:

```rust
fn binary_search_max(lo: i64, hi: i64, feasible: impl Fn(i64) -> bool) -> i64 {
    let mut lo = lo;
    let mut hi = hi;

    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;  // Note: round UP to avoid infinite loop
        if feasible(mid) {
            lo = mid;       // maximize: look for larger feasible answer
        } else {
            hi = mid - 1;
        }
    }
    lo
}
```

Watch the rounding in `binary_search_max`. When `lo = mid` is possible, you must round
up: `lo + (hi - lo + 1) / 2`. Otherwise, when `hi = lo + 1`, `mid` rounds down to `lo`,
and if `feasible(lo)` is true, you set `lo = mid = lo` -- an infinite loop.

---

## Binary Search on Rotated Sorted Arrays

A rotated sorted array looks like this:

```
  Original sorted: [1, 2, 3, 4, 5, 6, 7, 8]
  Rotated by 3:    [4, 5, 6, 7, 8, 1, 2, 3]
                    ^^^^^^^^^^^^^^^  ^^^^^^^^
                    sorted segment   sorted segment
                                  ^
                                  pivot (where the "break" is)
```

The array is not globally sorted, but it consists of two sorted segments. Binary search
still works because at any midpoint, at least one half is sorted.

### Finding the Minimum (Pivot)

```rust
fn find_min_rotated(nums: &[i32]) -> i32 {
    let mut lo = 0;
    let mut hi = nums.len() - 1;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] > nums[hi] {
            // Mid is in the left (higher) segment. Min is to the right.
            lo = mid + 1;
        } else {
            // Mid is in the right (lower) segment. Mid could be the min.
            hi = mid;
        }
    }
    nums[lo]
}

fn main() {
    assert_eq!(find_min_rotated(&[4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(find_min_rotated(&[3, 1, 2]), 1);
    assert_eq!(find_min_rotated(&[1, 2, 3, 4, 5]), 1); // not rotated
}
```

Visual trace:

```
  nums = [4, 5, 6, 7, 0, 1, 2]
          0  1  2  3  4  5  6

  Iter 1:  lo=0  hi=6  mid=3   nums[3]=7 > nums[6]=2  -->  lo=4
  Iter 2:  lo=4  hi=6  mid=5   nums[5]=1 <= nums[6]=2  --> hi=5
  Iter 3:  lo=4  hi=5  mid=4   nums[4]=0 <= nums[5]=1  --> hi=4
  lo=4  hi=4  -->  answer is nums[4] = 0
```

### Searching in a Rotated Sorted Array

```rust
fn search_rotated(nums: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0;
    let mut hi = nums.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        if nums[mid] == target {
            return Some(mid);
        }

        if nums[lo] <= nums[mid] {
            // Left half [lo..mid] is sorted.
            if nums[lo] <= target && target < nums[mid] {
                hi = mid;       // Target is in the sorted left half.
            } else {
                lo = mid + 1;   // Target is in the right half.
            }
        } else {
            // Right half [mid..hi) is sorted.
            if nums[mid] < target && target <= nums[hi - 1] {
                lo = mid + 1;   // Target is in the sorted right half.
            } else {
                hi = mid;       // Target is in the left half.
            }
        }
    }
    None
}

fn main() {
    let nums = [4, 5, 6, 7, 0, 1, 2];
    assert_eq!(search_rotated(&nums, 0), Some(4));
    assert_eq!(search_rotated(&nums, 3), None);
    assert_eq!(search_rotated(&nums, 5), Some(1));
}
```

The key insight: at every step, one half is guaranteed to be sorted. You check whether
the target falls in the sorted half (easy range check), and if not, search the other
half.

```
  nums = [4, 5, 6, 7, 0, 1, 2]    target = 0

  Iter 1:  lo=0  hi=7  mid=3  nums[3]=7
    Left [4,5,6,7] is sorted (nums[0]=4 <= nums[3]=7).
    Is 0 in [4, 7)? No.  --> lo = 4

  Iter 2:  lo=4  hi=7  mid=5  nums[5]=1
    Left [0,1] -- nums[4]=0 <= nums[5]=1 -> sorted.
    Is 0 in [0, 1)? Yes! --> hi = 5

  Iter 3:  lo=4  hi=5  mid=4  nums[4]=0
    nums[4] == target --> return Some(4)
```

---

## Common Mistakes (and How to Avoid Them)

### 1. Forgetting to Sort First

Binary search requires a sorted input. This sounds obvious, but it is the number one bug
in practice: someone calls `binary_search()` on unsorted data and gets wrong results
silently. No panic, no error -- just a wrong answer.

```rust
let mut data = vec![5, 2, 8, 1, 9];

// BUG: data is not sorted!
let result = data.binary_search(&8); // Undefined behavior (not UB in Rust's
                                     // memory-safety sense, but logically wrong)

// Fix:
data.sort();
let result = data.binary_search(&8); // Now correct
```

### 2. Infinite Loops from Wrong Bound Updates

The most insidious bug. It happens when neither `lo` nor `hi` changes:

```rust
// BUG: lo = mid instead of lo = mid + 1
// When lo = 3, hi = 4: mid = 3, then lo = mid = 3. Loop forever.
while lo < hi {
    let mid = lo + (hi - lo) / 2;
    if sorted[mid] < target {
        lo = mid;   // <-- WRONG! Should be mid + 1
    } else {
        hi = mid;
    }
}
```

The fix: with the `[lo, hi)` convention, always use `lo = mid + 1` for the "go right"
case. The only time `lo = mid` is correct is in the "maximize" pattern with the ceiling
division `(hi - lo + 1) / 2`.

### 3. Off-by-One on Empty Slices

With the closed-interval convention `[lo, hi]`, initializing `hi = sorted.len() - 1`
panics when the slice is empty because `0usize - 1` wraps around. The half-open
convention `[lo, hi)` with `hi = sorted.len()` handles empty slices naturally: the loop
condition `lo < hi` is immediately false, so you return "not found."

### 4. Using the Wrong Comparison Direction

When adapting binary search for left-bound vs right-bound, the comparison operators are
subtly different:

```
  Left-bound  (first occurrence):   if sorted[mid] < target  { lo = mid + 1 } else { hi = mid }
  Right-bound (last occurrence):    if sorted[mid] <= target { lo = mid + 1 } else { hi = mid }
                                                ^^
                                          this one character is the entire difference
```

That single `=` in `<=` is the difference between "keep going right past equal elements"
and "stop at the first equal element." Get this wrong and you find the wrong boundary.

---

## A Decision Tree for Choosing Your Binary Search Variant

```
  "What are you searching for?"
       |
       |--- A specific value in a sorted array?
       |     |
       |     |--- Any index is fine?  --> slice.binary_search()
       |     |--- First occurrence?   --> partition_point(|x| x < target)
       |     |--- Last occurrence?    --> partition_point(|x| x <= target) - 1
       |
       |--- An insertion point?       --> partition_point(|x| x < target)
       |                                  (equivalently: Err case of binary_search)
       |
       |--- The minimum/maximum answer
       |    to an optimization problem? --> binary search on answer
       |
       |--- A value in a rotated array? --> custom logic (check which half is sorted)
```

---

## Putting It Together: partition_point as the Universal Binary Search

Once you internalize `partition_point`, you can solve almost every binary search variant
with it. Here is a summary of how each variant maps:

```rust
fn demo(sorted: &[i32], target: i32) {
    // Search insert position (or lower bound):
    let insert_pos = sorted.partition_point(|&x| x < target);

    // Upper bound (first element strictly greater):
    let upper = sorted.partition_point(|&x| x <= target);

    // First occurrence of target:
    let first = {
        let i = sorted.partition_point(|&x| x < target);
        if i < sorted.len() && sorted[i] == target { Some(i) } else { None }
    };

    // Last occurrence of target:
    let last = {
        let i = sorted.partition_point(|&x| x <= target);
        if i > 0 && sorted[i - 1] == target { Some(i - 1) } else { None }
    };

    // Count of target:
    let count = upper - insert_pos;

    println!("insert_pos={insert_pos}, upper={upper}");
    println!("first={first:?}, last={last:?}, count={count}");
}

fn main() {
    let sorted = [1, 3, 5, 5, 5, 7, 9, 11];
    demo(&sorted, 5);
    // insert_pos=2, upper=5
    // first=Some(2), last=Some(4), count=3
}
```

---

## Complexity Summary

| Variant                    | Time        | Space  |
|----------------------------|-------------|--------|
| Basic binary search        | O(log n)    | O(1)   |
| Left/right bound           | O(log n)    | O(1)   |
| Search insert position     | O(log n)    | O(1)   |
| Rotated array search       | O(log n)    | O(1)   |
| Binary search on answer    | O(f(n) * log R) | O(1) |

Where `f(n)` is the cost of the feasibility check and `R` is the size of the answer
range.

---

## Key Takeaways

1. **Binary search requires a sorted (or partitioned) input.** If the input is not
   sorted, sort it first (O(n log n)) or use a different approach.

2. **Use the half-open interval convention `[lo, hi)`.** It handles edge cases cleanly
   and matches Rust's range semantics.

3. **Use `lo + (hi - lo) / 2` for mid.** It avoids overflow and is a good habit across
   languages.

4. **`partition_point` is the Swiss Army knife.** Left-bound, right-bound, insert
   position -- they are all just different predicates passed to the same machinery.

5. **Binary search on answer** turns optimization problems into decision problems. If you
   can write a monotonic `feasible(x) -> bool` function, you can binary search the answer
   space.

6. **Watch out for infinite loops.** With `[lo, hi)`, always use `lo = mid + 1` for the
   "go right" case. The only exception is the "maximize" pattern, which requires ceiling
   division.

7. **Do not call `binary_search` on unsorted data.** Rust will not panic. It will just
   give you a wrong answer. This is your responsibility.

---

## Exercises

1. **Search Insert Position** -- Given a sorted array and a target, return the index where
   the target would be inserted. First write it from scratch, then rewrite it using
   `partition_point`.

2. **Count Occurrences** -- Given a sorted array with duplicates, count how many times a
   given value appears, in O(log n) time.

3. **Find Peak Element** -- Given an array where `nums[i] != nums[i+1]` for all i, find
   any local maximum (an element greater than its neighbors). Do it in O(log n).
   (Hint: binary search works even though the array is not sorted. Think about which
   direction the peak must be relative to mid.)

4. **Koko Eating Bananas** (LeetCode 875) -- Binary search on answer. Koko can eat
   bananas at speed `k` per hour. Given piles and hours available, find the minimum `k`.

5. **Search in Rotated Sorted Array** -- Implement `search_rotated` from scratch. Then
   extend it to handle duplicates (what changes?).

6. **Square Root** -- Implement integer square root using binary search: find the largest
   `x` such that `x * x <= n`. Use the "maximize" pattern.

---

*Next up: [Lesson 09 -- Sorting Algorithms](09_sorting.md) -- where we see why O(n log n)
is the best you can do for comparison-based sorting, and learn the algorithms that
achieve it.*
