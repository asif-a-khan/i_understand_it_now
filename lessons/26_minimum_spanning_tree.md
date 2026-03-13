# Lesson 26: Minimum Spanning Trees -- Kruskal's & Prim's

## The Problem: Connect Everything, Minimize Cost

You are a network engineer tasked with connecting five office buildings on a campus with
fiber optic cable. Every pair of buildings *could* be connected, but each link has a
different cost depending on distance, terrain, permits, and existing conduit. You need every
building reachable from every other building -- but you want to spend as little money as
possible on cable.

You do not need a direct link between every pair. You just need *some* path between every
pair. And you want the cheapest set of links that achieves this.

This is the **minimum spanning tree** (MST) problem. It shows up everywhere:

- **Road networks**: connect all towns with roads at minimum total paving cost.
- **Electrical grids**: wire all houses to the grid using the least total copper.
- **Computer networks**: connect all nodes in a LAN with minimum total cable length.
- **Circuit design**: connect pins on a chip with minimum total wire.
- **Clustering**: MST-based clustering groups data points by removing the longest edges.

Two classic algorithms solve it: **Kruskal's** and **Prim's**. Both are greedy, both produce
an optimal solution, and both run in O(E log E) or O(E log V) time. They just approach the
problem from different angles.

---

## Prerequisites

This lesson assumes familiarity with graph representations (adjacency lists, edge lists) from
[Lesson 21](./21_graph_representations.md) and with heaps / priority queues from
[Lesson 18](./18_heaps_priority_queues.md). We will also use a data structure called
**Union-Find** (disjoint set union). This lesson introduces it just enough to implement
Kruskal's -- the next lesson covers Union-Find in full depth.

---

## What Is a Spanning Tree?

Given a connected, undirected graph G with V vertices, a **spanning tree** is a subgraph
that:

1. **Includes all V vertices.**
2. **Is a tree** -- connected and acyclic.
3. **Has exactly V - 1 edges.** (Any connected acyclic graph on V vertices has V - 1 edges.)

A single graph can have many spanning trees. Consider this graph with 4 vertices:

```
    A ---2--- B
    |       / |
    4     3   1
    |   /     |
    C ---5--- D
```

Some spanning trees of this graph (V = 4, so each uses 3 edges):

```
  Tree 1:            Tree 2:            Tree 3:
  A --2-- B          A --2-- B          A       B
  |       |                  |          |       |
  4       1          3       1          4       1
  |       |        /         |          |       |
  C       D      C           D          C --5-- D
  Cost: 2+4+1=7  Cost: 2+3+1=6         Cost: 4+5+1=10
```

All three span every vertex. But their total edge weights differ.

---

## What Makes It "Minimum"?

A **minimum spanning tree** is the spanning tree whose total edge weight is the smallest
among all possible spanning trees of the graph.

For the graph above, let's enumerate the edges:

```
  Edge    Weight
  ─────────────
  B-D       1
  A-B       2
  B-C       3
  A-C       4
  C-D       5
```

The MST uses edges {B-D(1), A-B(2), B-C(3)} for a total weight of **6**. No spanning tree
can do better.

```
  The MST:

    A ---2--- B
              |
        3     1
      /       |
    C         D

  Total weight: 1 + 2 + 3 = 6
```

**Key fact:** if all edge weights are distinct (no ties), the MST is unique. If there are
ties, there may be multiple MSTs, but they all have the same total weight.

---

## Why Greedy Works: The Cut Property (Intuition)

Both Kruskal's and Prim's are greedy algorithms. Greedy algorithms do not always produce
optimal solutions -- so why do they work here?

The answer is the **cut property**. Informally:

> Take any "cut" that divides the vertices into two non-empty groups S and V-S. Among all
> edges that cross this cut (one endpoint in S, one in V-S), the lightest one **must** be in
> every MST (assuming distinct weights).

Think of it this way: if you have two disconnected islands of vertices, the cheapest bridge
between them is always worth building. Using any other bridge would only increase total cost,
because you could swap it for the cheaper one and get a lower-weight spanning tree.

This property guarantees that both Kruskal's and Prim's -- which always pick the cheapest
available edge that makes progress -- will produce an MST.

We will not prove this formally. The intuition is: any time you pick an edge that is NOT the
cheapest across some cut, you can swap it for the cheapest one, reducing total weight. So the
cheapest-across-every-cut strategy cannot be beaten.

---

## Kruskal's Algorithm

### The Idea

Kruskal's builds the MST by processing edges in order of increasing weight. For each edge,
it asks: "Would adding this edge create a cycle?" If no, include it. If yes, skip it.

The algorithm:

1. Sort all edges by weight (ascending).
2. Initialize each vertex as its own component (a forest of isolated nodes).
3. For each edge (u, v, w) in sorted order:
   - If u and v are in **different** components: add this edge to the MST, merge their
     components.
   - If u and v are in the **same** component: skip (adding it would create a cycle).
4. Stop when you have V - 1 edges in the MST (or you run out of edges).

### Why It Works

Each time Kruskal's adds an edge, that edge is the lightest edge crossing the cut between
the two components it connects. The cut property guarantees this edge belongs in the MST.

### Detecting Cycles: Union-Find

The "are u and v in the same component?" question could be answered by running BFS/DFS
each time, but that would be slow (O(V) per query). Instead, we use **Union-Find** (also
called Disjoint Set Union, DSU).

Union-Find supports two operations:

- **find(x)**: return the representative (root) of x's component.
- **union(x, y)**: merge the components containing x and y.

With **path compression** and **union by rank**, both operations run in nearly O(1) amortized
time -- specifically O(alpha(n)), where alpha is the inverse Ackermann function, which is
effectively constant for any practical input size.

The next lesson covers Union-Find in detail: how it works internally, why path compression
and union by rank matter, and the amortized analysis. For now, we just need the interface.

### Step-by-Step Example

Let's trace Kruskal's on this graph:

```
    A ---4--- B ---2--- C
    |       / |         |
    7     6   3         8
    |   /     |         |
    D ---5--- E ---1--- F
```

Edges sorted by weight:

```
  Edge    Weight
  ─────────────
  E-F       1
  B-C       2
  B-E       3
  A-B       4
  D-E       5
  B-D       6
  A-D       7
  C-F       8
```

Vertices: {A, B, C, D, E, F}. We need V-1 = 5 edges.

```
  Step 1: E-F (weight 1)
  E and F are in different components. ADD.
  Components: {A} {B} {C} {D} {E, F}
  MST edges: {E-F}

      A       B       C



      D       E --1-- F


  Step 2: B-C (weight 2)
  B and C are in different components. ADD.
  Components: {A} {B, C} {D} {E, F}
  MST edges: {E-F, B-C}

      A       B --2-- C



      D       E --1-- F


  Step 3: B-E (weight 3)
  B is in {B, C}, E is in {E, F}. Different components. ADD.
  Components: {A} {B, C, E, F} {D}
  MST edges: {E-F, B-C, B-E}

      A       B --2-- C
              |
              3
              |
      D       E --1-- F


  Step 4: A-B (weight 4)
  A is in {A}, B is in {B, C, E, F}. Different components. ADD.
  Components: {A, B, C, E, F} {D}
  MST edges: {E-F, B-C, B-E, A-B}

      A --4-- B --2-- C
              |
              3
              |
      D       E --1-- F


  Step 5: D-E (weight 5)
  D is in {D}, E is in {A, B, C, E, F}. Different components. ADD.
  Components: {A, B, C, D, E, F}
  MST edges: {E-F, B-C, B-E, A-B, D-E}

      A --4-- B --2-- C
              |
              3
              |
      D --5-- E --1-- F

  We have 5 edges (V-1). DONE.
  Total MST weight: 1 + 2 + 3 + 4 + 5 = 15
```

What about the remaining edges?

- B-D (weight 6): B and D are already in the same component. SKIP (would create cycle
  B-E-D-B).
- A-D (weight 7): same component. SKIP.
- C-F (weight 8): same component. SKIP.

### Kruskal's in Rust

```rust
/// Minimal Union-Find (Disjoint Set Union).
/// Full treatment in the next lesson.
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(), // each node is its own parent
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false; // already in the same component
        }
        // Union by rank: attach smaller tree under larger tree's root.
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
        true
    }
}

/// An edge: (weight, vertex_u, vertex_v).
type Edge = (u64, usize, usize);

/// Kruskal's algorithm. Returns the MST edges and total weight.
/// Vertices are numbered 0..num_vertices.
fn kruskal(num_vertices: usize, edges: &mut Vec<Edge>) -> (Vec<Edge>, u64) {
    // Step 1: sort edges by weight.
    edges.sort_unstable();

    let mut uf = UnionFind::new(num_vertices);
    let mut mst: Vec<Edge> = Vec::new();
    let mut total_weight: u64 = 0;

    // Step 2: greedily add edges that do not create cycles.
    for &(w, u, v) in edges.iter() {
        if uf.union(u, v) {
            mst.push((w, u, v));
            total_weight += w;
            if mst.len() == num_vertices - 1 {
                break; // MST is complete
            }
        }
    }

    (mst, total_weight)
}

fn main() {
    // Encode the example graph.
    // Vertices: A=0, B=1, C=2, D=3, E=4, F=5
    let mut edges: Vec<Edge> = vec![
        (4, 0, 1), // A-B
        (2, 1, 2), // B-C
        (3, 1, 4), // B-E
        (6, 1, 3), // B-D
        (7, 0, 3), // A-D
        (5, 3, 4), // D-E
        (1, 4, 5), // E-F
        (8, 2, 5), // C-F
    ];

    let (mst, total) = kruskal(6, &mut edges);

    println!("MST edges:");
    let names = ['A', 'B', 'C', 'D', 'E', 'F'];
    for (w, u, v) in &mst {
        println!("  {}-{} (weight {})", names[*u], names[*v], w);
    }
    println!("Total weight: {total}");

    // Output:
    //   E-F (weight 1)
    //   B-C (weight 2)
    //   B-E (weight 3)
    //   A-B (weight 4)
    //   D-E (weight 5)
    //   Total weight: 15
}
```

### Kruskal's Complexity

| Step | Cost |
|------|------|
| Sort edges | O(E log E) |
| Process each edge (find + union) | O(E * alpha(V)) ~ O(E) |
| **Total** | **O(E log E)** |

Since E <= V^2, log E <= 2 log V, so O(E log E) = O(E log V). Both expressions are used
interchangeably.

**Space:** O(V + E) -- the edge list and the union-find structure.

---

## Prim's Algorithm

### The Idea

While Kruskal's considers edges globally (cheapest edge that does not form a cycle), Prim's
grows the MST from a single starting vertex, one edge at a time. It is reminiscent of
Dijkstra's shortest-path algorithm -- and in fact uses the same priority-queue-driven
structure.

Think of it like building a road network outward from your capital city:

1. Start at any vertex. Mark it as "in the MST."
2. Look at all edges from MST vertices to non-MST vertices. Pick the cheapest one.
3. Add that edge and the new vertex to the MST.
4. Repeat until all vertices are in the MST.

At every step, you are choosing the lightest edge that crosses the cut between "vertices in
the MST so far" and "vertices not yet in the MST." The cut property guarantees this is safe.

### Step-by-Step Example

Same graph as before, starting from vertex A:

```
    A ---4--- B ---2--- C
    |       / |         |
    7     6   3         8
    |   /     |         |
    D ---5--- E ---1--- F
```

```
  Start: MST = {A}, edges from A to non-MST vertices:
    A-B (4), A-D (7)

  Step 1: Cheapest crossing edge: A-B (4). Add B.
  MST = {A, B}

      A --4-- B

  New edges from B: B-C (2), B-E (3), B-D (6)
  Frontier: A-D (7), B-C (2), B-E (3), B-D (6)

  Step 2: Cheapest crossing edge: B-C (2). Add C.
  MST = {A, B, C}

      A --4-- B --2-- C

  New edges from C: C-F (8)
  Frontier: A-D (7), B-E (3), B-D (6), C-F (8)

  Step 3: Cheapest crossing edge: B-E (3). Add E.
  MST = {A, B, C, E}

      A --4-- B --2-- C
              |
              3
              |
              E

  New edges from E: E-F (1), E-D (5)
  Frontier: A-D (7), B-D (6), C-F (8), E-F (1), E-D (5)
  (Note: B-D(6) and A-D(7) are still candidates for reaching D.
   C-F(8) and E-F(1) are both candidates for reaching F.)

  Step 4: Cheapest crossing edge: E-F (1). Add F.
  MST = {A, B, C, E, F}

      A --4-- B --2-- C
              |
              3
              |
              E --1-- F

  Remaining frontier for D: A-D (7), B-D (6), E-D (5)

  Step 5: Cheapest crossing edge to D: E-D (5). Add D.
  MST = {A, B, C, D, E, F}

      A --4-- B --2-- C
              |
              3
              |
      D --5-- E --1-- F

  All vertices included. DONE.
  Total MST weight: 4 + 2 + 3 + 1 + 5 = 15
```

Same result as Kruskal's -- as expected.

### Using a Priority Queue

The "pick the cheapest crossing edge" step is where the priority queue shines. Without it,
you would scan all edges each time -- O(E) per step, O(V * E) total. With a min-heap, you
get O(log E) per operation.

The implementation mirrors Dijkstra's: push candidate edges into a min-heap, pop the
cheapest, and if the destination vertex is not yet in the MST, add it and push its neighbors.

### Prim's in Rust

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// Adjacency list: adj[u] contains (weight, neighbor).
type AdjList = Vec<Vec<(u64, usize)>>;

/// Prim's algorithm. Returns MST edges as (weight, u, v) and total weight.
/// Starts from vertex 0.
fn prim(adj: &AdjList) -> (Vec<(u64, usize, usize)>, u64) {
    let n = adj.len();
    let mut in_mst = vec![false; n];
    let mut mst_edges: Vec<(u64, usize, usize)> = Vec::new();
    let mut total_weight: u64 = 0;

    // Min-heap of (weight, destination, source).
    // source tracks which MST vertex this edge comes from (for recording the edge).
    let mut heap: BinaryHeap<Reverse<(u64, usize, usize)>> = BinaryHeap::new();

    // Start from vertex 0.
    in_mst[0] = true;
    for &(w, neighbor) in &adj[0] {
        heap.push(Reverse((w, neighbor, 0)));
    }

    while let Some(Reverse((w, u, from))) = heap.pop() {
        if in_mst[u] {
            continue; // u is already in the MST; skip stale entry.
        }

        // Add u to the MST via edge (from, u).
        in_mst[u] = true;
        mst_edges.push((w, from, u));
        total_weight += w;

        if mst_edges.len() == n - 1 {
            break; // MST complete
        }

        // Push all edges from u to vertices not yet in the MST.
        for &(w2, neighbor) in &adj[u] {
            if !in_mst[neighbor] {
                heap.push(Reverse((w2, neighbor, u)));
            }
        }
    }

    (mst_edges, total_weight)
}

fn main() {
    // Build adjacency list for the example graph.
    // Vertices: A=0, B=1, C=2, D=3, E=4, F=5
    let mut adj: AdjList = vec![vec![]; 6];

    // Helper: add undirected edge.
    let mut add_edge = |u: usize, v: usize, w: u64| {
        adj[u].push((w, v));
        adj[v].push((w, u));
    };

    add_edge(0, 1, 4); // A-B
    add_edge(1, 2, 2); // B-C
    add_edge(1, 4, 3); // B-E
    add_edge(1, 3, 6); // B-D
    add_edge(0, 3, 7); // A-D
    add_edge(3, 4, 5); // D-E
    add_edge(4, 5, 1); // E-F
    add_edge(2, 5, 8); // C-F

    let (mst, total) = prim(&adj);

    let names = ['A', 'B', 'C', 'D', 'E', 'F'];
    println!("MST edges (Prim's):");
    for (w, u, v) in &mst {
        println!("  {}-{} (weight {})", names[*u], names[*v], w);
    }
    println!("Total weight: {total}");

    // Output:
    //   A-B (weight 4)
    //   B-C (weight 2)
    //   B-E (weight 3)
    //   E-F (weight 1)
    //   E-D (weight 5)
    //   Total weight: 15
}
```

Notice the similarity to the Dijkstra implementation from Lesson 18. The key differences:

- Dijkstra pushes *cumulative distance* from the source. Prim pushes *edge weight* alone.
- Dijkstra finds shortest paths. Prim finds the minimum spanning tree.
- Both use the "lazy deletion" pattern: push entries into the heap and skip stale ones on
  pop, rather than trying to decrease keys in place.

### Prim's Complexity

| Step | Cost |
|------|------|
| Each vertex is added to MST once | V iterations |
| Each edge is pushed to the heap at most twice (once per endpoint) | O(E) pushes |
| Each heap push/pop | O(log E) = O(log V) |
| **Total** | **O((V + E) log V)** |

For connected graphs, E >= V - 1, so this simplifies to **O(E log V)**.

**Space:** O(V + E) -- the adjacency list, the `in_mst` array, and the heap (which can hold
up to O(E) entries).

---

## Kruskal's vs Prim's: When to Use Which

Both algorithms produce the same MST (or an MST with the same total weight if there are
tied edge weights). The choice between them depends on graph density and representation.

```
  Graph density spectrum:

  Sparse graph              Dense graph
  E ~ V                     E ~ V^2
  |<========================>|

  Kruskal's: O(E log E)     Kruskal's: O(V^2 log V)
  Prim's:    O(E log V)     Prim's:    O(V^2 log V)
```

**Prefer Kruskal's when:**
- The graph is **sparse** (E is close to V).
- You already have an **edge list** representation.
- The graph may be **disconnected** -- Kruskal's naturally produces a minimum spanning
  *forest* (one tree per connected component).
- You want a simple implementation and already have Union-Find available.

**Prefer Prim's when:**
- The graph is **dense** (E is close to V^2).
- You already have an **adjacency list** or **adjacency matrix** representation.
- With a Fibonacci heap (not covered here), Prim's achieves O(E + V log V), which beats
  Kruskal's for dense graphs.
- You want to grow the tree from a specific starting vertex.

In practice, for the kinds of graphs you encounter in programming contests and most software
engineering, either algorithm works well. Kruskal's is often simpler to implement if you
already have a Union-Find.

---

## A Larger Worked Example

Let's trace both algorithms on a slightly larger graph to solidify the intuition.

```
        1
    0 ───── 1
    |\      /|
    | 3   2  |
    |  \ /   |
  6 |   X    | 4
    |  / \   |
    | 5   7  |
    |/      \|
    3 ───── 2
        8

  Edges sorted by weight:
    0-1: 1
    1-0 (same): handled as undirected
    0-2: 2
    0-3: 3 (via the diagonal -- wait, let me clarify)
```

Let me use a cleaner graph:

```
      0
     /|\
    1  6  2
   /   |   \
  1    3    2
   \   |   /
    7  5  3
     \   /
      \ /
       4

  Vertex 0 connects to: 1 (wt 1), 2 (wt 2), 3 (wt 6)
  Vertex 1 connects to: 0 (wt 1), 3 (wt 7), 4 (wt 3)
  Vertex 2 connects to: 0 (wt 2), 3 (wt 5), 4 (wt 3)
  Vertex 3 connects to: 0 (wt 6), 1 (wt 7), 2 (wt 5), 4 (wt 4)
  Vertex 4 connects to: 1 (wt 3), 2 (wt 3), 3 (wt 4)
```

**Edges sorted by weight:**

```
  Edge    Weight
  ─────────────
  0-1       1
  0-2       2
  1-4       3
  2-4       3
  3-4       4
  2-3       5
  0-3       6
  1-3       7
```

**Kruskal's trace:**

```
  Edge 0-1 (1): Different components {0} and {1}. ADD.     MST: {0-1}
  Edge 0-2 (2): Different components {0,1} and {2}. ADD.   MST: {0-1, 0-2}
  Edge 1-4 (3): Different components {0,1,2} and {4}. ADD. MST: {0-1, 0-2, 1-4}
  Edge 2-4 (3): 2 and 4 are in SAME component. SKIP.      (would create 0-2-4-1-0 cycle)
  Edge 3-4 (4): Different components {0,1,2,4} and {3}. ADD.
                MST: {0-1, 0-2, 1-4, 3-4}

  4 edges for 5 vertices. DONE.
  Total weight: 1 + 2 + 3 + 4 = 10
```

**Prim's trace (starting from vertex 0):**

```
  MST = {0}. Frontier: 0-1(1), 0-2(2), 0-3(6)

  Pop 0-1(1). Add vertex 1.
    MST = {0, 1}. Push: 1-3(7), 1-4(3)
    Heap: [0-2(2), 1-4(3), 0-3(6), 1-3(7)]

  Pop 0-2(2). Add vertex 2.
    MST = {0, 1, 2}. Push: 2-3(5), 2-4(3)
    Heap: [1-4(3), 2-4(3), 2-3(5), 0-3(6), 1-3(7)]

  Pop 1-4(3). Add vertex 4.
    MST = {0, 1, 2, 4}. Push: 4-3(4)
    Heap: [2-4(3), 4-3(4), 2-3(5), 0-3(6), 1-3(7)]

  Pop 2-4(3). Vertex 4 already in MST. SKIP.

  Pop 4-3(4). Add vertex 3.
    MST = {0, 1, 2, 3, 4}. All vertices included. DONE.

  MST edges: {0-1(1), 0-2(2), 1-4(3), 4-3(4)}
  Total weight: 1 + 2 + 3 + 4 = 10
```

Same total weight. Different edge ordering in the output (Kruskal's processes by global edge
weight; Prim's processes by growth from the start vertex), but the same tree.

---

## Edge Cases and Practical Considerations

### Disconnected Graphs

If the graph is not connected, no spanning tree exists (you cannot span all vertices with a
single tree). Kruskal's handles this gracefully -- it will produce a **minimum spanning
forest**: one MST per connected component. You can detect this by checking if the result has
fewer than V - 1 edges.

Prim's, as written above, will only span the component containing the start vertex. To get a
full forest with Prim's, you would need to restart it from an unvisited vertex for each
component.

### Self-Loops and Parallel Edges

Self-loops (an edge from a vertex to itself) are never useful in an MST -- they contribute
weight but connect nothing new. Skip them.

Parallel edges (multiple edges between the same pair of vertices) are handled naturally: both
algorithms will prefer the lighter one.

### Negative Edge Weights

Both algorithms work correctly with negative edge weights. There is no issue analogous to
Dijkstra's failure with negative weights. The MST problem is about total weight, not path
lengths.

### Integer vs Float Weights

The algorithms work with any ordered weight type. In Rust, be cautious with `f64` because
it does not implement `Ord` (due to `NaN`). If you need float weights, wrap them in a newtype
that implements `Ord`, or use the `ordered_float` crate, or use `total_cmp`.

---

## MST Uniqueness

A useful theoretical property:

> **If all edge weights are distinct, the MST is unique.**

Proof sketch: suppose two distinct MSTs exist. They must differ by at least one edge. Take
the cheapest edge that is in one MST but not the other. Adding it to the second MST creates
a cycle, and in that cycle there must be an edge heavier than it (since weights are distinct
and the algorithm would have preferred the lighter one). Swap them to get a lighter tree --
contradicting the assumption that both were MSTs.

When edge weights have ties, multiple MSTs may exist with the same total weight. Both
Kruskal's and Prim's will find one of them, depending on how ties are broken.

---

## Complete Summary of Complexities

```
  Algorithm   Time              Space     Graph Representation
  ─────────────────────────────────────────────────────────────
  Kruskal's   O(E log E)        O(V + E)  Edge list
  Prim's      O((V+E) log V)    O(V + E)  Adjacency list + binary heap
  Prim's*     O(E + V log V)    O(V + E)  Adjacency list + Fibonacci heap
  ─────────────────────────────────────────────────────────────
  * Fibonacci heap is rarely used in practice due to high constant factors.

  For sparse graphs (E ~ V):
    Kruskal's: O(V log V)
    Prim's:    O(V log V)

  For dense graphs (E ~ V^2):
    Kruskal's: O(V^2 log V)
    Prim's:    O(V^2 log V)  with binary heap
    Prim's:    O(V^2)         with Fibonacci heap (theoretical win)
```

In practice, both are fast enough for all but the most enormous graphs. Choose based on
which representation you already have and which is simpler to implement in your context.

---

## Common Pitfalls

**1. Forgetting that the graph must be undirected.**
MST is defined for undirected graphs. If you have a directed graph, you need the
"minimum spanning arborescence" (Edmonds' algorithm), which is a much harder problem.

**2. Confusing MST with shortest paths.**
The MST minimizes *total edge weight*. It does NOT minimize the path between any two specific
vertices. Dijkstra's solves shortest paths; Kruskal's/Prim's solve MST. Different problems.

**3. Off-by-one on termination.**
An MST on V vertices has exactly V-1 edges. If you get fewer, the graph is disconnected.
If your loop does not terminate at V-1, you will waste time processing unnecessary edges.

**4. Using Prim's without a priority queue.**
Without a heap, Prim's degrades to O(V * E) or O(V^2). Always use a priority queue.

**5. Not handling the `in_mst` check in Prim's.**
The heap may contain stale entries for vertices already added to the MST. Always check
`in_mst[u]` after popping and skip if the vertex is already included. This is the same
"lazy deletion" pattern used in Dijkstra's.

---

## Key Takeaways

1. **A spanning tree** connects all vertices with V-1 edges and no cycles. A **minimum**
   spanning tree does so with the smallest total edge weight.

2. **The cut property** guarantees that greedy edge selection produces an optimal MST. The
   lightest edge crossing any cut must be in the MST (assuming distinct weights).

3. **Kruskal's** sorts all edges and greedily adds them using Union-Find to avoid cycles.
   O(E log E) time. Works naturally with edge lists and disconnected graphs.

4. **Prim's** grows the tree from a start vertex using a priority queue. O((V+E) log V)
   time. Mirrors Dijkstra's structure. Works naturally with adjacency lists.

5. **Both produce the same total weight.** If all edge weights are distinct, the MST is
   unique. If there are ties, both algorithms find a valid MST.

6. **Kruskal's suits sparse graphs; Prim's suits dense graphs.** In practice, both are
   fine for most workloads.

7. **Union-Find** is the key data structure powering Kruskal's. The next lesson covers it
   in detail -- how path compression and union by rank achieve nearly O(1) amortized
   operations.

---

## Exercises

1. **Trace by hand.** Run both Kruskal's and Prim's on this graph and verify they produce
   the same total weight:

   ```
       A ---3--- B ---1--- C
       |         |         |
       5         4         6
       |         |         |
       D ---2--- E ---7--- F
   ```

2. **Implement and test.** Take the Kruskal's and Prim's implementations above, combine them
   in one program, and assert that both produce the same total MST weight on the same graph.

3. **Disconnected graph.** Modify Kruskal's to return a minimum spanning *forest* (a vector
   of trees, one per component). Test on a graph with 3 connected components.

4. **Edge with equal weights.** Construct a graph where multiple MSTs exist (use duplicate
   edge weights). Verify that Kruskal's and Prim's both find a valid MST, even if they
   differ in which edges they select.

5. **Performance comparison.** Generate a random dense graph (V = 1000, E ~ V^2/2) and a
   random sparse graph (V = 10000, E ~ 3V). Time both algorithms on each. Do the results
   match the theoretical predictions about which performs better on sparse vs dense graphs?

6. **Negative weights.** Create a graph with some negative edge weights. Verify that both
   algorithms still produce the correct MST. (Hint: unlike shortest-path algorithms, MST
   algorithms handle negatives just fine.)

<details>
<summary>Hint for Exercise 1</summary>

Edges sorted for Kruskal's: B-C(1), D-E(2), A-B(3), B-E(4), A-D(5), C-F(6), E-F(7).

Process them in order. You need 5 edges for 6 vertices. The MST should have total weight
1 + 2 + 3 + 4 + 6 = 16.

For Prim's starting from A: first add A-B(3), then B-C(1), then from {A,B,C} the cheapest
crossing edge is B-E(4), then D-E(2), then C-F(6). Same total: 16.

</details>

---

*Next up: [Lesson 27 -- Union-Find (Disjoint Set Union)](./27_union_find.md), where we
unpack the data structure that makes Kruskal's efficient: how it works, why path compression
and union by rank are essential, and the surprising amortized analysis behind its nearly-O(1)
operations.*
