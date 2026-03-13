# Lesson 27: Union-Find (Disjoint Set Union)

## The Problem: Who Belongs Together?

Think about a social network with millions of users. Two fundamental questions come up
constantly:

1. **"Are Alice and Bob in the same friend group?"** -- a connectivity query.
2. **"Alice just befriended Bob. Merge their friend groups."** -- a merge operation.

A friend group here means a connected component: if Alice knows Carol and Carol knows Bob,
then Alice and Bob are in the same group even if they have never met directly. As new
friendships form, groups merge. You never need to *split* a group -- friendships, in this
model, are permanent.

You could answer these questions with BFS or DFS: build a graph, run a traversal from Alice,
see if you reach Bob. That works, but it costs O(V + E) per query. If you have millions of
users and millions of queries, that is far too slow.

Union-Find (also called Disjoint Set Union, or DSU) is a data structure purpose-built for
exactly this scenario. It supports two operations:

- **Find(x):** Which group does element x belong to? (Returns a representative for the group.)
- **Union(x, y):** Merge the groups containing x and y into a single group.

With two key optimizations -- **path compression** and **union by rank** -- both operations
run in nearly O(1) amortized time. Not O(log n). Not O(sqrt n). Effectively constant. That
is what makes Union-Find special.

---

## The Core Idea: Trees as Sets

Union-Find represents each group (set) as a **rooted tree**. Every element points to a parent.
The **root** of the tree serves as the representative (or "leader") of the group. Two elements
are in the same group if and only if they have the same root.

Starting state -- everyone is their own group:

```
  Elements: 0  1  2  3  4  5  6  7

  Each element is its own root (parent points to itself):

  [0] [1] [2] [3] [4] [5] [6] [7]

  parent: [0, 1, 2, 3, 4, 5, 6, 7]
           ^  ^  ^  ^  ^  ^  ^  ^
           |  |  |  |  |  |  |  |
           each element is its own parent
```

Eight separate sets, each containing one element.

### Union(1, 2): Merge the groups containing 1 and 2

Make one root point to the other. Let's say 1's root becomes a child of 2's root:

```
  [0] [2] [3] [4] [5] [6] [7]
       |
      [1]

  parent: [0, 2, 2, 3, 4, 5, 6, 7]
              ^
              1 now points to 2
```

### Union(3, 4): Merge 3 and 4

```
  [0] [2] [4] [5] [6] [7]
       |   |
      [1] [3]

  parent: [0, 2, 2, 4, 4, 5, 6, 7]
```

### Union(1, 3): Merge the group {1, 2} with the group {3, 4}

Find the root of 1: follow 1 -> 2. Root is 2.
Find the root of 3: follow 3 -> 4. Root is 4.
Make one root point to the other. Say 2 becomes a child of 4:

```
  [0]   [4]   [5] [6] [7]
        / \
      [2] [3]
       |
      [1]

  parent: [0, 2, 4, 4, 4, 5, 6, 7]
                 ^
                 2 now points to 4
```

### Find(1): Which group is 1 in?

Follow the parent pointers: 1 -> 2 -> 4. The root is 4.

### Connected(1, 3): Are 1 and 3 in the same group?

Find(1) = 4, Find(3) = 4. Same root, so yes.

### Connected(1, 5): Are 1 and 5 in the same group?

Find(1) = 4, Find(5) = 5. Different roots, so no.

---

## Naive Implementation

Here is the simplest possible version. It works, but it has a performance problem we will
fix shortly.

```rust
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        // Each element starts as its own parent (its own group).
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    /// Find the root (representative) of the group containing x.
    fn find(&self, mut x: usize) -> usize {
        while self.parent[x] != x {
            x = self.parent[x];
        }
        x
    }

    /// Merge the groups containing x and y.
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x] = root_y;
        }
    }

    /// Are x and y in the same group?
    fn connected(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
```

This works. `find` follows parent pointers until it reaches a root (a node whose parent is
itself). `union` connects the two roots. `connected` checks if the roots match.

### Why This Is Slow

The problem is that the trees can become **degenerate** -- long chains instead of wide, shallow
trees. Consider this sequence:

```
  union(0, 1):  1 <- 0
  union(1, 2):  2 <- 1 <- 0
  union(2, 3):  3 <- 2 <- 1 <- 0
  union(3, 4):  4 <- 3 <- 2 <- 1 <- 0

  Tree for the group {0, 1, 2, 3, 4}:

  [4]
   |
  [3]
   |
  [2]
   |
  [1]
   |
  [0]
```

This is a linked list, not a tree. `find(0)` must traverse 4 edges. After n unions in this
pattern, `find` is O(n). If you do n finds, total cost is O(n^2). We can do much better.

---

## Optimization 1: Union by Rank

The degenerate chain happens because we blindly attach one root under the other without
considering which tree is "taller." If we always attach the **shorter** tree under the
**taller** tree, the result stays balanced.

We track a **rank** for each root -- an upper bound on the height of its subtree. When
merging two trees:

- If the ranks differ, attach the smaller-rank root under the larger-rank root. The rank of
  the larger root does not change (the tree did not get taller).
- If the ranks are equal, pick either root, and increment the new root's rank by 1.

### Step-by-Step Example

```
  Start: [0] [1] [2] [3] [4] [5]
  rank:   0   0   0   0   0   0

  union(0, 1): ranks equal (0 == 0), attach 0 under 1, rank[1] = 1

       [1]          [2] [3] [4] [5]
        |
       [0]
  rank: 0  1          0   0   0   0

  union(2, 3): ranks equal, attach 2 under 3, rank[3] = 1

       [1]          [3]     [4] [5]
        |            |
       [0]          [2]
  rank: 0  1         0  1    0   0

  union(0, 2): find(0)=1 (rank 1), find(2)=3 (rank 1).
               Ranks equal, attach 1 under 3, rank[3] = 2.

            [3]            [4] [5]
           / \
         [1] [2]
          |
         [0]
  rank:   0  1  0  2        0   0

  union(4, 5): ranks equal, attach 4 under 5, rank[5] = 1

            [3]            [5]
           / \              |
         [1] [2]           [4]
          |
         [0]
  rank:   0  1  0  2        0  1

  union(0, 4): find(0)=3 (rank 2), find(4)=5 (rank 1).
               rank 2 > rank 1, attach 5 under 3.

               [3]
             / | \
           [1][2][5]
            |     |
           [0]   [4]
  rank:    0  1  0  2  0  1
```

Compare this to the naive approach: after 5 unions, the tree has height 2 instead of 5. With
union by rank, the maximum tree height is O(log n).

**Why O(log n)?** A tree with rank r contains at least 2^r nodes (you can prove this by
induction). Since we have n nodes total, the rank is at most log2(n).

---

## Optimization 2: Path Compression

Even with union by rank keeping trees short, `find` still has to walk from a node to the
root every time. Path compression makes a simple observation: once you have found the root
of a node, you might as well point that node (and every node along the way) directly at the
root. Future `find` calls on those nodes will then take O(1).

### Before and After Path Compression

```
  Tree before find(0):

         [4]
          |
         [3]
          |
         [2]
          |
         [1]
          |
         [0]

  find(0) walks the path: 0 -> 1 -> 2 -> 3 -> 4 (root found: 4)

  With path compression, every node on the path now points directly to 4:

         [4]
       / | | \ \
     [3][2][1][0]

  Before:                        After:
  parent: [1, 2, 3, 4, 4]       parent: [4, 4, 4, 4, 4]

  Now find(0), find(1), find(2), find(3) are all O(1).
```

The implementation is a small change to `find`. Instead of just returning the root, we
update every node's parent along the way:

```rust
fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
        self.parent[x] = self.find(self.parent[x]); // path compression
    }
    self.parent[x]
}
```

That is it. One line of recursion that flattens the tree as a side effect of the query.
Note that `find` now takes `&mut self` because it modifies the parent array.

If you prefer an iterative version (avoids stack overflow on very deep trees):

```rust
fn find(&mut self, mut x: usize) -> usize {
    // First pass: find the root.
    let mut root = x;
    while self.parent[root] != root {
        root = self.parent[root];
    }
    // Second pass: compress the path.
    while self.parent[x] != root {
        let next = self.parent[x];
        self.parent[x] = root;
        x = next;
    }
    root
}
```

Both versions achieve the same result. The recursive one is more elegant; the iterative one
is safer for very large inputs.

---

## The Full Implementation

Combining both optimizations:

```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    /// Number of distinct connected components.
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }

    /// Find the root of the group containing x, with path compression.
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Merge the groups containing x and y. Returns false if already connected.
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // already in the same group
        }

        // Union by rank: attach the shorter tree under the taller one.
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }

        self.count -= 1;
        true
    }

    /// Are x and y in the same group?
    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// How many distinct groups remain?
    fn count(&self) -> usize {
        self.count
    }
}
```

A few design choices worth noting:

- **`count` field:** Tracks the number of distinct components. Every successful `union`
  decrements it by 1. This is useful for problems like "number of connected components" where
  you do not care *which* component an element is in, just how many there are.

- **`union` returns `bool`:** Returns `false` if x and y were already connected (no merge
  needed). This is handy for cycle detection -- if `union` returns `false`, the edge (x, y)
  would create a cycle.

- **`find` takes `&mut self`:** Path compression modifies the internal structure. This is a
  common source of friction in Rust -- you cannot call `find` on an immutable reference. In
  practice this is fine; you rarely need shared access to a Union-Find.

---

## Complexity: The Inverse Ackermann Function

With both path compression and union by rank, the amortized cost of m operations (find or
union) on n elements is:

    O(m * alpha(n))

where alpha(n) is the **inverse Ackermann function**. You do not need to understand the
Ackermann function in detail. What matters is this: alpha(n) grows so slowly that for any
input size you will ever encounter in practice -- even if n is the number of atoms in the
observable universe -- alpha(n) <= 5.

In other words, **each find and union is effectively O(1) amortized**. This is not true O(1)
in the mathematical sense, but you will never observe the difference in practice.

Here is the comparison:

```
  n                alpha(n)
  ────────────────────────
  1                0
  2                1
  4                2
  16               3
  65536            4
  2^65536          5       <-- a number with ~20,000 digits

  For any practical n, alpha(n) <= 4.
```

This makes Union-Find one of the most efficient data structures in existence for its use case.

---

## When to Use Union-Find vs. BFS/DFS

Both Union-Find and graph traversals (BFS/DFS) can answer connectivity questions. When should
you use which?

| Scenario | Preferred | Why |
|----------|-----------|-----|
| Static graph, one-time "find all components" | BFS/DFS | Single O(V+E) pass does it all |
| Static graph, many connectivity queries | Either | Precompute components with BFS/DFS, or build UF once |
| **Edges arrive incrementally (online)** | **Union-Find** | Process each edge in ~O(1), no recomputation |
| Need to know the full path between nodes | BFS/DFS | UF only answers "same group?", not "how to get there" |
| Need to split groups | Neither trivially | UF does not support split; BFS/DFS would need recomputation |
| Kruskal's MST | Union-Find | Cycle detection per edge in ~O(1) |
| Detecting a single cycle in undirected graph | Union-Find | Return false on redundant union |

The key insight: **Union-Find excels when edges arrive one at a time and you need to answer
connectivity queries between arrivals.** BFS/DFS excels when you have the full graph upfront
and need structural information (paths, distances, traversal order).

---

## Application 1: Kruskal's MST

You saw Kruskal's algorithm in [Lesson 26](./26_mst.md). Here is the core of it, showing
how Union-Find makes it efficient:

```rust
fn kruskal(n: usize, mut edges: Vec<(usize, usize, i64)>) -> Vec<(usize, usize, i64)> {
    // Sort edges by weight.
    edges.sort_by_key(|&(_, _, w)| w);

    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();

    for (u, v, w) in edges {
        // If u and v are not already connected, this edge is safe to add.
        if uf.union(u, v) {
            mst.push((u, v, w));
            if mst.len() == n - 1 {
                break; // MST complete
            }
        }
        // If union returned false, u and v are already connected.
        // Adding this edge would create a cycle. Skip it.
    }

    mst
}
```

Without Union-Find, you would need BFS/DFS per edge to check for cycles: O(E * (V + E))
total. With Union-Find, each cycle check is ~O(1), so the total is O(E log E) dominated by
the initial sort.

---

## Application 2: Number of Connected Components

Given n nodes and a list of edges, how many connected components are there?

```rust
fn count_components(n: usize, edges: &[(usize, usize)]) -> usize {
    let mut uf = UnionFind::new(n);
    for &(u, v) in edges {
        uf.union(u, v);
    }
    uf.count()
}
```

That is the entire solution. Start with n components. Each successful union merges two
components into one, decrementing the count. After processing all edges, the count tells you
the answer.

---

## Application 3: Cycle Detection in Undirected Graphs

An undirected graph has a cycle if and only if there exists an edge (u, v) where u and v are
already in the same connected component when the edge is encountered.

```rust
fn has_cycle(n: usize, edges: &[(usize, usize)]) -> bool {
    let mut uf = UnionFind::new(n);
    for &(u, v) in edges {
        if !uf.union(u, v) {
            return true; // u and v already connected -- cycle found
        }
    }
    false
}
```

This is exactly what Kruskal's algorithm uses to skip cycle-forming edges. The same idea
stands on its own as a cycle detection method.

Note: this only works for undirected graphs. For directed graphs, cycle detection requires
DFS with a recursion stack (the "gray node" technique).

---

## Application 4: Dynamic Connectivity

Imagine a network of servers. Servers come online and form connections over time. Between
connection events, you receive queries: "Can server A communicate with server B?"

```rust
fn process_network(n: usize, operations: &[(&str, usize, usize)]) {
    let mut uf = UnionFind::new(n);

    for &(op, x, y) in operations {
        match op {
            "connect" => {
                uf.union(x, y);
                println!("Connected {} and {}", x, y);
            }
            "query" => {
                if uf.connected(x, y) {
                    println!("{} and {} can communicate", x, y);
                } else {
                    println!("{} and {} are isolated from each other", x, y);
                }
            }
            _ => {}
        }
    }
}
```

Each operation is ~O(1). BFS/DFS would require rebuilding or re-traversing the graph for
each query, making it far slower for interleaved connect/query workloads.

---

## Interview Problem 1: Number of Provinces (LeetCode 547)

**Problem:** Given an n x n adjacency matrix `is_connected` where `is_connected[i][j] = 1`
means city i and city j are directly connected, find the number of provinces (connected
components).

```rust
fn find_circle_num(is_connected: &[Vec<i32>]) -> usize {
    let n = is_connected.len();
    let mut uf = UnionFind::new(n);

    for i in 0..n {
        for j in (i + 1)..n {
            if is_connected[i][j] == 1 {
                uf.union(i, j);
            }
        }
    }

    uf.count()
}
```

**Time:** O(n^2 * alpha(n)) -- effectively O(n^2), which is the cost of scanning the
adjacency matrix. The Union-Find operations add negligible overhead.

You could also solve this with DFS in O(n^2). Both are fine for this problem. The Union-Find
version is arguably cleaner.

---

## Interview Problem 2: Redundant Connection (LeetCode 684)

**Problem:** A tree with n nodes (labeled 1 to n) has one extra edge, making it no longer
a tree. Find and return the edge that, if removed, would restore it to a tree. If there are
multiple answers, return the one that appears last in the input.

A tree with n nodes has exactly n-1 edges. If we have n edges, exactly one is redundant --
the one that creates a cycle.

```rust
fn find_redundant_connection(edges: &[(usize, usize)]) -> (usize, usize) {
    let n = edges.len();
    let mut uf = UnionFind::new(n + 1); // nodes are 1-indexed

    for &(u, v) in edges {
        if !uf.union(u, v) {
            return (u, v); // this edge creates a cycle
        }
    }

    unreachable!("Problem guarantees a redundant edge exists")
}
```

This is the cycle detection pattern from earlier. The first edge whose `union` returns false
is the cycle-forming edge. Since we process edges in input order, and we want the last such
edge -- wait, actually, in a graph with exactly one extra edge, there is exactly one
cycle-forming edge, so the first (and only) one we encounter is the answer.

**Time:** O(n * alpha(n)), effectively O(n).

---

## Interview Problem 3: Accounts Merge (LeetCode 721)

**Problem:** Given a list of accounts where each account is `[name, email1, email2, ...]`,
merge accounts that share any email address. Two accounts with the same name might belong to
different people, but if they share an email, they are the same person.

This is a classic Union-Find problem. The shared-email relationship is transitive: if account
A shares an email with account B, and B shares an email with C, then A, B, and C are all the
same person.

```rust
use std::collections::HashMap;

fn accounts_merge(accounts: &[Vec<String>]) -> Vec<Vec<String>> {
    let n = accounts.len();
    let mut uf = UnionFind::new(n);

    // Map each email to the first account index that contains it.
    let mut email_to_account: HashMap<&str, usize> = HashMap::new();

    for (i, account) in accounts.iter().enumerate() {
        // account[0] is the name; account[1..] are emails.
        for email in &account[1..] {
            if let Some(&existing) = email_to_account.get(email.as_str()) {
                // This email has been seen in another account. Merge them.
                uf.union(i, existing);
            } else {
                email_to_account.insert(email.as_str(), i);
            }
        }
    }

    // Group all emails by their root account.
    let mut groups: HashMap<usize, Vec<&str>> = HashMap::new();
    for (email, &account_idx) in &email_to_account {
        let root = uf.find(account_idx);
        groups.entry(root).or_default().push(email);
    }

    // Build the result: [name, sorted emails...]
    let mut result = Vec::new();
    for (root, mut emails) in groups {
        emails.sort();
        let mut merged = vec![accounts[root][0].clone()];
        merged.extend(emails.into_iter().map(String::from));
        result.push(merged);
    }

    result
}
```

The insight: each account is a node, and shared emails create edges between accounts. After
all unions, each connected component is one person. We collect all emails belonging to each
component and sort them.

**Time:** O(n * k * alpha(n * k)) where k is the average number of emails per account.
Effectively O(n * k) plus the cost of sorting the output.

---

## Step-by-Step Visual: Processing a Series of Unions

Let us trace through a complete example with both optimizations active.

```
  Elements: 0, 1, 2, 3, 4, 5, 6, 7
  Initial state:

  parent: [0, 1, 2, 3, 4, 5, 6, 7]
  rank:   [0, 0, 0, 0, 0, 0, 0, 0]
  Trees:  [0] [1] [2] [3] [4] [5] [6] [7]
  Components: 8

  ─────────────────────────────────────────
  union(0, 1):
    find(0) = 0, find(1) = 1
    Ranks equal (0 == 0). Attach 1 under 0. rank[0] = 1.

    parent: [0, 0, 2, 3, 4, 5, 6, 7]
    rank:   [1, 0, 0, 0, 0, 0, 0, 0]
    Trees:  [0]  [2] [3] [4] [5] [6] [7]
             |
            [1]
    Components: 7

  ─────────────────────────────────────────
  union(2, 3):
    find(2) = 2, find(3) = 3
    Ranks equal. Attach 3 under 2. rank[2] = 1.

    parent: [0, 0, 2, 2, 4, 5, 6, 7]
    rank:   [1, 0, 1, 0, 0, 0, 0, 0]
    Trees:  [0]  [2]  [4] [5] [6] [7]
             |    |
            [1]  [3]
    Components: 6

  ─────────────────────────────────────────
  union(4, 5):
    find(4) = 4, find(5) = 5
    Ranks equal. Attach 5 under 4. rank[4] = 1.

    parent: [0, 0, 2, 2, 4, 4, 6, 7]
    rank:   [1, 0, 1, 0, 1, 0, 0, 0]
    Trees:  [0]  [2]  [4]  [6] [7]
             |    |    |
            [1]  [3]  [5]
    Components: 5

  ─────────────────────────────────────────
  union(6, 7):
    find(6) = 6, find(7) = 7
    Ranks equal. Attach 7 under 6. rank[6] = 1.

    parent: [0, 0, 2, 2, 4, 4, 6, 6]
    rank:   [1, 0, 1, 0, 1, 0, 1, 0]
    Trees:  [0]  [2]  [4]  [6]
             |    |    |    |
            [1]  [3]  [5]  [7]
    Components: 4

  ─────────────────────────────────────────
  union(0, 2):
    find(0) = 0 (rank 1), find(2) = 2 (rank 1)
    Ranks equal. Attach 2 under 0. rank[0] = 2.

    parent: [0, 0, 0, 2, 4, 4, 6, 6]
    rank:   [2, 0, 1, 0, 1, 0, 1, 0]
    Trees:       [0]       [4]  [6]
                / \         |    |
              [1] [2]      [5]  [7]
                   |
                  [3]
    Components: 3

  ─────────────────────────────────────────
  union(4, 6):
    find(4) = 4 (rank 1), find(6) = 6 (rank 1)
    Ranks equal. Attach 6 under 4. rank[4] = 2.

    parent: [0, 0, 0, 2, 4, 4, 4, 6]
    rank:   [2, 0, 1, 0, 2, 0, 1, 0]
    Trees:       [0]         [4]
                / \         / \
              [1] [2]     [5] [6]
                   |           |
                  [3]         [7]
    Components: 2

  ─────────────────────────────────────────
  union(0, 4):
    find(0) = 0 (rank 2), find(4) = 4 (rank 2)
    Ranks equal. Attach 4 under 0. rank[0] = 3.

    parent: [0, 0, 0, 2, 0, 4, 4, 6]
    rank:   [3, 0, 1, 0, 2, 0, 1, 0]

    Tree:
                    [0]
                  / | \
                [1][2][4]
                    |  / \
                   [3][5][6]
                          |
                         [7]
    Components: 1

  ─────────────────────────────────────────
  Now: find(7) with path compression.
    Follow: 7 -> 6 -> 4 -> 0. Root is 0.
    Compress: set parent[7] = 0, parent[6] = 0, parent[4] = 0.

    parent: [0, 0, 0, 2, 0, 4, 0, 0]

    Tree after compression:
                    [0]
               / / | | \ \
             [1][2][4][5][6][7]
                  |
                 [3]

    Node 7, 6, and 4 now point directly to 0.
    Future find(7) is O(1).
```

Notice how path compression flattened the tree. Over many operations, the trees become
nearly flat -- which is why the amortized cost approaches O(1).

---

## Union by Size (Alternative to Rank)

Instead of tracking rank (an upper bound on height), you can track the **size** (number of
nodes) of each tree and always attach the smaller tree under the larger one. This gives the
same asymptotic complexity and has a practical advantage: the size field remains accurate,
while rank becomes a loose upper bound after path compression.

```rust
struct UnionFindBySize {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFindBySize {
    fn new(n: usize) -> Self {
        UnionFindBySize {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        // Attach smaller tree under larger tree.
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        self.count -= 1;
        true
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Return the size of the component containing x.
    fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}
```

The `component_size` method is a bonus: you can ask "how big is this group?" which union-by-rank
cannot answer directly.

Both union-by-rank and union-by-size, combined with path compression, yield the same
O(alpha(n)) amortized bound. Choose whichever is more convenient for the problem at hand.

---

## The Full Toolkit

Here is the complete, production-ready Union-Find with both optimizations:

```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    /// Create a Union-Find for n elements (0..n).
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }

    /// Find the representative of the set containing x.
    /// Uses path compression for amortized near-O(1).
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Merge the sets containing x and y.
    /// Returns true if a merge happened (x and y were in different sets).
    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
        self.count -= 1;
        true
    }

    /// Check if x and y are in the same set.
    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Number of disjoint sets.
    fn count(&self) -> usize {
        self.count
    }
}

fn main() {
    let mut uf = UnionFind::new(10);

    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(0, 3);

    assert!(uf.connected(0, 2));   // 0-1-...-3-2, all connected
    assert!(uf.connected(1, 3));
    assert!(!uf.connected(0, 4));  // 4 is still alone

    assert_eq!(uf.count(), 7);    // {0,1,2,3}, {4}, {5}, {6}, {7}, {8}, {9}

    println!("All assertions passed.");
}
```

---

## Complexity Summary

| Operation | Without Optimizations | With Rank Only | With Compression Only | With Both |
|-----------|:---------------------:|:--------------:|:---------------------:|:---------:|
| find      | O(n) worst           | O(log n)       | O(log n) amortized    | O(alpha(n)) amortized |
| union     | O(n) worst           | O(log n)       | O(log n) amortized    | O(alpha(n)) amortized |
| connected | O(n) worst           | O(log n)       | O(log n) amortized    | O(alpha(n)) amortized |
| Space     | O(n)                 | O(n)           | O(n)                  | O(n)      |

Either optimization alone brings the cost down to O(log n). Both together bring it down to
O(alpha(n)), which is effectively O(1) for all practical purposes.

---

## Common Pitfalls

**1. Forgetting that `find` mutates the structure.**
Path compression modifies the parent array. In Rust, this means `find` requires `&mut self`.
If you need multiple simultaneous finds (e.g., iterating over elements while querying), you
may need to restructure your code or use interior mutability (`Cell` or `RefCell`). In
practice, this is rarely a problem -- just call `find` sequentially.

**2. Off-by-one on 1-indexed problems.**
Many LeetCode problems use 1-indexed nodes. Either allocate `n + 1` elements (wasting index
0) or subtract 1 from every index. The first approach is simpler and less error-prone:

```rust
let mut uf = UnionFind::new(n + 1); // node IDs 1..=n
```

**3. Using Union-Find for directed graphs.**
Union-Find models undirected connectivity. If you need to reason about directed reachability,
you need different tools (DFS, topological sort, strongly connected components).

**4. Trying to split (undo unions).**
Standard Union-Find does not support splitting a group. If you need undo, you would need a
persistent or rollback-capable variant, which is significantly more complex. For most
problems, the one-directional merge model is sufficient.

**5. Forgetting to check if the roots are the same before merging.**
Always check `root_x != root_y` before modifying the parent array. Without this check, you
might incorrectly modify a node's parent or decrement the component count when no merge
happened.

---

## Key Takeaways

1. **Union-Find answers one question efficiently: are these two elements in the same group?**
   It supports merging groups but not splitting them.

2. **The two optimizations are critical.** Path compression flattens trees during find. Union
   by rank (or size) keeps trees balanced during union. Together, they achieve O(alpha(n))
   amortized per operation -- effectively O(1).

3. **The implementation is tiny.** The entire data structure is a parent array, a rank array,
   and two short methods. Memorizable in an interview.

4. **Core applications:** Kruskal's MST, counting connected components, cycle detection in
   undirected graphs, dynamic connectivity, accounts merge.

5. **Use Union-Find when edges arrive online** (incrementally) and you need connectivity
   answers between arrivals. Use BFS/DFS when you have the full graph and need structural
   information like paths or distances.

6. **Union-Find is a one-trick pony, but it does that trick better than anything else.**
   When the problem is about grouping, merging, and querying membership, Union-Find is
   almost certainly the right tool.

---

## Exercises

1. **Basic implementation.** Implement `UnionFind` from scratch (without looking at the code
   above). Test it by processing the edges `[(0,1), (1,2), (3,4), (4,5), (2,4)]` and
   verifying that `connected(0, 5)` is true and `connected(0, 6)` is false.

2. **Number of Provinces (LeetCode 547).** Solve it using Union-Find. Then solve it again
   using DFS. Compare the two approaches in terms of code complexity and runtime.

3. **Redundant Connection (LeetCode 684).** Find the cycle-forming edge using Union-Find.

4. **Accounts Merge (LeetCode 721).** Use Union-Find to group accounts that share emails,
   then collect and sort the results.

5. **Number of Islands (LeetCode 200).** You can solve this with BFS/DFS (the standard
   approach) or with Union-Find. Try the Union-Find approach: treat each '1' cell as a node,
   union adjacent '1' cells, and count the components.

6. **Graph Valid Tree (LeetCode 261).** A graph is a valid tree if it has exactly n-1 edges
   and is connected. Use Union-Find to check both conditions: process each edge with `union`,
   check for cycles (union returns false), and verify the final component count is 1.

7. **Kruskal's MST.** Revisit [Lesson 26](./26_mst.md) and implement Kruskal's algorithm
   using the Union-Find from this lesson. Test it on a graph with 6+ nodes and verify the
   MST weight.

---

*Next up: more advanced graph algorithms building on the foundations from lessons 21-26.*
