# Lesson 13: Counting Sort & Radix Sort

## Breaking the Speed Limit

Every comparison-based sort you have seen so far -- merge sort, quicksort, heapsort
-- has a hard floor: **O(n log n)**. That is not a limitation of any particular
algorithm. It is a provable mathematical lower bound on *any* sort that works by
comparing pairs of elements.

The argument goes like this. A comparison sort makes binary decisions: "is a < b?"
Each decision splits the remaining possible orderings in half (at best). There are
n! possible permutations of n elements. You need enough yes/no decisions to
distinguish all of them:

    number of comparisons >= log2(n!)

By Stirling's approximation, log2(n!) is Theta(n log n). So no matter how clever
you are with comparisons, you cannot sort faster than O(n log n) in the worst case.

But what if you **don't compare elements at all**?

That is exactly what counting sort and radix sort do. They exploit the *structure*
of the keys themselves -- the fact that they are integers in a known range -- to
sort in linear time. The tradeoff: they only work when your keys have that structure.

---

## Counting Sort

### The Analogy: Sorting Mail

Imagine you work in a mail room. Letters arrive with apartment numbers on them,
ranging from 0 to 9. You have 10 mailboxes on the wall, labeled 0 through 9.

You don't compare letters to each other. You just read the number on each letter
and drop it into the corresponding mailbox. When you are done, you collect letters
from mailbox 0, then 1, then 2, and so on. Sorted.

That is counting sort in a nutshell. Instead of asking "which element is bigger?",
you ask "what value does this element have?" and place it directly.

### Step by Step

Let's sort this array:

```
input:  [4, 2, 2, 8, 3, 3, 1]
         0  1  2  3  4  5  6    (indices)

range of values: 0..=8  (so k = 9 possible values)
```

**Step 1: Count occurrences.**

Walk through the input and tally how many times each value appears:

```
value:   0  1  2  3  4  5  6  7  8
count:  [0, 1, 2, 2, 1, 0, 0, 0, 1]
              ^  ^  ^  ^           ^
              |  |  |  |           one 8
              |  |  |  one 4
              |  |  two 3s
              |  two 2s
              one 1
```

**Step 2: Compute cumulative (prefix) sums.**

Transform the count array so that count[i] tells you how many elements have value
<= i. This tells you *where* each group ends in the output:

```
value:        0  1  2  3  4  5  6  7  8
count:       [0, 1, 2, 2, 1, 0, 0, 0, 1]
cumulative:  [0, 1, 3, 5, 6, 6, 6, 6, 7]
                  ^  ^  ^  ^              ^
                  |  |  |  |              7 elements total <= 8
                  |  |  |  6 elements <= 4
                  |  |  5 elements <= 3
                  |  3 elements <= 2
                  1 element <= 1
```

The cumulative count tells you: "elements with value 2 go at output positions
1 and 2" (positions cumulative[1] through cumulative[2] - 1, i.e. indices 1..3).

**Step 3: Build the output (right to left for stability).**

Walk the input array **from right to left**. For each element, use the cumulative
count to place it, then decrement that count:

```
Processing input right to left:

  input[6] = 1  ->  cumulative[1] = 1  ->  output[0] = 1  ->  dec cumulative[1] to 0
  input[5] = 3  ->  cumulative[3] = 5  ->  output[4] = 3  ->  dec cumulative[3] to 4
  input[4] = 3  ->  cumulative[3] = 4  ->  output[3] = 3  ->  dec cumulative[3] to 3
  input[3] = 8  ->  cumulative[8] = 7  ->  output[6] = 8  ->  dec cumulative[8] to 6
  input[2] = 2  ->  cumulative[2] = 3  ->  output[2] = 2  ->  dec cumulative[2] to 2
  input[1] = 2  ->  cumulative[2] = 2  ->  output[1] = 2  ->  dec cumulative[2] to 1
  input[0] = 4  ->  cumulative[4] = 6  ->  output[5] = 4  ->  dec cumulative[4] to 5

Output after all placements:

  index:   0  1  2  3  4  5  6
  output: [1, 2, 2, 3, 3, 4, 8]
```

Here is the full picture as ASCII art:

```
  INPUT:  [ 4 | 2 | 2 | 8 | 3 | 3 | 1 ]

           |       Count occurrences
           v
  COUNT:  [ 0 | 1 | 2 | 2 | 1 | 0 | 0 | 0 | 1 ]
            0   1   2   3   4   5   6   7   8

           |       Prefix sum
           v
  CUMUL:  [ 0 | 1 | 3 | 5 | 6 | 6 | 6 | 6 | 7 ]
            0   1   2   3   4   5   6   7   8

           |       Place elements (right to left)
           v
  OUTPUT: [ 1 | 2 | 2 | 3 | 3 | 4 | 8 ]
```

### Rust Implementation

```rust
/// Sorts a slice of non-negative integers using counting sort.
/// `max_val` is the maximum possible value in the input (inclusive).
/// Returns a new sorted Vec.
fn counting_sort(input: &[usize], max_val: usize) -> Vec<usize> {
    let n = input.len();
    if n == 0 {
        return vec![];
    }

    // Step 1: Count occurrences
    let mut count = vec![0usize; max_val + 1];
    for &val in input {
        count[val] += 1;
    }

    // Step 2: Cumulative sum -- count[i] becomes the number of
    // elements with value <= i
    for i in 1..=max_val {
        count[i] += count[i - 1];
    }

    // Step 3: Build output array, iterating right-to-left for stability
    let mut output = vec![0usize; n];
    for &val in input.iter().rev() {
        count[val] -= 1;
        output[count[val]] = val;
    }

    output
}

fn main() {
    let data = vec![4, 2, 2, 8, 3, 3, 1];
    let sorted = counting_sort(&data, 8);
    println!("{:?}", sorted); // [1, 2, 2, 3, 3, 4, 8]
}
```

A few things to notice:

1. **No comparisons anywhere.** We never ask "is a < b?". We index directly.
2. **Two allocations**: the count array (size k+1) and the output array (size n).
3. **We iterate right-to-left in step 3.** This is what makes the sort **stable**.
   More on that in a moment.

### Complexity

| | Time | Space |
|---|---|---|
| **Counting Sort** | O(n + k) | O(n + k) |

Where n is the number of elements and k is the range of possible values
(max_val + 1).

- We scan the input once: O(n).
- We compute prefix sums over the count array: O(k).
- We build the output: O(n).
- Total: O(n + k).

When k is O(n) or smaller, this is linear. When k is enormous -- say you are
sorting 100 integers but they can range up to 10 billion -- you would need a
10-billion-element count array. That is when counting sort stops being practical
and you reach for something else.

---

## Stability, and Why It Matters

A sort is **stable** if elements with equal keys keep their original relative
order. If your input has two 3s and the first 3 appeared before the second 3 in
the input, a stable sort guarantees the first 3 still comes first in the output.

Why does this matter? Two reasons:

1. **Sorting by multiple keys.** If you sort a list of employees first by
   department, then stable-sort by salary, employees within the same salary
   bracket stay grouped by department. Each successive stable sort preserves the
   work of previous sorts.

2. **It is a prerequisite for radix sort.** Radix sort works by sorting digit
   by digit, from least significant to most significant. Each digit-sort must
   be stable so that the ordering from previous digits is preserved.

This is why we iterate right-to-left in counting sort's placement step. If we
went left-to-right, elements with equal values would end up in reversed order
relative to the input -- which is anti-stable. Right-to-left preserves the
original order.

To see why: when two elements have the same value, the one that appears *later*
in the input gets processed first (right-to-left) and gets a higher output index.
The one that appears *earlier* gets processed second and gets a lower output
index. Earlier element ends up before later element. Stable.

---

## Radix Sort

### The Analogy: Sorting Playing Cards

Imagine you have a deck of cards and you want to sort them first by suit
(clubs < diamonds < hearts < spades) and then by rank (2 through Ace) within
each suit.

One approach: sort by the *least significant* attribute first (rank), then by the
*most significant* attribute (suit), using a stable sort each time.

After sorting by rank, all the 2s are together, all the 3s, etc. Then when you
stable-sort by suit, cards within the same suit stay in rank order because the
sort is stable.

This is the **Least Significant Digit (LSD) radix sort** strategy applied to
two "digits": rank and suit. For integers, the "digits" are literal digits.

### Step by Step

Let's sort these numbers using base-10 radix sort:

```
input: [329, 457, 657, 839, 436, 720, 355]
```

We process digit by digit, starting from the **rightmost (least significant)**:

```
=== Pass 1: Sort by ones digit (d = 1) ===

  329 -> ones digit 9
  457 -> ones digit 7
  657 -> ones digit 7
  839 -> ones digit 9
  436 -> ones digit 6
  720 -> ones digit 0
  355 -> ones digit 5

  Counting sort by ones digit:

  digit:  0    1    2    3    4    5    6    7    8    9
        [720] [ ] [ ]  [ ]  [ ] [355] [436] [457] [ ] [329]
                                              [657]    [839]

  After collecting (stable order within each bucket):

  [720, 355, 436, 457, 657, 329, 839]

=== Pass 2: Sort by tens digit (d = 2) ===

  720 -> tens digit 2
  355 -> tens digit 5
  436 -> tens digit 3
  457 -> tens digit 5
  657 -> tens digit 5
  329 -> tens digit 2
  839 -> tens digit 3

  Counting sort by tens digit:

  digit:  0    1    2    3    4    5    6    7    8    9
        [ ]  [ ] [720] [436] [ ] [355] [ ]  [ ] [ ]  [ ]
                  [329] [839]    [457]
                                 [657]

  After collecting:

  [720, 329, 436, 839, 355, 457, 657]

=== Pass 3: Sort by hundreds digit (d = 3) ===

  720 -> hundreds digit 7
  329 -> hundreds digit 3
  436 -> hundreds digit 4
  839 -> hundreds digit 8
  355 -> hundreds digit 3
  457 -> hundreds digit 4
  657 -> hundreds digit 6

  Counting sort by hundreds digit:

  digit:  0    1    2    3    4    5    6    7    8    9
        [ ]  [ ]  [ ] [329] [436] [ ] [657] [720] [839] [ ]
                       [355] [457]

  After collecting:

  [329, 355, 436, 457, 657, 720, 839]   <-- sorted!
```

The key insight: after pass 1, the ones digits are sorted. After pass 2, numbers
are sorted by their last two digits (because sorting by tens is stable, so equal
tens digits preserve the ones-digit ordering from pass 1). After pass 3,
everything is fully sorted.

Here it is more compactly:

```
Start:     [329, 457, 657, 839, 436, 720, 355]

By ones:   [720, 355, 436, 457, 657, 329, 839]
                                ^^^  ^^^
                         (457 before 657 -- stable, and
                          329 before 839 -- stable)

By tens:   [720, 329, 436, 839, 355, 457, 657]
                                ^^^  ^^^  ^^^
                         (355, 457, 657 share tens=5x,
                          but ones order is preserved)

By hunds:  [329, 355, 436, 457, 657, 720, 839]   -- done
```

### Rust Implementation

We use counting sort as a subroutine, but now we sort by a specific digit
rather than by the full value. We will use base 256 in practice (one byte at
a time), which is more cache-friendly than base 10. But for clarity, the code
below uses a configurable radix:

```rust
/// Radix sort (LSD) for non-negative integers.
/// Uses counting sort on each digit, from least significant to most significant.
fn radix_sort(input: &mut [u64]) {
    if input.len() <= 1 {
        return;
    }

    // Find the maximum value to know how many digits to process
    let max_val = match input.iter().max() {
        Some(&m) => m,
        None => return,
    };

    let radix: u64 = 10; // base 10 for clarity; use 256 in production
    let mut exp: u64 = 1; // current digit place: 1, 10, 100, ...

    let mut output = vec![0u64; input.len()];

    // Process each digit position until we've passed the largest value
    while max_val / exp > 0 {
        counting_sort_by_digit(input, &mut output, exp, radix);

        // Copy output back into input for the next pass
        input.copy_from_slice(&output);

        exp *= radix;
    }
}

/// Stable counting sort on one digit position.
/// `exp` is the digit place (1 for ones, 10 for tens, etc.)
/// `radix` is the base (number of possible digit values).
fn counting_sort_by_digit(input: &[u64], output: &mut [u64], exp: u64, radix: u64) {
    let r = radix as usize;
    let mut count = vec![0usize; r];

    // Count occurrences of each digit
    for &val in input.iter() {
        let digit = ((val / exp) % radix) as usize;
        count[digit] += 1;
    }

    // Cumulative sum
    for i in 1..r {
        count[i] += count[i - 1];
    }

    // Build output -- right to left for stability
    for &val in input.iter().rev() {
        let digit = ((val / exp) % radix) as usize;
        count[digit] -= 1;
        output[count[digit]] = val;
    }
}

fn main() {
    let mut data = vec![329, 457, 657, 839, 436, 720, 355];
    radix_sort(&mut data);
    println!("{:?}", data); // [329, 355, 436, 457, 657, 720, 839]
}
```

### A Higher-Performance Variant: Base 256

In practice, sorting one decimal digit at a time is slow because you need many
passes (a 64-bit number has up to 20 decimal digits). A common trick: use
base 256 and treat each **byte** of the integer as a digit.

A `u32` has 4 bytes, so you need exactly 4 passes. A `u64` needs 8 passes.
The count array has 256 entries, which fits comfortably in L1 cache.

```rust
/// Radix sort by bytes (base 256) for u32 values.
fn radix_sort_u32(input: &mut [u32]) {
    if input.len() <= 1 {
        return;
    }

    let mut output = vec![0u32; input.len()];

    // 4 passes, one per byte (least significant byte first)
    for byte_index in 0..4u32 {
        let shift = byte_index * 8;
        let mut count = [0usize; 256];

        // Count
        for &val in input.iter() {
            let byte = ((val >> shift) & 0xFF) as usize;
            count[byte] += 1;
        }

        // Cumulative
        for i in 1..256 {
            count[i] += count[i - 1];
        }

        // Place (right to left, stable)
        for &val in input.iter().rev() {
            let byte = ((val >> shift) & 0xFF) as usize;
            count[byte] -= 1;
            output[count[byte]] = val;
        }

        // Swap input and output for the next pass
        input.copy_from_slice(&output);
    }
}
```

This is four passes over the data with a tiny 256-element count array. For large
arrays, this absolutely destroys comparison-based sorts because the constant
factor is small and the memory access pattern is sequential.

---

## Complexity Analysis

### Counting Sort

| | Complexity |
|---|---|
| **Time** | O(n + k) |
| **Space** | O(n + k) |

Where k is the range of values (max - min + 1).

Best, worst, and average cases are all the same. There are no comparisons, no
branches that depend on element ordering. The algorithm always does the same
work regardless of the input distribution.

### Radix Sort

| | Complexity |
|---|---|
| **Time** | O(d * (n + k)) |
| **Space** | O(n + k) |

Where:
- **d** = number of digits (passes). For base-256 on u32, d = 4.
- **n** = number of elements.
- **k** = the radix (number of possible digit values). For base-256, k = 256.

Each of the d passes runs counting sort, which is O(n + k). Multiply: O(d(n + k)).

For fixed-width integers (32-bit, 64-bit), d and k are both constants. So radix
sort on fixed-width integers is **O(n)** -- genuinely linear.

---

## When to Use These

### Counting Sort Is Great When...

- Keys are **non-negative integers** (or can be mapped to them).
- The range k is **not much larger than n**. Sorting 1 million integers in the
  range 0..1000? Perfect. Sorting 100 integers in the range 0..10^9? Terrible --
  you would allocate a billion-element count array.
- You need a **stable** sort.

### Radix Sort Is Great When...

- You are sorting **fixed-width integers or strings of bounded length**.
- The dataset is **large**. Radix sort's advantage over O(n log n) comparison
  sorts grows with n. For n = 1,000, the difference is negligible. For
  n = 10,000,000, radix sort can be 3-5x faster than an optimized quicksort.
- You can afford the **O(n) extra memory** for the output buffer.

### When NOT to Use These

- **Keys are floating-point.** You can make it work by bit-casting floats to
  integers with careful sign handling, but it is tricky.
- **Keys are complex objects** that can only be compared, not decomposed into
  digits.
- **The range k is huge relative to n.** If k >> n, counting sort wastes enormous
  memory and time initializing the count array.
- **n is small.** For small arrays, insertion sort or even the standard library's
  `sort()` (which is typically a hybrid introsort/mergesort) will be faster due
  to lower overhead.

---

## Practical Considerations

### Memory

Both counting sort and radix sort are **not in-place**. They need an auxiliary
output array of size n. Counting sort also needs the count array of size k.

For radix sort with base 256, the count array is only 256 entries (2 KB for
`usize` on 64-bit). The output buffer is the main memory cost: one extra copy of
the input. If your data is 100 million u64s (800 MB), you need another 800 MB
for the output buffer. That is the price you pay for linear time.

Comparison sorts like heapsort are in-place (O(1) extra memory). If memory is
tight, that matters.

### Signed Integers

Counting sort as shown above works on non-negative integers. To handle signed
integers, you can:

1. Find the minimum value and shift everything so the minimum becomes 0.
2. Sort the shifted values.
3. Shift back.

For radix sort on signed integers, you need to handle the sign bit specially.
The most significant byte of a two's complement integer has the sign bit flipped
relative to the unsigned ordering. One approach: flip the sign bit before sorting,
then flip it back.

```rust
/// Radix-sortable transformation for i32:
/// Flip the sign bit so that negative numbers sort before positive ones.
fn to_sortable(val: i32) -> u32 {
    (val as u32) ^ (1 << 31)
}

fn from_sortable(val: u32) -> i32 {
    (val ^ (1 << 31)) as i32
}
```

### Strings

Radix sort works on strings too -- process character by character from the last
character to the first (for fixed-length strings). For variable-length strings,
you need **MSD (Most Significant Digit) radix sort**, which is more complex and
uses recursion. We won't cover MSD radix sort here, but know it exists.

---

## Brief Aside: Bucket Sort

Bucket sort is a cousin of counting sort. Instead of one slot per possible value,
you divide the range into **buckets** (sub-ranges) and distribute elements into
them. Then you sort each bucket individually (often with insertion sort) and
concatenate.

```
  Input: [0.78, 0.17, 0.39, 0.26, 0.72, 0.94, 0.21, 0.12, 0.23, 0.68]

  10 buckets for range [0.0, 1.0):

  Bucket 0 [0.0, 0.1):  [ ]
  Bucket 1 [0.1, 0.2):  [0.17, 0.12]
  Bucket 2 [0.2, 0.3):  [0.26, 0.21, 0.23]
  Bucket 3 [0.3, 0.4):  [0.39]
  Bucket 4 [0.4, 0.5):  [ ]
  Bucket 5 [0.5, 0.6):  [ ]
  Bucket 6 [0.6, 0.7):  [0.68]
  Bucket 7 [0.7, 0.8):  [0.78, 0.72]
  Bucket 8 [0.8, 0.9):  [ ]
  Bucket 9 [0.9, 1.0):  [0.94]

  Sort each bucket, then concatenate.
```

Bucket sort is O(n) on average when the input is uniformly distributed. Worst
case (all elements in one bucket) degrades to whatever the per-bucket sort is --
typically O(n^2) if using insertion sort, or O(n log n) with mergesort.

It is useful when you know your data is roughly uniformly distributed over a
known range and you want simple, fast code.

---

## Putting It All Together

Here is a complete, runnable example that demonstrates both sorts and verifies
correctness:

```rust
fn counting_sort(input: &[usize], max_val: usize) -> Vec<usize> {
    let n = input.len();
    if n == 0 {
        return vec![];
    }

    let mut count = vec![0usize; max_val + 1];
    for &val in input {
        count[val] += 1;
    }

    for i in 1..=max_val {
        count[i] += count[i - 1];
    }

    let mut output = vec![0usize; n];
    for &val in input.iter().rev() {
        count[val] -= 1;
        output[count[val]] = val;
    }

    output
}

fn radix_sort_u32(data: &mut [u32]) {
    if data.len() <= 1 {
        return;
    }

    let mut buffer = vec![0u32; data.len()];

    for byte_idx in 0..4u32 {
        let shift = byte_idx * 8;
        let mut count = [0usize; 256];

        for &val in data.iter() {
            count[((val >> shift) & 0xFF) as usize] += 1;
        }

        for i in 1..256 {
            count[i] += count[i - 1];
        }

        for &val in data.iter().rev() {
            let byte = ((val >> shift) & 0xFF) as usize;
            count[byte] -= 1;
            buffer[count[byte]] = val;
        }

        data.copy_from_slice(&buffer);
    }
}

fn main() {
    // Counting sort demo
    let data = vec![4, 2, 2, 8, 3, 3, 1];
    let sorted = counting_sort(&data, 8);
    assert_eq!(sorted, vec![1, 2, 2, 3, 3, 4, 8]);
    println!("Counting sort: {:?}", sorted);

    // Radix sort demo
    let mut data2: Vec<u32> = vec![329, 457, 657, 839, 436, 720, 355];
    radix_sort_u32(&mut data2);
    assert_eq!(data2, vec![329, 355, 436, 457, 657, 720, 839]);
    println!("Radix sort:    {:?}", data2);

    // Radix sort with larger values
    let mut data3: Vec<u32> = vec![170, 45, 75, 90, 802, 24, 2, 66];
    radix_sort_u32(&mut data3);
    assert_eq!(data3, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    println!("Radix sort:    {:?}", data3);
}
```

---

## Cheat Sheet

```
+------------------+------------------+----------+----------+---------+
| Algorithm        | Time             | Space    | Stable?  | In-place|
+------------------+------------------+----------+----------+---------+
| Counting Sort    | O(n + k)         | O(n + k) | Yes      | No      |
+------------------+------------------+----------+----------+---------+
| Radix Sort       | O(d * (n + k))   | O(n + k) | Yes      | No      |
|  (LSD, base 256) |  = O(n) for      |          |          |         |
|                  |  fixed-width ints |          |          |         |
+------------------+------------------+----------+----------+---------+
| Bucket Sort      | O(n) avg         | O(n + b) | Depends  | No      |
|                  | O(n^2) worst     |          |          |         |
+------------------+------------------+----------+----------+---------+

Where:
  n = number of elements
  k = range of values (counting sort) or radix (radix sort)
  d = number of digit positions (passes)
  b = number of buckets
```

### Decision Guide

```
  Is the data integers (or fixed-width keys)?
    |
    +-- No  --> Use a comparison sort (merge sort, quicksort, etc.)
    |
    +-- Yes
         |
         Is the range k small relative to n?
           |
           +-- Yes, k ~ O(n) --> Counting sort
           |
           +-- No, but keys are fixed-width (u32, u64, etc.)
                 |
                 +-- Radix sort (base 256)
                 |
                 (k too large for counting sort, but radix sort
                  decomposes the key into small digits)
```

---

## Key Takeaways

1. **Comparison sorts have a provable O(n log n) lower bound.** You cannot do
   better if your only operation is comparing pairs of elements.

2. **Counting sort sidesteps comparisons entirely.** It uses keys as array
   indices, achieving O(n + k) time. The cost is O(n + k) extra memory and the
   requirement that keys are integers in a bounded range.

3. **Stability is not optional for counting sort** if you plan to use it as a
   subroutine for radix sort. The right-to-left placement loop is what makes it
   stable.

4. **Radix sort runs counting sort once per digit.** LSD (least significant digit
   first) is the standard approach. For base-256 on 32-bit integers, that is
   exactly 4 passes -- giving true O(n) performance with small constants.

5. **These sorts are not universally better than comparison sorts.** They require
   integer (or integer-like) keys, extra memory, and a key range that is
   manageable. For general-purpose sorting of arbitrary comparable objects, stick
   with your language's built-in sort.

6. **In Rust specifically**, the standard library's `slice::sort()` is a stable
   merge sort variant and `slice::sort_unstable()` is a pattern-defeating
   quicksort. Both are O(n log n). If you are sorting millions of fixed-width
   integers and need every last bit of performance, a hand-rolled radix sort can
   beat them -- but measure first.
