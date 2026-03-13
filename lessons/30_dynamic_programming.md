# Lesson 30: Dynamic Programming

## The Idea: Stop Solving the Same Problem Twice

Imagine you are planning a cross-country road trip from New York to Los Angeles. You have a
map of every city along possible routes, and you know the fuel cost between each pair of
adjacent cities. You want to find the cheapest route.

You could try every possible path, but many paths share long stretches. The cheapest way
from New York to Chicago is the same whether you are ultimately heading to LA, Denver, or
Phoenix. If you compute that cost once and write it down, you never have to recompute it.

That is dynamic programming in one sentence: **solve each subproblem once, store the result,
and reuse it whenever that subproblem comes up again.**

Now contrast this with the greedy approach from the last lesson. Greedy says "pick the
cheapest next city and commit." That might lead you through a cheap local stretch that
connects to an expensive final leg. Dynamic programming considers *all* options but is
smart about not redoing work. It gives you the globally optimal answer for problems where
greedy falls short.

### Another Analogy: Building Up Savings

You want to know the maximum savings you can accumulate over 12 months given various
investment options each month. Month 6's best choice depends on what you did in months 1
through 5. You could try every combination of choices across all 12 months -- that is brute
force. Or you could compute the best savings achievable by the end of month 1, then month 2
(using month 1's result), then month 3, and so on. Each month builds on the answer from the
previous month. You are filling in a table, one entry at a time, where each entry depends on
earlier entries.

---

## Two Key Properties

A problem is a good candidate for dynamic programming when it has both of these:

### 1. Optimal Substructure

The optimal solution to the whole problem can be constructed from optimal solutions to its
subproblems. You saw this in greedy algorithms too. The difference is that greedy commits to
one choice at each step, while DP considers all choices and picks the best.

### 2. Overlapping Subproblems

The same subproblem gets solved multiple times in the naive recursive approach. If every
subproblem were unique (no overlap), you would just have plain divide-and-conquer (like
merge sort), and there would be nothing to cache.

When both properties hold, DP lets you trade space for time: store subproblem results in
a table (or cache), and the exponential blowup of naive recursion collapses into something
polynomial.

If a problem has overlapping subproblems but *not* optimal substructure, DP will not give you
the right answer. If it has optimal substructure but *no* overlapping subproblems, DP works
but gives no speedup over plain recursion -- divide and conquer is sufficient.

---

## The Recursion Tree: Seeing the Overlap

The classic example is computing the nth Fibonacci number. The recursion tree for `fib(6)`:

```
                          fib(6)
                        /        \
                   fib(5)          fib(4)
                  /      \         /     \
             fib(4)     fib(3)  fib(3)   fib(2)
             /    \     /    \   /   \    /   \
         fib(3) fib(2) f(2) f(1) f(2) f(1) f(1) f(0)
         /   \   / \   / \
       f(2) f(1) f(1) f(0) f(1) f(0)
       / \
     f(1) f(0)
```

Count how many times each value is computed:

```
    fib(6):  1 time
    fib(5):  1 time
    fib(4):  2 times    <-- overlap!
    fib(3):  3 times    <-- overlap!
    fib(2):  5 times    <-- overlap!
    fib(1):  8 times    <-- overlap!
    fib(0):  5 times    <-- overlap!
```

Without memoization, `fib(n)` makes O(2^n) calls. With memoization, we compute each unique
subproblem exactly once: O(n) calls total. That is the power of DP.

---

## Two Approaches: Memoization vs Tabulation

There are exactly two ways to implement DP. They are duals of each other.

### Top-Down (Memoization)

Keep the recursive structure. Add a cache. Before computing a subproblem, check if the
answer is already in the cache. If yes, return it. If no, compute it, store it, then
return it.

You start at the top (the original problem) and recurse downward, filling the cache on
the way back up. Hence "top-down."

**How it works with `fib(6)`:**

```
  Start:    fib(6) -- not cached, recurse
              |
            fib(5) -- not cached, recurse
              |
            fib(4) -- not cached, recurse
              |
            fib(3) -- not cached, recurse
              |
            fib(2) -- not cached, recurse
              |
            fib(1) -- base case, return 1
            fib(0) -- base case, return 0
            fib(2) = 1, CACHE IT
            fib(1) -> base case, return 1
          fib(3) = 2, CACHE IT
          fib(2) -> CACHED! return 1 immediately
        fib(4) = 3, CACHE IT
        fib(3) -> CACHED! return 2 immediately
      fib(5) = 5, CACHE IT
      fib(4) -> CACHED! return 3 immediately
    fib(6) = 8, done.
```

**Pros:**
- Easy to write: start with the recursive solution, add a HashMap or Vec for caching.
- Only computes subproblems that are actually needed (lazy evaluation).
- Directly mirrors the mathematical recurrence.

**Cons:**
- Recursive calls consume stack space. Deep recursion can cause stack overflow.
- Function call overhead per subproblem.
- HashMap lookups are slower than array indexing.

### Bottom-Up (Tabulation)

Flip the direction. Instead of starting from `fib(n)` and recursing down, start from
`fib(0)` and iterate up, filling a table entry by entry. Each entry depends only on
previously filled entries.

**How it works:**

```
  Table:  index:  0   1   2   3   4   5   6
          value: [0] [1] [ ] [ ] [ ] [ ] [ ]    <-- base cases filled in

  Step 1: dp[2] = dp[1] + dp[0] = 1 + 0 = 1
          value: [0] [1] [1] [ ] [ ] [ ] [ ]

  Step 2: dp[3] = dp[2] + dp[1] = 1 + 1 = 2
          value: [0] [1] [1] [2] [ ] [ ] [ ]

  Step 3: dp[4] = dp[3] + dp[2] = 2 + 1 = 3
          value: [0] [1] [1] [2] [3] [ ] [ ]

  Step 4: dp[5] = dp[4] + dp[3] = 3 + 2 = 5
          value: [0] [1] [1] [2] [3] [5] [ ]

  Step 5: dp[6] = dp[5] + dp[4] = 5 + 3 = 8
          value: [0] [1] [1] [2] [3] [5] [8]

  Answer: dp[6] = 8
```

**Pros:**
- No recursion, no stack overflow risk.
- Simple loops are faster than function calls.
- Array indexing is faster than HashMap lookups.
- Easier to apply space optimization (often can reduce from O(n) to O(1)).

**Cons:**
- Must figure out the correct iteration order (which subproblems to solve first).
- Always computes *all* subproblems, even if some are not needed for the final answer.
- The recurrence relationship can be less obvious from the code.

### Side-by-Side Comparison

| Aspect               | Memoization (Top-Down)          | Tabulation (Bottom-Up)        |
|-----------------------|---------------------------------|-------------------------------|
| Implementation        | Recursive + cache               | Iterative + table             |
| Subproblems solved    | Only those actually needed      | All of them, in order         |
| Stack overflow risk   | Yes, for deep recursion         | No (iterative)                |
| Space optimization    | Harder (need full cache)        | Easier (rolling array)        |
| Thinking direction    | Natural (mirrors the recurrence)| Requires figuring out order   |
| Constant factors      | HashMap overhead, recursion     | Array access, simple loops    |

**Practical advice:** Start by writing the recurrence relation. Implement top-down with
memoization first -- it is more natural and harder to get wrong. Once it works, convert to
bottom-up if you need better constant factors or space optimization. In interviews, either
approach is usually acceptable.

---

## Problem 1: Fibonacci (The Hello World of DP)

### Naive Recursion -- O(2^n) time, O(n) stack space

```rust
fn fib_naive(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    fib_naive(n - 1) + fib_naive(n - 2)
}
```

### Top-Down (Memoization) -- O(n) time, O(n) space

```rust
use std::collections::HashMap;

fn fib_memo(n: u32, cache: &mut HashMap<u32, u64>) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    if let Some(&val) = cache.get(&n) {
        return val;
    }
    let result = fib_memo(n - 1, cache) + fib_memo(n - 2, cache);
    cache.insert(n, result);
    result
}

fn main() {
    let mut cache = HashMap::new();
    println!("{}", fib_memo(50, &mut cache)); // 12586269025, instant
}
```

The cache ensures every `fib(k)` is computed at most once. The call tree collapses from
O(2^n) nodes to O(n) unique computations.

### Bottom-Up (Tabulation) -- O(n) time, O(n) space

```rust
fn fib_table(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let n = n as usize;
    let mut dp = vec![0u64; n + 1];
    dp[0] = 0;
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}
```

### Space-Optimized -- O(n) time, O(1) space

Each cell depends on only the previous two cells. Two variables suffice:

```rust
fn fib_optimized(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut prev2 = 0u64; // fib(i-2)
    let mut prev1 = 1u64; // fib(i-1)
    for _ in 2..=n {
        let curr = prev1 + prev2;
        prev2 = prev1;
        prev1 = curr;
    }
    prev1
}
```

---

## Problem 2: Climbing Stairs

**Problem:** You are climbing a staircase with `n` steps. Each time you can climb 1 or 2
steps. How many distinct ways can you reach the top?

**State design:** `dp[i]` = number of ways to reach step `i`.

**Recurrence:** To reach step `i`, you either came from step `i-1` (one step) or step `i-2`
(two steps). So `dp[i] = dp[i-1] + dp[i-2]`.

**Base cases:** `dp[0] = 1` (one way to stand at the ground), `dp[1] = 1` (one step).

This is literally Fibonacci with different base cases.

```
    Step:      0    1    2    3    4    5
    Ways:     [1]  [1]  [2]  [3]  [5]  [8]

    dp[2] = dp[1] + dp[0] = 1 + 1 = 2   (take 1+1 or take 2)
    dp[3] = dp[2] + dp[1] = 2 + 1 = 3   (1+1+1, 1+2, 2+1)
    dp[4] = dp[3] + dp[2] = 3 + 2 = 5
    dp[5] = dp[4] + dp[3] = 5 + 3 = 8
```

### Rust Implementation -- O(n) time, O(1) space

```rust
fn climb_stairs(n: u32) -> u64 {
    if n <= 1 {
        return 1;
    }
    let mut prev2 = 1u64; // dp[i-2]
    let mut prev1 = 1u64; // dp[i-1]
    for _ in 2..=n {
        let curr = prev1 + prev2;
        prev2 = prev1;
        prev1 = curr;
    }
    prev1
}

fn main() {
    for n in 0..=10 {
        println!("climb_stairs({n}) = {}", climb_stairs(n));
    }
}
```

Time: O(n). Space: O(1).

---

## Problem 3: 0/1 Knapsack

**Problem:** You have `n` items, each with a weight and a value. You have a knapsack with
capacity `W`. Each item can be taken or left (0/1 -- no fractions). Maximize total value
without exceeding capacity.

This is the quintessential DP problem. Greedy does not work here because taking the item
with the best value-to-weight ratio might block you from a better overall combination.

**State design:** `dp[i][w]` = maximum value achievable using items `0..i` with capacity `w`.

**Recurrence:** For each item `i` with weight `wt[i]` and value `val[i]`:
- Skip item `i`: `dp[i][w] = dp[i-1][w]`
- Take item `i` (if `wt[i] <= w`): `dp[i][w] = dp[i-1][w - wt[i]] + val[i]`
- Take the max of both choices.

**Base case:** `dp[0][w] = 0` for all `w` (no items means no value).

### Visualizing the DP Table

Items: `[(wt=1, val=1), (wt=3, val=4), (wt=4, val=5), (wt=5, val=7)]`, Capacity `W = 7`

```
              Capacity w -->
              0    1    2    3    4    5    6    7
           +----+----+----+----+----+----+----+----+
  0 items  |  0 |  0 |  0 |  0 |  0 |  0 |  0 |  0 |
           +----+----+----+----+----+----+----+----+
  item 0   |  0 |  1 |  1 |  1 |  1 |  1 |  1 |  1 |  (wt=1, val=1)
  (wt=1)   +----+----+----+----+----+----+----+----+
  item 1   |  0 |  1 |  1 |  4 |  5 |  5 |  5 |  5 |  (wt=3, val=4)
  (wt=3)   +----+----+----+----+----+----+----+----+
  item 2   |  0 |  1 |  1 |  4 |  5 |  6 |  6 |  9 |  (wt=4, val=5)
  (wt=4)   +----+----+----+----+----+----+----+----+
  item 3   |  0 |  1 |  1 |  4 |  5 |  7 |  8 |  9 |  (wt=5, val=7)
  (wt=5)   +----+----+----+----+----+----+----+----+

  Answer: dp[4][7] = 9  (take items 1 and 2: wt=3+4=7, val=4+5=9)

  How dp[3][7] was computed (items 0-2, capacity 7):
    Skip item 2: dp[2][7] = 5
    Take item 2: dp[2][7-4] + 5 = dp[2][3] + 5 = 4 + 5 = 9
    dp[3][7] = max(5, 9) = 9
```

### Rust -- 2D Table -- O(n*W) time, O(n*W) space

```rust
fn knapsack(weights: &[usize], values: &[u32], capacity: usize) -> u32 {
    let n = weights.len();
    // dp[i][w]: best value using first i items with capacity w
    let mut dp = vec![vec![0u32; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 0..=capacity {
            // Option 1: skip item i-1
            dp[i][w] = dp[i - 1][w];
            // Option 2: take item i-1 (if it fits)
            if weights[i - 1] <= w {
                let take = dp[i - 1][w - weights[i - 1]] + values[i - 1];
                dp[i][w] = dp[i][w].max(take);
            }
        }
    }

    dp[n][capacity]
}

fn main() {
    let weights = [1, 3, 4, 5];
    let values = [1, 4, 5, 7];
    println!("{}", knapsack(&weights, &values, 7)); // 9
}
```

### Rust -- 1D Space Optimization -- O(n*W) time, O(W) space

Since row `i` depends only on row `i-1`, we can use a single 1D array. The trick: iterate
capacity in **reverse** so we do not overwrite values we still need:

```rust
fn knapsack_optimized(weights: &[usize], values: &[u32], capacity: usize) -> u32 {
    let mut dp = vec![0u32; capacity + 1];

    for i in 0..weights.len() {
        // Iterate capacity in REVERSE to avoid using item i twice
        for w in (weights[i]..=capacity).rev() {
            dp[w] = dp[w].max(dp[w - weights[i]] + values[i]);
        }
    }

    dp[capacity]
}
```

Why reverse? If we went left to right, `dp[w - weights[i]]` might already reflect the
"take item i" decision, meaning we could take item i multiple times. That would be the
**unbounded** knapsack, a different problem. Right to left ensures we only read "previous
row" values.

---

## Problem 4: Longest Common Subsequence (LCS)

**Problem:** Given two strings, find the length of their longest common subsequence. A
subsequence is formed by deleting zero or more characters without changing order.

Example: `"ABCBDAB"` and `"BDCAB"` have LCS `"BCAB"` with length 4.

**State design:** `dp[i][j]` = length of LCS of `s1[0..i]` and `s2[0..j]`.

**Recurrence:**
- If `s1[i-1] == s2[j-1]`: characters match, `dp[i][j] = dp[i-1][j-1] + 1`
- Else: `dp[i][j] = max(dp[i-1][j], dp[i][j-1])`

**Base case:** `dp[0][j] = 0` and `dp[i][0] = 0` (empty string has LCS 0 with anything).

### Visualizing the DP Table

`s1 = "ABCBDAB"`, `s2 = "BDCAB"`

```
            ""   B    D    C    A    B
        +----+----+----+----+----+----+
    ""  |  0 |  0 |  0 |  0 |  0 |  0 |
        +----+----+----+----+----+----+
     A  |  0 |  0 |  0 |  0 |  1 |  1 |
        +----+----+----+----+----+----+
     B  |  0 |  1 |  1 |  1 |  1 |  2 |   s1[1]='B' == s2[0]='B'
        +----+----+----+----+----+----+
     C  |  0 |  1 |  1 |  2 |  2 |  2 |   s1[2]='C' == s2[2]='C'
        +----+----+----+----+----+----+
     B  |  0 |  1 |  1 |  2 |  2 |  3 |   s1[3]='B' == s2[4]='B'
        +----+----+----+----+----+----+
     D  |  0 |  1 |  2 |  2 |  2 |  3 |   s1[4]='D' == s2[1]='D'
        +----+----+----+----+----+----+
     A  |  0 |  1 |  2 |  2 |  3 |  3 |   s1[5]='A' == s2[3]='A'
        +----+----+----+----+----+----+
     B  |  0 |  1 |  2 |  2 |  3 |  4 |   s1[6]='B' == s2[4]='B'
        +----+----+----+----+----+----+

    Answer: dp[7][5] = 4    LCS = "BCAB"
```

### Rust Implementation -- O(m*n) time, O(m*n) space

```rust
fn lcs(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let (m, n) = (a.len(), b.len());

    // dp[i][j] = LCS length of a[0..i] and b[0..j]
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}

fn main() {
    println!("{}", lcs("ABCBDAB", "BDCAB")); // 4
    println!("{}", lcs("abcde", "ace"));      // 3
}
```

### Space Optimization -- O(m*n) time, O(min(m,n)) space

Each row only depends on the previous row:

```rust
fn lcs_optimized(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    // Make b the shorter string for less memory
    let (a, b) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let (m, n) = (a.len(), b.len());

    let mut prev = vec![0usize; n + 1];
    let mut curr = vec![0usize; n + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                curr[j] = prev[j - 1] + 1;
            } else {
                curr[j] = prev[j].max(curr[j - 1]);
            }
        }
        std::mem::swap(&mut prev, &mut curr);
        curr.fill(0);
    }
    prev[n]
}
```

---

## Problem 5: Coin Change (Minimum Coins)

**Problem:** Given coin denominations and a target amount, find the minimum number of coins
needed to make that amount. Each denomination can be used unlimited times. Return -1 if
impossible.

This is the problem where greedy *fails*. Denominations `[1, 3, 4]`, target `6`:
- Greedy: pick 4, then 1, then 1 = 3 coins
- Optimal: pick 3, then 3 = 2 coins

**State design:** `dp[a]` = minimum coins to make amount `a`.

**Recurrence:** For each coin `c`, if `c <= a`: `dp[a] = min(dp[a], dp[a - c] + 1)`.

**Base case:** `dp[0] = 0` (zero coins for zero amount).

### Visualizing the Table

Coins: `[1, 3, 4]`, Amount: `6`

```
    amount:    0    1    2    3    4    5    6
    dp:       [0]  [1]  [2]  [1]  [1]  [2]  [2]

    dp[0] = 0                          (base case)
    dp[1] = dp[1-1] + 1 = 1            (use coin 1)
    dp[2] = dp[2-1] + 1 = 2            (use coin 1)
    dp[3] = min(dp[3-1]+1, dp[3-3]+1)
          = min(3, 1) = 1              (use coin 3)
    dp[4] = min(dp[4-1]+1, dp[4-3]+1, dp[4-4]+1)
          = min(2, 2, 1) = 1           (use coin 4)
    dp[5] = min(dp[5-1]+1, dp[5-3]+1, dp[5-4]+1)
          = min(2, 3, 2) = 2           (use coins 1+4)
    dp[6] = min(dp[6-1]+1, dp[6-3]+1, dp[6-4]+1)
          = min(3, 2, 3) = 2           (use coins 3+3)
```

### DP as a DAG: Another Way to See It

Think of each amount as a node. Each coin creates an edge. Finding minimum coins is finding
the shortest path from node 0 to node `amount`:

```
    (0) --1--> (1) --1--> (2) --1--> (3) --1--> (4) --1--> (5) --1--> (6)
     |                      ^          |                      ^          ^
     |          +3          |          |          +3          |          |
     +----------+-----------+          +----------+-----------+          |
     |                                 |                                |
     |                  +4             |                  +4            |
     +------------------+--------------+------------------+-------------+

    Shortest path 0 -> 6 = path through 0 -> 3 -> 6 = 2 edges = 2 coins
```

### Rust Implementation -- O(amount * coins.len()) time, O(amount) space

```rust
fn coin_change(coins: &[usize], amount: usize) -> i32 {
    // Initialize with a value larger than any possible answer.
    // amount + 1 works because you can never need more than `amount` coins.
    let mut dp = vec![amount + 1; amount + 1];
    dp[0] = 0;

    for a in 1..=amount {
        for &coin in coins {
            if coin <= a && dp[a - coin] + 1 < dp[a] {
                dp[a] = dp[a - coin] + 1;
            }
        }
    }

    if dp[amount] > amount {
        -1 // impossible
    } else {
        dp[amount] as i32
    }
}

fn main() {
    println!("{}", coin_change(&[1, 3, 4], 6));     // 2 (coins: 3+3)
    println!("{}", coin_change(&[1, 5, 10, 25], 87)); // 6
    println!("{}", coin_change(&[2], 3));             // -1 (impossible)
}
```

---

## Problem 6: Edit Distance (Levenshtein Distance)

**Problem:** Given two strings `word1` and `word2`, find the minimum number of operations to
convert `word1` into `word2`. Allowed operations: insert, delete, or replace a character.

This is used everywhere: spell checkers, DNA sequence alignment, diff tools.

**State design:** `dp[i][j]` = edit distance between `word1[0..i]` and `word2[0..j]`.

**Recurrence:**
- If `word1[i-1] == word2[j-1]`: no edit needed, `dp[i][j] = dp[i-1][j-1]`
- Else: `dp[i][j] = 1 + min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1])`
  - `dp[i-1][j] + 1` = delete from word1
  - `dp[i][j-1] + 1` = insert into word1
  - `dp[i-1][j-1] + 1` = replace in word1

**Base cases:** `dp[i][0] = i` (delete all of word1), `dp[0][j] = j` (insert all of word2).

### Visualizing the DP Table

`word1 = "horse"`, `word2 = "ros"`

```
            ""   r    o    s
        +----+----+----+----+
    ""  |  0 |  1 |  2 |  3 |
        +----+----+----+----+
     h  |  1 |  1 |  2 |  3 |   h!=r, 1+min(0,1,1)=1
        +----+----+----+----+
     o  |  2 |  2 |  1 |  2 |   o==o, dp[1][1]=1
        +----+----+----+----+
     r  |  3 |  2 |  2 |  2 |   r!=s, 1+min(1,2,1)=2
        +----+----+----+----+
     s  |  4 |  3 |  3 |  2 |   s==s, dp[3][2]=2
        +----+----+----+----+
     e  |  5 |  4 |  4 |  3 |   e!=s, 1+min(2,3,2)=3
        +----+----+----+----+

    Answer: dp[5][3] = 3
    Operations: horse -> rorse (replace h with r)
                rorse -> rose  (delete second r)
                rose  -> ros   (delete e)
```

Each cell reads from at most three neighbors:

```
    dp[i-1][j-1]   dp[i-1][j]
         \            |
          \           |  (delete)
   (replace)\         v
              +---> dp[i][j]
              ^
              |
    dp[i][j-1]
      (insert)
```

### Rust Implementation -- O(m*n) time, O(m*n) space

```rust
fn min_distance(word1: &str, word2: &str) -> usize {
    let a: Vec<char> = word1.chars().collect();
    let b: Vec<char> = word2.chars().collect();
    let (m, n) = (a.len(), b.len());

    // dp[i][j] = edit distance between a[0..i] and b[0..j]
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    // Base cases
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]; // no edit needed
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1]  // replace
                              .min(dp[i - 1][j])   // delete
                              .min(dp[i][j - 1]);  // insert
            }
        }
    }

    dp[m][n]
}

fn main() {
    println!("{}", min_distance("horse", "ros"));         // 3
    println!("{}", min_distance("intention", "execution")); // 5
}
```

### Space-Optimized -- O(m*n) time, O(min(m,n)) space

```rust
fn min_distance_optimized(word1: &str, word2: &str) -> usize {
    let a: Vec<char> = word1.chars().collect();
    let b: Vec<char> = word2.chars().collect();
    let (m, n) = (a.len(), b.len());

    let mut prev: Vec<usize> = (0..=n).collect(); // dp[i-1][..]
    let mut curr = vec![0usize; n + 1];

    for i in 1..=m {
        curr[0] = i; // base case: dp[i][0] = i
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                curr[j] = prev[j - 1];
            } else {
                curr[j] = 1 + prev[j - 1].min(prev[j]).min(curr[j - 1]);
            }
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}
```

---

## State Design: The Art of DP

The hardest part of DP is not writing the code -- it is designing the state. Here is a
systematic approach.

### Step 1: Identify What Changes Between Subproblems

Ask yourself: "What variables determine the answer to a subproblem?" Those variables become
the dimensions of your DP table.

```
    Problem                 State Variables                 DP Table Shape
    -------                 ---------------                 --------------
    Fibonacci               index i                         1D: dp[i]
    Climbing Stairs         step i                          1D: dp[i]
    Coin Change             amount a                        1D: dp[a]
    House Robber            house index i                   1D: dp[i]
    0/1 Knapsack            item index i, capacity w        2D: dp[i][w]
    LCS                     index i in s1, index j in s2    2D: dp[i][j]
    Edit Distance           index i in w1, index j in w2    2D: dp[i][j]
    Matrix Chain Mult.      start i, end j of chain         2D: dp[i][j]
```

### Step 2: Define What the State Represents

Write in plain English: "dp[i][j] represents ____." If you cannot finish that sentence
clearly, your state design is wrong. Examples:

- "dp[i] represents the maximum money I can rob from houses 0 through i."
- "dp[i][j] represents the edit distance between word1[0..i] and word2[0..j]."
- "dp[a] represents the minimum number of coins to make amount a."

### Step 3: Write the Recurrence

Express `dp[current]` in terms of smaller subproblems. This is where you think about what
*choices* you make at each step:
- Take or leave an item? (Knapsack)
- Match characters or pay a cost? (Edit Distance)
- Which previous step did you come from? (Climbing Stairs)
- Use which coin as the last coin? (Coin Change)

### Step 4: Identify Base Cases

What are the smallest subproblems you can answer directly? Usually when an index is 0 or
when the input is empty.

### Step 5: Determine Iteration Order

For bottom-up, you must fill the table so that every value you read has already been
computed. Look at which cells the recurrence reads from:
- If `dp[i]` reads `dp[i-1]` and `dp[i-2]`: iterate `i` from small to large.
- If `dp[i]` reads `dp[i+1]` and `dp[i+2]`: iterate `i` from large to small.
- If `dp[i][j]` reads `dp[i-1][j]`, `dp[i][j-1]`, `dp[i-1][j-1]`: iterate `i` then `j`
  from small to large.

### Step 6: Extract the Answer

It is usually `dp[n]`, `dp[n][m]`, or `dp[n][W]` -- the entry corresponding to the full
problem. Sometimes you need to scan the entire table (e.g., longest increasing subsequence
where the answer is `max(dp[0..n])`).

---

## Space Optimization Techniques

### Technique 1: Rolling Array (Two Rows)

When row `i` only depends on row `i-1`, keep only two rows and alternate:

```rust
// Instead of:  let mut dp = vec![vec![0; cols]; rows];
// Use:
let mut prev = vec![0; cols];
let mut curr = vec![0; cols];

for i in 0..rows {
    // Fill curr using prev
    for j in 0..cols {
        curr[j] = /* some function of prev[j], prev[j-1], curr[j-1], etc. */;
    }
    std::mem::swap(&mut prev, &mut curr);
    curr.fill(0); // reset for next iteration
}
// Answer is in prev (because of the final swap)
```

Space drops from O(rows * cols) to O(cols). This is the most common DP space optimization.

### Technique 2: Single Row with Reverse Iteration

For 0/1 Knapsack, we reduced 2D to 1D by iterating capacity in reverse. This works when
each cell in the current row depends on cells to its *left* in the previous row. By going
right-to-left, we read old values before overwriting them.

```
    Forward iteration (WRONG for 0/1 knapsack):
    dp: [0] [1] [1] [4] [5] [5] ...
                      ^   ^
                      |   Uses updated dp[1], which already includes item i
                      |   -> could take item i twice!
                      Already updated for item i

    Reverse iteration (CORRECT for 0/1 knapsack):
    dp: [0] [1] [1] [4] [5] [5] ...
         ^                      ^
         |                      Process this first
         Process this last      (reads dp[3] which is still from previous row)
```

### Technique 3: Two Variables

When you only look back 1 or 2 positions in a 1D DP (Fibonacci, Climbing Stairs, House
Robber), replace the entire array with two variables. Space drops from O(n) to O(1).

### When You Cannot Optimize

If the recurrence needs values from arbitrary positions in previous rows (not just the
immediately previous row), you cannot easily reduce space. Matrix chain multiplication
needs `dp[i][k]` and `dp[k+1][j]` for arbitrary `k`, so you need the full table.

---

## 1D DP Patterns

1D DP problems have a single state variable. The table is a flat array.

**Common shape:**

```
    dp[i] depends on dp[i-1], dp[i-2], ..., or dp[i-k]
```

**Examples:**
- Fibonacci: `dp[i] = dp[i-1] + dp[i-2]`
- Climbing Stairs: same recurrence, different base cases
- House Robber: `dp[i] = max(dp[i-1], dp[i-2] + nums[i])`
- Coin Change: `dp[a] = min over coins c of (dp[a-c] + 1)`
- Word Break: `dp[i] = OR over valid words ending at position i`

**Template:**

```rust
fn solve_1d(input: &[i32]) -> i32 {
    let n = input.len();
    let mut dp = vec![0; n + 1]; // or some sentinel value

    // Base case
    dp[0] = /* ... */;

    for i in 1..=n {
        // Transition: dp[i] depends on dp[i-1], dp[i-2], etc.
        dp[i] = /* combine dp[i-1], dp[i-2], etc. with input[i-1] */;
    }

    dp[n] // or max of dp, etc.
}
```

### Worked 1D Example: House Robber

To tie the 1D pattern together, let's walk through a problem the way you would in an
interview.

**Problem:** You are a robber planning to rob houses along a street. Each house has a certain
amount of money. You cannot rob two adjacent houses (alarm system). Maximize total money.

**Step 1: Recognize DP.** At each house, you choose to rob or skip. The optimal answer
depends on past choices. There are overlapping subproblems (deciding about house `i` depends
on decisions about houses `i-1` and `i-2`).

**Step 2: Define state.** `dp[i]` = maximum money from houses `0..=i`.

**Step 3: Recurrence.**
- Rob house `i`: `dp[i-2] + nums[i]` (skip adjacent, add current)
- Skip house `i`: `dp[i-1]` (best without this house)
- `dp[i] = max(dp[i-1], dp[i-2] + nums[i])`

**Step 4: Base cases.** `dp[0] = nums[0]`, `dp[1] = max(nums[0], nums[1])`.

**Step 5: Code it.**

```rust
fn rob(houses: &[i32]) -> i32 {
    match houses.len() {
        0 => return 0,
        1 => return houses[0],
        _ => {}
    }
    // Space-optimized: only need previous two values
    let mut prev2 = houses[0];               // dp[i-2]
    let mut prev1 = houses[0].max(houses[1]); // dp[i-1]

    for i in 2..houses.len() {
        let current = prev1.max(prev2 + houses[i]);
        prev2 = prev1;
        prev1 = current;
    }
    prev1
}

fn main() {
    println!("{}", rob(&[1, 2, 3, 1]));    // 4  (rob houses 0 and 2)
    println!("{}", rob(&[2, 7, 9, 3, 1])); // 12 (rob houses 0, 2, 4)
}
```

**Step 6: Complexity.** O(n) time, O(1) space.

---

## 2D DP Patterns

2D DP problems have two state variables. The table is a matrix.

**Common shapes:**

```
    Type 1: Two sequences    dp[i][j] for indices into s1 and s2
            Examples: LCS, Edit Distance, Interleaving Strings

    Type 2: Interval          dp[i][j] for start and end of a range
            Examples: Matrix Chain Multiply, Burst Balloons, Palindrome Partitioning

    Type 3: Index + resource  dp[i][w] for item index and remaining resource
            Examples: 0/1 Knapsack, Partition Equal Subset Sum
```

**Two-sequence template:**

```rust
fn solve_two_seq(s1: &[char], s2: &[char]) -> usize {
    let (m, n) = (s1.len(), s2.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    // Base cases: dp[0][j] and dp[i][0] already 0 (or set them)

    for i in 1..=m {
        for j in 1..=n {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1; // characters match
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]); // take the better
            }
        }
    }
    dp[m][n]
}
```

**Interval DP template:** (solving subproblems by increasing interval length)

```rust
fn solve_interval(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut dp = vec![vec![0i32; n]; n];

    // Base case: intervals of length 1
    for i in 0..n {
        dp[i][i] = /* base value for single element */;
    }

    // Fill by increasing interval length
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = i32::MAX; // or i32::MIN depending on min/max
            for k in i..j {
                // Try splitting the interval at every point k
                let cost = dp[i][k] + dp[k + 1][j] + /* merge cost */;
                dp[i][j] = dp[i][j].min(cost);
            }
        }
    }
    dp[0][n - 1]
}
```

### Grid DP: Unique Paths

A common 2D pattern is DP on a grid, where you can only move right or down.

**Problem:** Given an `m x n` grid, count unique paths from top-left to bottom-right.

```
    3x4 grid:

    1  1  1  1
    1  2  3  4
    1  3  6  10  <-- answer: 10

    Each cell = cell above + cell to the left.
```

```rust
fn unique_paths(m: usize, n: usize) -> u64 {
    // Space-optimized: single row updated in-place left to right
    let mut row = vec![1u64; n];
    for _ in 1..m {
        for c in 1..n {
            row[c] += row[c - 1];
        }
    }
    row[n - 1]
}

fn main() {
    println!("{}", unique_paths(3, 4)); // 10
    println!("{}", unique_paths(3, 7)); // 28
}
```

Time: O(m*n). Space: O(n).

---

## Complexity Summary

```
    Problem                 Time            Space           Optimized Space
    -------                 ----            -----           ---------------
    Fibonacci               O(n)            O(n)            O(1)
    Climbing Stairs         O(n)            O(n)            O(1)
    House Robber            O(n)            O(n)            O(1)
    Coin Change             O(a * k)        O(a)            O(a)   [k=#coins, a=amount]
    0/1 Knapsack            O(n * W)        O(n * W)        O(W)
    LCS                     O(m * n)        O(m * n)        O(min(m,n))
    Edit Distance           O(m * n)        O(m * n)        O(min(m,n))
    Unique Paths            O(m * n)        O(m * n)        O(n)
    LIS (basic DP)          O(n^2)          O(n)            O(n)
    Interval DP (generic)   O(n^3)          O(n^2)          O(n^2)
```

---

## Recognizing DP Problems in Interviews

DP problems do not announce themselves. Here are the signals to watch for.

### Signal 1: "Count the number of ways..."

Ways to climb stairs, ways to decode a string, ways to partition a set. Counting problems
with overlapping choices almost always use DP.

### Signal 2: "Find the minimum/maximum..."

Minimum cost path, maximum profit, minimum edits. Optimization over a space of choices where
greedy does not obviously work.

### Signal 3: "Can you achieve X?" (Yes/No feasibility)

Can you make this amount with these coins? Can you partition into equal subsets? These are
often knapsack variants.

### Signal 4: Choices at Each Step

If at each step you face a choice (take/skip, go left/right, use this character or not), and
the optimal answer depends on trying all choices, think DP.

### Signal 5: The Problem Has Small Constraints

If the input size is suspiciously small (n <= 1000, or two dimensions each <= 500), the
expected solution is often O(n^2) or O(n*m) DP. Brute force would be exponential and too
slow, but DP with polynomial complexity fits.

### Signal 6: Recursive Solution Has Repeated Work

If you write a recursive solution and notice the same arguments appearing multiple times,
slap a cache on it -- you have DP.

### The Interview Approach

1. **Identify** that it is a DP problem (signals above).
2. **Define the state** in plain English: "dp[i] represents ____."
3. **Write the recurrence** relating dp[i] to smaller subproblems.
4. **Identify base cases.**
5. **Code it** top-down first if unsure, then convert to bottom-up.
6. **Optimize space** if asked.
7. **Analyze complexity.**

Communicate each step to the interviewer. They want to see your thought process, not just
the code.

---

## Defining States and Recurrences: A Full Worked Example

Let's walk through the thought process for a new problem from scratch.

**Problem: Decode Ways (LeetCode 91).** A message of letters A-Z is encoded as numbers:
A=1, B=2, ..., Z=26. Given a digit string, count the number of ways to decode it.

Example: "226" can be decoded as "BZ" (2,26), "VF" (22,6), or "BBF" (2,2,6). Answer: 3.

**Step 1 -- Can I use DP?** Yes. "Count the number of ways" is a strong signal. Decoding
the first character leaves a smaller string that is the same kind of problem. Different
choices for the first character lead to the same remaining suffix (overlapping subproblems).

**Step 2 -- Define state.** `dp[i]` = number of ways to decode `s[i..]` (suffix from `i`).

**Step 3 -- Recurrence.** At position `i`:
- If `s[i]` is '1'-'9': decode as single letter, contributes `dp[i+1]` ways.
- If `s[i..i+2]` is 10-26: decode as two-digit letter, contributes `dp[i+2]` ways.
- If `s[i]` is '0': cannot start a decoding here, `dp[i] = 0`.

**Step 4 -- Base cases.** `dp[n] = 1` (empty suffix: one way to decode nothing).

**Step 5 -- Iteration order.** Right to left (dp[i] depends on dp[i+1] and dp[i+2]).

**Step 6 -- Space optimize.** Two variables suffice (only look ahead 1 and 2 positions).

```rust
fn num_decodings(s: &str) -> u64 {
    let s = s.as_bytes();
    let n = s.len();
    if n == 0 || s[0] == b'0' {
        return 0;
    }

    let mut next1 = 1u64; // dp[i+1]
    let mut next2 = 1u64; // dp[i+2]

    for i in (0..n).rev() {
        let mut curr = 0u64;
        if s[i] != b'0' {
            curr += next1; // single digit decode

            // Two digit decode
            if i + 1 < n {
                let two_digit = (s[i] - b'0') as u64 * 10 + (s[i + 1] - b'0') as u64;
                if (10..=26).contains(&two_digit) {
                    curr += next2;
                }
            }
        }
        next2 = next1;
        next1 = curr;
    }

    next1
}

fn main() {
    println!("{}", num_decodings("226"));   // 3
    println!("{}", num_decodings("06"));    // 0 (leading zero)
    println!("{}", num_decodings("11106")); // 2
}
```

Time: O(n). Space: O(1).

---

## Common Mistakes

**1. Off-by-one errors in table dimensions.** If your state is `dp[i]` for items `0..n`,
your table needs `n+1` entries. Two strings of length `m` and `n` need a `(m+1) x (n+1)`
table. This trips people up constantly.

**2. Wrong iteration order.** If `dp[i]` depends on `dp[i+1]`, you must iterate from high
to low. For space-optimized 0/1 knapsack, iterate capacity in *reverse*. Draw the dependency
arrows before writing the loop.

**3. Forgetting the base case.** An uninitialized DP table (all zeros) might accidentally
work for some inputs but fail on edge cases.

**4. State definition too narrow.** If your recurrence does not fully determine the answer
from subproblems, you are missing a dimension. For example, trying to solve LIS with
`dp[i] = LIS length in nums[0..i]` (without "ending at i") gives no clean recurrence.

**5. Confusing DP with greedy.** If you find yourself making an irrevocable choice without
considering all options, you are doing greedy, not DP. DP explicitly considers all choices.

**6. Integer overflow.** DP tables often accumulate large sums. In Rust, use `u64` or `i64`
generously. Debug builds panic on overflow; release builds silently wrap. Neither is what
you want if your numbers are large.

**7. Using DP when greedy suffices.** Not every optimization problem needs DP. If the greedy
choice property holds, greedy is simpler and faster. DP is the heavier tool.

---

## Practice Problems

### Easy (Build Foundations)

| # | Problem | Key Concept |
|---|---------|-------------|
| 1 | [Fibonacci Number](https://leetcode.com/problems/fibonacci-number/) (LC 509) | Basic recurrence, memoization intro |
| 2 | [Climbing Stairs](https://leetcode.com/problems/climbing-stairs/) (LC 70) | Fibonacci variant |
| 3 | [Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs/) (LC 746) | 1D DP with costs |
| 4 | [House Robber](https://leetcode.com/problems/house-robber/) (LC 198) | Take/skip decision pattern |
| 5 | [Maximum Subarray](https://leetcode.com/problems/maximum-subarray/) (LC 53) | Kadane's algorithm as DP |

### Medium (Interview Core)

| # | Problem | Key Concept |
|---|---------|-------------|
| 1 | [Coin Change](https://leetcode.com/problems/coin-change/) (LC 322) | Unbounded knapsack variant |
| 2 | [Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence/) (LC 1143) | Two-sequence 2D DP |
| 3 | [Unique Paths](https://leetcode.com/problems/unique-paths/) (LC 62) | Grid DP |
| 4 | [Word Break](https://leetcode.com/problems/word-break/) (LC 139) | String DP with dictionary |
| 5 | [Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/) (LC 416) | 0/1 Knapsack boolean variant |

### Hard (Stretch Goals)

| # | Problem | Key Concept |
|---|---------|-------------|
| 1 | [Edit Distance](https://leetcode.com/problems/edit-distance/) (LC 72) | Two-sequence with 3 operations |
| 2 | [Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/) (LC 300) | DP + binary search for O(n log n) |
| 3 | [Burst Balloons](https://leetcode.com/problems/burst-balloons/) (LC 312) | Interval DP |
| 4 | [Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/) (LC 10) | Complex 2D DP with wildcards |
| 5 | [Distinct Subsequences](https://leetcode.com/problems/distinct-subsequences/) (LC 115) | Counting 2D DP |

---

## Key Takeaways

1. **DP = recursion + caching.** If you can solve it recursively and there is overlapping
   work, you can make it DP.

2. **State design is everything.** Spend most of your thinking time here. The rest follows
   mechanically from a good state definition.

3. **Top-down first, optimize later.** In an interview, get a correct memoized solution
   first, then convert to bottom-up if time allows.

4. **Space optimization is the cherry on top.** Interviewers love seeing you reduce O(n*m)
   to O(min(n,m)). Rolling arrays and reverse iteration are the two main techniques.

5. **Practice the patterns, not the problems.** Once you recognize "this is a two-sequence
   DP" or "this is a knapsack variant," the solution structure writes itself.

6. **DP and greedy are cousins.** DP explores all choices and picks the best. Greedy commits
   to one choice. If greedy works (provably), use it. If not, use DP.

7. **Draw the table.** On a whiteboard or paper, filling in a small DP table by hand is the
   fastest way to verify your recurrence is correct before coding.
