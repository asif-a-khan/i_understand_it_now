# Lesson 07: Recursion

## What Is Recursion?

A recursive function is a function that calls itself. That is the entire definition. But
that sentence alone is about as useful as defining "swimming" as "moving through water" --
technically correct, practically insufficient. Let's build real understanding.

Recursion is a problem-solving strategy: you solve a problem by reducing it to a smaller
instance of *the same problem*, and you keep reducing until you hit a case so simple you
can answer it directly. Then the answers bubble back up, combining to solve the original
problem.

### Real-World Analogies

**Russian nesting dolls (matryoshka).** You open a doll and find a smaller doll inside.
You open that one and find an even smaller doll. You keep going until you reach the
smallest solid doll that does not open -- that is your base case. To find it, you had
to open every doll along the way. To reassemble, you close them back up in reverse order.

**Two mirrors facing each other.** You see a reflection of a reflection of a reflection,
each one slightly smaller, stretching toward infinity. Recursion without a base case is
like this -- it goes on forever (or until you run out of stack space).

**Looking up a word in a dictionary.** You look up "ephemeral" and the definition says
"see: transient." You look up "transient" and it says "see: fleeting." You look up
"fleeting" and finally get a definition you understand. Each lookup is the same action
(look up a word), and you stop when you reach a definition that stands on its own.

---

## Base Case and Recursive Case

Every recursive function has exactly two kinds of logic:

1. **Base case**: The condition where the function *stops* calling itself and returns a
   direct answer. Without this, you get infinite recursion (and a stack overflow).

2. **Recursive case**: The part where the function calls itself with a *smaller* or
   *simpler* input, moving toward the base case.

The simplest possible example:

```rust
fn countdown(n: i32) {
    if n <= 0 {
        // Base case: stop here
        println!("Done!");
        return;
    }
    // Recursive case: print, then count down from n-1
    println!("{n}");
    countdown(n - 1);
}

fn main() {
    countdown(5);
    // Prints: 5, 4, 3, 2, 1, Done!
}
```

The input `n` gets smaller by 1 each call. Eventually `n <= 0` and we stop. If you
removed the base case, `countdown` would call itself forever -- or rather, until the
stack overflows.

---

## The Call Stack: What Actually Happens

When a function calls another function (including itself), the CPU needs to remember
where to come back to and what the local variables were. It stores this in a **stack
frame** on the **call stack**. Each recursive call pushes a new frame. When the call
returns, its frame is popped.

Let's trace `factorial(4)`:

```rust
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;  // base case
    }
    n * factorial(n - 1)  // recursive case
}
```

Here is what the call stack looks like as the function recurses deeper:

```
  CALL PHASE (pushing frames):

  factorial(4)                          Stack (grows downward):
    -> 4 * factorial(3)                 +------------------+
                                        | factorial(4)     |
      -> 3 * factorial(2)              | n=4, waiting...  |
                                        +------------------+
                                        | factorial(3)     |
        -> 2 * factorial(1)            | n=3, waiting...  |
                                        +------------------+
                                        | factorial(2)     |
          -> returns 1                 | n=2, waiting...  |
                                        +------------------+
                                        | factorial(1)     |
                                        | n=1, returns 1   |
                                        +------------------+

  RETURN PHASE (popping frames):

  factorial(1) returns 1
  factorial(2) returns 2 * 1 = 2
  factorial(3) returns 3 * 2 = 6
  factorial(4) returns 4 * 6 = 24
```

Each frame sits in memory, consuming stack space, until the base case is reached. Then
the frames unwind in reverse order. This is why recursion has an inherent space cost:
O(n) stack frames for a function that recurses n levels deep.

### The Stack Has a Size Limit

On most Linux systems, the default stack size is 8 MB. Each stack frame is relatively
small (maybe a few dozen bytes for local variables plus return address), but if you
recurse deeply enough, you run out of space. That is a **stack overflow**.

```rust
// This will crash with a stack overflow for large n:
fn sum_to(n: u64) -> u64 {
    if n == 0 { 0 } else { n + sum_to(n - 1) }
}

fn main() {
    // sum_to(10) is fine. sum_to(1_000_000) will overflow the stack.
    println!("{}", sum_to(1_000_000));
}
```

This is not a theoretical concern. In Rust, it is one of the main reasons you should
think twice before reaching for recursion when a simple loop will do.

---

## Factorial: The Hello World of Recursion

You have already seen it above. Let's look at both the recursive and iterative versions
side by side.

```rust
// Recursive
fn factorial_rec(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial_rec(n - 1) }
}

// Iterative
fn factorial_iter(n: u64) -> u64 {
    let mut result = 1u64;
    for i in 2..=n {
        result *= i;
    }
    result
}
```

Both compute the same answer. The iterative version uses O(1) space (just a counter and
an accumulator). The recursive version uses O(n) space on the call stack. For this
problem, the iterative version is strictly better in every practical way.

So why show the recursive version at all? Because it maps directly to the mathematical
definition: `n! = n * (n-1)!` with `0! = 1`. When the problem *is* recursive in nature,
expressing it recursively can make the logic clearer. Factorial is too simple to justify
recursion, but it teaches the mechanics.

---

## Fibonacci: The Cautionary Tale

The naive recursive Fibonacci is the most important "bad example" in all of CS:

```rust
fn fib(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    fib(n - 1) + fib(n - 2)
}
```

This is elegant and mirrors the mathematical definition: `F(n) = F(n-1) + F(n-2)`.
It is also catastrophically slow. Here is the recursion tree for `fib(5)`:

```
                            fib(5)
                          /        \
                     fib(4)          fib(3)
                    /      \        /      \
               fib(3)    fib(2)  fib(2)  fib(1)
              /    \     /    \   /    \     |
          fib(2) fib(1) f(1) f(0) f(1) f(0) 1
          /   \    |     |    |    |    |
       f(1) f(0)   1     1    0    1    0
         |    |
         1    0
```

Notice all the repeated work. `fib(3)` is computed twice. `fib(2)` is computed three
times. As `n` grows, the tree explodes exponentially. The time complexity is O(2^n).
Calling `fib(50)` on a modern machine takes *minutes*. `fib(100)` would not finish
in your lifetime.

The fix is to not recompute the same subproblems. You can memoize (cache results) or
just iterate:

```rust
// Iterative Fibonacci: O(n) time, O(1) space
fn fib_iter(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}
```

The lesson: just because you *can* write something recursively does not mean you
*should*. Naive recursive Fibonacci is a trap that teaches you to think about the cost
of recursion.

---

## Thinking Recursively

The real skill is not writing `factorial`. It is learning to decompose a problem into
smaller identical subproblems. Here is the mental framework:

1. **What is the smallest (trivial) version of this problem?** That is your base case.
2. **If I had the answer for a slightly smaller input, how would I combine it with the
   current element to get the answer for the full input?** That is your recursive case.
3. **Does each recursive call make the problem strictly smaller?** If not, you have
   infinite recursion.

### Example: Sum of an Array

"Sum all elements in a slice."

- **Base case**: Empty slice has sum 0.
- **Recursive case**: Sum of `[first, rest...]` is `first + sum(rest)`.

```rust
fn sum(slice: &[i32]) -> i64 {
    match slice {
        [] => 0,
        [first, rest @ ..] => *first as i64 + sum(rest),
    }
}

fn main() {
    assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
}
```

Rust's pattern matching makes recursive decomposition of slices very clean. But again,
for a simple sum you would just use `iter().sum()` in practice.

### Example: Reverse a String

"Reverse a string."

- **Base case**: Empty string or single character is already reversed.
- **Recursive case**: Move the first character to the end of the reversed rest.

```rust
fn reverse(s: &str) -> String {
    if s.len() <= 1 {
        return s.to_string();
    }
    let first = &s[..1];
    let rest = &s[1..];
    let mut reversed = reverse(rest);
    reversed.push_str(first);
    reversed
}

fn main() {
    assert_eq!(reverse("hello"), "olleh");
    assert_eq!(reverse("a"), "a");
    assert_eq!(reverse(""), "");
}
```

This is O(n^2) in time because of all the string allocations and copies. The iterative
version (`s.chars().rev().collect()`) is O(n). But the recursive version demonstrates
the *thinking pattern*: peel off one piece, recurse on the rest, combine.

---

## Common Recursive Patterns

### Pattern 1: Compute and Return

The function computes something from the recursive result. Factorial and sum are examples.

```
fn solve(input) -> Answer {
    if base_case(input) { return base_answer; }
    let sub_answer = solve(smaller_input);
    combine(current_element, sub_answer)
}
```

### Pattern 2: Accumulator

Instead of building up the answer on the way *back* from recursion, you carry an
accumulator forward and return it at the base case:

```rust
fn factorial_acc(n: u64, acc: u64) -> u64 {
    if n <= 1 {
        acc             // base case: return the accumulated result
    } else {
        factorial_acc(n - 1, acc * n)  // pass updated accumulator forward
    }
}

fn factorial(n: u64) -> u64 {
    factorial_acc(n, 1)
}
```

The accumulator pattern is the recursive equivalent of a loop with a running total. It
often results in **tail recursive** code (more on that shortly).

### Pattern 3: Divide and Conquer (Preview)

Split the input in half, recurse on both halves, combine the results. Merge sort is
the canonical example. You have already seen it in the Big-O lesson:

```
fn solve(input) -> Answer {
    if small_enough(input) { return base_answer; }
    let (left, right) = split(input);
    let left_answer = solve(left);
    let right_answer = solve(right);
    merge(left_answer, right_answer)
}
```

This pattern produces O(n log n) algorithms when the split and merge are each O(n).
We will dig deeper into this with sorting algorithms later.

---

## Tail Recursion (and Why Rust Does Not Optimize It)

A recursive call is in **tail position** if it is the very last thing the function does --
no computation happens after the call returns. The accumulator factorial above is tail
recursive: the recursive call `factorial_acc(n - 1, acc * n)` is the last operation.

The original factorial is *not* tail recursive:

```rust
// NOT tail recursive: must multiply by n AFTER the recursive call returns
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
```

There is still work to do (`n * ...`) after `factorial(n - 1)` returns. The stack frame
cannot be discarded early.

Why does tail position matter? In theory, a compiler can optimize a tail call by *reusing*
the current stack frame instead of creating a new one. This turns recursion into a loop
behind the scenes -- O(1) stack space instead of O(n). Languages like Scheme guarantee
this optimization (it is required by the language spec). Haskell and some functional
languages do it routinely.

**Rust does not guarantee tail call optimization (TCO).** The LLVM backend *sometimes*
performs it as an optimization, but you cannot rely on it. The Rust team has discussed
adding guaranteed TCO (there is even an experimental `become` keyword proposal), but as
of 2025 it is not stabilized.

What this means in practice: **do not rely on tail recursion in Rust to save you from
stack overflow.** If you need deep recursion without stack overflow, convert to iteration
explicitly.

---

## Converting Between Recursion and Iteration

Any recursive function can be rewritten iteratively, and vice versa. Here are the
mechanical techniques.

### Recursion to Iteration: Simple Cases (Use a Loop)

When the recursion is linear (one recursive call per invocation), the conversion is
straightforward -- use a loop and a mutable accumulator:

```rust
// Recursive
fn sum_rec(slice: &[i32]) -> i64 {
    match slice {
        [] => 0,
        [first, rest @ ..] => *first as i64 + sum_rec(rest),
    }
}

// Iterative equivalent
fn sum_iter(slice: &[i32]) -> i64 {
    let mut total: i64 = 0;
    for &val in slice {
        total += val as i64;
    }
    total
}
```

The pattern: replace the base case with the loop's initial state, replace the recursive
case with the loop body, and let the accumulator carry the state.

### Recursion to Iteration: Complex Cases (Use an Explicit Stack)

When the recursion involves multiple recursive calls (like tree traversal) or complex
state, you simulate the call stack with a `Vec`:

```rust
// Recursive tree traversal (preview -- we will cover trees in a later lesson)
// fn traverse(node) {
//     if node is null { return; }
//     process(node);
//     traverse(node.left);
//     traverse(node.right);
// }

// Iterative equivalent using an explicit stack:
// fn traverse(root) {
//     let mut stack = vec![root];
//     while let Some(node) = stack.pop() {
//         process(node);
//         if let Some(right) = node.right { stack.push(right); }
//         if let Some(left) = node.left { stack.push(left); }
//     }
// }
```

This is exactly the DFS pattern from Lesson 05. You are just making the implicit call
stack explicit.

### Iteration to Recursion

Going the other way is less common in practice but useful to understand. Any loop can
be expressed as recursion by turning the loop variable into a function parameter:

```rust
// Iterative
fn count_evens_iter(data: &[i32]) -> usize {
    let mut count = 0;
    for &val in data {
        if val % 2 == 0 {
            count += 1;
        }
    }
    count
}

// Recursive equivalent
fn count_evens_rec(data: &[i32]) -> usize {
    match data {
        [] => 0,
        [first, rest @ ..] => {
            let is_even = if first % 2 == 0 { 1 } else { 0 };
            is_even + count_evens_rec(rest)
        }
    }
}
```

---

## Recursion vs Iteration: When to Use Which

Here is an honest assessment.

### Prefer Iteration When:

- The problem is naturally linear (summing, searching, counting, filtering).
- Performance matters and the recursive version adds overhead (function call cost,
  stack frames).
- The recursion depth could be large and you risk stack overflow.
- The recursive solution is not meaningfully clearer than the iterative one.

### Prefer Recursion When:

- The data structure is recursive (trees, graphs, nested structures). Traversing a
  binary tree iteratively requires an explicit stack and is harder to read. The
  recursive version often maps directly to the structure.
- The problem has a natural recursive decomposition (divide and conquer, backtracking).
- The recursion depth is bounded and small (e.g., log(n) for balanced trees).
- Clarity matters more than squeezing out the last bit of performance.

### The Honest Truth

Many problems that *can* be solved recursively *should* be solved iteratively in Rust.
Recursion has real costs:

1. **Stack space**: Each call uses a frame. Rust's stack is limited and not growable
   at runtime (unlike, say, Go's goroutines which have growable stacks).
2. **Function call overhead**: Pushing and popping frames, saving and restoring
   registers. It is not free. For tight inner loops, it can matter.
3. **No guaranteed TCO**: You cannot count on the compiler to turn tail recursion
   into a loop.

But there are domains where recursion is practically required:

- **Tree operations**: Inorder, preorder, postorder traversal. Recursive definitions
  are dramatically simpler.
- **Graph algorithms**: DFS is naturally recursive. Converting to iteration means
  managing your own stack anyway.
- **Parsing**: Recursive descent parsers, handling nested structures.
- **Divide and conquer**: Merge sort, quicksort, binary search (though binary search
  is typically iterative for performance).
- **Backtracking**: Generating permutations, solving sudoku, N-queens.

The key insight: recursion shines when the *problem structure* is recursive. A flat list
is not recursive in structure. A tree is. When the data looks like a tree, your code
probably should too.

---

## Recursive Data Structures (Preview)

Recursion is not just a control flow technique. Some data structures are *defined*
recursively. You have already encountered one in Lesson 04.

### Linked List

A linked list is either empty, or it is a node followed by... another linked list.

```rust
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
```

The definition refers to itself. `Cons` contains a value and a pointer to another
`List`, which could be `Nil` (base case) or another `Cons` (recursive case). This is
why linked list operations are so natural to write recursively.

```rust
use List::{Cons, Nil};

fn length<T>(list: &List<T>) -> usize {
    match list {
        Nil => 0,
        Cons(_, tail) => 1 + length(tail),
    }
}
```

### Binary Tree

A binary tree is either empty, or it is a node with a value and two children -- each of
which is... a binary tree.

```rust
enum Tree<T> {
    Empty,
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}
```

Again, the structure is self-referential. Operations on trees map beautifully to
recursion:

```rust
fn tree_sum(tree: &Tree<i32>) -> i32 {
    match tree {
        Tree::Empty => 0,
        Tree::Node { value, left, right } => {
            value + tree_sum(left) + tree_sum(right)
        }
    }
}

fn tree_height<T>(tree: &Tree<T>) -> usize {
    match tree {
        Tree::Empty => 0,
        Tree::Node { left, right, .. } => {
            1 + tree_height(left).max(tree_height(right))
        }
    }
}
```

Notice how each recursive function mirrors the structure of the data type: one arm for
`Empty` (base case), one arm for `Node` (recursive case). The code almost writes itself.

We will explore trees in depth in a later lesson. For now, just notice how recursive
data structures and recursive functions are two sides of the same coin.

---

## Worked Example: Power Function

Compute `base^exp` using recursion. This illustrates divide and conquer.

**Naive approach**: multiply `base` by itself `exp` times. O(n) where n = exp.

**Better approach**: notice that `x^8 = (x^4)^2 = ((x^2)^2)^2`. We can halve the
exponent at each step.

```rust
fn power(base: i64, exp: u32) -> i64 {
    match exp {
        0 => 1,                                      // base case
        e if e % 2 == 0 => {
            let half = power(base, exp / 2);          // recurse on half
            half * half                                // combine
        }
        _ => {
            base * power(base, exp - 1)               // reduce to even case
        }
    }
}

fn main() {
    assert_eq!(power(2, 10), 1024);
    assert_eq!(power(3, 0), 1);
    assert_eq!(power(5, 3), 125);
}
```

The recursion depth is O(log n) because we halve the exponent each step (at worst,
two steps per halving: one to make it even, one to halve). This is *fast exponentiation*
and it matters for cryptography and large-number arithmetic.

Call stack for `power(2, 10)`:

```
  power(2, 10)
    -> exp is even, half = power(2, 5)
       power(2, 5)
         -> exp is odd, base * power(2, 4)
            power(2, 4)
              -> exp is even, half = power(2, 2)
                 power(2, 2)
                   -> exp is even, half = power(2, 1)
                      power(2, 1)
                        -> exp is odd, base * power(2, 0)
                           power(2, 0) -> returns 1
                        -> returns 2 * 1 = 2
                   -> returns 2 * 2 = 4
              -> returns 4 * 4 = 16
         -> returns 2 * 16 = 32
    -> returns 32 * 32 = 1024
```

Seven function calls instead of ten multiplications. For `power(2, 1000)`, this is
about 15 calls instead of 1000 iterations. That is the power of divide and conquer.

---

## Worked Example: Binary Search (Recursive vs Iterative)

You saw iterative binary search in Lesson 01. Here is the recursive version:

```rust
fn binary_search_rec(sorted: &[i32], target: i32) -> Option<usize> {
    fn helper(sorted: &[i32], target: i32, lo: usize, hi: usize) -> Option<usize> {
        if lo >= hi {
            return None;  // base case: empty range
        }
        let mid = lo + (hi - lo) / 2;
        match sorted[mid].cmp(&target) {
            std::cmp::Ordering::Equal => Some(mid),
            std::cmp::Ordering::Less => helper(sorted, target, mid + 1, hi),
            std::cmp::Ordering::Greater => helper(sorted, target, lo, mid),
        }
    }
    helper(sorted, target, 0, sorted.len())
}
```

The recursion depth is O(log n) -- we halve the search space each time. This is shallow
enough that stack overflow is not a concern (log2 of a billion is about 30). But the
iterative version is still preferred in Rust because it avoids function call overhead
and is no harder to read:

```rust
fn binary_search_iter(sorted: &[i32], target: i32) -> Option<usize> {
    let (mut lo, mut hi) = (0, sorted.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match sorted[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}
```

This is a case where the recursive version is not clearer, not shorter, and not faster.
Use the loop.

---

## Debugging Recursive Functions

When a recursive function produces wrong results, here is a systematic approach:

1. **Check your base case.** Is it correct? Does it handle all edge inputs (empty
   collection, n=0, n=1)?

2. **Check your recursive case.** Does the input *actually get smaller* with each call?
   If the recursive call does not move toward the base case, you have infinite recursion.

3. **Add print tracing.** Print the input and output at each call:

```rust
fn factorial_debug(n: u64, depth: usize) -> u64 {
    let indent = "  ".repeat(depth);
    println!("{indent}factorial({n})");
    let result = if n <= 1 {
        1
    } else {
        n * factorial_debug(n - 1, depth + 1)
    };
    println!("{indent}-> returns {result}");
    result
}
```

Running `factorial_debug(4, 0)` prints:

```
factorial(4)
  factorial(3)
    factorial(2)
      factorial(1)
      -> returns 1
    -> returns 2
  -> returns 6
-> returns 24
```

4. **Think about the "leap of faith."** When reasoning about recursion, assume the
   recursive call works correctly for the smaller input. Then verify that *given a
   correct answer for the smaller input*, the current step produces the correct answer.
   You do not need to mentally trace every level of recursion -- that way lies madness.

---

## Summary: The Cost-Benefit Analysis

```
  RECURSION                              ITERATION
  ---------                              ---------
  Elegant for recursive structures       Efficient for flat structures
  Maps to mathematical definitions       Maps to step-by-step processes
  Uses O(depth) stack space              Uses O(1) extra space (usually)
  Function call overhead per level       Loop overhead is negligible
  Risk of stack overflow                 No stack overflow risk
  Natural for trees, graphs, parsing     Natural for arrays, lists, streams
  Rust does NOT guarantee TCO            Loops are always "optimized"
```

### Key Takeaways

1. **Recursion = solving a problem by solving smaller instances of itself.** Every
   recursive function needs a base case (when to stop) and a recursive case (how to
   reduce).

2. **The call stack is real memory.** Each recursive call consumes stack space. Deep
   recursion can and will overflow the stack in Rust. The default stack is 8 MB.

3. **Rust does not guarantee tail call optimization.** Do not rely on tail recursion
   to save you. If you need deep recursion without stack growth, convert to iteration.

4. **Recursion shines on recursive data structures.** Trees, graphs, and nested
   structures have shapes that mirror recursive function calls. Fighting this with
   iteration means managing your own stack, which is often harder to read.

5. **For flat, linear problems, prefer iteration.** Summing an array, searching a list,
   computing factorial -- these are all better as loops in Rust.

6. **Naive recursive Fibonacci is O(2^n).** It is the canonical example of recursion
   done wrong. Always be aware of overlapping subproblems -- they are the gateway to
   dynamic programming (a future lesson).

7. **The three recursive patterns** -- compute-and-return, accumulator, divide and
   conquer -- cover most recursive algorithms you will encounter.

---

## Exercises

1. **Recursive Sum of Digits**: Write a function that takes a positive integer and
   returns the sum of its digits. For example, `sum_digits(1234)` returns 10.
   Base case: single digit number. Recursive case: `n % 10 + sum_digits(n / 10)`.

2. **Recursive Palindrome Check**: Write a function that checks if a string slice is
   a palindrome. Base case: length 0 or 1. Recursive case: first char equals last
   char, and the substring between them is also a palindrome.

3. **Flatten a Nested Vec**: Given `Vec<Vec<i32>>`, write a recursive function to
   flatten it into `Vec<i32>`. Then write the iterative version and compare.

4. **Recursive Power (with modular arithmetic)**: Extend the power function to compute
   `base^exp mod m` without overflow. This is a real technique used in cryptography.

5. **Towers of Hanoi**: The classic recursion puzzle. Move n disks from peg A to peg C
   using peg B as auxiliary. Rules: move one disk at a time, never place a larger disk
   on a smaller one. The recursive solution is elegant; the iterative one is painful.
   This is a genuine case where recursion wins.

6. **Iterative Fibonacci vs Recursive with Memoization**: Implement both. Use a
   `HashMap<u32, u64>` for the memoized version. Compare their performance on `fib(40)`.

---

Next lesson: [08 - Trees](./08_trees.md)
