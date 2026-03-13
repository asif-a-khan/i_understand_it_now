# Lesson 25: Shortest Path -- Dijkstra's & Bellman-Ford

## The Problem: Finding the Cheapest Route

In [Lesson 22](./22_graph_bfs_dfs.md) we learned to traverse graphs with BFS and DFS. BFS
gives us the shortest path in terms of *number of edges* -- the fewest hops from A to B. But
the real world is not that simple. Roads have different lengths. Network links have different
latencies. Flights have different costs. When edges carry *weights*, "shortest" means
"lowest total weight," and BFS is no longer the right tool.

This lesson covers the **single-source shortest path** (SSSP) problem: given a weighted
graph and a starting vertex, find the minimum-cost path from that vertex to every other
vertex. We will build two algorithms:

1. **Dijkstra's algorithm** -- fast, greedy, uses a priority queue. Works when all edge
   weights are non-negative.
2. **Bellman-Ford algorithm** -- slower but more general. Handles negative edge weights and
   detects negative-weight cycles.

Both algorithms are foundational. Dijkstra's powers GPS navigation, network routing (OSPF),
and game pathfinding. Bellman-Ford underpins distance-vector routing protocols (like RIP) and
serves as the backbone of more advanced algorithms. You will see them everywhere.

---

## Real-World Analogy: GPS Navigation

Imagine you are driving from Austin to Denver. Your GPS knows the road network: cities are
vertices, roads are edges, and each edge has a weight representing driving time in hours.

```
                    [Kansas City]
                   / 6hr       \ 7hr
                  /             \
  [Austin] ---8hr--- [Dallas] ---5hr--- [Oklahoma City]
      \                                      |
       \                                     | 4hr
        \                                    |
         -------12hr-------- [Albuquerque] --6hr-- [Denver]
```

BFS would count *hops*: Austin -> Dallas -> Oklahoma City -> Denver (3 hops). But that
path costs 8 + 5 + 4 = 17 hours... except wait, that does not even reach Denver. Let us
think about it properly. The point is that fewest hops and lowest cost are different
questions. The GPS needs an algorithm that accounts for edge weights.

Dijkstra's algorithm is that GPS. It explores outward from your starting city, always
expanding the *closest unexplored city* (by total driving time from the start), and
progressively discovers the shortest route to every reachable city.

---

## Why BFS Fails on Weighted Graphs

Before we solve the problem, let us see exactly why BFS breaks.

BFS explores vertices in layers: first all vertices 1 edge away, then 2 edges away, and so
on. This guarantees shortest paths only when every edge has the same cost (or equivalently,
cost 1). Watch what happens with weights:

```
  Graph:
        A ---1--- B ---1--- D
        |                   ^
        +--------10---------+

  BFS from A (exploring by hop count):
    Layer 0: A
    Layer 1: B, D       <-- BFS reaches D via the direct edge A->D (1 hop)
    Layer 2: (D already visited)

  BFS says: shortest path to D = A -> D, cost 10
  Actual shortest: A -> B -> D, cost 1 + 1 = 2
```

BFS found D in one hop and marked it as visited. It never considered that the two-hop path
through B is cheaper. BFS minimizes *hops*, not *total weight*. We need a fundamentally
different expansion strategy.

---

## Dijkstra's Algorithm

### The Core Idea

Dijkstra's is a **greedy** algorithm. It maintains a tentative distance to every vertex,
initially infinity for all except the source (which is 0). At each step, it picks the
unvisited vertex with the *smallest tentative distance*, marks it as finalized, and
**relaxes** all its outgoing edges -- checking whether traveling through this vertex offers
a shorter path to any neighbor.

The key insight: once we finalize a vertex (pop it from the priority queue with the smallest
distance), we know that distance is optimal. No future path can improve it, *because all
remaining unvisited vertices have equal or greater tentative distances, and all edge weights
are non-negative*. This is the greedy property that makes Dijkstra's correct.

### Relaxation

"Relaxation" is a term you will see everywhere in shortest path algorithms. It means:

```
  if dist[u] + weight(u, v) < dist[v]:
      dist[v] = dist[u] + weight(u, v)
      parent[v] = u
```

We ask: "Is the known shortest path to `v` longer than going through `u` first?" If yes,
we update. The name comes from the idea of a rubber band being "relaxed" to a shorter
length -- we are tightening our estimate.

### Step-by-Step Example

Let us trace Dijkstra's on this graph, starting from vertex A:

```
  Weighted directed graph:

        A ---4--- B
        |       / |
        2     1   3
        |   /     |
        C ---5--- D ---2--- E
```

Edge list (directed):
- A -> B (4), A -> C (2)
- B -> C (1), B -> D (3)
- C -> B (1), C -> D (5)
- D -> E (2)

**Initialization:**

```
  Vertex:    A    B    C    D    E
  dist:      0    inf  inf  inf  inf
  parent:    -    -    -    -    -
  finalized: no   no   no   no   no

  Priority queue (min-heap): [(0, A)]
```

**Step 1: Pop (0, A) -- finalize A**

Relax edges from A:
- A -> B: dist[B] = min(inf, 0 + 4) = 4. Update. parent[B] = A.
- A -> C: dist[C] = min(inf, 0 + 2) = 2. Update. parent[C] = A.

```
  Vertex:    A    B    C    D    E
  dist:      0    4    2    inf  inf
  parent:    -    A    A    -    -
  finalized: YES  no   no   no   no

  Priority queue: [(2, C), (4, B)]
```

**Step 2: Pop (2, C) -- finalize C**

Relax edges from C:
- C -> B: dist[B] = min(4, 2 + 1) = 3. Update! parent[B] = C.
- C -> D: dist[D] = min(inf, 2 + 5) = 7. Update. parent[D] = C.

```
  Vertex:    A    B    C    D    E
  dist:      0    3    2    7    inf
  parent:    -    C    A    C    -
  finalized: YES  no   YES  no   no

  Priority queue: [(3, B), (4, B), (7, D)]
                    ^new    ^stale -- will be skipped when popped
```

Note: the old entry (4, B) is still in the queue. When we pop it later, we will see that B
is already finalized and skip it. This is the "lazy deletion" approach -- simpler than
decrease-key.

**Step 3: Pop (3, B) -- finalize B**

Relax edges from B:
- B -> C: dist[C] = min(2, 3 + 1) = 2. No improvement. Skip.
- B -> D: dist[D] = min(7, 3 + 3) = 6. Update! parent[D] = B.

```
  Vertex:    A    B    C    D    E
  dist:      0    3    2    6    inf
  parent:    -    C    A    B    -
  finalized: YES  YES  YES  no   no

  Priority queue: [(4, B), (6, D), (7, D)]
                    ^stale          ^stale
```

**Step 4: Pop (4, B) -- B already finalized. Skip.**

**Step 5: Pop (6, D) -- finalize D**

Relax edges from D:
- D -> E: dist[E] = min(inf, 6 + 2) = 8. Update. parent[E] = D.

```
  Vertex:    A    B    C    D    E
  dist:      0    3    2    6    8
  parent:    -    C    A    B    D
  finalized: YES  YES  YES  YES  no

  Priority queue: [(7, D), (8, E)]
                    ^stale
```

**Step 6: Pop (7, D) -- D already finalized. Skip.**

**Step 7: Pop (8, E) -- finalize E**

No outgoing edges from E.

```
  FINAL RESULT:

  Vertex:    A    B    C    D    E
  dist:      0    3    2    6    8
  parent:    -    C    A    B    D
```

**Path reconstruction** (follow parent pointers backward):
- A -> E: E <- D <- B <- C <- A, so path is A -> C -> B -> D -> E, cost 8.
- A -> D: D <- B <- C <- A, so path is A -> C -> B -> D, cost 6.

### Dijkstra's in Rust

Rust's standard library provides `BinaryHeap`, which is a max-heap. For Dijkstra's we need
a min-heap. The standard approach is to wrap entries in `std::cmp::Reverse`. We covered
`BinaryHeap` in [Lesson 18](./18_heaps_priority_queues.md) -- now we put it to work.

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Adjacency list representation: graph[u] contains (neighbor, weight) pairs.
type Graph = Vec<Vec<(usize, u64)>>;

/// Returns (dist, parent) where:
///   dist[v]   = shortest distance from `source` to `v` (u64::MAX if unreachable)
///   parent[v] = previous vertex on the shortest path (usize::MAX if none)
fn dijkstra(graph: &Graph, source: usize) -> (Vec<u64>, Vec<usize>) {
    let n = graph.len();
    let mut dist = vec![u64::MAX; n];
    let mut parent = vec![usize::MAX; n];

    dist[source] = 0;

    // Min-heap of (distance, vertex). Reverse turns BinaryHeap into a min-heap.
    let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, source)));

    while let Some(Reverse((d, u))) = heap.pop() {
        // If we already found a shorter path to u, skip this stale entry.
        if d > dist[u] {
            continue;
        }

        // Relax all outgoing edges from u.
        for &(v, weight) in &graph[u] {
            let new_dist = dist[u] + weight;
            if new_dist < dist[v] {
                dist[v] = new_dist;
                parent[v] = u;
                heap.push(Reverse((new_dist, v)));
            }
        }
    }

    (dist, parent)
}

/// Reconstruct the shortest path from `source` to `target`.
/// Returns None if `target` is unreachable.
fn reconstruct_path(parent: &[usize], source: usize, target: usize) -> Option<Vec<usize>> {
    if parent[target] == usize::MAX && target != source {
        return None; // unreachable
    }

    let mut path = Vec::new();
    let mut current = target;

    while current != usize::MAX {
        path.push(current);
        if current == source {
            break;
        }
        current = parent[current];
    }

    path.reverse();

    if path.first() == Some(&source) {
        Some(path)
    } else {
        None
    }
}

fn main() {
    // Build the example graph:
    //   0=A, 1=B, 2=C, 3=D, 4=E
    let graph: Graph = vec![
        vec![(1, 4), (2, 2)],       // A -> B(4), C(2)
        vec![(2, 1), (3, 3)],       // B -> C(1), D(3)
        vec![(1, 1), (3, 5)],       // C -> B(1), D(5)
        vec![(4, 2)],               // D -> E(2)
        vec![],                      // E -> (nothing)
    ];

    let (dist, parent) = dijkstra(&graph, 0);

    let labels = ["A", "B", "C", "D", "E"];
    for (i, label) in labels.iter().enumerate() {
        let path = reconstruct_path(&parent, 0, i);
        let path_str = match &path {
            Some(p) => p.iter().map(|&v| labels[v]).collect::<Vec<_>>().join(" -> "),
            None => "unreachable".to_string(),
        };
        println!("{}: dist={}, path={}", label, dist[i], path_str);
    }
}
```

Output:

```
A: dist=0, path=A
B: dist=3, path=A -> C -> B
C: dist=2, path=A -> C
D: dist=6, path=A -> C -> B -> D
E: dist=8, path=A -> C -> B -> D -> E
```

### Complexity Analysis

Let V = number of vertices, E = number of edges.

- Each vertex is popped from the heap at most once (stale entries are skipped in O(1)).
- Each edge is relaxed at most once, and each relaxation may push to the heap.
- The heap holds at most E entries (one per relaxation).
- Each push/pop is O(log E). Since E <= V^2, log E <= 2 log V, so O(log E) = O(log V).

Total: **O((V + E) log V)**.

For sparse graphs (E ~ V), this is O(V log V). For dense graphs (E ~ V^2), this is
O(V^2 log V). A dense graph can be handled in O(V^2) with a simple array instead of a heap
(scan for the minimum each time), which is Dijkstra's original formulation.

### Why Dijkstra's Fails with Negative Edges

The greedy property depends on a critical assumption: *once we finalize a vertex, no future
path through unvisited vertices can improve its distance*. This holds only when all edge
weights are non-negative, because visiting more vertices can only add to the path cost.

With negative edges, a longer path through more vertices might actually be cheaper:

```
  Graph with a negative edge:

      A ---1---> B ---(-4)---> C
       \         ^
        2       -2
         \      /
          +--> D

  Edges: A->B (1), A->D (2), D->B (-2), B->C (-4)

  Dijkstra from A:
    Pop A (dist 0).  Relax: dist[B]=1, dist[D]=2.
    Pop B (dist 1).  Finalize B. Relax: dist[C] = 1+(-4) = -3.
    Pop D (dist 2).  Finalize D. Relax: dist[B] = min(1, 2+(-2)) = 0.
                     But B is already finalized! Dijkstra SKIPS this.
    Pop C (dist -3). Finalize C.

    Dijkstra's answer: dist[B] = 1, dist[C] = -3.

    True shortest to B: A -> D -> B = 2 + (-2) = 0.     <-- WRONG (should be 0, got 1)
    True shortest to C: A -> D -> B -> C = 0 + (-4) = -4. <-- WRONG (should be -4, got -3)
```

The core problem: Dijkstra's processes vertices in order of increasing distance. Once a
vertex is finalized, it is never revisited. With negative edges, a vertex popped later
could relax an edge that improves a previously-finalized vertex. Dijkstra's will never
discover this improvement.

The rule is simple: **if your graph has any negative edge weights, do not use Dijkstra's.**
Use Bellman-Ford instead.

---

## Bellman-Ford Algorithm

### The Core Idea

Bellman-Ford takes a fundamentally different approach. Instead of greedily finalizing one
vertex at a time, it **relaxes every edge, repeatedly**. The key insight:

> A shortest path in a graph with V vertices contains at most V-1 edges (otherwise it
> would revisit a vertex, implying a cycle). Therefore, if we relax all edges V-1 times,
> we are guaranteed to have found all shortest paths.

That is it. The entire algorithm:

1. Initialize: dist[source] = 0, all others = infinity.
2. Repeat V-1 times: for every edge (u, v, w), relax it.
3. (Optional) Do one more pass. If any distance improves, a negative-weight cycle exists.

### Why V-1 Rounds?

Think of it this way. After round 1, we have found the shortest path to every vertex
reachable within 1 edge. After round 2, the shortest paths using up to 2 edges. After
round k, the shortest paths using up to k edges. Since no shortest path (without negative
cycles) uses more than V-1 edges, V-1 rounds suffice.

### Step-by-Step Example

```
  Graph with a negative edge (directed):

      A ---6--- B ---(-2)--- D
      |         ^            ^
      |         |            |
      +---3--- C ----4------+

  Edges: (A,B,6), (A,C,3), (C,B,-2), (C,D,4), (B,D,-2)

  Vertices: A=0, B=1, C=2, D=3.  Source = A.
```

**Initialization:**

```
  Vertex:  A    B    C    D
  dist:    0    inf  inf  inf
  parent:  -    -    -    -
```

**Round 1: Relax all edges**

Process edges in order: (A,B,6), (A,C,3), (C,B,-2), (C,D,4), (B,D,-2)

```
  Edge (A,B,6):  dist[B] = min(inf, 0+6)   = 6.   parent[B] = A.
  Edge (A,C,3):  dist[C] = min(inf, 0+3)   = 3.   parent[C] = A.
  Edge (C,B,-2): dist[B] = min(6,  3+(-2)) = 1.   parent[B] = C.
  Edge (C,D,4):  dist[D] = min(inf, 3+4)   = 7.   parent[D] = C.
  Edge (B,D,-2): dist[B] is 1, so dist[D] = min(7, 1+(-2)) = -1. parent[D] = B.

  After round 1:
  Vertex:  A    B    C    D
  dist:    0    1    3    -1
  parent:  -    C    A    B
```

**Round 2: Relax all edges**

```
  Edge (A,B,6):  dist[B] = min(1, 0+6)     = 1.   No change.
  Edge (A,C,3):  dist[C] = min(3, 0+3)     = 3.   No change.
  Edge (C,B,-2): dist[B] = min(1, 3+(-2))  = 1.   No change.
  Edge (C,D,4):  dist[D] = min(-1, 3+4)    = -1.  No change.
  Edge (B,D,-2): dist[D] = min(-1, 1+(-2)) = -1.  No change.

  After round 2:
  Vertex:  A    B    C    D
  dist:    0    1    3    -1
  parent:  -    C    A    B

  No changes in round 2 -- we can stop early!
```

**Round 3 (V-1 = 3): Same result, no changes.**

**Negative cycle check (round V = round 4):**

Relax all edges one more time. No distance improves, so there is no negative-weight cycle.

**Final result:**

```
  Vertex:  A    B    C    D
  dist:    0    1    3    -1
  parent:  -    C    A    B

  Shortest paths:
    A -> A: cost 0
    A -> B: A -> C -> B, cost 3 + (-2) = 1
    A -> C: A -> C, cost 3
    A -> D: A -> C -> B -> D, cost 3 + (-2) + (-2) = -1
```

### Negative-Weight Cycles

A negative-weight cycle is a cycle whose total edge weight is negative. If such a cycle is
reachable from the source, shortest paths are undefined -- you could loop around the cycle
infinitely and drive the cost to negative infinity.

```
  Negative cycle example:

      A ---1--- B
                |
               -1
                |
                v
      D ---(-3)--- C     (D -> C costs -3, forming cycle B->C->D->B)
       \         ^       total: -1 + 2 + (-3) = -2  (negative!)
        2       |
         \     /
          v   /
           B

  Cycle: B -> C -> D -> B, total weight = -1 + 2 + (-3) = -2
```

Bellman-Ford detects this: after V-1 relaxation rounds, do one more pass. If any edge can
still be relaxed (i.e., `dist[u] + weight < dist[v]`), then a negative-weight cycle is
reachable from the source. This detection is one of the main reasons to use Bellman-Ford
even when you expect non-negative weights -- it provides a safety check.

### Bellman-Ford in Rust

```rust
/// An edge in the graph: (from, to, weight).
/// Weight is i64 to support negative edges.
#[derive(Debug, Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    weight: i64,
}

/// Result of Bellman-Ford.
enum BellmanFordResult {
    /// Shortest distances and parent pointers.
    Success {
        dist: Vec<i64>,
        parent: Vec<usize>,
    },
    /// A negative-weight cycle is reachable from the source.
    NegativeCycle,
}

fn bellman_ford(num_vertices: usize, edges: &[Edge], source: usize) -> BellmanFordResult {
    const INF: i64 = i64::MAX / 2; // Use MAX/2 to avoid overflow on addition
    let mut dist = vec![INF; num_vertices];
    let mut parent = vec![usize::MAX; num_vertices];

    dist[source] = 0;

    // Relax all edges V-1 times.
    for round in 0..num_vertices - 1 {
        let mut any_update = false;

        for edge in edges {
            if dist[edge.from] < INF
                && dist[edge.from] + edge.weight < dist[edge.to]
            {
                dist[edge.to] = dist[edge.from] + edge.weight;
                parent[edge.to] = edge.from;
                any_update = true;
            }
        }

        // Early termination: if no distances changed, we are done.
        if !any_update {
            println!("Converged early after {} round(s).", round + 1);
            break;
        }
    }

    // Check for negative-weight cycles: one more round of relaxation.
    for edge in edges {
        if dist[edge.from] < INF
            && dist[edge.from] + edge.weight < dist[edge.to]
        {
            return BellmanFordResult::NegativeCycle;
        }
    }

    BellmanFordResult::Success { dist, parent }
}

fn reconstruct_path(parent: &[usize], source: usize, target: usize) -> Option<Vec<usize>> {
    if parent[target] == usize::MAX && target != source {
        return None;
    }

    let mut path = Vec::new();
    let mut current = target;

    while current != usize::MAX {
        path.push(current);
        if current == source {
            break;
        }
        current = parent[current];
    }

    path.reverse();

    if path.first() == Some(&source) {
        Some(path)
    } else {
        None
    }
}

fn main() {
    let edges = vec![
        Edge { from: 0, to: 1, weight: 6 },   // A -> B
        Edge { from: 0, to: 2, weight: 3 },   // A -> C
        Edge { from: 2, to: 1, weight: -2 },  // C -> B
        Edge { from: 2, to: 3, weight: 4 },   // C -> D
        Edge { from: 1, to: 3, weight: -2 },  // B -> D
    ];

    let labels = ["A", "B", "C", "D"];

    match bellman_ford(4, &edges, 0) {
        BellmanFordResult::Success { dist, parent } => {
            for (i, label) in labels.iter().enumerate() {
                let path = reconstruct_path(&parent, 0, i);
                let path_str = match &path {
                    Some(p) => p.iter()
                        .map(|&v| labels[v])
                        .collect::<Vec<_>>()
                        .join(" -> "),
                    None => "unreachable".to_string(),
                };
                println!("{}: dist={}, path={}", label, dist[i], path_str);
            }
        }
        BellmanFordResult::NegativeCycle => {
            println!("Negative-weight cycle detected!");
        }
    }
}
```

Output:

```
Converged early after 1 round(s).
A: dist=0, path=A
B: dist=1, path=A -> C -> B
C: dist=3, path=A -> C
D: dist=-1, path=A -> C -> B -> D
```

### Complexity Analysis

- **Time: O(V * E).** We do V-1 passes, each relaxing all E edges. The negative cycle
  check adds one more pass of E edges, which does not change the asymptotic bound.
- **Space: O(V)** for the dist and parent arrays, plus O(E) for the edge list (which you
  need regardless).

This is slower than Dijkstra's O((V + E) log V), but the tradeoff is generality: it
handles negative edges and detects negative cycles.

### Early Termination

Notice the `any_update` flag in the code above. If a full pass over all edges produces no
updates, the algorithm has converged and we can stop early. In many practical cases, this
significantly reduces the number of rounds. Worst case is still V-1 rounds, but average
case can be much better.

---

## Path Reconstruction

Both algorithms maintain a `parent` (or predecessor) array. When we relax an edge
(u, v) and improve dist[v], we set `parent[v] = u`. After the algorithm completes, we
reconstruct the path by following parent pointers backward from the target to the source:

```
  To find the path from source S to target T:

      path = [T]
      current = T
      while current != S:
          current = parent[current]
          path.push(current)
      path.reverse()

  Example: parent = [-, C, A, B] for vertices [A, B, C, D]
  Path to D: D -> parent[D]=B -> parent[B]=C -> parent[C]=A = source. Stop.
  Reverse: A -> C -> B -> D
```

This reconstruction is O(V) in the worst case (the path visits every vertex). The
`reconstruct_path` function in both Rust implementations above follows this pattern.

A common mistake: forgetting to check for unreachable vertices. If `dist[target]` is still
infinity (or `parent[target]` is unset and target is not the source), there is no path.

---

## Comparison: When to Use Which

```
  ┌─────────────────────┬──────────────────────┬──────────────────────┐
  │                     │  Dijkstra's          │  Bellman-Ford        │
  ├─────────────────────┼──────────────────────┼──────────────────────┤
  │ Time complexity     │ O((V+E) log V)       │ O(V * E)            │
  │                     │ with binary heap      │                     │
  ├─────────────────────┼──────────────────────┼──────────────────────┤
  │ Space complexity    │ O(V + E)             │ O(V + E)            │
  ├─────────────────────┼──────────────────────┼──────────────────────┤
  │ Negative edges?     │ NO                   │ YES                 │
  ├─────────────────────┼──────────────────────┼──────────────────────┤
  │ Negative cycle      │ No (undefined        │ YES -- detects them │
  │ detection?          │ behavior)            │                     │
  ├─────────────────────┼──────────────────────┼──────────────────────┤
  │ Graph type          │ Directed or          │ Directed (can handle│
  │                     │ undirected           │ undirected by adding │
  │                     │                      │ edges both ways)     │
  ├─────────────────────┼──────────────────────┼──────────────────────┤
  │ Data structure      │ Priority queue       │ Edge list            │
  │                     │ (binary heap)        │ (simple array/vec)   │
  ├─────────────────────┼──────────────────────┼──────────────────────┤
  │ Implementation      │ Moderate (need       │ Simple (nested       │
  │ complexity          │ min-heap, adjacency  │ loops over edges)    │
  │                     │ list)                │                     │
  ├─────────────────────┼──────────────────────┼──────────────────────┤
  │ Best for            │ Road networks, GPS,  │ Financial systems,   │
  │                     │ network routing,     │ currency arbitrage,  │
  │                     │ game pathfinding     │ graphs with negative │
  │                     │                      │ weights              │
  └─────────────────────┴──────────────────────┴──────────────────────┘
```

**Decision flowchart:**

```
  Need shortest path from single source?
      │
      ├── All edges non-negative?
      │       │
      │       ├── YES --> Use Dijkstra's (faster)
      │       │
      │       └── NO  --> Use Bellman-Ford
      │                     │
      │                     └── Need to detect negative cycles?
      │                             │
      │                             └── YES --> Bellman-Ford has you covered
      │
      └── Need all-pairs shortest path? --> See Floyd-Warshall below
```

In practice, the vast majority of shortest-path problems you will encounter (LeetCode,
systems programming, game dev) have non-negative weights. Dijkstra's is your default.
Reach for Bellman-Ford when negative weights are explicitly part of the problem or when
you need cycle detection.

---

## Brief Note: Floyd-Warshall (All-Pairs Shortest Path)

Both Dijkstra's and Bellman-Ford solve the **single-source** problem: shortest paths from
one vertex to all others. Sometimes you need shortest paths between *every pair* of
vertices. You could run Dijkstra's from each vertex (O(V * (V+E) log V)), but there is a
cleaner approach.

**Floyd-Warshall** computes all-pairs shortest paths in O(V^3) time and O(V^2) space using
dynamic programming. The idea: for each intermediate vertex k, check whether the path from
i to j through k is shorter than the current best.

```rust
/// Floyd-Warshall: all-pairs shortest path.
/// `adj` is a V x V adjacency matrix. adj[i][j] = weight of edge i->j,
/// or i64::MAX/2 if no edge exists. adj[i][i] = 0.
fn floyd_warshall(adj: &mut Vec<Vec<i64>>) {
    let v = adj.len();
    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                if adj[i][k] + adj[k][j] < adj[i][j] {
                    adj[i][j] = adj[i][k] + adj[k][j];
                }
            }
        }
    }
    // After this, adj[i][j] is the shortest distance from i to j.
    // If adj[i][i] < 0 for any i, a negative cycle exists.
}
```

Floyd-Warshall is elegant and easy to implement but O(V^3), so it is practical only for
small graphs (up to ~1000 vertices). It handles negative edges but not negative cycles (it
will produce incorrect results if one exists, though you can detect them by checking the
diagonal). We will not go deeper here -- this is a topic for a future lesson.

---

## Common Pitfalls

### 1. Integer Overflow in Distance Initialization

When using `u64::MAX` or `i64::MAX` as "infinity," adding any weight causes overflow. The
Bellman-Ford implementation above uses `i64::MAX / 2` and guards against this with
`if dist[edge.from] < INF`. Always think about this.

### 2. Forgetting to Handle Unreachable Vertices

If a vertex is not reachable from the source, its distance remains infinity. Your path
reconstruction must check for this, or you will follow garbage parent pointers into an
infinite loop.

### 3. Using Dijkstra's on Graphs with Negative Edges

It might *seem* to work on small test cases and then silently produce wrong answers on
larger inputs. If your problem mentions negative weights, switch to Bellman-Ford.

### 4. Undirected Graphs with Negative Edges

An undirected negative edge between u and v means you can bounce back and forth
(u -> v -> u -> v -> ...) forever, creating an infinite negative cycle. Bellman-Ford will
correctly detect this, but the shortest path is undefined. Negative edges in undirected
graphs almost always imply negative cycles.

### 5. Dense vs. Sparse Graph Choice

For dense graphs (E close to V^2), the simple O(V^2) version of Dijkstra (scanning an
array instead of using a heap) outperforms the heap-based version because the heap
operations' log factor becomes overhead. For sparse graphs (E close to V), the heap version
wins. Most graph problems in practice and in interviews are sparse.

---

## Practical Applications

**Dijkstra's:**
- GPS and mapping services (Google Maps, Waze) -- road networks have non-negative distances
- Network routing protocols (OSPF uses a variant of Dijkstra's)
- Game AI pathfinding (often combined with A*, which extends Dijkstra's with a heuristic)
- Social network "degrees of separation"

**Bellman-Ford:**
- Currency exchange arbitrage detection (negative cycle = profit opportunity)
- Distance-vector routing protocols (RIP)
- Any graph problem where edge weights can be negative (e.g., reward/penalty models)
- As a subroutine in Johnson's algorithm (all-pairs shortest path for sparse graphs)

---

## Summary

The single-source shortest path problem is one of the most fundamental problems in graph
theory. Two algorithms cover the vast majority of cases:

1. **Dijkstra's** -- greedy, fast, requires non-negative weights. Use a min-heap (in Rust,
   `BinaryHeap<Reverse<(cost, node)>>`). O((V+E) log V).

2. **Bellman-Ford** -- brute-force relaxation, slower, but handles negative edges and
   detects negative cycles. O(V * E).

Both track shortest distances and predecessor pointers. Path reconstruction follows
predecessors backward from target to source.

The heap-based Dijkstra's from this lesson builds directly on the priority queue concepts
from [Lesson 18](./18_heaps_priority_queues.md), and the graph representation builds on
the adjacency list concepts from [Lesson 22](./22_graph_bfs_dfs.md). These pieces compose
naturally -- the sign of a well-designed curriculum.

---

## Exercises

1. **Warm-up**: Modify the Dijkstra's implementation to work with undirected graphs. (Hint:
   add edges in both directions when building the adjacency list.)

2. **Path counting**: Extend Dijkstra's to also count the *number* of distinct shortest
   paths from source to each vertex.

3. **Flight network**: Given a list of flights `(origin, destination, cost)`, find the
   cheapest route from city A to city B. Implement using Dijkstra's.

4. **Negative cycle detection**: Construct a graph with a negative-weight cycle. Run
   Bellman-Ford and verify it detects the cycle. Then try running Dijkstra's on the same
   graph and observe the incorrect result.

5. **Constrained shortest path**: Find the shortest path from source to target that uses
   at most K edges. (Hint: Bellman-Ford naturally solves this -- just stop after K rounds
   instead of V-1.)

6. **Floyd-Warshall implementation**: Implement Floyd-Warshall with path reconstruction
   (track a `next` matrix so you can reconstruct the actual path between any pair).
