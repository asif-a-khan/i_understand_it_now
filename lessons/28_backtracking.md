# Lesson 28: Backtracking

## What Is Backtracking?

In Lesson 07 you learned recursion: solve a problem by reducing it to smaller instances
of itself. Backtracking is recursion with a twist -- *you make a choice, explore the
consequences, and undo the choice if it does not lead to a valid solution.* It is
systematic trial and error with pruning.

Think about solving a maze. You stand at a fork and pick the left corridor. You walk
forward, take another left, hit a dead end. Now you back up to the last fork and try the
right corridor instead. You keep doing this -- choosing, exploring, backing up -- until you
find the exit or exhaust every path. You never wander randomly. You methodically try every
option, abandoning a direction the moment you know it cannot work.

That is backtracking in one paragraph: **make a choice, recurse, undo the choice.**

### Another Analogy: Sudoku by Hand

When you solve a Sudoku puzzle, you sometimes reach a cell where multiple numbers could
fit. You pick one, write it in pencil, and continue. If three moves later you hit a
contradiction (two 7s in the same row), you erase back to where you guessed and try a
different number. You are not trying all 9^81 possible grids -- you are pruning entire
branches of possibilities as soon as a contradiction appears.

This "pencil and eraser" strategy is backtracking. The pencil is the *choose* step, the
continued solving is the *explore* step, and the eraser is the *unchoose* step. The early
detection of contradictions is *pruning*, and it is what makes backtracking practical
despite exploring exponential search spaces.

---

## Backtracking vs Brute Force

Brute force generates **all** possible candidates and checks each one after construction
is complete. If you have n binary decisions, brute force evaluates 2^n candidates no
matter what.

Backtracking also explores a decision tree, but it **prunes** branches early. The moment
a partial candidate violates a constraint, you chop off that entire subtree. In the best
case, pruning reduces an exponential search space dramatically. In the worst case (no
pruning possible), backtracking degenerates to brute force.

```
  BRUTE FORCE                     BACKTRACKING
  -----------                     ------------
  Generate ALL candidates         Build candidates incrementally
  Check each at the end           Check constraints at each step
  No early termination            Prune invalid branches immediately
  Always exponential              Often much better (depends on pruning)
  Simple but wasteful             Systematic and efficient
```

The formal structure: backtracking performs a depth-first search on the **state space
tree** (also called the decision tree). Each node represents a partial solution. Each
edge represents a choice. Leaf nodes are either complete solutions or dead ends. When
you reach a dead end, you climb back up to the most recent decision point and try the
next option -- that climb is the "backtrack."

---

## The State Space Tree

Consider generating all subsets of {1, 2, 3}. At each step, you decide: include this
element or exclude it.

```
                          {}
                       /      \
                 {1}              {}
               /     \          /    \
          {1,2}      {1}     {2}      {}
          /   \     /   \   /   \   /   \
     {1,2,3} {1,2} {1,3} {1} {2,3} {2} {3} {}

     Leaves are all 2^3 = 8 subsets.
```

Every root-to-leaf path represents one complete decision sequence. Without pruning,
backtracking visits all leaves -- same as brute force. But add a constraint (say,
"subset sum must not exceed 4") and entire branches get chopped:

```
                          {}   (sum=0)
                       /                  \
                   {1} (sum=1)             {} (sum=0)
                 /        \              /        \
            {1,2} (s=3)   {1} (s=1)   {2} (s=2)   {} (s=0)
            /     \       /    \      /    \       /    \
      {1,2,3}  {1,2}  {1,3} {1}  {2,3} {2}    {3}    {}
      (s=6)X  (s=3)   (s=4) (s=1)(s=5)X(s=2) (s=3)  (s=0)

      X = pruned (sum exceeds 4)
```

Nodes marked X are never expanded further. Their children are never generated. Every
branch we prune saves us from exploring its entire subtree.

---

## The Backtracking Template

Almost every backtracking problem fits the same skeleton. Once you internalize this
template, the individual problems become exercises in figuring out what "choice" means for
each one.

```
fn backtrack(state, choices, results):
    if state is a complete solution:
        record the solution in results
        return

    for each candidate in choices:
        if candidate is valid (pruning check):
            choose: modify state to include candidate
            backtrack(updated state, remaining choices, results)
            unchoose: undo the modification to state
```

In Rust, this looks like:

```rust
fn backtrack(
    current: &mut Vec<i32>,    // the partial solution being built
    choices: &[i32],           // what we can still pick from
    results: &mut Vec<Vec<i32>>, // all valid complete solutions
) {
    if is_complete(current) {
        results.push(current.clone());
        return;
    }

    for &candidate in choices {
        if is_valid(current, candidate) {
            current.push(candidate);       // choose
            backtrack(current, choices, results);  // explore
            current.pop();                 // unchoose
        }
    }
}
```

The three steps -- **choose, explore, unchoose** -- form the heartbeat of every
backtracking algorithm. The `push` adds a candidate to the partial solution, the recursive
call explores everything reachable from that state, and the `pop` restores the state so
the next candidate starts from a clean slate.

### Decision Tree Visualization

Backtracking explores a decision tree. Each node represents a state (a partial solution),
and each edge represents a choice. Leaves are either complete solutions or dead ends. When
you hit a dead end, you climb back up the tree to try the next branch.

Here is the decision tree for generating all subsets of `{1, 2, 3}`, where each level
decides "include this element or not":

```
                          {}
                       /      \
                 {1}              {}
               /     \          /    \
          {1,2}      {1}     {2}      {}
          /   \     /   \   /   \   /   \
     {1,2,3} {1,2} {1,3} {1} {2,3} {2} {3} {}

     Leaves are all 2^3 = 8 subsets.
```

Pruning cuts branches early. If a constraint says "no subset should sum above 4," we can
prune the `{1,2,3}` branch as soon as `{1,2}` is formed (sum=3, adding 3 would make 6):

```
                          {}
                       /      \
                 {1}              {}
               /     \          /    \
          {1,2}      {1}     {2}      {}
            |       /   \   /   \   /   \
          {1,2}  {1,3} {1} {2,3} {2} {3} {}
          (stop:     ^
         sum=3,   also pruned
         3 would   if needed
         exceed 4)
```

---

## Pattern 1: Permutations

**Problem:** Given a list of distinct integers, return all possible orderings.

For `[1, 2, 3]`, the answer is all 3! = 6 permutations:
`[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]`

### How It Fits the Template

- **State:** The partial permutation built so far.
- **Choices:** All elements not yet used.
- **Complete:** The partial permutation has the same length as the input.
- **Choose:** Add an unused element.
- **Unchoose:** Remove it.

### Decision Tree

```
                             []
                     /        |        \
                  [1]        [2]       [3]
                /    \      /    \    /    \
            [1,2]  [1,3] [2,1] [2,3] [3,1] [3,2]
              |      |     |     |     |      |
          [1,2,3] [1,3,2] [2,1,3] [2,3,1] [3,1,2] [3,2,1]
```

Every path from root to leaf is one permutation.

### Rust Implementation

```rust
fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];
    permute_helper(nums, &mut used, &mut current, &mut results);
    results
}

fn permute_helper(
    nums: &[i32],
    used: &mut Vec<bool>,
    current: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    if current.len() == nums.len() {
        results.push(current.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue; // skip already-used elements
        }

        used[i] = true;            // choose
        current.push(nums[i]);

        permute_helper(nums, used, current, results);  // explore

        current.pop();              // unchoose
        used[i] = false;
    }
}

fn main() {
    let perms = permutations(&[1, 2, 3]);
    for p in &perms {
        println!("{:?}", p);
    }
    assert_eq!(perms.len(), 6);
}
```

The `used` array tracks which elements are in the current partial permutation. This is
cleaner than removing elements from the input slice and reinserting them. The choose/unchoose
symmetry is clear: `push` and `pop`, `true` and `false`.

**Complexity:** There are n! permutations, each of length n, so the output itself is
O(n * n!). The algorithm does O(n) work per node in the decision tree, and the tree has
roughly n! leaves, giving O(n * n!) total. You cannot do better -- you have to produce
all n! results.

---

## Pattern 2: Combinations

**Problem:** Given integers `n` and `k`, return all combinations of `k` numbers chosen
from `1..=n`.

For `n=4, k=2`: `[1,2], [1,3], [1,4], [2,3], [2,4], [3,4]`

### How It Fits the Template

- **State:** The partial combination built so far.
- **Choices:** Numbers from some starting point up to n (to avoid duplicates, we only
  pick numbers in increasing order).
- **Complete:** The combination has `k` elements.
- **Pruning:** If there are not enough numbers left to fill the remaining slots, stop
  early.

### Decision Tree (n=4, k=2)

```
                              []
                    /       |       \        \
                 [1]       [2]      [3]     [4]
               / | \      / \       |      (need 1 more
           [1,2][1,3][1,4] [2,3][2,4]  [3,4]  but none left
             *    *    *     *    *      *      -- pruned)
```

Nodes marked `*` are complete solutions. Notice `[4]` is pruned because after choosing 4,
there are no numbers > 4 to complete the pair.

### Rust Implementation

```rust
fn combinations(n: i32, k: usize) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    combine_helper(n, k, 1, &mut current, &mut results);
    results
}

fn combine_helper(
    n: i32,
    k: usize,
    start: i32,         // only consider numbers >= start (avoids duplicates)
    current: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    if current.len() == k {
        results.push(current.clone());
        return;
    }

    // Pruning: if there aren't enough numbers left, stop.
    // We need (k - current.len()) more elements, and we have (n - start + 1) available.
    let remaining_needed = k - current.len();
    let remaining_available = (n - start + 1) as usize;
    if remaining_available < remaining_needed {
        return;
    }

    for num in start..=n {
        current.push(num);                          // choose
        combine_helper(n, k, num + 1, current, results);  // explore
        current.pop();                              // unchoose
    }
}

fn main() {
    let combs = combinations(4, 2);
    for c in &combs {
        println!("{:?}", c);
    }
    assert_eq!(combs.len(), 6); // C(4,2) = 6
}
```

The `start` parameter is the key insight. By only considering numbers >= `start`, we
guarantee combinations are generated in sorted order, so `[1,3]` appears but `[3,1]`
does not.

The pruning check (`remaining_available < remaining_needed`) looks minor but can
eliminate large portions of the tree. For `combinations(20, 18)`, without pruning you
would explore many branches that can never produce 18-element combinations. With it,
you cut them off immediately.

**Complexity:** There are C(n, k) combinations, each of length k. The algorithm produces
exactly C(n, k) results, so the output is O(k * C(n, k)). The pruning keeps exploration
close to the output size.

---

## Pattern 3: Subsets (Power Set)

**Problem:** Given a set of distinct integers, return all possible subsets.

For `[1, 2, 3]`, the answer is: `[], [1], [2], [3], [1,2], [1,3], [2,3], [1,2,3]`

### How It Fits the Template

Subsets are just combinations of *all* sizes. At each element, you make a binary choice:
include it or skip it.

- **State:** The current subset being built.
- **Choices:** For each element (in order), include or exclude.
- **Complete:** Every state is a valid subset -- you record it at every node, not just at
  leaves.

### Rust Implementation

```rust
fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    subsets_helper(nums, 0, &mut current, &mut results);
    results
}

fn subsets_helper(
    nums: &[i32],
    index: usize,         // which element we're deciding about
    current: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    // Every partial state is a valid subset.
    results.push(current.clone());

    for i in index..nums.len() {
        current.push(nums[i]);                  // choose: include nums[i]
        subsets_helper(nums, i + 1, current, results);  // explore remaining
        current.pop();                          // unchoose
    }
}

fn main() {
    let subs = subsets(&[1, 2, 3]);
    for s in &subs {
        println!("{:?}", s);
    }
    assert_eq!(subs.len(), 8); // 2^3 = 8
}
```

Notice: `results.push(current.clone())` happens *before* the loop, not inside an
`if current.len() == k` check. Every node in the decision tree produces a valid subset.

**Complexity:** There are 2^n subsets. Each subset is at most length n. Total output is
O(n * 2^n). The algorithm visits exactly 2^n nodes.

---

## Pattern 4: N-Queens

**Problem:** Place n queens on an n x n chessboard such that no two queens threaten each
other. Queens can attack along rows, columns, and diagonals.

This is the classic backtracking problem. It shows constraint satisfaction: you build a
solution piece by piece, and prune immediately when a constraint is violated.

### Strategy

Place queens one row at a time (this handles the "one queen per row" constraint
automatically). For each row, try every column. Before placing a queen, check if it
conflicts with any queen already on the board.

### Board State Visualization (4-Queens)

Here is how the algorithm explores placements for a 4x4 board, showing how it backtracks
when it hits conflicts:

```
  Try row 0, col 0:          Try row 1, col 2:
  +---+---+---+---+          +---+---+---+---+
  | Q |   |   |   |          | Q |   |   |   |
  +---+---+---+---+          +---+---+---+---+
  |   |   |   |   |          |   |   | Q |   |
  +---+---+---+---+          +---+---+---+---+
  |   |   |   |   |          |   |   |   |   |
  +---+---+---+---+          +---+---+---+---+
  |   |   |   |   |          |   |   |   |   |
  +---+---+---+---+          +---+---+---+---+

  Try row 2: col 0 attacked   Row 2, col 1 attacked by
  by Q at (0,0). Col 1        diagonal from (1,2).
  attacked by diagonal from   Try col 3... attacked by
  (1,2). Try col 3... but     diagonal from (0,0)? No,
  wait, let's see:            but (1,2) attacks col 2
                               and diag. Dead end!
                               Backtrack to row 1.

  After more backtracking, a solution is found:

  +---+---+---+---+
  |   | Q |   |   |     Q at (0,1)
  +---+---+---+---+
  |   |   |   | Q |     Q at (1,3)
  +---+---+---+---+
  | Q |   |   |   |     Q at (2,0)
  +---+---+---+---+
  |   |   | Q |   |     Q at (3,2)
  +---+---+---+---+

  Verify: no two queens share a row, column, or diagonal.
```

### Conflict Detection

Two queens at positions `(r1, c1)` and `(r2, c2)` conflict if:
- Same column: `c1 == c2`
- Same diagonal: `|r1 - r2| == |c1 - c2|`

(Same row is impossible by construction since we place one queen per row.)

For efficient checking, we track three sets:
- `cols`: columns that have a queen.
- `diag1`: the "row - col" diagonals (all cells on the same `/` diagonal share the same
  `row - col` value).
- `diag2`: the "row + col" diagonals (all cells on the same `\` diagonal share the same
  `row + col` value).

### Rust Implementation

```rust
use std::collections::HashSet;

fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
    let mut results = Vec::new();
    let mut queens: Vec<usize> = Vec::new(); // queens[row] = col
    let mut cols = HashSet::new();
    let mut diag1 = HashSet::new(); // row - col (use i32 because it can be negative)
    let mut diag2 = HashSet::new(); // row + col

    place_queen(n, 0, &mut queens, &mut cols, &mut diag1, &mut diag2, &mut results);
    results
}

fn place_queen(
    n: usize,
    row: usize,
    queens: &mut Vec<usize>,
    cols: &mut HashSet<usize>,
    diag1: &mut HashSet<i32>,
    diag2: &mut HashSet<usize>,
    results: &mut Vec<Vec<String>>,
) {
    if row == n {
        // All queens placed. Record the board.
        let board = queens.iter().map(|&col| {
            let mut row_str = ".".repeat(n);
            // Safety: col is always < n, and each char is one byte ('.')
            unsafe { row_str.as_bytes_mut()[col] = b'Q'; }
            row_str
        }).collect();
        results.push(board);
        return;
    }

    for col in 0..n {
        let d1 = row as i32 - col as i32;
        let d2 = row + col;

        // Pruning: skip if this column or diagonal is already occupied.
        if cols.contains(&col) || diag1.contains(&d1) || diag2.contains(&d2) {
            continue;
        }

        // Choose
        queens.push(col);
        cols.insert(col);
        diag1.insert(d1);
        diag2.insert(d2);

        // Explore
        place_queen(n, row + 1, queens, cols, diag1, diag2, results);

        // Unchoose
        queens.pop();
        cols.remove(&col);
        diag1.remove(&d1);
        diag2.remove(&d2);
    }
}

fn main() {
    let solutions = solve_n_queens(4);
    println!("4-Queens has {} solutions:", solutions.len());
    for (i, board) in solutions.iter().enumerate() {
        println!("\nSolution {}:", i + 1);
        for row in board {
            println!("  {}", row);
        }
    }
    // 4-Queens has 2 solutions.
    // 8-Queens has 92 solutions.
}
```

The choose/unchoose pattern is more involved here -- you add to three tracking structures
and undo all three. But the structure is identical to permutations: try a candidate, recurse,
undo.

**Complexity:** The worst case is O(n!) since at each row, there are at most n columns to
try, then n-1 for the next row (roughly, due to column conflicts), and so on. In practice,
diagonal pruning cuts the search much further. For 8-Queens, the algorithm explores
thousands of nodes, not 8^8 = 16 million.

---

## Pattern 5: Sudoku Solver

**Problem:** Fill a 9x9 grid so that each row, column, and 3x3 box contains the digits
1 through 9 exactly once.

Sudoku is a pure constraint satisfaction problem. Backtracking is the standard approach.

### Strategy

1. Find the next empty cell.
2. Try digits 1 through 9.
3. For each digit, check if placing it violates any constraint (row, column, or box).
4. If valid, place it and recurse to the next empty cell.
5. If no digit works, undo and backtrack.

### Rust Implementation

```rust
fn solve_sudoku(board: &mut [[u8; 9]; 9]) -> bool {
    // Find the next empty cell (marked with 0).
    let empty = find_empty(board);
    let (row, col) = match empty {
        Some(pos) => pos,
        None => return true, // No empty cells -- puzzle solved!
    };

    for digit in 1..=9u8 {
        if is_valid_placement(board, row, col, digit) {
            board[row][col] = digit;         // choose

            if solve_sudoku(board) {         // explore
                return true;                 // solution found, propagate up
            }

            board[row][col] = 0;             // unchoose (backtrack)
        }
    }

    false // no digit works here -- trigger backtracking
}

fn find_empty(board: &[[u8; 9]; 9]) -> Option<(usize, usize)> {
    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] == 0 {
                return Some((r, c));
            }
        }
    }
    None
}

fn is_valid_placement(board: &[[u8; 9]; 9], row: usize, col: usize, digit: u8) -> bool {
    // Check row
    for c in 0..9 {
        if board[row][c] == digit {
            return false;
        }
    }

    // Check column
    for r in 0..9 {
        if board[r][col] == digit {
            return false;
        }
    }

    // Check 3x3 box
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for r in box_row..box_row + 3 {
        for c in box_col..box_col + 3 {
            if board[r][c] == digit {
                return false;
            }
        }
    }

    true
}

fn main() {
    // A sample puzzle (0 = empty)
    let mut board = [
        [5,3,0, 0,7,0, 0,0,0],
        [6,0,0, 1,9,5, 0,0,0],
        [0,9,8, 0,0,0, 0,6,0],

        [8,0,0, 0,6,0, 0,0,3],
        [4,0,0, 8,0,3, 0,0,1],
        [7,0,0, 0,2,0, 0,0,6],

        [0,6,0, 0,0,0, 2,8,0],
        [0,0,0, 4,1,9, 0,0,5],
        [0,0,0, 0,8,0, 0,7,9],
    ];

    if solve_sudoku(&mut board) {
        println!("Solved:");
        for row in &board {
            let line: Vec<String> = row.iter().map(|d| d.to_string()).collect();
            println!("  {}", line.join(" "));
        }
    } else {
        println!("No solution exists.");
    }
}
```

Notice how this differs slightly from the earlier problems. Instead of collecting *all*
solutions, we return `true` as soon as we find *one*. The `if solve_sudoku(board)` check
propagates success back up the call chain, and we only undo (`board[row][col] = 0`) if
the recursive call failed.

**Complexity:** The theoretical worst case is O(9^m) where m is the number of empty cells,
but pruning is aggressive. Most valid Sudoku puzzles are solved in milliseconds because
the constraints eliminate most candidates at each cell. A cell in a row with five filled
digits has at most four candidates, not nine.

---

## Pruning Strategies

Pruning is what separates backtracking from brute force. Without it, you visit every
node in the decision tree. With it, you skip entire subtrees. Here are the five main
strategies, roughly ordered from most common to most specialized.

### 1. Constraint Checking (Feasibility Pruning)

The most basic form: before making a choice, verify it does not violate any constraint.
Every example so far uses this. In N-Queens, `is_safe` checks column and diagonal
conflicts. In Sudoku, `is_valid_placement` checks row, column, and box uniqueness.

The rule: **check constraints as early as possible.** The earlier you detect a violation,
the larger the subtree you skip.

### 2. Ordering Heuristics

**Choose the most constrained variable first.** In Sudoku, instead of always picking the
first empty cell (top-left to bottom-right), pick the cell with the *fewest* valid
candidates. This is the "Minimum Remaining Values" (MRV) heuristic. A cell with only
one valid digit forces a placement with no branching. A cell with nine valid digits
creates nine branches. Choosing the most constrained cell first triggers failures (and
thus backtracking) sooner, pruning more of the tree.

For value ordering: **try the most likely value first.** If you have domain knowledge
about which choices are more promising, try those before less promising ones. This does
not reduce the worst case, but it finds solutions faster on average.

### 3. Duplicate Skipping

When the input contains duplicate values, sort it first and skip consecutive equal
elements at the same decision level:

```rust
// Inside the loop of a backtracking function, after sorting candidates:
for i in start..candidates.len() {
    // Skip duplicates at the same decision level.
    if i > start && candidates[i] == candidates[i - 1] {
        continue;
    }
    // ... choose, explore, unchoose ...
}
```

This is essential for problems like "Subsets II" or "Combination Sum II" where the
input has repeated values. Without this check, you generate duplicate results.

### 4. Symmetry Breaking

If the problem has symmetrical solutions (rotations, reflections), fix one element to
avoid exploring redundant branches. For N-Queens, you can restrict the first queen to
the left half of the first row and multiply the count by 2 (handling the odd middle
column separately). This cuts the search space roughly in half.

### 5. Bounding (Branch and Bound)

Track a running cost and compare it to the best known solution. If the partial cost
already exceeds (or cannot possibly beat) the best complete solution found so far, prune.
This turns backtracking into **branch and bound**, which is widely used in optimization
problems like the traveling salesman problem.

```
  Example: knapsack with backtracking
  - Current weight exceeds capacity -> prune (feasibility)
  - Current value + optimistic estimate of remaining items
    < best solution found so far -> prune (bounding)
```

### Pruning Impact: N-Queens

```
  Without pruning (try every placement):
    8^8 = 16,777,216 board configurations to check.

  With column-only pruning (one queen per column):
    8! = 40,320 configurations.

  With column + diagonal pruning:
    About 15,720 nodes explored to find all 92 solutions.

  That is a 1,000x reduction from the naive approach.
```

---

## Combination Sum: Pruning in Action

**Problem:** Given an array of distinct integers `candidates` and a target, find all
unique combinations where the candidates sum to the target. Each number may be used
unlimited times.

For `candidates = [2, 3, 6, 7]` and `target = 7`:
- `[2, 2, 3]` (2+2+3 = 7)
- `[7]` (7 = 7)

```rust
fn combination_sum(candidates: &mut Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort(); // Sort to enable pruning!
    let mut results = Vec::new();
    let mut current = Vec::new();
    combo_helper(candidates, target, 0, &mut current, &mut results);
    results
}

fn combo_helper(
    candidates: &[i32],
    remaining: i32,
    start: usize,
    current: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    if remaining == 0 {
        results.push(current.clone());
        return;
    }

    for i in start..candidates.len() {
        let c = candidates[i];

        // PRUNING: since candidates is sorted, if this candidate exceeds
        // the remaining target, all subsequent candidates will too.
        if c > remaining {
            break; // not `continue` -- everything after is also too large
        }

        current.push(c);
        // Pass `i` (not `i+1`) because we can reuse the same candidate.
        combo_helper(candidates, remaining - c, i, current, results);
        current.pop();
    }
}

fn main() {
    let mut candidates = vec![2, 3, 6, 7];
    let results = combination_sum(&mut candidates, 7);
    for r in &results {
        println!("{:?}", r);
    }
    // [2, 2, 3]
    // [7]
}
```

The `break` instead of `continue` is the pruning magic. Because candidates are sorted,
once one is too large, all remaining ones are too. This single optimization can cut
the search space dramatically.

**Complexity:** Bounded by O(n^(T/min)) where T is the target and min is the smallest
candidate. Each level of recursion picks a candidate, and the maximum depth is T/min.
Pruning makes the practical runtime much better.

---

## Word Search on a Grid

**Problem:** Given a 2D grid of characters and a word, determine if the word exists in
the grid. The word can be constructed from letters of sequentially adjacent cells (up,
down, left, right). Each cell may be used at most once per word.

```
Grid:              Word: "ABCCED"
  A B C E
  S F C S          Path: A(0,0) -> B(0,1) -> C(0,2) -> C(1,2)
  A D E E                -> E(2,2) -> D(2,1)
```

### Rust Implementation

```rust
fn exist(board: &[Vec<char>], word: &str) -> bool {
    let word: Vec<char> = word.chars().collect();
    let rows = board.len();
    let cols = board[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            if board[r][c] == word[0] {
                if search(board, &word, 0, r, c, &mut visited) {
                    return true;
                }
            }
        }
    }
    false
}

fn search(
    board: &[Vec<char>],
    word: &[char],
    index: usize,
    row: usize,
    col: usize,
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    if index == word.len() {
        return true; // all characters matched
    }

    // Bounds check, character match, and visited check (pruning).
    if row >= board.len()
        || col >= board[0].len()
        || board[row][col] != word[index]
        || visited[row][col]
    {
        return false;
    }

    visited[row][col] = true;   // choose: mark this cell as used

    // Explore all four directions.
    let found = search(board, word, index + 1, row + 1, col, visited)
        || search(board, word, index + 1, row.wrapping_sub(1), col, visited)
        || search(board, word, index + 1, row, col + 1, visited)
        || search(board, word, index + 1, row, col.wrapping_sub(1), visited);

    visited[row][col] = false;  // unchoose: unmark for other paths

    found
}

fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];

    assert!(exist(&board, "ABCCED"));
    assert!(exist(&board, "SEE"));
    assert!(!exist(&board, "ABCB"));
    println!("All assertions passed.");
}
```

A note on the `wrapping_sub(1)` calls: since `row` and `col` are `usize` (unsigned), we
cannot subtract 1 from 0 without underflow. `wrapping_sub(1)` produces `usize::MAX`, which
fails the bounds check in the next call, so it works out. An alternative is to cast to
`i32` or use explicit `if row > 0` checks.

The choose/unchoose here is the visited grid. We mark a cell when entering it and unmark
when leaving. This ensures each cell is used at most once *within a single path*, but can
be reused in different paths explored from different starting points.

**Complexity:** O(m * n * 4^L) where m and n are the grid dimensions and L is the word
length. At each cell we branch into at most 4 directions (really 3 since we came from
one), up to L levels deep. Character mismatches and visited checks prune heavily.

---

## Full Worked Example: Generate Parentheses

**Problem:** Given n pairs of parentheses, generate all valid (well-formed) combinations.

For n=3: `"((()))", "(()())", "(())()", "()(())", "()()()"`

This is a classic backtracking problem that does not involve arrays or grids, showing the
versatility of the technique.

```rust
fn generate_parentheses(n: usize) -> Vec<String> {
    let mut results = Vec::new();
    let mut current = String::new();
    gen_helper(n, 0, 0, &mut current, &mut results);
    results
}

fn gen_helper(
    n: usize,
    open: usize,    // number of '(' placed so far
    close: usize,   // number of ')' placed so far
    current: &mut String,
    results: &mut Vec<String>,
) {
    if current.len() == 2 * n {
        results.push(current.clone());
        return;
    }

    if open < n {
        current.push('(');                             // choose '('
        gen_helper(n, open + 1, close, current, results);  // explore
        current.pop();                                 // unchoose
    }

    if close < open {
        current.push(')');                             // choose ')'
        gen_helper(n, open, close + 1, current, results);  // explore
        current.pop();                                 // unchoose
    }
}

fn main() {
    let parens = generate_parentheses(3);
    for p in &parens {
        println!("{}", p);
    }
    assert_eq!(parens.len(), 5); // Catalan number C(3) = 5
}
```

The decision tree for n=2:

```
                         ""
                          |
                         "("
                       /     \
                    "(("      "()"
                     |          |
                   "(()"      "()()"  <-- complete (len=4), record it
                     |
                   "(())"  <-- complete, record it

  Results: "(())", "()()"
```

The pruning is elegant:
- You can place `(` only if you have not used all n opening parens (`open < n`).
- You can place `)` only if it would not create an imbalance (`close < open`).

These two rules alone ensure every generated string is valid. No post-filtering needed.

**Complexity:** The number of valid parenthesizations is the nth Catalan number,
C(n) = (2n)! / ((n+1)! * n!), which is O(4^n / sqrt(n)). Each result has length 2n.

---

## How Each Problem Maps to the Template

```
Problem          | State             | Choices          | Complete When       | Pruning
-----------------+-------------------+------------------+---------------------+-------------------
Permutations     | partial ordering  | unused elements  | len == n            | used[i] check
Combinations     | partial combo     | nums >= start    | len == k            | not enough left
Subsets          | partial subset    | nums >= index    | always (record all) | (none needed)
N-Queens         | queens per row    | columns 0..n     | row == n            | col/diag conflict
Sudoku           | partially filled  | digits 1..9      | no empty cells      | row/col/box check
Word Search      | grid position     | 4 neighbors      | matched full word   | char mismatch
Combination Sum  | partial combo     | candidates >= i  | remaining == 0      | candidate > remain
Parentheses      | partial string    | '(' or ')'       | len == 2*n          | open/close counts
```

The beauty of backtracking is that once you understand the template, each new problem is
just a matter of identifying what the state, choices, completion condition, and pruning
criteria are.

---

## Complexity Summary

| Problem         | Time Complexity        | Space Complexity | Notes                          |
|-----------------|------------------------|------------------|--------------------------------|
| Permutations    | O(n * n!)              | O(n) recursion   | n! permutations, each len n    |
| Combinations    | O(k * C(n,k))          | O(k) recursion   | Pruned subset generation       |
| Subsets         | O(n * 2^n)             | O(n) recursion   | 2^n subsets, each cloned       |
| N-Queens        | O(n!) upper bound      | O(n)             | Heavily pruned in practice     |
| Sudoku (9x9)    | O(9^m)                | O(m)             | m = empty cells; fast w/ prune |
| Word Search     | O(m * n * 4^L)         | O(L)             | L = word length                |
| Combination Sum | O(n^(T/min))           | O(T/min)         | T = target; min = smallest     |
| Parentheses     | O(4^n / sqrt(n))       | O(n)             | nth Catalan number             |

Space complexities are for recursion depth only, not counting output storage. The output
itself can be exponential.

---

## How to Recognize Backtracking Problems in Interviews

In interviews, backtracking problems share telltale signals. Look for these patterns.

### Signal 1: "Find All..." or "Generate All..."

When the problem asks for *all* valid configurations, permutations, combinations, or
paths, backtracking is almost certainly the approach. "Generate all valid parentheses,"
"find all paths from source to target," "list all permutations."

### Signal 2: Constraint Satisfaction

The problem defines rules that a solution must satisfy. No two queens attack each other.
Each row has unique digits. The path must visit every node. Building a solution
incrementally while checking constraints at each step is textbook backtracking.

### Signal 3: "Can You...?" Existence Questions

"Can you place n queens?", "Is there a path?", "Can the string be segmented into
dictionary words?" These often use backtracking with early return on the first valid
solution.

### Signal 4: The Input Size Is Small

Backtracking is exponential. If n <= 15-20, the interviewer likely expects backtracking
or bitmask DP. If n is in the thousands, look for polynomial algorithms instead.

### Signal 5: Choices with Undo

If the problem naturally decomposes into "pick one option from a set, see what happens,
maybe try a different option," that is a backtracking structure. The "undo" is the
giveaway.

### Backtracking vs Dynamic Programming

This is a common source of confusion. The key distinction:

- **Backtracking** explores a tree of choices and prunes invalid branches. It is for
  *search* and *enumeration* problems. Subproblems are typically *not* overlapping.
- **Dynamic programming** also has optimal substructure, but subproblems *overlap*
  (the same subproblem is solved multiple times). DP stores results to avoid recomputation.

If you find yourself memoizing recursive calls in a backtracking solution, you are
transitioning into DP territory. The key question: are there repeated subproblems?
If yes, memoize (DP). If no, backtrack.

```
  BACKTRACKING                          DYNAMIC PROGRAMMING
  ------------                          -------------------
  Explores decision tree                Fills a table of subproblems
  Prunes invalid branches               Memoizes overlapping subproblems
  "Find all / any solutions"            "Find the optimal value / count"
  No repeated subproblems (usually)     Many repeated subproblems
  Exponential time (inherent)           Polynomial time (often)
  N-Queens, Sudoku, permutations        Knapsack, edit distance, LCS
```

---

## Backtracking vs DFS

You might notice that backtracking looks a lot like DFS on a tree. That is not a
coincidence -- backtracking *is* DFS on a decision tree. The difference is conceptual:

- **DFS** is a graph/tree traversal technique. You are visiting nodes in an *existing*
  structure.
- **Backtracking** is DFS on an *implicit* decision tree. The tree does not exist in
  memory -- you generate it on the fly by making and undoing choices.

In plain DFS on a graph, you mark a node as visited and leave it visited. In backtracking,
you undo your changes so the state is clean for the next branch. That undo step is the
defining characteristic.

---

## Rust-Specific Tips

### 1. `&mut Vec` Is Your Friend

Pass the current state as a mutable reference. Push before recursing, pop after. This
avoids allocating new vectors at every level of recursion.

### 2. Clone Only When Recording a Solution

The `results.push(current.clone())` is the only allocation in the hot path. Do not
clone prematurely. If you only need to count solutions, increment a counter instead.

### 3. Use `usize` Carefully

When subtracting indices, watch out for underflow. Rust's `usize` is unsigned, so
`0usize - 1` panics in debug mode. Use `wrapping_sub`, `checked_sub`, or convert to
`isize` for grid problems with negative offsets.

### 4. Iterators for Constraint Checking

Rust's iterator chains are great for validation logic:

```rust
// Check if any queen in `queens` occupies the same column or diagonal as (row, col).
fn is_safe_idiomatic(queens: &[usize], row: usize, col: usize) -> bool {
    queens.iter().enumerate().all(|(r, &c)| {
        c != col && (row - r) != col.abs_diff(c)
    })
}
```

### 5. Bitmask State for Small n

For problems with n <= 20 or so, represent the "used" set as a bitmask (`u32` or `u64`)
instead of `Vec<bool>`. Copying and comparing becomes trivially fast:

```rust
fn permute_bitmask(nums: &[i32], used: u32, current: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
    if current.len() == nums.len() {
        results.push(current.clone());
        return;
    }
    for i in 0..nums.len() {
        if used & (1 << i) != 0 {
            continue;
        }
        current.push(nums[i]);
        permute_bitmask(nums, used | (1 << i), current, results);
        current.pop();
    }
}
```

### 6. Sorting Enables Pruning

Many backtracking problems benefit from sorting the input first. This allows early
`break` (instead of `continue`) when values exceed a threshold, as in Combination Sum.

---

## Practice Problems

### Easy (Build Familiarity with the Template)

1. **Subsets** (LeetCode 78) -- Generate all subsets of a set of distinct integers.
   Direct application of the subsets pattern. Focus on getting the template right.

2. **Combinations** (LeetCode 77) -- Choose k from 1..=n. Direct application of the
   combinations pattern with the `start` parameter.

3. **Letter Case Permutation** (LeetCode 784) -- Given a string, generate all strings by
   toggling each letter's case. Digits stay fixed. Binary-choice backtracking at each
   character position.

4. **Binary Watch** (LeetCode 401) -- Given a number of LEDs turned on, return all
   possible times the watch could display. Small search space, pure enumeration.

5. **Generate Parentheses** (LeetCode 22) -- Generate all valid combinations of n pairs
   of parentheses. Constraint: never more close parens than open at any prefix. Covered
   in detail in this lesson.

### Medium (Apply Pruning and Handle Constraints)

1. **Permutations II** (LeetCode 47) -- Permutations with duplicate elements. Requires
   sorting the input and skip logic (`if i > 0 && nums[i] == nums[i-1] && !used[i-1]`)
   to avoid generating duplicate permutations.

2. **Combination Sum** (LeetCode 39) -- Find combinations summing to a target with
   unlimited reuse. Requires sorted-order pruning with `break`. Covered in this lesson.

3. **Combination Sum II** (LeetCode 40) -- Same as above, but each candidate may only
   be used once, and the input can have duplicates. Combines `i+1` start with duplicate
   skipping.

4. **Word Search** (LeetCode 79) -- Find a word in a character grid. Grid-based
   backtracking with visited tracking. Covered in this lesson.

5. **Palindrome Partitioning** (LeetCode 131) -- Partition a string so every segment is
   a palindrome. Return all valid partitionings. Combine backtracking with palindrome
   checking at each cut point.

### Hard (Combine Multiple Techniques)

1. **N-Queens** (LeetCode 51) -- Place n non-attacking queens on an n x n board. Classic
   constraint satisfaction. Covered in this lesson. The "hard" is in optimizing the
   conflict detection.

2. **Sudoku Solver** (LeetCode 37) -- Fill a valid Sudoku board. Covered in this lesson.
   Optimize further with bitmask constraint tracking and MRV heuristic for cell selection.

3. **Word Search II** (LeetCode 212) -- Find multiple words in a grid simultaneously.
   Combine grid backtracking with a Trie (Lesson 20) for prefix-based pruning. Much more
   efficient than running Word Search once per word.

4. **Expression Add Operators** (LeetCode 282) -- Insert +, -, * between digits of a
   string to reach a target value. Requires tracking multiplication precedence during
   backtracking. Tricky edge cases with leading zeros and operator precedence.

5. **Stickers to Spell Word** (LeetCode 691) -- Use stickers (multisets of characters)
   to spell a target string, minimizing the number of stickers. Backtracking with
   memoization -- crosses into DP territory. Good practice for recognizing when
   backtracking needs memoization.

---

## Summary

Backtracking is recursion with **choose, explore, unchoose**. It systematically explores
a decision tree, pruning branches that cannot lead to valid solutions.

### Key Takeaways

1. **The template is universal.** Almost every backtracking problem fits the same pattern:
   iterate over choices, pick one, recurse, undo it. Learn the template and you can solve
   new problems by identifying what "choice" and "valid" mean in each context.

2. **Pruning is everything.** Without pruning, backtracking is brute force. With pruning,
   it eliminates entire subtrees of the search space. The more constraints the problem has,
   the more effective pruning is.

3. **The complexity is exponential.** Backtracking explores exponential search spaces --
   O(n!), O(2^n), O(k^n). This is inherent to the problems it solves. Pruning reduces the
   practical runtime, but does not change the complexity class.

4. **Backtracking is DFS on an implicit decision tree.** The tree is not stored in memory;
   it is generated and destroyed through the choose/unchoose mechanism.

5. **Five pruning strategies:** Constraint checking, ordering heuristics (MRV), duplicate
   skipping, symmetry breaking, and bounding (branch and bound). Most interview problems
   use the first three.

6. **Common interview problems:** Permutations, combinations, subsets, N-Queens, Sudoku,
   word search, parentheses generation, combination sum. These form the core repertoire.

7. **When to use it:** "Find all" + small n + constraints + choices with undo. When a
   polynomial algorithm exists, prefer that instead.

---

*Previous: [27 - Union-Find](./27_union_find.md) | Next: [29 - Greedy Algorithms](./29_greedy_algorithms.md)*
