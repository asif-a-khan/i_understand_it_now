# Lesson 23: Matrix / Grid Problems

## From Graphs to Grids

In Lesson 22 you learned BFS and DFS on explicit graphs -- nodes stored in adjacency
lists, edges spelled out. But many problems hand you a 2D grid of values and ask you
to traverse it, find connected regions, or compute shortest paths. No adjacency list
in sight.

The insight: **a 2D grid is an implicit graph**. Every cell is a node. Every pair of
adjacent cells (sharing an edge, not just a corner) is connected by an edge. You never
build the adjacency list -- you compute neighbors on the fly from the row and column
indices.

Once you see this, the entire BFS/DFS toolkit from Lesson 22 transfers directly. The
patterns are so common that grid problems have their own templates, and that is what
this lesson builds.

---

## The Mental Model: City Blocks

Think of a grid as a city laid out in perfect blocks. Each cell is a city block. You
can walk north, south, east, or west to an adjacent block, but not diagonally (unless
the problem says otherwise).

```
     col 0   col 1   col 2   col 3
    +-------+-------+-------+-------+
row 0 |  .    |  .    |  #    |  .    |    . = open road
    +-------+-------+-------+-------+    # = building (blocked)
row 1 |  .    |  #    |  #    |  .    |
    +-------+-------+-------+-------+
row 2 |  .    |  .    |  .    |  .    |
    +-------+-------+-------+-------+
row 3 |  #    |  .    |  #    |  .    |
    +-------+-------+-------+-------+
```

If you are standing at block (0,0) and want the shortest walking route to (3,3), that
is BFS on a grid. If you want to find all blocks reachable from (0,0) without crossing
a building, that is DFS (or BFS) flood fill. If you look at this from a satellite and
count disconnected land masses surrounded by water, that is the "number of islands"
problem.

All three use the same underlying idea: treat cells as graph nodes, move to neighbors,
track what you have visited.

---

## Grid as Implicit Graph: The Key Mapping

| Graph concept    | Grid equivalent                                          |
|------------------|----------------------------------------------------------|
| Node             | Cell at position `(row, col)`                            |
| Edge             | Adjacency between neighboring cells (up/down/left/right) |
| Adjacency list   | Computed on the fly from `(row, col)` + direction offsets |
| Visited set      | A 2D boolean grid, or mark cells in-place                |
| Start node       | Some specific cell, or "all cells matching a condition"  |

The grid itself is typically a `Vec<Vec<T>>` or a flat `Vec<T>` with row-major indexing.
For this lesson we use `Vec<Vec<T>>` since it maps cleanly to the mental model.

---

## Neighbor Patterns

### 4-Directional (Cardinal)

The standard. Each cell has up to 4 neighbors: up, down, left, right.

```
          [up]
           |
  [left]--[*]--[right]
           |
         [down]
```

In code, represent the four directions as `(row_offset, col_offset)` pairs:

```rust
const DIRS_4: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
//                                  up      down     left     right
```

### 8-Directional (Including Diagonals)

Some problems allow diagonal movement. Add the four diagonal offsets:

```
  [NW] [N] [NE]
     \  |  /
  [W]--[*]--[E]
     /  |  \
  [SW] [S] [SE]
```

```rust
const DIRS_8: [(i32, i32); 8] = [
    (-1, 0), (1, 0), (0, -1), (0, 1),     // cardinal
    (-1, -1), (-1, 1), (1, -1), (1, 1),   // diagonal
];
```

### Boundary Checking in Rust

When you compute a neighbor position, it might fall outside the grid. You must check
bounds before accessing the cell. Rust's `usize` type (which cannot be negative) adds
a wrinkle: subtracting 1 from row 0 wraps around to `usize::MAX`, not -1.

The cleanest approach: cast to `i32` (or `isize`) for the arithmetic, then check bounds
before casting back.

```rust
fn neighbors_4(r: usize, c: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut result = Vec::new();
    for (dr, dc) in DIRS {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
            result.push((nr as usize, nc as usize));
        }
    }
    result
}
```

You will see this bounds-check pattern in every grid problem. Some people inline it
rather than calling a helper, but the logic is always the same: compute the signed
position, reject anything out of range, cast to `usize`.

---

## Visited Tracking

When traversing a grid, you must track which cells you have already visited to avoid
infinite loops (going back and forth between two cells forever). Two main approaches:

### Approach 1: Separate Boolean Grid

Allocate a `Vec<Vec<bool>>` of the same dimensions. Set `visited[r][c] = true` when
you first reach a cell.

```rust
let mut visited = vec![vec![false; cols]; rows];
// ...
if !visited[nr][nc] {
    visited[nr][nc] = true;
    // process (nr, nc)
}
```

**Pros**: Does not modify the input. Clean separation of concerns.
**Cons**: Extra O(R * C) memory.

### Approach 2: Modify the Grid In-Place

If the problem allows it, overwrite visited cells with a sentinel value. For example,
in an island counting problem where land is `'1'` and water is `'0'`, you can set
visited land cells to `'0'` (sink the island as you visit it).

```rust
// Mark visited by "sinking" land
grid[nr][nc] = '0';
```

**Pros**: No extra memory. Slightly faster (no second array lookup).
**Cons**: Destroys the input. Not always acceptable.

Which to use depends on the problem constraints. If you need the original grid intact
afterward, use a separate visited grid. If the problem is "find and count things" and
you do not need the grid again, in-place modification is simpler.

---

## Grid BFS Template

BFS on a grid finds the **shortest path** (measured in number of steps) between cells.
The structure mirrors BFS on an explicit graph, but neighbors are computed from
coordinates instead of looked up in an adjacency list.

```rust
use std::collections::VecDeque;

/// BFS from (start_r, start_c). Returns the distance grid where
/// dist[r][c] = shortest distance from start, or -1 if unreachable.
fn bfs_grid(grid: &[Vec<i32>], start_r: usize, start_c: usize) -> Vec<Vec<i32>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dist = vec![vec![-1i32; cols]; rows];

    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut queue = VecDeque::new();
    dist[start_r][start_c] = 0;
    queue.push_back((start_r, start_c));

    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in DIRS {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                // Skip walls and already-visited cells
                if grid[nr][nc] == 0 && dist[nr][nc] == -1 {
                    dist[nr][nc] = dist[r][c] + 1;
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    dist
}
```

The skeleton:
1. Initialize a distance (or visited) structure.
2. Seed the queue with the starting cell(s).
3. Process cells FIFO. For each cell, check all neighbors. If a neighbor is valid,
   unvisited, and passable, record its distance and enqueue it.

BFS guarantees that the first time you reach a cell, you reach it via the shortest
path. This is because BFS explores cells in order of their distance from the source.

### Visualizing BFS on a Grid

Start at S, find shortest path to E. `#` = wall, `.` = open.

```
  Grid:               BFS distance from S:
  . . # . .           0 1 # . .
  . # # . .           1 # # . .
  S . . . #           0 1 2 3 #
  # . # . .           # 2 # 4 5
  # . . . E           # 3 4 5 6*  <-- shortest path to E is 6 steps

  The BFS wavefront expands outward:

  Step 0:  S            Step 1:  S .         Step 2:  S . .
           .                     . .                  . . .
                                                      .
  (etc. -- each step adds all cells at distance d)
```

The path itself can be reconstructed by storing a `parent` grid and tracing back
from E to S (same technique as graph BFS path reconstruction in Lesson 22).

---

## Grid DFS Template

DFS on a grid explores as deep as possible along one path before backtracking. It is
the natural choice for flood fill and connected component problems where you do not
need shortest paths -- just reachability.

```rust
/// DFS flood fill starting from (r, c). Visits all connected cells where
/// grid[r][c] matches `target`. Marks visited cells by setting them to `fill`.
fn dfs_fill(grid: &mut Vec<Vec<char>>, r: usize, c: usize, target: char, fill: char) {
    let rows = grid.len();
    let cols = grid[0].len();

    if grid[r][c] != target {
        return;
    }
    grid[r][c] = fill; // mark visited (in-place)

    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in DIRS {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
            dfs_fill(grid, nr as usize, nc as usize, target, fill);
        }
    }
}
```

The skeleton:
1. Check the base case: out of bounds, wrong value, or already visited.
2. Mark the current cell as visited.
3. Recurse into all valid neighbors.

DFS can also be written iteratively with an explicit stack (just like graph DFS), which
avoids stack overflow on very large grids:

```rust
fn dfs_fill_iterative(
    grid: &mut Vec<Vec<char>>,
    start_r: usize,
    start_c: usize,
    target: char,
    fill: char,
) {
    let rows = grid.len();
    let cols = grid[0].len();
    if grid[start_r][start_c] != target {
        return;
    }

    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut stack = vec![(start_r, start_c)];
    grid[start_r][start_c] = fill;

    while let Some((r, c)) = stack.pop() {
        for (dr, dc) in DIRS {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] == target {
                    grid[nr][nc] = fill;
                    stack.push((nr, nc));
                }
            }
        }
    }
}
```

For Rust specifically, the iterative version is worth knowing. The default stack size
on Linux is 8 MB, and a 1000x1000 grid with DFS can easily produce a million recursive
calls, each consuming stack space. The iterative version uses heap-allocated `Vec`
instead.

---

## Problem 1: Number of Islands (Classic)

Given a 2D grid where `'1'` is land and `'0'` is water, count the number of islands.
An island is a group of `'1'` cells connected horizontally or vertically.

### The Satellite Analogy

Imagine you are looking at a satellite image. Blue pixels are ocean, green pixels are
land. An "island" is any connected blob of green pixels. Two green pixels belong to
the same island if you can walk from one to the other stepping only on green pixels
(no diagonal steps, no swimming).

```
  Grid:                  Islands marked:
  1 1 0 0 0              A A . . .
  1 1 0 0 0              A A . . .
  0 0 1 0 0              . . B . .
  0 0 0 1 1              . . . C C

  3 islands: A, B, C
```

### The Algorithm

Scan every cell. When you find an unvisited `'1'`, you have discovered a new island.
Run DFS (or BFS) from that cell to visit every cell belonging to this island, marking
them so you do not count them again. Increment the island counter.

```rust
fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    if rows == 0 { return 0; }
    let cols = grid[0].len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                count += 1;
                // Sink this entire island so we don't count it again
                sink_island(grid, r, c);
            }
        }
    }
    count
}

fn sink_island(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    grid[r][c] = '0'; // sink (mark visited)

    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in DIRS {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] == '1' {
                sink_island(grid, nr, nc);
            }
        }
    }
}
```

### Step-by-Step Trace

```
  Initial grid:        Scan (0,0): land! count=1. DFS sinks island A.
  1 1 0 0 0            0 0 0 0 0
  1 1 0 0 0            0 0 0 0 0
  0 0 1 0 0            0 0 1 0 0
  0 0 0 1 1            0 0 0 1 1

  Scan continues...    Scan (2,2): land! count=2. DFS sinks island B.
  (0,1)-(1,1) already  0 0 0 0 0
  sunk to '0'          0 0 0 0 0
                       0 0 0 0 0
                       0 0 0 1 1

  Scan (3,3): land!    count=3. DFS sinks island C.
  0 0 0 0 0
  0 0 0 0 0
  0 0 0 0 0
  0 0 0 0 0            Final count: 3
```

**Time**: O(R * C) -- every cell is visited at most twice (once in the scan, once in DFS).
**Space**: O(R * C) in the worst case for the DFS call stack (if the entire grid is land).

---

## Problem 2: Flood Fill

Given a grid, a starting cell `(sr, sc)`, and a new color, change the color of the
starting cell and all cells connected to it (same original color, 4-directional) to
the new color.

This is the "paint bucket" tool in image editors. Click a pixel, and all connected
pixels of the same color get filled.

### Visualization

```
  Before (fill from (1,1), color 1 -> color 2):

  1 1 1        2 2 2
  1 1 0   ->   2 2 0
  1 0 1        2 0 1

  The flood starts at (1,1) which is color 1.
  It spreads to all 4-connected cells that are also color 1.
  Cell (2,2) is color 1 but NOT connected to (1,1) through color-1 cells,
  so it stays unchanged.
```

### Step-by-Step Flood Fill

```
  Start at (1,1), color=1, new_color=2

  Step 0: Visit (1,1). Color 1 -> 2.    Step 1: Visit (0,1). Color 1 -> 2.
          1 1 1                                   1 2 1
          1 2 0                                   1 2 0
          1 0 1                                   1 0 1

  Step 2: Visit (0,0). Color 1 -> 2.    Step 3: Visit (0,2). Color 1 -> 2.
          2 2 1                                   2 2 2
          1 2 0                                   1 2 0
          1 0 1                                   1 0 1

  Step 4: Visit (1,0). Color 1 -> 2.    Step 5: Visit (2,0). Color 1 -> 2.
          2 2 2                                   2 2 2
          2 2 0                                   2 2 0
          1 0 1                                   2 0 1

  Done. No more connected cells of color 1.
```

### Implementation

```rust
fn flood_fill(grid: &mut Vec<Vec<i32>>, sr: usize, sc: usize, new_color: i32) {
    let original = grid[sr][sc];
    if original == new_color {
        return; // Avoid infinite loop when new color equals original
    }
    fill(grid, sr, sc, original, new_color);
}

fn fill(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, original: i32, new_color: i32) {
    let rows = grid.len();
    let cols = grid[0].len();
    if grid[r][c] != original {
        return;
    }
    grid[r][c] = new_color;

    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in DIRS {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
            fill(grid, nr as usize, nc as usize, original, new_color);
        }
    }
}
```

The early return when `original == new_color` is easy to forget and causes an infinite
loop without it -- the DFS keeps "visiting" cells that are already the new color because
the color matches the original. This is a common gotcha.

**Time**: O(R * C). **Space**: O(R * C) for the call stack.

---

## Problem 3: Shortest Path in Binary Matrix

Given an `n x n` binary grid where `0` is passable and `1` is blocked, find the
shortest path from top-left `(0,0)` to bottom-right `(n-1, n-1)`. This problem allows
**8-directional** movement (including diagonals). Return the path length (number of
cells), or -1 if no path exists.

This is BFS. DFS would find *a* path but not necessarily the shortest.

```rust
use std::collections::VecDeque;

fn shortest_path_binary_matrix(grid: &Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
        return -1; // start or end is blocked
    }

    const DIRS: [(i32, i32); 8] = [
        (-1, 0), (1, 0), (0, -1), (0, 1),
        (-1, -1), (-1, 1), (1, -1), (1, 1),
    ];

    let mut visited = vec![vec![false; n]; n];
    let mut queue = VecDeque::new();

    visited[0][0] = true;
    queue.push_back((0usize, 0usize, 1i32)); // (row, col, path_length)

    while let Some((r, c, dist)) = queue.pop_front() {
        if r == n - 1 && c == n - 1 {
            return dist;
        }
        for (dr, dc) in DIRS {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] == 0 && !visited[nr][nc] {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc, dist + 1));
                }
            }
        }
    }

    -1 // no path found
}
```

### Visualizing the BFS Wavefront

```
  Grid (0=open, 1=wall):       BFS path length from (0,0):

  0 0 0                        1 2 3
  1 1 0                        # # 4
  1 1 0                        # # 5

  Shortest path: (0,0)->(0,1)->(0,2)->(1,2)->(2,2) = 5 cells

  But with 8-directional movement:

  0 0 0                        1 2 .
  1 1 0                        # # 2
  1 1 0                        # # 3

  Diagonal: (0,0)->(1,1) is blocked (wall at (1,1)).
  (0,0)->(0,1)->(1,2)->(2,2) = 4 cells? Let's check:
  Actually (0,0) can go diag to... we need to check.
  The BFS handles this correctly -- it explores all 8 neighbors at each step.
```

**Time**: O(n^2). **Space**: O(n^2).

---

## Problem 4: Rotting Oranges (Multi-Source BFS)

A grid contains three values: `0` (empty), `1` (fresh orange), `2` (rotten orange).
Every minute, each rotten orange rots all 4-directionally adjacent fresh oranges.
Return the minimum number of minutes until no fresh orange remains, or -1 if it is
impossible.

### The Contagion Analogy

Think of this as a disease spreading through a population. Multiple infected people
(rotten oranges) exist at the start, and each minute the disease spreads to all their
immediate neighbors. You want to know how long until everyone is infected -- or whether
some people are isolated and can never be reached.

This is **multi-source BFS**: instead of starting from one cell, you start from *all*
rotten oranges simultaneously.

### Visualization

```
  Minute 0:       Minute 1:       Minute 2:       Minute 3:       Minute 4:
  2 1 1           2 2 1           2 2 2           2 2 2           2 2 2
  1 1 0           2 1 0           2 2 0           2 2 0           2 2 0
  0 1 1           0 1 1           0 2 1           0 2 2           0 2 2

  [rotten=2 spreads to adjacent fresh=1 each minute]
  Answer: 4 minutes
```

### Implementation

```rust
use std::collections::VecDeque;

fn oranges_rotting(grid: &mut Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut queue = VecDeque::new();
    let mut fresh_count = 0;

    // Seed the queue with ALL initially rotten oranges
    for r in 0..rows {
        for c in 0..cols {
            match grid[r][c] {
                2 => queue.push_back((r, c)),
                1 => fresh_count += 1,
                _ => {}
            }
        }
    }

    if fresh_count == 0 {
        return 0; // nothing to rot
    }

    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut minutes = 0;

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut any_rotted = false;

        for _ in 0..level_size {
            let (r, c) = queue.pop_front().unwrap();
            for (dr, dc) in DIRS {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] == 1 {
                        grid[nr][nc] = 2; // rot it
                        fresh_count -= 1;
                        any_rotted = true;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }

        if any_rotted {
            minutes += 1;
        }
    }

    if fresh_count == 0 { minutes } else { -1 }
}
```

The key trick is **level-by-level BFS** (the same technique from Lesson 16's
level-order traversal). Each "level" of the BFS corresponds to one minute of time.
All oranges that rot during the same minute are processed in the same level.

**Why multi-source?** Imagine you have 5 rotten oranges scattered across the grid.
The rot spreads simultaneously from all 5. If you ran BFS from each rotten orange
one at a time, you would overcount the time. By seeding the queue with all rotten
oranges at distance 0, the BFS wavefront naturally handles simultaneous spreading.

**Time**: O(R * C). **Space**: O(R * C).

---

## Problem 5: Surrounded Regions

Given a grid of `'X'` and `'O'`, capture all `'O'` regions that are completely
surrounded by `'X'`. An `'O'` on the border of the grid (or connected to a border
`'O'`) cannot be captured.

### The Battlefield Analogy

Think of `'X'` cells as your territory and `'O'` cells as enemy territory. You can
capture (flip to `'X'`) any enemy region that is fully enclosed by your territory.
But if the enemy has an escape route to the edge of the map, you cannot capture that
region.

### The Trick: Think in Reverse

Instead of finding surrounded regions (hard -- you need to check if a region touches
any border), find the **un-surrounded** regions (easy -- they are the ones connected
to a border `'O'`). Everything else is surrounded.

Algorithm:
1. Walk the border of the grid. For every `'O'` on the border, DFS/BFS to mark all
   connected `'O'` cells as "safe" (use a temporary marker like `'S'`).
2. Scan the entire grid:
   - `'O'` cells that were NOT marked safe are surrounded. Flip them to `'X'`.
   - `'S'` cells are safe. Flip them back to `'O'`.

### Visualization

```
  Input:           Step 1: Mark border-connected O's as S:
  X X X X          X X X X
  X O O X          X O O X
  X X O X          X X O X
  X O X X          X S X X       <- (3,1) is on-border-adjacent, mark S

  Wait -- (3,1) is O and it is on row 3 (last row in a 4-row grid). But
  let's say the grid is bigger:

  X X X X X        X X X X X
  X O O X X        X O O X X
  X X O X X        X X O X X
  X O O X X        X O O X X
  X X X O X        X X X S X    <- (4,3) is border-adjacent
  X X X X X        X X X X X

  Now DFS from (4,3): only (4,3) itself is connected on the border.
  The O-region at (1,1)-(1,2)-(2,2)-(3,1)-(3,2) is not connected to
  any border O, so it gets captured.

  Result:
  X X X X X
  X X X X X
  X X X X X
  X X X X X
  X X X O X    <- the S was restored to O
  X X X X X
```

### Implementation

```rust
fn solve(board: &mut Vec<Vec<char>>) {
    let rows = board.len();
    if rows == 0 { return; }
    let cols = board[0].len();

    // Step 1: Mark all border-connected 'O' cells as 'S' (safe)
    for r in 0..rows {
        for c in 0..cols {
            let is_border = r == 0 || r == rows - 1 || c == 0 || c == cols - 1;
            if is_border && board[r][c] == 'O' {
                mark_safe(board, r, c);
            }
        }
    }

    // Step 2: Flip remaining 'O' to 'X' (captured), and 'S' back to 'O' (safe)
    for r in 0..rows {
        for c in 0..cols {
            match board[r][c] {
                'O' => board[r][c] = 'X',  // surrounded, capture it
                'S' => board[r][c] = 'O',  // safe, restore it
                _ => {}
            }
        }
    }
}

fn mark_safe(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
    let rows = board.len();
    let cols = board[0].len();
    board[r][c] = 'S';

    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in DIRS {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
            let (nr, nc) = (nr as usize, nc as usize);
            if board[nr][nc] == 'O' {
                mark_safe(board, nr, nc);
            }
        }
    }
}
```

**Time**: O(R * C). **Space**: O(R * C) for the recursion stack.

---

## Template Summary

Almost every grid problem is a variation of these two templates. Here they are
side by side for reference.

### Grid BFS (Shortest Path / Level-by-Level)

```rust
use std::collections::VecDeque;

fn grid_bfs(grid: &[Vec<i32>], start: (usize, usize)) {
    let rows = grid.len();
    let cols = grid[0].len();
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();

    visited[start.0][start.1] = true;
    queue.push_back(start);

    while let Some((r, c)) = queue.pop_front() {
        // Process cell (r, c) here

        for (dr, dc) in DIRS {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if !visited[nr][nc] && grid[nr][nc] != 1 /* passable */ {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc));
                }
            }
        }
    }
}
```

### Grid DFS (Flood Fill / Connected Components)

```rust
fn grid_dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, target: char) {
    let rows = grid.len();
    let cols = grid[0].len();
    if grid[r][c] != target { return; }

    grid[r][c] = '#'; // mark visited (choose appropriate sentinel)

    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in DIRS {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
            grid_dfs(grid, nr as usize, nc as usize, target);
        }
    }
}
```

The differences between the two:
- **BFS** uses a queue (`VecDeque`), finds shortest paths, processes level-by-level.
- **DFS** uses a stack (call stack or explicit `Vec`), explores full branches, good for
  "visit everything reachable."
- Both have the same time and space complexity: O(R * C).

---

## When to Use BFS vs DFS on Grids

| Problem type                          | Use BFS           | Use DFS              |
|---------------------------------------|-------------------|----------------------|
| Shortest path (unweighted)            | Yes               | No (finds a path, not shortest) |
| Level-by-level processing (minutes)   | Yes               | No                   |
| Multi-source simultaneous spread      | Yes               | No                   |
| Flood fill                            | Either works      | Simpler code         |
| Count connected components (islands)  | Either works      | Simpler code         |
| Find if any path exists               | Either works      | Either works         |
| Topological ordering on grid          | Depends           | Depends              |

Rule of thumb: if the problem mentions "shortest", "minimum steps", or "simultaneous
spread," reach for BFS. If it says "fill", "connected", "reachable", or "count
regions," DFS is usually cleaner.

---

## Common Pitfalls

### 1. Forgetting the `usize` Wrapping Issue

```rust
// Bug: if r is 0, r - 1 wraps to usize::MAX
let nr = r - 1; // WRONG when r == 0

// Fix: use signed arithmetic
let nr = r as i32 - 1;
if nr >= 0 { /* safe to cast back */ }
```

### 2. Marking Visited Too Late

```rust
// Bug: mark visited AFTER popping from queue -> duplicate entries
while let Some((r, c)) = queue.pop_front() {
    if visited[r][c] { continue; } // too late, duplicates already in queue
    visited[r][c] = true;
    // ...
}

// Fix: mark visited WHEN ENQUEUEING
if !visited[nr][nc] {
    visited[nr][nc] = true;        // mark NOW
    queue.push_back((nr, nc));
}
```

Marking when you enqueue (not when you dequeue) prevents the same cell from being
added to the queue multiple times. This does not affect correctness but can blow up
time and memory.

### 3. Infinite Loop in Flood Fill

If the fill color equals the original color, the DFS will visit cells endlessly
because they always "match" the target. Always check for this:

```rust
if original_color == new_color { return; }
```

### 4. Stack Overflow on Large Grids

Recursive DFS on a 1000x1000 grid can produce up to 1,000,000 recursive calls. On a
default 8 MB stack, this will overflow. Solutions:
- Use iterative DFS with an explicit `Vec` stack.
- Use BFS (queue-based, no deep recursion).
- Increase the stack size with `std::thread::Builder::new().stack_size(...)`.

### 5. Off-by-One in Grid Dimensions

```rust
// Remember: rows = grid.len(), cols = grid[0].len()
// Valid indices: r in 0..rows, c in 0..cols
// A common mistake: using rows where you mean cols (or vice versa)
if nr < rows as i32 && nc < rows as i32  // Bug: should be cols for nc
```

---

## Complexity Summary

For a grid with R rows and C columns:

| Operation              | Time      | Space     | Notes                            |
|------------------------|-----------|-----------|----------------------------------|
| BFS shortest path      | O(R*C)    | O(R*C)    | Visit each cell at most once     |
| DFS flood fill         | O(R*C)    | O(R*C)    | Call stack or explicit stack      |
| Number of islands      | O(R*C)    | O(R*C)    | Full scan + DFS per island       |
| Multi-source BFS       | O(R*C)    | O(R*C)    | All sources seeded at once       |
| Surrounded regions     | O(R*C)    | O(R*C)    | Border DFS + full scan           |

The space is O(R*C) in the worst case because the queue/stack could hold every cell
(imagine a grid that is entirely open). In practice, BFS queue size is bounded by the
maximum wavefront width, and DFS stack depth is bounded by the longest path, but the
worst case is always O(R*C).

---

## Exercises

1. **Max Area of Island**: Given a grid of `0` and `1`, find the island with the
   largest area (number of cells). Modify the island counting code to track size
   during each DFS.

2. **Walls and Gates** (LeetCode 286): A grid has empty rooms (`INF`), walls (`-1`),
   and gates (`0`). Fill each empty room with the distance to its nearest gate. Hint:
   this is multi-source BFS from all gates simultaneously.

3. **Pacific Atlantic Water Flow** (LeetCode 417): A grid of heights where water can
   flow to adjacent cells of equal or lower height. The left/top edges drain to the
   Pacific, the right/bottom to the Atlantic. Find cells where water can reach both
   oceans. Hint: BFS inward from each ocean's border, then intersect.

4. **Word Search** (LeetCode 79): Given a grid of characters and a target word, check
   if the word can be formed by a path of adjacent cells (no cell reused). This is DFS
   with backtracking.

5. **01 Matrix** (LeetCode 542): Given a binary matrix, find the distance from each
   cell to the nearest `0`. Hint: multi-source BFS from all `0` cells.

6. **Number of Enclaves** (LeetCode 1020): Given a grid of land and sea, count land
   cells that cannot walk off the grid boundary. This is the same pattern as
   surrounded regions.

---

## Key Takeaways

1. **A 2D grid is an implicit graph.** Each cell is a node, and 4-directional (or
   8-directional) neighbors are edges. You never build an adjacency list -- you compute
   neighbors on the fly from `(row, col)` + direction offsets.

2. **BFS on a grid finds shortest paths.** Seed a `VecDeque` with starting cell(s),
   process level by level, mark visited when enqueueing. Multi-source BFS handles
   simultaneous spreading from multiple start points.

3. **DFS on a grid does flood fill and connected component counting.** Recurse (or use
   an explicit stack) into all matching neighbors. Mark visited to avoid cycles.

4. **The direction array pattern** (`const DIRS: [(i32, i32); 4]`) is the backbone of
   every grid traversal. It decouples the neighbor-generation logic from the traversal
   logic. Use 4 entries for cardinal movement, 8 for diagonal-inclusive.

5. **Boundary checking requires care in Rust.** Cast `usize` coordinates to `i32`
   before adding offsets, check `>= 0` and `< dimension`, then cast back.

6. **Visited tracking** can be a separate boolean grid or in-place mutation of the
   input. Choose based on whether you need the original grid afterward.

7. **Watch for stack overflow** with recursive DFS on large grids. Iterative DFS or
   BFS avoids this. For Rust specifically, the default 8 MB stack can overflow on grids
   larger than roughly 1000x1000 with fully-connected DFS.

8. **The "reverse thinking" trick** (surrounded regions, Pacific-Atlantic) is powerful:
   instead of checking if a region satisfies a condition, start from cells that
   definitely satisfy it and mark everything they connect to.

---

Next lesson: [24 - Shortest Path Algorithms (Dijkstra, Bellman-Ford)](./24_shortest_path_algorithms.md)
