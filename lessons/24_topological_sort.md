# Lesson 24: Topological Sort

## The Big Idea

You are building a project with `cargo`. Your crate depends on `serde`, which
depends on `serde_derive`, which depends on `proc-macro2`, which depends on
`unicode-ident`. Cargo does not compile them in random order. It figures out
that `unicode-ident` has no dependencies, so it builds that first. Then
`proc-macro2`, then `serde_derive`, then `serde`, then your crate. Each library
is compiled only after everything it depends on is ready.

That is a topological sort: given a set of items with ordering constraints
("A must come before B"), produce a linear sequence that respects every
constraint.

You encounter this pattern everywhere:

- **Course prerequisites.** You cannot take Algorithms until you have taken Data
  Structures. You cannot take Data Structures until you have taken Intro to
  Programming. A valid semester plan is a topological ordering of the course
  dependency graph.
- **Build systems.** `make`, `cargo`, `bazel` -- they all resolve a dependency
  graph into a build order. Compile the leaves first, then work inward.
- **Task scheduling.** "Deploy database migrations before starting the app
  server. Start the app server before running integration tests." Any workflow
  with ordering constraints is a topological sort problem.
- **Spreadsheet evaluation.** Cell B2 contains `=A1+A3`. Cell A3 contains
  `=A1*2`. The spreadsheet must evaluate A1 first, then A3, then B2.

The requirement for all of these: the dependency graph must have **no cycles**.
If A depends on B and B depends on A, there is no valid ordering. The graph must
be a **DAG** -- a Directed Acyclic Graph.

---

## Directed Acyclic Graphs (DAGs)

Before we sort, we need to be precise about what we are sorting.

A **directed graph** is a set of nodes connected by edges that have a direction.
Edge (A -> B) means "A points to B" -- or in our context, "A must come before B"
or "B depends on A."

A **cycle** in a directed graph is a path that leads back to its starting node:
A -> B -> C -> A. If your dependency graph has a cycle, there is no way to order
the nodes -- you would need A before B, B before C, and C before A, which is
impossible.

A **DAG** is a directed graph with no cycles. Every finite DAG has at least one
node with no incoming edges (a "source") and at least one node with no outgoing
edges (a "sink"). This is what makes topological sorting possible.

```
  A directed graph WITH a cycle (not a DAG):

      A ----> B
      ^       |
      |       v
      D <---- C

  A -> B -> C -> D -> A  (cycle!)
  No valid topological ordering exists.


  A DAG (no cycles):

      A ----> B ----> D
      |               ^
      v               |
      C --------------+

  Valid orderings: [A, B, C, D] or [A, C, B, D]
  Both respect all edges.
```

**Key property:** A topological ordering exists if and only if the graph is a
DAG. This means topological sort doubles as a cycle detector -- if the algorithm
cannot process all nodes, the graph contains a cycle.

---

## What Is a Topological Ordering?

A **topological ordering** of a DAG is a linear sequence of all its nodes such
that for every directed edge (U -> V), node U appears before node V in the
sequence.

There is usually more than one valid ordering:

```
      0 ----> 2
      |       ^
      v       |
      1       3

  Edges: 0->1, 0->2, 3->2

  Valid orderings:
    [0, 1, 3, 2]   -- fine: 0 before 1, 0 before 2, 3 before 2
    [0, 3, 1, 2]   -- also fine
    [3, 0, 1, 2]   -- also fine
    [3, 0, 2, 1]   -- also fine

  Many orderings work because 1 and 3 have no constraint between them.
```

---

## Graph Representation

Before writing algorithms, we need to represent the graph in code. We will use
an **adjacency list** -- for each node, we store a list of nodes it points to.
This is the standard representation when the graph is sparse (few edges relative
to nodes), which is the common case for dependency graphs.

```rust
use std::collections::VecDeque;

/// Build an adjacency list from a number of nodes and a list of edges.
/// Each edge (u, v) means u -> v ("u must come before v").
fn build_adjacency_list(num_nodes: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; num_nodes];
    for &(from, to) in edges {
        adj[from].push(to);
    }
    adj
}
```

For a graph with 5 nodes and edges `[(0,1), (0,2), (1,3), (2,3), (3,4)]`:

```
  adj[0] = [1, 2]    -- node 0 points to 1 and 2
  adj[1] = [3]        -- node 1 points to 3
  adj[2] = [3]        -- node 2 points to 3
  adj[3] = [4]        -- node 3 points to 4
  adj[4] = []         -- node 4 points to nothing (sink)

  Visually:

      0 ----> 1
      |       |
      v       v
      2 ----> 3 ----> 4
```

---

## Approach 1: Kahn's Algorithm (BFS-Based)

Kahn's algorithm is the intuitive approach. The idea maps directly onto the
real-world analogy of a build system:

1. Find all nodes with no incoming edges (in-degree 0). These are the "leaves"
   -- items with no dependencies. They can be processed immediately.
2. Process those nodes (add them to the output).
3. Remove their outgoing edges. This may reduce the in-degree of other nodes
   to 0.
4. Repeat until no nodes remain.

If the queue empties before all nodes are processed, the graph has a cycle.

### In-Degree

The **in-degree** of a node is the number of edges pointing *into* it. A node
with in-degree 0 has no prerequisites -- it is ready to go.

### Step-by-Step Walkthrough

```
  Graph:
      0 ----> 1 ----> 3
      |               ^
      v               |
      2 -----> 4 -----+

  Edges: 0->1, 0->2, 1->3, 2->4, 4->3

  Step 0: Compute in-degrees
  +---------+----------+
  | Node    | In-degree|
  +---------+----------+
  |   0     |    0     |  <-- no edges point to 0
  |   1     |    1     |  <-- edge 0->1
  |   2     |    1     |  <-- edge 0->2
  |   3     |    2     |  <-- edges 1->3 and 4->3
  |   4     |    1     |  <-- edge 2->4
  +---------+----------+

  Queue (in-degree 0): [0]
  Output: []

  Step 1: Dequeue 0. Add to output. Decrement neighbors' in-degrees.
    - 1's in-degree: 1 -> 0  => enqueue 1
    - 2's in-degree: 1 -> 0  => enqueue 2

  Queue: [1, 2]
  Output: [0]

  +---------+----------+
  | Node    | In-degree|
  +---------+----------+
  |   1     |    0     |
  |   2     |    0     |
  |   3     |    2     |
  |   4     |    1     |
  +---------+----------+

  Step 2: Dequeue 1. Add to output. Decrement neighbors.
    - 3's in-degree: 2 -> 1  (not 0 yet, do not enqueue)

  Queue: [2]
  Output: [0, 1]

  Step 3: Dequeue 2. Add to output. Decrement neighbors.
    - 4's in-degree: 1 -> 0  => enqueue 4

  Queue: [4]
  Output: [0, 1, 2]

  Step 4: Dequeue 4. Add to output. Decrement neighbors.
    - 3's in-degree: 1 -> 0  => enqueue 3

  Queue: [3]
  Output: [0, 1, 2, 4]

  Step 5: Dequeue 3. Add to output. No neighbors.

  Queue: []
  Output: [0, 1, 2, 4, 3]

  All 5 nodes processed. This is a valid topological ordering.
```

### The Code

```rust
use std::collections::VecDeque;

/// Kahn's algorithm: BFS-based topological sort.
/// Returns Some(ordering) if the graph is a DAG, or None if it contains a cycle.
fn topo_sort_kahn(num_nodes: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    // Build adjacency list and compute in-degrees.
    let mut adj = vec![vec![]; num_nodes];
    let mut in_degree = vec![0usize; num_nodes];

    for &(from, to) in edges {
        adj[from].push(to);
        in_degree[to] += 1;
    }

    // Seed the queue with all nodes that have in-degree 0.
    let mut queue: VecDeque<usize> = VecDeque::new();
    for node in 0..num_nodes {
        if in_degree[node] == 0 {
            queue.push_back(node);
        }
    }

    let mut order = Vec::with_capacity(num_nodes);

    while let Some(node) = queue.pop_front() {
        order.push(node);

        for &neighbor in &adj[node] {
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    // If we processed all nodes, the graph is a DAG.
    // If not, the remaining nodes are part of (or blocked by) a cycle.
    if order.len() == num_nodes {
        Some(order)
    } else {
        None // Cycle detected
    }
}

fn main() {
    // The graph from our walkthrough:
    //   0 -> 1 -> 3
    //   0 -> 2 -> 4 -> 3
    let edges = vec![(0, 1), (0, 2), (1, 3), (2, 4), (4, 3)];

    match topo_sort_kahn(5, &edges) {
        Some(order) => println!("Topological order: {:?}", order),
        None => println!("Cycle detected! No valid ordering."),
    }
    // Output: Topological order: [0, 1, 2, 4, 3]
}
```

**Why it works:** A node is only added to the queue when its in-degree reaches 0
-- meaning all its predecessors have already been processed. This guarantees the
ordering constraint is respected. If a cycle exists, the nodes in the cycle
never reach in-degree 0 (they are waiting on each other forever), so they never
enter the queue.

---

## Approach 2: DFS-Based Topological Sort (Reverse Post-Order)

The DFS approach is less intuitive but equally valid, and in some ways more
elegant. The insight: in a DFS, a node's "finish time" (the moment we are done
exploring all its descendants) gives us the topological order in reverse.

Think of it this way: if A -> B, then when we DFS from A, we will explore B
before finishing A. B finishes first. So in the list of finish times, B appears
before A. Reverse that list, and A appears before B -- exactly what we want.

### The Algorithm

1. Run DFS from every unvisited node.
2. When a node finishes (all its descendants are explored), push it onto a
   stack (or prepend it to a list).
3. The stack (read top to bottom) is the topological ordering.

### Step-by-Step Walkthrough

```
  Same graph:
      0 ----> 1 ----> 3
      |               ^
      v               |
      2 -----> 4 -----+

  Edges: 0->1, 0->2, 1->3, 2->4, 4->3

  DFS from node 0:
    Visit 0
      Visit 1 (neighbor of 0)
        Visit 3 (neighbor of 1)
          3 has no unvisited neighbors -> FINISH 3, push to stack
        Back to 1 -> FINISH 1, push to stack
      Visit 2 (neighbor of 0)
        Visit 4 (neighbor of 2)
          Visit 3 -- already visited, skip
          4 has no more unvisited neighbors -> FINISH 4, push to stack
        Back to 2 -> FINISH 2, push to stack
      Back to 0 -> FINISH 0, push to stack

  Stack (top to bottom): [0, 2, 4, 1, 3]

  Read top to bottom: [0, 2, 4, 1, 3]

  Verify:
    0 before 1? Yes.  0 before 2? Yes.
    1 before 3? Yes.  2 before 4? Yes.  4 before 3? Yes.
    All edges respected. Valid topological ordering.
```

Note this produced a different valid ordering than Kahn's algorithm gave us
([0, 1, 2, 4, 3]). Both are correct -- topological orderings are not unique.

### Cycle Detection in DFS

The DFS approach uses three states for each node:

- **Unvisited** -- not yet discovered.
- **In Progress** -- discovered, currently being explored (on the DFS call stack).
- **Done** -- fully explored, already pushed to the result.

If during DFS we encounter a node that is **In Progress**, we have found a back
edge -- a cycle. The node is an ancestor of the current node in the DFS tree,
and the edge leads back to it.

```
  Graph with cycle:  A -> B -> C -> A

  DFS from A:
    A: In Progress
      B: In Progress
        C: In Progress
          Neighbor A is "In Progress" -- CYCLE DETECTED!
```

### The Code

```rust
/// DFS-based topological sort (reverse post-order).
/// Returns Some(ordering) if the graph is a DAG, or None if it contains a cycle.
fn topo_sort_dfs(num_nodes: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; num_nodes];
    for &(from, to) in edges {
        adj[from].push(to);
    }

    #[derive(Clone, Copy, PartialEq)]
    enum State {
        Unvisited,
        InProgress,
        Done,
    }

    let mut state = vec![State::Unvisited; num_nodes];
    let mut result = Vec::with_capacity(num_nodes);

    /// Returns false if a cycle is detected.
    fn dfs(
        node: usize,
        adj: &[Vec<usize>],
        state: &mut [State],
        result: &mut Vec<usize>,
    ) -> bool {
        state[node] = State::InProgress;

        for &neighbor in &adj[node] {
            match state[neighbor] {
                State::InProgress => return false, // Back edge = cycle
                State::Unvisited => {
                    if !dfs(neighbor, adj, state, result) {
                        return false; // Propagate cycle detection
                    }
                }
                State::Done => {} // Already fully processed, skip
            }
        }

        state[node] = State::Done;
        result.push(node); // Post-order: push when finished
        true
    }

    // Start DFS from every unvisited node (the graph may be disconnected).
    for node in 0..num_nodes {
        if state[node] == State::Unvisited {
            if !dfs(node, &adj, &mut state, &mut result) {
                return None; // Cycle detected
            }
        }
    }

    // Result is in reverse post-order, so reverse it.
    result.reverse();
    Some(result)
}

fn main() {
    let edges = vec![(0, 1), (0, 2), (1, 3), (2, 4), (4, 3)];

    match topo_sort_dfs(5, &edges) {
        Some(order) => println!("Topological order: {:?}", order),
        None => println!("Cycle detected!"),
    }
    // Output: Topological order: [0, 2, 4, 1, 3]
}
```

---

## Kahn's vs DFS: When to Use Which

Both algorithms have the same time complexity: **O(V + E)** where V is the
number of vertices (nodes) and E is the number of edges. Both detect cycles.
So when should you prefer one over the other?

| Property                   | Kahn's (BFS)           | DFS-based              |
|----------------------------|:----------------------:|:----------------------:|
| Time complexity            | O(V + E)               | O(V + E)               |
| Space complexity           | O(V + E)               | O(V + E)               |
| Cycle detection            | Yes (count processed)  | Yes (back edge check)  |
| Iterative implementation   | Natural (queue-based)  | Needs explicit stack   |
| Parallel task scheduling   | Natural (process all in-degree-0 nodes at once) | Not natural |
| Gives lexicographically smallest order | Easy (use min-heap instead of queue) | Harder |
| Simpler to code in interviews | Usually yes          | Slightly trickier      |

**Practical guidance:**

- **Kahn's** is typically easier to reason about in interviews. The in-degree
  tracking maps directly onto "which courses can I take right now?" thinking.
  If the problem asks for a *specific* ordering (like the lexicographically
  smallest), swap the queue for a min-heap.

- **DFS-based** is natural when you are already doing DFS for another reason
  (e.g., exploring a graph's structure) or when you want to detect cycles with
  the three-state model. It is also the foundation for algorithms like finding
  strongly connected components (Tarjan's algorithm).

---

## Complexity Analysis

**Time: O(V + E)**

Both algorithms visit every node exactly once and traverse every edge exactly
once. Kahn's does this through the BFS loop (each node dequeued once, each edge
used to decrement in-degree once). DFS does this through the recursive calls
(each node entered once, each adjacency list scanned once).

**Space: O(V + E)**

The adjacency list itself takes O(V + E). The in-degree array (Kahn's) or state
array (DFS) takes O(V). The queue (Kahn's) or recursion stack (DFS) holds at
most O(V) nodes. Total: O(V + E).

For sparse graphs (E is close to V), this is essentially O(V). For dense graphs
(E approaches V^2), it is O(V^2). Dependency graphs are almost always sparse.

---

## Multiple Valid Orderings

As we saw earlier, a DAG can have many valid topological orderings. The number
depends on the graph's structure. A completely disconnected graph of V nodes
has V! orderings (any permutation works). A single chain 0 -> 1 -> 2 -> ... -> n
has exactly one.

If a problem asks for **all** valid orderings, you need backtracking. If it asks
for a **specific** ordering (e.g., lexicographically smallest), you modify the
algorithm:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// Kahn's algorithm producing the lexicographically smallest topological ordering.
/// Uses a min-heap instead of a plain queue.
fn topo_sort_lex_smallest(num_nodes: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; num_nodes];
    let mut in_degree = vec![0usize; num_nodes];

    for &(from, to) in edges {
        adj[from].push(to);
        in_degree[to] += 1;
    }

    // Min-heap: always pick the smallest available node.
    let mut heap = BinaryHeap::new();
    for node in 0..num_nodes {
        if in_degree[node] == 0 {
            heap.push(Reverse(node)); // Reverse for min-heap behavior
        }
    }

    let mut order = Vec::with_capacity(num_nodes);

    while let Some(Reverse(node)) = heap.pop() {
        order.push(node);
        for &neighbor in &adj[node] {
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                heap.push(Reverse(neighbor));
            }
        }
    }

    if order.len() == num_nodes {
        Some(order)
    } else {
        None
    }
}
```

The heap adds a log(V) factor to each insertion/extraction, making this
O(V log V + E) instead of O(V + E). For most practical purposes this does not
matter.

---

## Cycle Detection via Topological Sort

One of the most useful properties of topological sort: **it fails cleanly on
graphs with cycles.** This gives you a cycle detector for free.

With Kahn's algorithm, if `order.len() < num_nodes` after the BFS finishes,
the unprocessed nodes are involved in or blocked by a cycle. With DFS, hitting
a node in the `InProgress` state is direct evidence of a back edge (a cycle).

```
  Graph with cycle:
      0 ----> 1
      ^       |
      |       v
      3 <---- 2

  Edges: 0->1, 1->2, 2->3, 3->0

  Kahn's: In-degrees are 0:1, 1:1, 2:1, 3:1.
    No node has in-degree 0. Queue starts empty.
    Processed: 0 nodes out of 4. => Cycle detected.

  DFS from node 0:
    0: InProgress -> 1: InProgress -> 2: InProgress -> 3: InProgress
      -> Neighbor 0 is InProgress => CYCLE DETECTED!
```

---

## Common Applications

### Build Systems (cargo, make, bazel)

Every build system is fundamentally a topological sort engine. The "nodes" are
build targets (crates, object files, libraries). The "edges" are dependencies.
The build order is a topological ordering. Cycles in the dependency graph are
build errors.

```
  Cargo resolving dependencies:

  my-app
    |-- serde
    |     |-- serde_derive
    |           |-- proc-macro2
    |                 |-- unicode-ident
    |-- tokio
          |-- mio
          |-- pin-project-lite

  Topological build order (one possibility):
  unicode-ident -> proc-macro2 -> serde_derive -> serde
  -> pin-project-lite -> mio -> tokio -> my-app
```

### Course Planning

Nodes are courses, edges are prerequisites. "Can I complete all courses?" is
the question "does a valid topological ordering exist?" -- which is the question
"is the prerequisite graph a DAG?"

### Task Scheduling

"Task A takes 3 hours, Task B takes 2 hours, B depends on A." Topological sort
gives you the execution order. Combined with task durations and the critical
path method, it gives you the minimum total time -- but that is a topic for
another lesson.

### Package Managers

`npm`, `pip`, `apt` -- they all resolve dependency trees and install packages in
dependency order. Circular dependencies are errors (or handled with special
hacks like lazy loading).

### Spreadsheet Evaluation

Cells reference other cells. Evaluating a spreadsheet means topologically
sorting the cell dependency graph and computing values in that order. A circular
reference ("A1 = B1, B1 = A1") is detected as a cycle.

---

## Interview Problems

### Problem 1: Course Schedule (LeetCode 207)

**Problem:** There are `num_courses` courses labeled 0 to num_courses-1. You are
given a list of prerequisite pairs: `[course, prerequisite]` meaning you must
take `prerequisite` before `course`. Return whether it is possible to finish all
courses.

**Translation:** Is the directed graph (prerequisite -> course) a DAG?

```rust
fn can_finish(num_courses: usize, prerequisites: &[(usize, usize)]) -> bool {
    // Edge direction: prerequisite -> course
    // prerequisites[i] = (course, prereq) means prereq -> course
    let edges: Vec<(usize, usize)> = prerequisites
        .iter()
        .map(|&(course, prereq)| (prereq, course))
        .collect();

    topo_sort_kahn(num_courses, &edges).is_some()
}

fn main() {
    // Course 1 requires course 0. Course 0 has no prereqs.
    assert!(can_finish(2, &[(1, 0)]));

    // Course 1 requires 0, course 0 requires 1 -- cycle!
    assert!(!can_finish(2, &[(1, 0), (0, 1)]));

    println!("All assertions passed.");
}
```

That is the entire solution. The hard part is recognizing that "can finish all
courses" is equivalent to "is the dependency graph a DAG."

### Problem 2: Course Schedule II (LeetCode 210)

**Problem:** Same as above, but return a valid ordering of courses (not just
whether one exists).

**Translation:** Return a topological ordering if one exists.

This is literally what `topo_sort_kahn` already returns.

```rust
fn find_order(num_courses: usize, prerequisites: &[(usize, usize)]) -> Vec<usize> {
    let edges: Vec<(usize, usize)> = prerequisites
        .iter()
        .map(|&(course, prereq)| (prereq, course))
        .collect();

    topo_sort_kahn(num_courses, &edges).unwrap_or_default()
}
```

### Problem 3: Alien Dictionary (LeetCode 269)

**Problem:** Given a sorted list of words in an alien language, derive the
ordering of characters in the alien alphabet.

**Translation:** Build a graph of character ordering constraints from adjacent
words, then topologically sort it.

The key insight: if two adjacent words in the sorted list are "wrt" and "wrf",
they share the prefix "wr" and differ at position 2: 't' vs 'f'. Since the list
is sorted, 't' comes before 'f' in the alien alphabet. That gives us the edge
t -> f.

```rust
use std::collections::{HashMap, HashSet, VecDeque};

fn alien_order(words: &[&str]) -> String {
    // Collect all unique characters.
    let mut in_degree: HashMap<char, usize> = HashMap::new();
    for word in words {
        for ch in word.chars() {
            in_degree.entry(ch).or_insert(0);
        }
    }

    // Build edges by comparing adjacent words.
    let mut adj: HashMap<char, Vec<char>> = HashMap::new();
    for pair in words.windows(2) {
        let w1: Vec<char> = pair[0].chars().collect();
        let w2: Vec<char> = pair[1].chars().collect();

        // Edge case: "abc" before "ab" is invalid (longer word cannot
        // come before its own prefix in a sorted list).
        if w1.len() > w2.len() && w1.starts_with(&w2) {
            return String::new(); // Invalid input
        }

        // Find the first differing character.
        for (a, b) in w1.iter().zip(w2.iter()) {
            if a != b {
                adj.entry(*a).or_default().push(*b);
                *in_degree.entry(*b).or_insert(0) += 1;
                break; // Only the first difference matters
            }
        }
    }

    // Kahn's algorithm over the character graph.
    let mut queue: VecDeque<char> = VecDeque::new();
    for (&ch, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(ch);
        }
    }

    let mut result = String::new();
    while let Some(ch) = queue.pop_front() {
        result.push(ch);
        if let Some(neighbors) = adj.get(&ch) {
            for &next in neighbors {
                let deg = in_degree.get_mut(&next).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    if result.len() == in_degree.len() {
        result
    } else {
        String::new() // Cycle detected
    }
}

fn main() {
    let words = vec!["wrt", "wrf", "er", "ett", "rftt"];
    let order = alien_order(&words);
    println!("Alien alphabet order: {}", order);
    // One valid output: "wertf"
    // Derived edges: t->f (from wrt/wrf), w->e (from wrt/er),
    //                r->t (from er/ett), e->r (from ett/rftt)
}
```

This problem is the trifecta: graph construction + topological sort + cycle
detection. It is a hard problem, but once you see it as "build a graph from
constraints, then topo sort," each piece is straightforward.

---

## Common Mistakes and Edge Cases

**1. Forgetting disconnected components.** A graph might have multiple connected
components. Kahn's handles this naturally (all in-degree-0 nodes are seeded into
the queue). For DFS, you must start DFS from every unvisited node, not just node
0.

**2. Confusing edge direction.** In course prerequisite problems, the edge often
means "prereq -> course" (take prereq first). But the input format varies.
LeetCode 207 gives `[course, prereq]`, so you build the edge as
`prereq -> course`. Read the problem statement carefully.

**3. Self-loops.** An edge from a node to itself (0 -> 0) is a trivial cycle.
Both algorithms detect this: Kahn's sees in-degree never drops to 0, DFS sees
the node is InProgress when revisiting it.

**4. Duplicate edges.** If the input contains `(0, 1)` twice, Kahn's will count
in-degree as 2, meaning node 1 needs two decrements to reach 0. This still
works correctly but is technically wrong (in-degree should be 1). If the
problem can have duplicate edges and you want exact in-degrees, deduplicate
first or use a `HashSet` for adjacency.

**5. Empty graph.** Zero nodes, zero edges: the topological ordering is the
empty list. Both algorithms handle this correctly.

---

## Key Takeaways

1. **Topological sort** produces a linear ordering of nodes in a DAG such that
   for every edge (U -> V), U comes before V. It only works on DAGs.

2. **Kahn's algorithm** (BFS) tracks in-degrees, starts from nodes with no
   incoming edges, and peels them off layer by layer. Intuitive and easy to
   implement.

3. **DFS-based topological sort** uses reverse post-order (finish times). A
   node is pushed to the result when all its descendants are done, then the
   result is reversed.

4. **Cycle detection** comes free with both algorithms. In Kahn's, not all
   nodes being processed means a cycle exists. In DFS, encountering an
   InProgress node means a cycle exists.

5. **O(V + E)** time and space for both algorithms. For the lexicographically
   smallest ordering, use a min-heap in Kahn's at the cost of O(V log V + E).

6. **Multiple valid orderings** exist for most DAGs. The algorithm you use and
   the order you process ties determines which one you get.

7. **The pattern to recognize:** any time a problem talks about ordering
   with constraints, prerequisites, dependencies, or "what comes first," think
   topological sort.

---

## Practice Problems

- **Course Schedule** (LeetCode 207) -- Can you finish all courses? (Cycle
  detection on a directed graph.)
- **Course Schedule II** (LeetCode 210) -- Return a valid course ordering.
  (Topological sort, return empty if cycle.)
- **Alien Dictionary** (LeetCode 269) -- Derive character ordering from sorted
  alien words. (Build graph from constraints + topo sort.)
- **Minimum Height Trees** (LeetCode 310) -- Not strictly topological sort, but
  uses a similar leaf-peeling approach (Kahn's-like BFS inward from leaves of an
  undirected tree).
- **Parallel Courses** (LeetCode 1136) -- Find the minimum number of semesters
  to take all courses. (BFS level-by-level topological sort; the number of BFS
  levels is the answer.)
- **Sequence Reconstruction** (LeetCode 444) -- Check if a sequence is the
  unique shortest supersequence. (Topological sort with uniqueness check: at
  every step, the queue should have exactly one element.)
