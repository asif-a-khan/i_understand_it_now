# Lesson 22: BFS & DFS on Graphs

In Lesson 16 you learned BFS and DFS on binary trees. Trees are clean: they have a root,
no cycles, and exactly one path between any two nodes. You never needed to worry about
visiting a node twice because the structure itself prevents it.

Graphs are messier. A graph can have cycles, multiple paths between nodes, disconnected
components, and no obvious "root." BFS and DFS still work -- they are the same fundamental
ideas -- but you need extra machinery to handle the chaos. That machinery is the **visited
set**, and understanding when and how to use it is the core of this lesson.

Lesson 21 covered graph representations (adjacency list, adjacency matrix, edge list). This
lesson assumes you have an adjacency list and focuses entirely on *traversal*: how to
systematically visit every reachable node, and what useful information you can extract along
the way.

---

## From Trees to Graphs: What Changes?

On a tree, BFS and DFS are straightforward because the structure is acyclic and connected.
You start at the root, and every node is reachable exactly once by following edges downward.

On a graph, three things change:

1. **Cycles exist.** If node A connects to B, and B connects back to A (or through a longer
   path), a naive traversal will loop forever. You need a way to remember where you have been.

2. **No single root.** A graph might be disconnected -- multiple separate pieces with no edges
   between them. Starting from one node might not reach everything.

3. **Multiple paths.** There can be more than one way to reach a node. The first time you
   discover a node matters; revisiting it later wastes work (or worse, causes infinite loops).

The solution to all three: maintain a **visited set** (or a boolean array indexed by node).
Before processing a neighbor, check if you have already visited it. If yes, skip it.

```
Tree traversal:                    Graph traversal:
  for each child:                    for each neighbor:
    process(child)                     if not visited:
                                         mark visited
                                         process(neighbor)
```

That `if not visited` check is the entire conceptual difference. Everything else is the same
BFS queue or DFS stack you already know.

---

## BFS on Graphs: Ripples in Water

### The Analogy

Drop a stone into still water. Ripples spread outward in concentric circles. The stone's
impact point is your starting node. The first ring of ripples reaches all immediate neighbors
(distance 1). The second ring reaches their unvisited neighbors (distance 2). Each ring
expands one step further before the next ring begins.

BFS explores a graph exactly like this: all nodes at distance 0 (the start), then all at
distance 1, then distance 2, and so on. It uses a **queue** (FIFO) to maintain this
level-by-level discipline.

### The Algorithm

```
BFS(graph, start):
    create queue, push start
    mark start as visited
    while queue is not empty:
        node = queue.pop_front()
        process(node)
        for each neighbor of node:
            if neighbor not visited:
                mark neighbor as visited
                queue.push_back(neighbor)
```

### Step-by-Step Walkthrough

Consider this undirected graph:

```
    0 --- 1 --- 3
    |     |
    2     4 --- 5
```

Adjacency list:
```
0: [1, 2]
1: [0, 3, 4]
2: [0]
3: [1]
4: [1, 5]
5: [4]
```

BFS starting from node 0:

```
Step 0: Start
  Queue:   [0]
  Visited: {0}
  Output:  (none yet)

Step 1: Dequeue 0, explore neighbors 1, 2
  Queue:   [1, 2]
  Visited: {0, 1, 2}
  Output:  0

Step 2: Dequeue 1, explore neighbors 0(skip), 3, 4
  Queue:   [2, 3, 4]
  Visited: {0, 1, 2, 3, 4}
  Output:  0, 1

Step 3: Dequeue 2, explore neighbors 0(skip)
  Queue:   [3, 4]
  Visited: {0, 1, 2, 3, 4}
  Output:  0, 1, 2

Step 4: Dequeue 3, explore neighbors 1(skip)
  Queue:   [4]
  Visited: {0, 1, 2, 3, 4}
  Output:  0, 1, 2, 3

Step 5: Dequeue 4, explore neighbors 1(skip), 5
  Queue:   [5]
  Visited: {0, 1, 2, 3, 4, 5}
  Output:  0, 1, 2, 3, 4

Step 6: Dequeue 5, explore neighbors 4(skip)
  Queue:   []
  Visited: {0, 1, 2, 3, 4, 5}
  Output:  0, 1, 2, 3, 4, 5   <-- done
```

Notice the level structure:
```
  Level 0:  0
  Level 1:  1, 2          (distance 1 from start)
  Level 2:  3, 4          (distance 2 from start)
  Level 3:  5             (distance 3 from start)
```

This is the "ripple" effect. BFS naturally discovers nodes in order of their distance from
the starting node.

### BFS in Rust

```rust
use std::collections::VecDeque;

/// BFS traversal of an undirected graph represented as an adjacency list.
/// Returns the nodes in the order they were visited.
fn bfs(adj: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    order
}
```

A few things to note:

- **`VecDeque`** gives O(1) push/pop at both ends. A `Vec` would be O(n) for `pop_front`
  because it shifts all elements. You used `VecDeque` in Lesson 05.
- **`visited` is a `Vec<bool>`**, not a `HashSet`. When nodes are numbered 0..n-1 (which is
  the common adjacency list setup), a boolean array is faster than a hash set -- O(1) with no
  hashing overhead. Use `HashSet` when node identifiers are strings, large numbers, or sparse.
- **Mark visited when enqueuing, not when dequeuing.** This is a common mistake. If you wait
  until you dequeue a node to mark it visited, you might enqueue the same node multiple times
  from different neighbors. The queue could grow unnecessarily large, and while the output
  would still be correct, you waste time and memory.

### BFS for Shortest Path in Unweighted Graphs

Because BFS visits nodes in order of distance, it naturally finds the shortest path (fewest
edges) from the start to every other node. You just need to record how you got there.

```rust
use std::collections::VecDeque;

/// Returns the shortest distance from `start` to every node.
/// Unreachable nodes get distance usize::MAX.
fn bfs_distances(adj: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let n = adj.len();
    let mut dist = vec![usize::MAX; n];
    let mut queue = VecDeque::new();

    dist[start] = 0;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adj[node] {
            if dist[neighbor] == usize::MAX {
                dist[neighbor] = dist[node] + 1;
                queue.push_back(neighbor);
            }
        }
    }

    dist
}
```

Here `dist[neighbor] == usize::MAX` serves double duty: it is both the "not visited" check
and the distance tracker. The first time you reach a node is always via the shortest path
because BFS explores level by level.

To reconstruct the actual path, add a `parent` array:

```rust
use std::collections::VecDeque;

/// Returns (distances, parents). parent[start] = start as a sentinel.
/// parent[node] = usize::MAX means node is unreachable.
fn bfs_with_path(
    adj: &Vec<Vec<usize>>,
    start: usize,
) -> (Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let mut dist = vec![usize::MAX; n];
    let mut parent = vec![usize::MAX; n];
    let mut queue = VecDeque::new();

    dist[start] = 0;
    parent[start] = start; // sentinel: start is its own parent
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adj[node] {
            if dist[neighbor] == usize::MAX {
                dist[neighbor] = dist[node] + 1;
                parent[neighbor] = node;
                queue.push_back(neighbor);
            }
        }
    }

    (dist, parent)
}

/// Reconstruct the shortest path from `start` to `end`.
fn reconstruct_path(parent: &Vec<usize>, start: usize, end: usize) -> Option<Vec<usize>> {
    if parent[end] == usize::MAX {
        return None; // unreachable
    }
    let mut path = Vec::new();
    let mut current = end;
    while current != start {
        path.push(current);
        current = parent[current];
    }
    path.push(start);
    path.reverse();
    Some(path)
}
```

### BFS Level-by-Level Traversal

Sometimes you need to know exactly which nodes are at each level (distance). The trick: at
each iteration, drain the *entire current queue* before moving to the next level.

```rust
use std::collections::VecDeque;

/// Returns nodes grouped by level (distance from start).
fn bfs_levels(adj: &Vec<Vec<usize>>, start: usize) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    let mut levels: Vec<Vec<usize>> = Vec::new();

    visited[start] = true;
    queue.push_back(start);

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut current_level = Vec::new();

        for _ in 0..level_size {
            let node = queue.pop_front().unwrap();
            current_level.push(node);

            for &neighbor in &adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        levels.push(current_level);
    }

    levels
}
```

The key insight: `queue.len()` at the start of each outer loop iteration tells you exactly
how many nodes are at the current distance. You process exactly that many, and any new nodes
you enqueue belong to the next level.

---

## DFS on Graphs: Exploring a Maze

### The Analogy

You are in a maze. At each intersection, you pick one corridor and follow it as deep as it
goes. When you hit a dead end or a place you have already been, you backtrack to the last
intersection where you had an unexplored corridor and try that one. You keep going deeper
whenever possible, only backtracking when forced.

This is DFS: go deep, backtrack, repeat. It uses a **stack** (LIFO) -- either the call stack
via recursion, or an explicit stack data structure.

### The Algorithm (Recursive)

```
DFS(graph, node, visited):
    mark node as visited
    process(node)
    for each neighbor of node:
        if neighbor not visited:
            DFS(graph, neighbor, visited)
```

### The Algorithm (Iterative with Explicit Stack)

```
DFS(graph, start):
    create stack, push start
    while stack is not empty:
        node = stack.pop()
        if node is visited:
            continue
        mark node as visited
        process(node)
        for each neighbor of node:
            if neighbor not visited:
                stack.push(neighbor)
```

Note a subtle difference from BFS: in the iterative version, we check `visited` **after
popping**, not before pushing. Why? Because a node might get pushed multiple times from
different neighbors before it is popped and processed. You could also check before pushing
(like BFS does with the queue), but the "check after pop" pattern is more common for DFS
because it more closely mirrors the recursive version's behavior.

### Step-by-Step Walkthrough (Recursive DFS)

Same graph as before:

```
    0 --- 1 --- 3
    |     |
    2     4 --- 5
```

DFS starting from node 0 (choosing neighbors in adjacency list order):

```
Call DFS(0)
  Visited: {0}
  Neighbors of 0: [1, 2]
  -> 1 not visited, recurse

  Call DFS(1)
    Visited: {0, 1}
    Neighbors of 1: [0, 3, 4]
    -> 0 visited, skip
    -> 3 not visited, recurse

    Call DFS(3)
      Visited: {0, 1, 3}
      Neighbors of 3: [1]
      -> 1 visited, skip
    Return from DFS(3)

    -> 4 not visited, recurse

    Call DFS(4)
      Visited: {0, 1, 3, 4}
      Neighbors of 4: [1, 5]
      -> 1 visited, skip
      -> 5 not visited, recurse

      Call DFS(5)
        Visited: {0, 1, 3, 4, 5}
        Neighbors of 5: [4]
        -> 4 visited, skip
      Return from DFS(5)

    Return from DFS(4)

  Return from DFS(1)

  -> 2 not visited, recurse

  Call DFS(2)
    Visited: {0, 1, 3, 4, 5, 2}
    Neighbors of 2: [0]
    -> 0 visited, skip
  Return from DFS(2)

Return from DFS(0)

Visit order: 0, 1, 3, 4, 5, 2
```

Compare with BFS order `0, 1, 2, 3, 4, 5`. DFS went deep along `0 -> 1 -> 3` (dead end),
backtracked to 1, went deep along `1 -> 4 -> 5` (dead end), backtracked all the way to 0,
then finally visited 2. BFS explored level by level: `{0}, {1,2}, {3,4}, {5}`.

### Discovery vs. Finish Order

DFS has two natural orderings:

- **Discovery order (pre-order):** The order in which nodes are *first visited*. This is
  when you enter a node.
- **Finish order (post-order):** The order in which nodes are *fully processed* -- all their
  descendants have been explored. This is when you return from a node.

From the walkthrough above:
```
Discovery order:  0, 1, 3, 4, 5, 2
Finish order:     3, 5, 4, 1, 2, 0
```

Both orderings are useful. Discovery order is what you typically think of as "DFS order."
Finish order is critical for topological sorting (a preview is later in this lesson) and
strongly connected components.

```
  Discovery:     0 enters         Finish:        3 finishes first (dead end)
                  |                                5 finishes
                  v                                4 finishes (both kids done)
                0 -> 1 -> 3                        1 finishes (all subtree done)
                     |                             2 finishes
                     v                             0 finishes last (started first)
                     4 -> 5
                |
                v
                2
```

### DFS in Rust: Recursive

```rust
fn dfs_recursive(adj: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut order = Vec::new();

    fn dfs(
        node: usize,
        adj: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        order: &mut Vec<usize>,
    ) {
        visited[node] = true;
        order.push(node);
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                dfs(neighbor, adj, visited, order);
            }
        }
    }

    dfs(start, adj, &mut visited, &mut order);
    order
}
```

Rust does not allow closures that call themselves recursively (at least not without
gymnastics like `Fn` trait objects or Y-combinators). The standard pattern is a named inner
function (`fn dfs(...)`) that takes the shared state as mutable references.

### DFS in Rust: Iterative with Explicit Stack

```rust
fn dfs_iterative(adj: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut stack = vec![start];
    let mut order = Vec::new();

    while let Some(node) = stack.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        order.push(node);

        // Push neighbors in reverse order so that the first neighbor
        // in the adjacency list is processed first (popped last from stack).
        for &neighbor in adj[node].iter().rev() {
            if !visited[neighbor] {
                stack.push(neighbor);
            }
        }
    }

    order
}
```

Why reverse the neighbors? The stack is LIFO. If node 0's neighbors are `[1, 2]` and you
push 1 then 2, you pop 2 first. Reversing gives you the same traversal order as the
recursive version. In practice, the exact neighbor order rarely matters -- what matters is
that you visit everything.

**When to use iterative over recursive?** When the graph can be very deep (long chains of
nodes). Recursive DFS uses the call stack, which is limited (typically 1-8 MB depending on
your platform). A graph with 100,000 nodes in a chain would overflow the call stack. The
iterative version uses heap-allocated memory for the explicit stack, which can grow much
larger.

### DFS with Discovery and Finish Times

Tracking both orderings requires a small modification:

```rust
fn dfs_with_times(adj: &Vec<Vec<usize>>, start: usize) -> (Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut discovery = Vec::new();
    let mut finish = Vec::new();

    fn dfs(
        node: usize,
        adj: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        discovery: &mut Vec<usize>,
        finish: &mut Vec<usize>,
    ) {
        visited[node] = true;
        discovery.push(node); // pre-order: entering the node
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                dfs(neighbor, adj, visited, discovery, finish);
            }
        }
        finish.push(node); // post-order: leaving the node
    }

    dfs(start, adj, &mut visited, &mut discovery, &mut finish);
    (discovery, finish)
}
```

---

## Time and Space Complexity

Both BFS and DFS have the same complexity:

| | Time | Space |
|---|---|---|
| **BFS** | O(V + E) | O(V) |
| **DFS** | O(V + E) | O(V) |

Where V is the number of vertices and E is the number of edges.

**Time: O(V + E).** Every node is visited exactly once (the `visited` check ensures this).
For each node, we iterate through its adjacency list. The total work across all nodes is
the sum of all adjacency list lengths, which is 2E for undirected graphs (each edge appears
in two lists) or E for directed graphs. Adding the V (for processing each node once) gives
O(V + E).

**Space: O(V).** The visited set is O(V). The queue (BFS) or stack (DFS) can hold at most
O(V) nodes in the worst case. For BFS, the worst case is a "star" graph where one node
connects to all others -- the queue holds V-1 nodes at once. For recursive DFS, the call
stack depth can be O(V) in the worst case (a linear chain graph).

---

## Common Problems

### 1. Is the Graph Connected?

A graph is **connected** if there is a path between every pair of nodes. To check: run BFS
or DFS from any node. If you visit all V nodes, it is connected.

```rust
fn is_connected(adj: &Vec<Vec<usize>>) -> bool {
    if adj.is_empty() {
        return true;
    }
    let visited_count = bfs(adj, 0).len();
    visited_count == adj.len()
}
```

### 2. Count Connected Components

Run BFS/DFS from every unvisited node. Each time you start a new traversal, you have found
a new connected component.

```rust
fn count_components(adj: &Vec<Vec<usize>>) -> usize {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut components = 0;

    for node in 0..n {
        if !visited[node] {
            // BFS or DFS from this node, marking everything reachable
            let mut queue = std::collections::VecDeque::new();
            visited[node] = true;
            queue.push_back(node);

            while let Some(current) = queue.pop_front() {
                for &neighbor in &adj[current] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }

            components += 1;
        }
    }

    components
}
```

Visualizing on a disconnected graph:

```
    0 --- 1     3 --- 4     6
    |           |
    2           5

Component 1: {0, 1, 2}    (start BFS from 0)
Component 2: {3, 4, 5}    (start BFS from 3, the next unvisited node)
Component 3: {6}           (start BFS from 6)
Total: 3 components
```

### 3. Shortest Path in an Unweighted Graph

Already covered above in the BFS section. BFS gives shortest paths in unweighted graphs
because it explores level by level. DFS does **not** give shortest paths -- it might find a
path, but not necessarily the shortest one.

```
    0 --- 1 --- 3
    |           |
    2 --------- +

Shortest path from 0 to 3:
  BFS finds: 0 -> 1 -> 3  (length 2)  -- correct
  DFS might find: 0 -> 2 -> 3  (length 2) or 0 -> 1 -> 3  (length 2) -- correct by luck
  But on other graphs DFS can find a longer path first.
```

Use BFS when you need shortest paths in unweighted graphs. Full stop.

### 4. Cycle Detection

Cycle detection works differently for undirected and directed graphs.

#### Undirected Graphs: DFS with Parent Tracking

In an undirected graph, every edge appears in both adjacency lists: if 0-1 is an edge, then
1 is in `adj[0]` and 0 is in `adj[1]`. When doing DFS from 0 to 1, you will see 0 in 1's
neighbor list. That is not a cycle -- that is just the edge you arrived on. A cycle exists
only if you encounter a *visited* neighbor that is *not* the node you came from.

```rust
fn has_cycle_undirected(adj: &Vec<Vec<usize>>) -> bool {
    let n = adj.len();
    let mut visited = vec![false; n];

    fn dfs(
        node: usize,
        parent: usize,
        adj: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
    ) -> bool {
        visited[node] = true;
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                if dfs(neighbor, node, adj, visited) {
                    return true;
                }
            } else if neighbor != parent {
                return true; // visited neighbor that isn't our parent = cycle
            }
        }
        false
    }

    for node in 0..n {
        if !visited[node] {
            if dfs(node, usize::MAX, adj, &mut visited) {
                return true;
            }
        }
    }

    false
}
```

Step through an example:

```
    0 --- 1
    |     |       <-- This is a cycle: 0-1-2-0
    +--2--+

DFS from 0 (parent = MAX):
  Visit 0. Neighbors: [1, 2]
  -> DFS(1, parent=0):
       Visit 1. Neighbors: [0, 2]
       -> 0 is visited but 0 == parent, skip (just the edge we came on)
       -> 2 is not visited:
          DFS(2, parent=1):
            Visit 2. Neighbors: [0, 1]
            -> 0 is visited and 0 != parent(1). CYCLE FOUND.
```

#### Directed Graphs: DFS with Three-Color Marking

In a directed graph, the "parent" trick does not work. Edge A->B does not imply B->A, so
there is no "edge you came from" to exclude. Instead, you track three states for each node:

- **White (unvisited):** Not yet discovered.
- **Gray (in progress):** Currently on the recursion stack -- we started exploring it but
  have not finished all its descendants.
- **Black (done):** Fully explored, all descendants processed.

A cycle exists if and only if you encounter a **gray** node -- a node that is on the current
path, meaning you have found a path from that node back to itself.

```rust
fn has_cycle_directed(adj: &Vec<Vec<usize>>) -> bool {
    let n = adj.len();
    // 0 = white (unvisited), 1 = gray (in progress), 2 = black (done)
    let mut color = vec![0u8; n];

    fn dfs(node: usize, adj: &Vec<Vec<usize>>, color: &mut Vec<u8>) -> bool {
        color[node] = 1; // gray: entering
        for &neighbor in &adj[node] {
            if color[neighbor] == 1 {
                return true; // back edge to a gray node = cycle
            }
            if color[neighbor] == 0 && dfs(neighbor, adj, color) {
                return true;
            }
        }
        color[node] = 2; // black: done
        false
    }

    for node in 0..n {
        if color[node] == 0 {
            if dfs(node, adj, &mut color) {
                return true;
            }
        }
    }

    false
}
```

Visualizing the three colors:

```
Directed graph:  0 -> 1 -> 2 -> 0   (cycle)

DFS(0):
  color: [G, W, W]        (0 is gray)
  -> DFS(1):
       color: [G, G, W]   (0 and 1 are gray)
       -> DFS(2):
            color: [G, G, G]   (all gray)
            -> neighbor 0 is GRAY. Cycle detected!

Directed graph:  0 -> 1 -> 2    (no cycle)

DFS(0):
  color: [G, W, W]
  -> DFS(1):
       color: [G, G, W]
       -> DFS(2):
            color: [G, G, G]
            -> no neighbors
            color: [G, G, B]   (2 is black -- done)
       color: [G, B, B]        (1 is black)
  color: [B, B, B]             (0 is black)
  No gray neighbors encountered. No cycle.
```

Why does encountering a **black** node not indicate a cycle? Because a black node is fully
processed -- we already explored all its descendants and returned. Finding it again means
we reached it through a different path, not that we are in a loop.

### 5. Bipartite Check (BFS Two-Coloring)

A graph is **bipartite** if you can color every node with one of two colors such that no
two adjacent nodes share the same color. Equivalently: the graph contains no odd-length
cycles.

The algorithm: run BFS and alternate colors at each level. If you ever find an edge between
two nodes of the same color, the graph is not bipartite.

```
Bipartite:                    Not bipartite:
    0 --- 1                       0 --- 1
    |     |                       |     |
    3 --- 2                       2 ----+

    Color 0: A                    Color 0: A
    Color 1: B (neighbor of A)    Color 1: B
    Color 2: A (neighbor of B)    Color 2: A
    Color 3: B (neighbor of A)    But 2 -- 0 and both are A? No:
    Check: all edges go A-B.      0-1: A-B ok. 1-2: B-A ok. 2-0: A-A conflict!
    Bipartite!                    Not bipartite! (odd cycle: 0-1-2-0, length 3)
```

```rust
use std::collections::VecDeque;

/// Returns true if the graph is bipartite.
fn is_bipartite(adj: &Vec<Vec<usize>>) -> bool {
    let n = adj.len();
    // -1 = uncolored, 0 = color A, 1 = color B
    let mut color = vec![-1i8; n];

    for start in 0..n {
        if color[start] != -1 {
            continue; // already colored in a previous component
        }

        color[start] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            for &neighbor in &adj[node] {
                if color[neighbor] == -1 {
                    color[neighbor] = 1 - color[node]; // opposite color
                    queue.push_back(neighbor);
                } else if color[neighbor] == color[node] {
                    return false; // same color on both sides of an edge
                }
            }
        }
    }

    true
}
```

Note the outer loop over all nodes. This handles disconnected graphs -- each component must
be independently bipartite for the whole graph to be bipartite.

---

## Topological Sort: A Preview

You may have noticed that DFS finish order has an interesting property on directed acyclic
graphs (DAGs). If there is an edge from A to B, then A finishes *after* B (because A's DFS
call must wait for B's DFS call to complete before it can finish).

Reversing the finish order gives you a **topological ordering**: a linear arrangement of
nodes such that for every directed edge A -> B, A appears before B. This is like scheduling
tasks with dependencies -- you cannot start a task until all its prerequisites are done.

```
  Directed graph:   0 -> 1 -> 3
                    0 -> 2 -> 3

  Finish order:     3, 1, 2, 0   (or 3, 2, 1, 0)
  Reversed:         0, 2, 1, 3   (or 0, 1, 2, 3)

  Both are valid topological orderings.
```

We will cover topological sort in depth in a future lesson. For now, the key insight is:
DFS finish order on a DAG, reversed, gives a valid topological sort. This is one of the
reasons DFS discovery and finish orderings matter.

---

## BFS vs. DFS: When to Use Which

| Situation | Prefer | Why |
|---|---|---|
| Shortest path (unweighted) | **BFS** | BFS explores by distance; first path found is shortest |
| Level-by-level traversal | **BFS** | BFS naturally groups nodes by distance |
| Checking connectivity | **Either** | Both visit all reachable nodes |
| Connected components | **Either** | Both work; DFS is often slightly simpler |
| Cycle detection | **DFS** | Color-based approach is cleaner; BFS can work too |
| Topological sort | **DFS** | Finish order gives the answer directly |
| Bipartite check | **BFS** | Level-based coloring is intuitive; DFS also works |
| Deep/narrow graphs | **BFS** | DFS might stack overflow on deep chains |
| Wide/shallow graphs | **DFS** | BFS queue might use lots of memory on wide levels |

In practice, many problems can be solved with either. Choose the one that maps most
naturally to the problem. When in doubt and you need shortest paths, use BFS. When in doubt
and you need to explore all paths or detect structure (cycles, components, ordering), use
DFS.

---

## Putting It All Together: A Reusable Graph Module

Here is a small self-contained example that builds a graph and runs both traversals:

```rust
use std::collections::VecDeque;

/// Build an undirected adjacency list from a list of edges.
fn build_undirected(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    adj
}

/// Build a directed adjacency list from a list of edges.
fn build_directed(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
    }
    adj
}

fn bfs(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    order
}

fn dfs(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut order = Vec::new();

    fn go(node: usize, adj: &[Vec<usize>], visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[node] = true;
        order.push(node);
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                go(neighbor, adj, visited, order);
            }
        }
    }

    go(start, adj, &mut visited, &mut order);
    order
}

fn main() {
    //   0 --- 1 --- 3
    //   |     |
    //   2     4 --- 5
    let adj = build_undirected(6, &[(0, 1), (0, 2), (1, 3), (1, 4), (4, 5)]);

    println!("BFS from 0: {:?}", bfs(&adj, 0));
    // BFS from 0: [0, 1, 2, 3, 4, 5]

    println!("DFS from 0: {:?}", dfs(&adj, 0));
    // DFS from 0: [0, 1, 3, 4, 5, 2]
}
```

Notice the final module uses `&[Vec<usize>]` instead of `&Vec<Vec<usize>>`. Clippy (Rust's
linter) will suggest this: a slice reference is more general than a `Vec` reference, and it
communicates that the function does not need ownership or the ability to resize. Both work,
but `&[Vec<usize>]` is idiomatic.

---

## Common Mistakes

1. **Forgetting the visited set.** On trees this is fine. On graphs with cycles, you get
   infinite loops. Always have a visited check.

2. **Marking visited too late (BFS).** If you mark a node as visited when you *dequeue* it
   instead of when you *enqueue* it, you might add the same node to the queue multiple
   times. The algorithm still works but wastes memory and time.

3. **Confusing undirected vs. directed cycle detection.** In undirected graphs, the parent
   check is sufficient. In directed graphs, you need the three-color (white/gray/black)
   approach because a "cross edge" to a fully-processed (black) node is not a cycle.

4. **Assuming DFS gives shortest paths.** It does not. DFS finds *a* path, not the shortest.
   Only BFS guarantees shortest paths in unweighted graphs.

5. **Stack overflow on deep recursive DFS.** If the graph has a chain of 100,000+ nodes,
   recursive DFS will blow the call stack. Use the iterative version for large or potentially
   deep graphs.

6. **Forgetting disconnected components.** If the graph is not connected, a single BFS or
   DFS from one node will not visit everything. Loop over all nodes, starting a new traversal
   from each unvisited node.

---

## Key Takeaways

- BFS and DFS on graphs are the same as on trees, plus a **visited set** to handle cycles.
- **BFS** uses a queue, explores level by level, and finds shortest paths in unweighted
  graphs.
- **DFS** uses a stack (or recursion), explores as deep as possible before backtracking, and
  naturally captures discovery/finish ordering.
- Both run in **O(V + E)** time and **O(V)** space.
- BFS is the go-to for shortest path problems on unweighted graphs.
- DFS is the go-to for cycle detection, topological ordering, and connected component
  analysis.
- Most problems can be solved with either -- pick the one that fits the problem structure.
- In Rust, use `VecDeque` for BFS and `Vec` (as a stack) or recursion for DFS. Prefer a
  `Vec<bool>` visited array over `HashSet` when nodes are numbered 0..n-1.

---

## What's Next

This lesson gave you the mechanics of BFS and DFS on general graphs. Next, we will build on
these foundations with topological sort (scheduling dependencies with DFS finish order and
BFS with Kahn's algorithm), and eventually shortest paths on *weighted* graphs (Dijkstra,
Bellman-Ford) where BFS alone is no longer sufficient.
