# Lesson 21: Graph Representations

## The Big Idea

Every data structure you have worked with so far imposes a specific shape on data.
Arrays are linear. Trees are hierarchical. But the real world is full of relationships
that are neither -- relationships where anything can connect to anything else, with no
single root, no required ordering, and no restriction on who links to whom.

Think about:

- **A social network.** People are entities. Friendships are connections between them.
  Alice might be friends with Bob and Carol. Bob might be friends with Carol too. There
  is no hierarchy -- it is just a web of relationships.
- **A road map.** Cities are entities. Roads are connections. Some roads are one-way
  (directed). Some have distances or travel times (weighted). You can get from city A
  to city B through many different paths.
- **The internet.** Routers are entities. Cables between them are connections. Data
  packets hop from router to router trying to find a path to the destination.
- **Task dependencies.** Tasks are entities. "Task A must finish before Task B can
  start" is a directed connection. If you have ever used a build system like `make`
  or `cargo`, it builds a graph of crate dependencies to figure out the compilation
  order.

All of these are **graphs**. A graph is the most general-purpose data structure for
modeling pairwise relationships. Trees, linked lists, and even arrays can be thought
of as special cases of graphs with extra constraints. In this lesson we focus on how
to represent graphs in memory -- the data structure side -- so that later lessons can
build algorithms (BFS, DFS, Dijkstra, etc.) on top of them.

---

## What Is a Graph?

A graph G is defined by two sets:

- **V** -- a set of **vertices** (also called **nodes**). These are the things.
- **E** -- a set of **edges** (also called **arcs** or **links**). These are the
  connections between things.

Each edge connects exactly two vertices. We write an edge between vertex u and vertex
v as `(u, v)`.

```
  Vertices: {0, 1, 2, 3, 4}

  Edges:    {(0,1), (0,2), (1,2), (1,3), (3,4)}

  Visual:

        0 --- 1 --- 3
        |   / |     |
        |  /  |     |
        | /   |     |
        2     |     4
              |
         (1,2 edge shown
          via the diagonal)
```

That is the whole definition. Vertices plus edges. Everything else -- directed vs
undirected, weighted vs unweighted, sparse vs dense -- is just a variation on this
core idea.

### Terminology Table

| Term | Definition | Example from diagram above |
|------|-----------|---------------------------|
| **Vertex / Node** | An element in the graph | `0`, `1`, `2`, `3`, `4` |
| **Edge** | A connection between two vertices | `(0, 1)`, `(3, 4)` |
| **Adjacent** | Two vertices connected by an edge | `0` and `1` are adjacent |
| **Neighbor** | A vertex adjacent to a given vertex | Neighbors of `1`: `{0, 2, 3}` |
| **Degree** | Number of edges incident to a vertex | Degree of `1` = 3 |
| **Path** | A sequence of vertices where each consecutive pair is connected by an edge | `0 -> 1 -> 3 -> 4` |
| **Cycle** | A path that starts and ends at the same vertex | `0 -> 1 -> 2 -> 0` |
| **Connected** | There exists a path between two vertices | `0` and `4` are connected |

---

## Directed vs. Undirected

In an **undirected** graph, edges have no direction. If there is an edge between u and
v, you can traverse it from u to v *and* from v to u. Friendships work this way -- if
Alice is friends with Bob, Bob is friends with Alice.

In a **directed** graph (digraph), each edge has a direction. An edge `(u, v)` means
you can go from u to v, but not necessarily from v to u. Think of Twitter follows:
Alice follows Bob, but Bob might not follow Alice.

```
  Undirected:                Directed:

    0 --- 1                    0 ---> 1
    |   / |                    |    / |
    |  /  |                    v   /  v
    2     3                    2 <-   3

  Edge (0,1) means             Edge (0,1) means only 0->1
  both 0->1 and 1->0           Edge (1,2) means only 1->2
                                Edge (0,2) means only 0->2
                                Edge (1,3) means only 1->3
```

### Degree in Directed Graphs

In an undirected graph, the **degree** of a vertex is simply the count of edges
touching it. In a directed graph, we split this into two:

- **In-degree**: number of edges pointing *into* the vertex.
- **Out-degree**: number of edges pointing *out of* the vertex.

From the directed graph above:
- Vertex 0: in-degree = 0, out-degree = 2 (edges to 1 and 2)
- Vertex 1: in-degree = 1, out-degree = 2 (edges to 2 and 3)
- Vertex 2: in-degree = 2, out-degree = 0
- Vertex 3: in-degree = 1, out-degree = 0

In-degree 0 means nothing points to this vertex -- it is a "source." Out-degree 0
means it points to nothing -- it is a "sink."

---

## Weighted vs. Unweighted

Sometimes edges carry a value. A road between two cities has a distance. A network
link has a bandwidth. A flight has a cost. When edges have values, the graph is
**weighted**.

```
  Weighted undirected graph (distances between cities):

        A ---5--- B ---2--- D
        |       / |
        3     7   4
        |   /     |
        C ---8--- E

  Edge (A,B) has weight 5
  Edge (A,C) has weight 3
  Edge (B,C) has weight 7
  Edge (B,D) has weight 2
  Edge (B,E) has weight 4
  Edge (C,E) has weight 8
```

When no weights are given, you can think of every edge as having weight 1 (or simply
"exists / does not exist").

---

## Connected Components

A **connected component** is a maximal set of vertices such that there is a path
between every pair. If a graph has more than one connected component, it means the
graph is "split" -- some vertices are completely unreachable from others.

```
  Two connected components:

    Component 1:        Component 2:

      0 --- 1               4 --- 5
      |   /                       |
      2                           6

  There is no edge connecting any vertex in {0,1,2} to any vertex in {4,5,6}.
```

In a directed graph, the concept splits into *strongly connected components* (every
vertex reachable from every other via directed paths) and *weakly connected components*
(connected if you ignore edge directions). We will revisit this in later lessons.

---

## Cycles

A **cycle** is a path that starts and ends at the same vertex without repeating any
other vertex. A graph with no cycles is called **acyclic**.

- A tree is a connected acyclic undirected graph.
- A **DAG** (Directed Acyclic Graph) is a directed graph with no cycles. Build systems,
  spreadsheet cell dependencies, and course prerequisite chains are all DAGs.

```
  Has a cycle (0 -> 1 -> 2 -> 0):       DAG (no cycles):

      0 ---> 1                            0 ---> 1
      ^     /                                    |
       \   v                              2 ---> 3
        2                                        |
                                                 v
                                                 4
```

---

## Sparse vs. Dense

This distinction matters a lot for choosing a representation.

A graph with V vertices can have at most V*(V-1)/2 edges (undirected) or V*(V-1)
edges (directed). That maximum is O(V^2).

- **Dense graph**: the number of edges E is close to V^2. Example: a social network
  where almost everyone is friends with everyone else (a small tight-knit group).
- **Sparse graph**: E is much closer to V (or even less). Example: a road map where
  each city connects to maybe 3-5 neighboring cities, regardless of how many cities
  there are total.

Most real-world graphs are sparse. The internet has billions of nodes but each node
connects to a small number of neighbors. Social networks have millions of users but
the average person has hundreds of friends, not millions. This sparsity drives the
choice of representation.

---

## Representation 1: Adjacency List

The adjacency list is the most common representation. For each vertex, you store a
list of its neighbors.

Let us use this undirected graph as a running example:

```
        0 --- 1 --- 3
        |   /       |
        |  /        |
        2           4
```

Edges: (0,1), (0,2), (1,2), (1,3), (3,4)

### The Adjacency List View

```
  Vertex  |  Neighbors
  --------|------------
    0     |  [1, 2]
    1     |  [0, 2, 3]
    2     |  [0, 1]
    3     |  [1, 4]
    4     |  [3]
```

Each vertex maps to a list of the vertices it connects to. For an undirected graph,
if (u, v) is an edge, v appears in u's list *and* u appears in v's list.

### Rust Implementation: Unweighted Adjacency List

The simplest adjacency list in Rust is `Vec<Vec<usize>>` -- a vector of vectors, where
`adj[u]` contains the neighbors of vertex u.

```rust
/// Build an undirected unweighted graph from a list of edges.
/// `n` is the number of vertices (labeled 0..n-1).
fn build_undirected(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u); // undirected: add both directions
    }
    adj
}

fn main() {
    let edges = vec![(0, 1), (0, 2), (1, 2), (1, 3), (3, 4)];
    let graph = build_undirected(5, &edges);

    // Print the adjacency list
    for (vertex, neighbors) in graph.iter().enumerate() {
        println!("{}: {:?}", vertex, neighbors);
    }
    // Output:
    // 0: [1, 2]
    // 1: [0, 2, 3]
    // 2: [0, 1]
    // 3: [1, 4]
    // 4: [3]
}
```

For a **directed** graph, you only add the edge in one direction:

```rust
fn build_directed(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v); // only u -> v, not v -> u
    }
    adj
}
```

### Rust Implementation: Weighted Adjacency List

When edges carry weights, each neighbor entry becomes a tuple `(neighbor, weight)`.
The type is `Vec<Vec<(usize, i64)>>`.

```rust
/// Build a weighted directed graph from edges of the form (u, v, weight).
fn build_weighted_directed(
    n: usize,
    edges: &[(usize, usize, i64)],
) -> Vec<Vec<(usize, i64)>> {
    let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
    }
    adj
}

/// Build a weighted undirected graph.
fn build_weighted_undirected(
    n: usize,
    edges: &[(usize, usize, i64)],
) -> Vec<Vec<(usize, i64)>> {
    let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }
    adj
}

fn main() {
    // Weighted directed edges: (from, to, weight)
    let edges = vec![(0, 1, 5), (0, 2, 3), (1, 2, 7), (1, 3, 2), (3, 4, 4)];
    let graph = build_weighted_directed(5, &edges);

    for (vertex, neighbors) in graph.iter().enumerate() {
        println!("{}: {:?}", vertex, neighbors);
    }
    // Output:
    // 0: [(1, 5), (2, 3)]
    // 1: [(2, 7), (3, 2)]
    // 2: []
    // 3: [(4, 4)]
    // 4: []
}
```

### Adjacency List: Complexity

| Operation | Time | Notes |
|-----------|------|-------|
| Check if edge (u, v) exists | O(degree(u)) | Scan u's neighbor list |
| Iterate all neighbors of u | O(degree(u)) | Direct traversal of `adj[u]` |
| Add edge (u, v) | O(1) | Push to end of `adj[u]` |
| Remove edge (u, v) | O(degree(u)) | Find and remove from `adj[u]` |
| Space | O(V + E) | One entry per vertex, one per edge direction |

### When to Use Adjacency Lists

- **Sparse graphs** (E << V^2). Most real-world graphs. The space is O(V + E)
  instead of O(V^2).
- **When you need to iterate over a vertex's neighbors.** This is the core operation in
  BFS, DFS, Dijkstra, and most graph algorithms. The adjacency list gives you direct
  access.
- **When you do not need to frequently check "does edge (u, v) exist?"** That check
  requires scanning u's neighbor list, which is O(degree(u)). For most traversal
  algorithms, you do not need this -- you iterate neighbors, not query specific edges.

If you need O(1) edge-existence checks *and* want an adjacency list, you can keep a
`HashSet<usize>` per vertex instead of a `Vec<usize>`. This trades some overhead for
O(1) lookups, but in practice the `Vec` version is faster for traversal due to cache
locality.

---

## Representation 2: Adjacency Matrix

An adjacency matrix is a 2D grid of size V x V. The cell at row u, column v indicates
whether there is an edge from u to v (and its weight, if weighted).

### Side-by-Side: Same Graph, Two Representations

Let us look at our example graph both ways:

```
  Graph:                        Adjacency List:        Adjacency Matrix:

    0 --- 1 --- 3                 0: [1, 2]                0  1  2  3  4
    |   /       |                 1: [0, 2, 3]          0 [ .  1  1  .  . ]
    |  /        |                 2: [0, 1]             1 [ 1  .  1  1  . ]
    2           4                 3: [1, 4]             2 [ 1  1  .  .  . ]
                                  4: [3]                3 [ .  1  .  .  1 ]
                                                        4 [ .  .  .  1  . ]

  (. means 0 / no edge)
```

For a **weighted** graph:

```
  Weighted graph:               Adjacency List:        Adjacency Matrix:

    0 --5-- 1 --2-- 3             0: [(1,5), (2,3)]          0    1    2    3    4
    |     /         |             1: [(0,5), (2,7), (3,2)] 0 [  0    5    3    0    0 ]
    3   7           4             2: [(0,3), (1,7)]        1 [  5    0    7    2    0 ]
    |  /            |             3: [(1,2), (4,4)]        2 [  3    7    0    0    0 ]
    2               4             4: [(3,4)]               3 [  0    2    0    0    4 ]
                                                           4 [  0    0    0    4    0 ]

  (0 means no edge; be careful if 0 is a valid weight -- use Option or sentinel values)
```

### Rust Implementation: Adjacency Matrix (Bool)

For an unweighted graph, a `Vec<Vec<bool>>` works:

```rust
fn build_matrix_undirected(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let mut mat = vec![vec![false; n]; n];
    for &(u, v) in edges {
        mat[u][v] = true;
        mat[v][u] = true; // undirected
    }
    mat
}

fn main() {
    let edges = vec![(0, 1), (0, 2), (1, 2), (1, 3), (3, 4)];
    let mat = build_matrix_undirected(5, &edges);

    // Check if edge exists -- O(1)
    println!("Edge (1,3)? {}", mat[1][3]); // true
    println!("Edge (0,4)? {}", mat[0][4]); // false

    // Iterate neighbors of vertex 1
    let neighbors_of_1: Vec<usize> = (0..5).filter(|&v| mat[1][v]).collect();
    println!("Neighbors of 1: {:?}", neighbors_of_1); // [0, 2, 3]
}
```

### Rust Implementation: Adjacency Matrix (Weighted)

For a weighted graph, store the weight directly. Use `0` or `i64::MAX` as a sentinel
for "no edge," or use `Option<i64>` for clarity:

```rust
fn build_weighted_matrix(
    n: usize,
    edges: &[(usize, usize, i64)],
) -> Vec<Vec<Option<i64>>> {
    let mut mat = vec![vec![None; n]; n];
    for &(u, v, w) in edges {
        mat[u][v] = Some(w);
        mat[v][u] = Some(w); // undirected
    }
    mat
}

fn main() {
    let edges = vec![(0, 1, 5), (0, 2, 3), (1, 2, 7), (1, 3, 2), (3, 4, 4)];
    let mat = build_weighted_matrix(5, &edges);

    // Check weight of edge (1, 3)
    match mat[1][3] {
        Some(w) => println!("Edge (1,3) weight: {}", w), // 2
        None => println!("No edge (1,3)"),
    }
}
```

### Adjacency Matrix: Complexity

| Operation | Time | Notes |
|-----------|------|-------|
| Check if edge (u, v) exists | O(1) | Just index `mat[u][v]` |
| Iterate all neighbors of u | O(V) | Must scan entire row u |
| Add edge (u, v) | O(1) | Set `mat[u][v] = true` |
| Remove edge (u, v) | O(1) | Set `mat[u][v] = false` |
| Space | O(V^2) | Always, regardless of edge count |

### When to Use an Adjacency Matrix

- **Dense graphs** (E is close to V^2). The O(V^2) space is not wasted if most cells
  are actually filled.
- **When you need O(1) edge-existence checks.** Some algorithms (like Floyd-Warshall
  for all-pairs shortest paths) need to repeatedly ask "what is the weight from u to v?"
  and the matrix makes this trivial.
- **Small graphs.** If V is small (say, under 1000), the V^2 space is negligible and
  the constant-time lookups are nice.
- **When the algorithm naturally operates on a matrix** (transitive closure,
  Floyd-Warshall, matrix exponentiation for counting paths).

The adjacency matrix is a poor choice for sparse graphs. A graph with 100,000 nodes
and 200,000 edges (common for road networks) would waste a 10-billion-cell matrix when
an adjacency list needs only about 300,000 entries.

---

## Representation 3: Edge List

The simplest representation: just store the edges themselves as a list of pairs (or
triples, if weighted).

```rust
// Unweighted edge list
let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 2), (1, 3), (3, 4)];

// Weighted edge list
let weighted_edges: Vec<(usize, usize, i64)> = vec![
    (0, 1, 5),
    (0, 2, 3),
    (1, 2, 7),
    (1, 3, 2),
    (3, 4, 4),
];
```

That is it. No adjacency structure at all.

### Edge List: Complexity

| Operation | Time | Notes |
|-----------|------|-------|
| Check if edge (u, v) exists | O(E) | Linear scan |
| Iterate all neighbors of u | O(E) | Scan all edges, filter |
| Add edge | O(1) | Push to end |
| Space | O(E) | |

### When to Use an Edge List

- **Kruskal's algorithm** for minimum spanning trees: sort edges by weight, then
  process them in order. An edge list is the natural input.
- **Bellman-Ford** algorithm: iterate over *all edges* V-1 times. An edge list makes
  this trivial.
- **Input format**: many competitive programming problems give you edges as input lines.
  You parse them into an edge list and then convert to an adjacency list for traversal.

You rarely use a raw edge list as your primary representation for graph traversal
algorithms (BFS, DFS, Dijkstra). It is too slow for "give me the neighbors of vertex u."
But it is a useful intermediate form.

---

## Head-to-Head Comparison

```
                    Adjacency List     Adjacency Matrix    Edge List
                    ──────────────     ────────────────    ─────────
  Space              O(V + E)           O(V^2)             O(E)

  Edge exists?       O(degree(u))       O(1)               O(E)

  Neighbors of u     O(degree(u))       O(V)               O(E)

  Add edge           O(1)               O(1)               O(1)

  Remove edge        O(degree(u))       O(1)               O(E)

  Best for           Sparse graphs,     Dense graphs,       Edge-centric
                     traversal          O(1) lookups,       algorithms
                     algorithms         small V             (Kruskal, BF)
```

For the vast majority of graph problems you will encounter, **use an adjacency list**.
It is the right default. Switch to a matrix when V is small or the algorithm calls for
it. Use an edge list when the algorithm processes edges in bulk.

---

## Putting It Together: A Reusable Graph Builder

Here is a slightly more structured approach that you might use as a starting template.
It is not a full-blown graph library -- just enough to be practical.

```rust
/// Unweighted graph using adjacency list.
struct Graph {
    adj: Vec<Vec<usize>>,
    directed: bool,
}

impl Graph {
    fn new(n: usize, directed: bool) -> Self {
        Graph {
            adj: vec![vec![]; n],
            directed,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        if !self.directed {
            self.adj[v].push(u);
        }
    }

    fn neighbors(&self, u: usize) -> &[usize] {
        &self.adj[u]
    }

    fn num_vertices(&self) -> usize {
        self.adj.len()
    }

    fn degree(&self, u: usize) -> usize {
        self.adj[u].len()
    }

    /// Build from an edge list.
    fn from_edges(n: usize, edges: &[(usize, usize)], directed: bool) -> Self {
        let mut g = Graph::new(n, directed);
        for &(u, v) in edges {
            g.add_edge(u, v);
        }
        g
    }
}

fn main() {
    // Undirected graph
    let g = Graph::from_edges(
        5,
        &[(0, 1), (0, 2), (1, 2), (1, 3), (3, 4)],
        false,
    );

    for u in 0..g.num_vertices() {
        println!("{}: {:?} (degree {})", u, g.neighbors(u), g.degree(u));
    }
    // 0: [1, 2] (degree 2)
    // 1: [0, 2, 3] (degree 3)
    // 2: [0, 1] (degree 2)
    // 3: [1, 4] (degree 2)
    // 4: [3] (degree 1)
}
```

And the weighted variant:

```rust
struct WeightedGraph {
    adj: Vec<Vec<(usize, i64)>>,
    directed: bool,
}

impl WeightedGraph {
    fn new(n: usize, directed: bool) -> Self {
        WeightedGraph {
            adj: vec![vec![]; n],
            directed,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, w: i64) {
        self.adj[u].push((v, w));
        if !self.directed {
            self.adj[v].push((u, w));
        }
    }

    fn neighbors(&self, u: usize) -> &[(usize, i64)] {
        &self.adj[u]
    }

    fn from_edges(
        n: usize,
        edges: &[(usize, usize, i64)],
        directed: bool,
    ) -> Self {
        let mut g = WeightedGraph::new(n, directed);
        for &(u, v, w) in edges {
            g.add_edge(u, v, w);
        }
        g
    }
}
```

---

## Reading Graph Input

In competitive programming and many problem sets, graph input comes as lines of text.
A typical format is:

```
5 6            <-- V vertices, E edges
0 1            <-- edge 1
0 2            <-- edge 2
1 2            <-- edge 3
1 3            <-- edge 4
3 4            <-- edge 5
2 4            <-- edge 6
```

Here is how you would parse this into an adjacency list in Rust:

```rust
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line: V E
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let v: usize = parts.next().unwrap().parse().unwrap();
    let e: usize = parts.next().unwrap().parse().unwrap();

    let mut adj = vec![vec![]; v];

    // Next E lines: u v (and optionally w for weighted)
    for _ in 0..e {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let v: usize = parts.next().unwrap().parse().unwrap();

        // Undirected
        adj[u].push(v);
        adj[v].push(u);
    }

    for (i, neighbors) in adj.iter().enumerate() {
        println!("{}: {:?}", i, neighbors);
    }
}
```

For weighted input where each line is `u v w`:

```rust
// Inside the edge-reading loop:
let w: i64 = parts.next().unwrap().parse().unwrap();
adj[u].push((v, w));
adj[v].push((u, w));
```

---

## Common Pitfalls

**1. Forgetting to add both directions for undirected graphs.**

This is the most common bug. If the graph is undirected and you only add `adj[u].push(v)`
without `adj[v].push(u)`, your traversal will miss paths. A BFS from vertex 3 would
never reach vertex 1 even though they are connected.

**2. Off-by-one on vertex numbering.**

Some problems use 1-indexed vertices. If you allocate `vec![vec![]; n]` but vertices go
from 1 to n, you will panic on `adj[n]`. Either allocate `n + 1` entries or subtract 1
from every vertex when reading.

**3. Confusing O(V^2) space with O(V + E) space.**

For a graph with 100,000 vertices and 200,000 edges:
- Adjacency list: ~300,000 entries (200k edges stored in both directions, plus 100k
  vertex headers). A few megabytes.
- Adjacency matrix: 10,000,000,000 cells. About 10 GB for booleans, 80 GB for i64.
  Your program crashes.

Always check the constraints. If V can be up to 10^5 or 10^6, the adjacency matrix
is not an option.

**4. Parallel / duplicate edges.**

If the input gives the same edge (u, v) twice, your adjacency list will have v in
`adj[u]` twice. Whether that matters depends on the algorithm. For BFS/DFS it is usually
harmless (you just visit v redundantly). For algorithms that count edges, it is a bug.
If it matters, deduplicate:

```rust
// After building, sort and deduplicate each neighbor list
for neighbors in adj.iter_mut() {
    neighbors.sort_unstable();
    neighbors.dedup();
}
```

**5. Self-loops.**

An edge (u, u) connects a vertex to itself. Most textbook algorithms assume no self-loops.
If they exist in the input, decide whether to keep or discard them based on the problem.

---

## Quick Reference: Decision Flowchart

```
  Is V small (say, < 1000)?
       |
       +-- Yes --> Do you need O(1) edge lookups?
       |               |
       |               +-- Yes --> Adjacency Matrix
       |               +-- No  --> Adjacency List (still fine)
       |
       +-- No  --> Is the graph dense (E close to V^2)?
                       |
                       +-- Yes --> Adjacency Matrix (if V^2 fits in memory)
                       +-- No  --> Adjacency List
```

When in doubt, default to the adjacency list. It is the workhorse of graph algorithms.

---

## Summary

- A **graph** is a set of vertices connected by edges. It is the most flexible data
  structure for modeling relationships.
- Graphs can be **directed** or **undirected**, **weighted** or **unweighted**.
- **Degree** counts edges at a vertex. Directed graphs split this into in-degree and
  out-degree.
- **Connected components** are maximal groups of mutually reachable vertices.
- A **cycle** is a path that loops back to its start. Graphs without cycles are acyclic.
- **Sparse** graphs (E << V^2) are common in practice; **dense** graphs (E near V^2)
  are the exception.
- The **adjacency list** (`Vec<Vec<usize>>`) is space-efficient O(V + E), great for
  neighbor iteration, and the default choice.
- The **adjacency matrix** (`Vec<Vec<bool>>`) gives O(1) edge lookups but costs O(V^2)
  space. Use it for dense or small graphs, or when the algorithm demands it.
- The **edge list** (`Vec<(usize, usize)>`) is useful as input format and for
  edge-centric algorithms like Kruskal's.
- In Rust, `Vec<Vec<usize>>` for unweighted and `Vec<Vec<(usize, i64)>>` for weighted
  adjacency lists are the bread and butter of graph work.

With the representation layer solid, the next lessons will build on this foundation:
BFS, DFS, shortest paths, topological sort, and more. Every one of them starts with
"given an adjacency list, do X." Now you know how to build that list.
