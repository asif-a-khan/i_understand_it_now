# Lesson 17: Binary Search Trees

## From Binary Trees to Searchable Trees

In Lesson 16 we built binary trees -- nodes with values, a left child, and a right child.
We could store data, traverse it in different orders, and compute properties like height
and size. But we had no *efficient* way to answer the question: "Is value X in this tree?"
The only option was to visit every node, which is O(n). We might as well use a list.

A **binary search tree** (BST) adds one rule on top of the binary tree structure, and that
single rule gives us O(log n) search, insert, and delete -- when things go well. When things
go badly, we get O(n) again, and understanding *why* is just as important as understanding
the happy path.

---

## The BST Property

A binary search tree is a binary tree where, for **every** node:

- All values in the **left** subtree are **less than** the node's value.
- All values in the **right** subtree are **greater than** the node's value.

This must hold recursively for every node, not just the root.

```
  A valid BST:

              [20]
             /    \
          [10]    [30]
         /    \      \
       [5]   [15]   [40]
       / \
     [3] [7]

  For node 20: everything left  (10, 5, 15, 3, 7) is < 20.  Yes.
               everything right (30, 40) is > 20.             Yes.
  For node 10: everything left  (5, 3, 7) is < 10.           Yes.
               everything right (15) is > 10.                 Yes.
  For node 5:  everything left  (3) is < 5.                   Yes.
               everything right (7) is > 5.                   Yes.
  ...and so on for every node.
```

### The Library Analogy

Imagine a library where the shelving system follows one rule: from any book you are looking
at, every book to the left comes *before* it alphabetically, and every book to the right
comes *after*. This applies not just to the shelf you are on but to entire sections. If you
are standing at the book titled "M", then the entire left wing of the library contains A-L
and the entire right wing contains N-Z. Within the left wing, the same rule applies
recursively -- the left half holds A-F and the right half holds G-L.

To find a specific book, you never need to scan every shelf. You make a simple comparison
at each decision point: "Is what I want before or after this?" Each comparison eliminates
roughly half the remaining collection.

This is exactly how binary search works (Lesson 08), but now the data is organized as a
tree instead of a sorted array.

### A Subtle Point: Duplicates

The strict BST property requires `left < node < right`. What about duplicate values? There
are different conventions:

- **No duplicates allowed** (the most common textbook definition, and what we use here).
- **Duplicates go left**: `left <= node < right`.
- **Duplicates go right**: `left < node <= right`.
- **Count field**: Store a count of occurrences in each node.

For this lesson, we assume all values are distinct. In practice, if you need duplicates,
the count field approach is cleanest.

---

## Why the BST Property Enables O(h) Operations

Because of the ordering property, search does not visit every node. It follows a single
path from the root to the target (or to a null child where the target would be):

```
  Searching for 15 in our BST:

              [20]         20 > 15, go left
             /    \
          [10]    [30]     10 < 15, go right
         /    \      \
       [5]   [15]   [40]  Found 15!
       / \
     [3] [7]

  Visited: 20 -> 10 -> 15.  Three nodes, not eight.
```

The number of comparisons is at most the **height** of the tree, which we call `h`. So
search, insert, and delete are all O(h).

But how tall is the tree? That depends entirely on the shape:

```
  BALANCED BST (h = log n):          DEGENERATE BST (h = n):

        [20]                          [10]
       /    \                            \
    [10]    [30]                         [20]
   /    \   /  \                            \
  [5] [15][25][35]                          [30]
                                               \
  7 nodes, height 2                            [40]
  h = floor(log2(7)) = 2                          \
  Search: at most 3 comparisons                   [50]

                                      5 nodes, height 4
                                      h = n - 1 = 4
                                      Search: at most 5 comparisons
                                      (this is just a linked list)
```

A balanced BST has height O(log n), so operations are O(log n).
A degenerate BST has height O(n), so operations degrade to O(n).

This is the fundamental tension of BSTs: the same set of values can produce vastly
different tree shapes depending on the insertion order. Insert [10, 20, 30, 40, 50] in
order and you get the degenerate linked-list shape. Insert [30, 20, 40, 10, 50] and you
get something balanced. The data structure itself does not enforce balance -- that is what
self-balancing trees (AVL, Red-Black) solve, which we cover in the next lesson.

For now, we analyze everything in terms of O(h) and keep in mind that h ranges from
log(n) to n.

---

## The Node and Tree Structs in Rust

We use a similar structure to Lesson 16's binary tree, but now the ordering property is
maintained by the insertion logic:

```rust
use std::cmp::Ordering;

type Link = Option<Box<BstNode>>;

#[derive(Debug)]
struct BstNode {
    value: i32,
    left: Link,
    right: Link,
}

impl BstNode {
    fn new(value: i32) -> Self {
        BstNode {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct Bst {
    root: Link,
}

impl Bst {
    fn new() -> Self {
        Bst { root: None }
    }
}
```

The `type Link = Option<Box<BstNode>>` pattern should be familiar from Lesson 16. `None`
represents an empty subtree, `Some(Box<BstNode>)` represents a heap-allocated node. The
`Box` gives us a known-size pointer that the compiler can work with, since a recursive
struct would otherwise have infinite size.

---

## Search

Searching a BST follows the left/right decision at each node. This is the recursive
algorithm from Lesson 07 applied to a tree structure:

```rust
impl Bst {
    fn search(&self, target: i32) -> bool {
        Self::search_node(&self.root, target)
    }

    fn search_node(node: &Link, target: i32) -> bool {
        match node {
            None => false,
            Some(n) => match target.cmp(&n.value) {
                Ordering::Equal => true,
                Ordering::Less => Self::search_node(&n.left, target),
                Ordering::Greater => Self::search_node(&n.right, target),
            },
        }
    }
}
```

### Trace

```
  Search for 7:

       [20]          7 < 20 --> go left
       /    \
    [10]    [30]     7 < 10 --> go left
   /    \
  [5]   [15]        7 > 5  --> go right
  / \
[3] [7]             7 == 7 --> found!

  Search for 12:

       [20]          12 < 20 --> go left
       /    \
    [10]    [30]     12 > 10 --> go right
   /    \
  [5]   [15]        12 < 15 --> go left
  / \
[3] [7]             left is None --> not found

  In both cases: O(h) comparisons.
```

### Iterative Search

Since each step replaces the current node with one of its children (no work after the
recursive call), we can write this iteratively without an explicit stack. This avoids
function-call overhead and stack-depth concerns:

```rust
impl Bst {
    fn search_iterative(&self, target: i32) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            match target.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
            }
        }
        false
    }
}
```

This is structurally identical to binary search on a sorted array (Lesson 08): at each
step, compare and go left or right. The difference is that in an array you move an index;
in a tree you follow a pointer.

---

## Insertion

To insert a value, we search for where it *would* be if it existed. When we hit a `None`
(an empty spot), that is where the new node goes.

```rust
impl Bst {
    fn insert(&mut self, value: i32) {
        Self::insert_node(&mut self.root, value);
    }

    fn insert_node(link: &mut Link, value: i32) {
        match link {
            None => {
                *link = Some(Box::new(BstNode::new(value)));
            }
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => Self::insert_node(&mut node.left, value),
                Ordering::Greater => Self::insert_node(&mut node.right, value),
                Ordering::Equal => {} // duplicate: do nothing
            },
        }
    }
}
```

### Step-by-Step: Building a BST

Let's insert the values [20, 10, 30, 5, 15, 25, 35] one at a time:

```
  Insert 20:  Tree is empty, 20 becomes the root.

      [20]

  Insert 10:  10 < 20, go left. Left is empty, insert here.

      [20]
      /
   [10]

  Insert 30:  30 > 20, go right. Right is empty, insert here.

      [20]
      /    \
   [10]   [30]

  Insert 5:   5 < 20, go left. 5 < 10, go left. Left is empty.

       [20]
      /    \
   [10]   [30]
   /
  [5]

  Insert 15:  15 < 20, go left. 15 > 10, go right. Right is empty.

       [20]
      /    \
   [10]   [30]
   /  \
  [5] [15]

  Insert 25:  25 > 20, go right. 25 < 30, go left. Left is empty.

       [20]
      /    \
   [10]   [30]
   /  \   /
  [5] [15][25]

  Insert 35:  35 > 20, go right. 35 > 30, go right. Right is empty.

         [20]
        /    \
     [10]   [30]
     /  \   /  \
   [5] [15][25][35]

  Final tree: balanced, height 2.
```

Now watch what happens with sorted input [10, 20, 30, 40, 50]:

```
  Insert 10:  [10]

  Insert 20:  20 > 10, go right.

              [10]
                 \
                 [20]

  Insert 30:  30 > 10, go right. 30 > 20, go right.

              [10]
                 \
                 [20]
                    \
                    [30]

  Insert 40:  ...and so on.

              [10]
                 \
                 [20]
                    \
                    [30]
                       \
                       [40]
                          \
                          [50]

  Degenerate! Height 4, basically a linked list.
  Every search is O(n).
```

This is why insertion order matters so much for unbalanced BSTs.

---

## Finding Min and Max

The BST property makes finding the minimum and maximum trivial:

- **Minimum**: Go left as far as possible. The leftmost node is the smallest.
- **Maximum**: Go right as far as possible. The rightmost node is the largest.

```
  In this tree:

         [20]
        /    \
     [10]   [30]
     /  \   /  \
   [5] [15][25][35]
   /
  [3]

  Min: 20 -> 10 -> 5 -> 3. Done. Min is 3.
  Max: 20 -> 30 -> 35. Done. Max is 35.
```

```rust
impl Bst {
    fn min(&self) -> Option<i32> {
        Self::min_node(&self.root)
    }

    fn min_node(link: &Link) -> Option<i32> {
        match link {
            None => None,
            Some(node) => {
                if node.left.is_none() {
                    Some(node.value)
                } else {
                    Self::min_node(&node.left)
                }
            }
        }
    }

    fn max(&self) -> Option<i32> {
        Self::max_node(&self.root)
    }

    fn max_node(link: &Link) -> Option<i32> {
        match link {
            None => None,
            Some(node) => {
                if node.right.is_none() {
                    Some(node.value)
                } else {
                    Self::max_node(&node.right)
                }
            }
        }
    }
}
```

Both are O(h) -- they walk a single path from some node down to a leaf.

---

## Deletion

Deletion is the most complex BST operation because removing a node must preserve the BST
property. There are three cases, depending on how many children the node has.

### Case 1: Deleting a Leaf (No Children)

The simplest case. Just remove the node. Nothing else to fix.

```
  Delete 3:

  Before:                      After:
         [20]                        [20]
        /    \                      /    \
     [10]   [30]                 [10]   [30]
     /  \                        /  \
   [5] [15]                    [5] [15]
   /
  [3]  <-- leaf, just remove

  Node 3 has no children. Set the parent's left pointer to None.
```

### Case 2: Deleting a Node with One Child

Replace the node with its only child. The child "moves up" to take the deleted node's
place.

```
  Delete 5 (has one child: 3):

  Before:                      After:
         [20]                        [20]
        /    \                      /    \
     [10]   [30]                 [10]   [30]
     /  \                        /  \
   [5] [15]                    [3] [15]
   /
  [3]

  Node 5 has one child (3). Replace 5 with 3.
  The BST property is preserved because 3 was already
  in the left subtree of 10 (so 3 < 10).
```

### Case 3: Deleting a Node with Two Children

This is the tricky one. You cannot just remove the node because you have two subtrees to
deal with. The solution: find the node's **in-order successor** (the smallest value in
its right subtree), copy that value into the node being deleted, then delete the successor
node from the right subtree.

Why the in-order successor? Because it is the smallest value that is still greater than
everything in the left subtree. Putting it in the deleted node's position preserves the
BST property.

Alternatively, you can use the **in-order predecessor** (the largest value in the left
subtree). Either works.

```
  Delete 20 (has two children: left=10, right=30):

  Step 1: Find in-order successor of 20.
          Go to right subtree (rooted at 30), then go left as far as possible.
          30 has a left child of 25. 25 has no left child.
          In-order successor is 25.

  Step 2: Copy 25 into node 20's position.

  Step 3: Delete 25 from the right subtree (25 is a leaf, so Case 1).

  Before:                      After step 2+3:
         [20]                        [25]
        /    \                      /    \
     [10]   [30]                 [10]   [30]
     /  \   /  \                 /  \      \
   [5] [15][25][35]            [5] [15]   [35]

  BST property still holds:
    25 > everything in left subtree (5, 10, 15). Yes.
    25 < everything in right subtree (30, 35). Yes.
```

### All Three Cases Visualized Together

```
  CASE 1: Leaf                CASE 2: One child           CASE 3: Two children
  ─────────────               ──────────────────          ────────────────────

  Delete [X]:                 Delete [X]:                 Delete [X]:

      [P]                         [P]                         [P]
      /                           /                           /
    [X]  (no children)          [X]  (one child: C)         [X]   (two children)
                                /                           / \
  Result:                     [C]                         [L] [R]
                                                               \
      [P]                     Result:                         [S] <-- in-order
      /                                                            successor
   None                           [P]                   Result:
                                  /
                                [C]                         [P]
                                                            /
                                                          [S]
                                                          / \
                                                        [L] [R]  (with S removed
                                                                   from R's subtree)
```

### Deletion in Rust

```rust
impl Bst {
    fn delete(&mut self, value: i32) {
        Self::delete_node(&mut self.root, value);
    }

    fn delete_node(link: &mut Link, value: i32) {
        if link.is_none() {
            return; // value not found
        }

        let node = link.as_mut().unwrap();

        match value.cmp(&node.value) {
            Ordering::Less => Self::delete_node(&mut node.left, value),
            Ordering::Greater => Self::delete_node(&mut node.right, value),
            Ordering::Equal => {
                // Found the node to delete.
                match (&node.left, &node.right) {
                    // Case 1: Leaf node -- just remove it.
                    (None, None) => {
                        *link = None;
                    }
                    // Case 2a: Only right child.
                    (None, _) => {
                        *link = node.right.take();
                    }
                    // Case 2b: Only left child.
                    (_, None) => {
                        *link = node.left.take();
                    }
                    // Case 3: Two children.
                    // Replace value with in-order successor, then delete successor.
                    _ => {
                        let successor_val = Self::min_node(&node.right).unwrap();
                        node.value = successor_val;
                        Self::delete_node(&mut node.right, successor_val);
                    }
                }
            }
        }
    }
}
```

A note on the Rust ownership: `node.right.take()` moves the child's `Option<Box<BstNode>>`
out and replaces it with `None`, then we assign it to `*link` to replace the current node.
This is idiomatic Rust -- the borrow checker ensures we do not accidentally leave dangling
pointers.

**Time complexity of delete:** O(h). We walk down to find the node (O(h)), and if it has
two children, finding the in-order successor walks down the right subtree (at most O(h)
more). Total is still O(h).

---

## In-Order Traversal: The Sorted Output

Here is one of the most elegant properties of a BST: an **in-order traversal** (left,
node, right) visits every node in ascending sorted order.

Why? Think about it inductively:

1. Everything in the left subtree is smaller than the current node.
2. The current node comes next.
3. Everything in the right subtree is larger than the current node.
4. Within each subtree, the same logic applies recursively.

So in-order traversal visits all smaller values first, then the current value, then all
larger values. That is sorted order.

```
         [20]
        /    \
     [10]   [30]
     /  \   /  \
   [5] [15][25][35]
   /
  [3]

  In-order traversal (left, visit, right -- recursively):

  Left subtree of 20:
    Left subtree of 10:
      Left subtree of 5:
        Visit 3
      Visit 5
      (no right child of 5)
    Visit 10
    Right subtree of 10:
      Visit 15
  Visit 20
  Right subtree of 20:
    Left subtree of 30:
      Visit 25
    Visit 30
    Right subtree of 30:
      Visit 35

  Output: 3, 5, 10, 15, 20, 25, 30, 35   <-- sorted!
```

```rust
impl Bst {
    /// Collect all values in sorted (ascending) order.
    fn in_order(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::in_order_walk(&self.root, &mut result);
        result
    }

    fn in_order_walk(link: &Link, result: &mut Vec<i32>) {
        if let Some(node) = link {
            Self::in_order_walk(&node.left, result);
            result.push(node.value);
            Self::in_order_walk(&node.right, result);
        }
    }
}
```

**Time complexity:** O(n). Every node is visited exactly once.
**Space complexity:** O(h) for the recursion stack (plus O(n) for the output vector).

This also means: if you have n elements and want them sorted, inserting them all into a BST
and then doing an in-order traversal produces a sorted sequence. The total cost is O(n*h)
for insertion plus O(n) for traversal. If the tree stays balanced, that is O(n log n) --
essentially a sorting algorithm (tree sort).

---

## In-Order Successor and Predecessor

The **in-order successor** of a node is the next node in sorted order (the smallest value
greater than the node's value). The **in-order predecessor** is the previous node (the
largest value smaller than the node's value).

Finding these is important for deletion (Case 3) and for iteration.

### In-Order Successor: Two Cases

1. **Node has a right subtree:** The successor is the minimum of the right subtree. Go
   right once, then left as far as possible.

2. **Node has no right subtree:** The successor is the nearest ancestor for which this
   node is in the *left* subtree. Walk up until you turn right.

```
         [20]
        /    \
     [10]   [30]
     /  \   /  \
   [5] [15][25][35]

  Successor of 15:
    15 has no right subtree.
    Walk up: 15 is right child of 10, keep going.
    10 is left child of 20. Stop. Successor is 20.

  Successor of 10:
    10 has a right subtree (rooted at 15).
    Min of right subtree = 15. Successor is 15.

  Successor of 5:
    5 has no right subtree.
    Walk up: 5 is left child of 10. Stop. Successor is 10.
```

### In-Order Predecessor: Mirror Image

1. **Node has a left subtree:** The predecessor is the maximum of the left subtree. Go
   left once, then right as far as possible.

2. **Node has no left subtree:** Walk up until you find an ancestor for which this node
   is in the *right* subtree.

In our BST implementation with `Option<Box<BstNode>>` and no parent pointers, finding the
successor when there is no right child requires traversing from the root. With parent
pointers, you can walk upward directly. This is a design trade-off.

---

## BST from a Sorted Array

Given a sorted array, we can build a *balanced* BST in O(n) time using divide and
conquer -- a direct application of what we learned in Lesson 07:

1. The middle element becomes the root.
2. Recursively build a BST from the left half (left subtree).
3. Recursively build a BST from the right half (right subtree).

```
  Sorted array: [3, 5, 10, 15, 20, 25, 30, 35]

  Middle (index 3): 15 becomes root.
  Left half [3, 5, 10]: middle is 5.
  Right half [20, 25, 30, 35]: middle is 25.

             [15]
            /    \
         [5]    [25]
        /   \   /   \
      [3]  [10][20] [30]
                        \
                        [35]

  Height: 3. For 8 elements, log2(8) = 3. Balanced.
```

```rust
impl Bst {
    fn from_sorted(sorted: &[i32]) -> Self {
        Bst {
            root: Self::build_balanced(sorted),
        }
    }

    fn build_balanced(sorted: &[i32]) -> Link {
        if sorted.is_empty() {
            return None;
        }
        let mid = sorted.len() / 2;
        let mut node = BstNode::new(sorted[mid]);
        node.left = Self::build_balanced(&sorted[..mid]);
        node.right = Self::build_balanced(&sorted[mid + 1..]);
        Some(Box::new(node))
    }
}
```

This is O(n) time (each element is visited once) and O(log n) stack space (the recursion
depth equals the height of the balanced tree).

---

## Complexity Summary

| Operation          | Time (avg, balanced) | Time (worst, degenerate) | Space    |
|--------------------|----------------------|--------------------------|----------|
| Search             | O(log n)             | O(n)                     | O(1)*    |
| Insert             | O(log n)             | O(n)                     | O(1)*    |
| Delete             | O(log n)             | O(n)                     | O(1)*    |
| Min / Max          | O(log n)             | O(n)                     | O(1)*    |
| In-order traversal | O(n)                 | O(n)                     | O(h)**   |
| Build from sorted  | O(n)                 | O(n)                     | O(log n) |

\* O(1) for iterative versions. Recursive versions use O(h) stack space.
\** O(h) for the recursion stack; O(n) if collecting into a Vec.

The critical takeaway: **all single-element operations are O(h)**, and h is anywhere from
log(n) (balanced) to n (degenerate). The BST gives you the *mechanism* for efficient
search, but not the *guarantee*. Guarantees require self-balancing.

---

## The Degenerate Case: Why Unbalanced BSTs Are Dangerous

The worst case is not theoretical. It happens in practice whenever data arrives in sorted
or nearly-sorted order:

```
  Insert: 1, 2, 3, 4, 5

        [1]
           \
           [2]
              \
              [3]
                 \
                 [4]
                    \
                    [5]

  This is a linked list with extra overhead (two null child pointers per node).
  Search for 5: visit all 5 nodes. O(n).
  Insert 6: walk all 5 nodes, then append. O(n).

  Insert: 5, 4, 3, 2, 1  (reverse sorted is equally bad)

              [5]
              /
           [4]
           /
        [3]
        /
     [2]
     /
  [1]

  Same problem, just leaning left instead of right.
```

How common is sorted data in practice? Very. Database rows often arrive in primary-key
order. Log entries are timestamp-ordered. Auto-increment IDs are inherently sorted. If
you use a plain BST to index such data, you get O(n) operations -- worse than a hash map.

This is precisely why self-balancing BSTs (AVL trees, Red-Black trees) exist. They
perform rotations after insertions and deletions to keep h = O(log n) regardless of
insertion order. We cover AVL trees in the next lesson.

---

## A Note on Rust's BTreeMap: It Is NOT a BST

You might expect Rust's standard library `BTreeMap<K, V>` to be a binary search tree. It
is not. It is a **B-tree**, which is a different data structure entirely.

A BST has at most 2 children per node and stores 1 key per node. A B-tree stores *many*
keys per node (sorted within the node) and has many children. The standard B-tree node
in Rust holds up to B keys (where B is a branching factor, typically 6 or so for
`BTreeMap`).

```
  BST node:                    B-tree node (B=3):

  +-----+                     +-----+-----+-----+
  | key |                     | k1  | k2  | k3  |
  +-----+                     +-----+-----+-----+
  /     \                     / |       |       \
 L       R                 c0  c1     c2       c3
                           (up to 4 child pointers)
```

Why does this matter?

- **Cache performance:** A B-tree node packs multiple keys into a contiguous block of
  memory. Searching within a node is a linear scan of a small array -- very cache-friendly.
  A BST scatters one key per node across the heap, causing more cache misses.
- **Fewer pointer dereferences:** A B-tree with branching factor 6 and a million keys has
  a height of about 8. A balanced BST would have height ~20. Fewer levels means fewer
  pointer chases.
- **Disk-friendly:** B-trees were originally designed for databases and file systems where
  each "pointer dereference" is a disk seek. Minimizing tree height is critical.

For in-memory ordered data in Rust, `BTreeMap` is the go-to. Do not confuse it with a
binary search tree -- they share the "search tree" concept but differ in structure and
performance characteristics.

---

## Common Interview Problems

### Problem 1: Validate a BST

"Given a binary tree, determine if it is a valid BST."

The naive approach (check that each node's left child is smaller and right child is larger)
is **wrong**. It does not catch cases where a deep node violates the property with respect
to an ancestor:

```
  This passes the naive check but is NOT a valid BST:

        [20]
       /    \
    [10]    [30]
       \
       [25]   <-- 25 > 10, so it passes the local check.
               But 25 > 20, so it should not be in 20's left subtree!
```

The correct approach: pass down valid bounds. Every node must be within (min, max):

```rust
fn is_valid_bst(link: &Link, min: Option<i32>, max: Option<i32>) -> bool {
    match link {
        None => true,
        Some(node) => {
            if let Some(min_val) = min {
                if node.value <= min_val {
                    return false;
                }
            }
            if let Some(max_val) = max {
                if node.value >= max_val {
                    return false;
                }
            }
            is_valid_bst(&node.left, min, Some(node.value))
                && is_valid_bst(&node.right, Some(node.value), max)
        }
    }
}

// Usage: is_valid_bst(&tree.root, None, None)
```

Alternative approach: do an in-order traversal and verify the output is strictly
increasing. If in-order traversal is sorted, the tree is a valid BST. This uses O(n)
space but is conceptually simpler.

### Problem 2: Kth Smallest Element

"Find the kth smallest element in a BST."

Since in-order traversal produces sorted output, the kth smallest is the kth element
visited during in-order traversal. You can do a full traversal and collect into a Vec,
then return index k-1. That is O(n) time and O(n) space.

Better: do an in-order traversal but count as you go, stopping as soon as you reach k.
This is O(h + k) time and O(h) space:

```rust
impl Bst {
    fn kth_smallest(&self, k: usize) -> Option<i32> {
        let mut count = 0;
        let mut result = None;
        Self::kth_walk(&self.root, k, &mut count, &mut result);
        result
    }

    fn kth_walk(link: &Link, k: usize, count: &mut usize, result: &mut Option<i32>) {
        if let Some(node) = link {
            // In-order: left first
            Self::kth_walk(&node.left, k, count, result);

            // Visit current node
            *count += 1;
            if *count == k {
                *result = Some(node.value);
                return;
            }

            // Then right
            if result.is_none() {
                Self::kth_walk(&node.right, k, count, result);
            }
        }
    }
}
```

### Problem 3: Lowest Common Ancestor (LCA) in a BST

"Given two values p and q both present in a BST, find the node that is their lowest common
ancestor."

The BST property makes this simpler than the general binary tree LCA. Starting from the
root:

- If both p and q are less than the current node, the LCA is in the left subtree.
- If both p and q are greater than the current node, the LCA is in the right subtree.
- Otherwise, the current node is the LCA (this is where p and q "split" into different
  subtrees, or one of them equals the current node).

```
  Find LCA of 5 and 15:

         [20]        Both 5 and 15 < 20 --> go left
        /    \
     [10]   [30]     5 < 10 but 15 > 10 --> split! LCA is 10.
     /  \
   [5] [15]
```

```rust
impl Bst {
    fn lca(&self, p: i32, q: i32) -> Option<i32> {
        let mut current = &self.root;
        while let Some(node) = current {
            if p < node.value && q < node.value {
                current = &node.left;
            } else if p > node.value && q > node.value {
                current = &node.right;
            } else {
                return Some(node.value);
            }
        }
        None
    }
}
```

Time: O(h). Space: O(1).

---

## Putting It All Together

With the structs and `impl` blocks defined above, here is a `main` that exercises every
operation:

```rust
fn main() {
    let mut tree = Bst::new();

    // Insert values
    for val in [20, 10, 30, 5, 15, 25, 35] {
        tree.insert(val);
    }

    // Search
    assert!(tree.search(15));
    assert!(!tree.search(99));

    // Min / Max
    assert_eq!(tree.min(), Some(5));
    assert_eq!(tree.max(), Some(35));

    // In-order traversal (sorted output)
    assert_eq!(tree.in_order(), vec![5, 10, 15, 20, 25, 30, 35]);

    // Delete leaf
    tree.delete(5);
    assert_eq!(tree.in_order(), vec![10, 15, 20, 25, 30, 35]);

    // Delete node with one child
    tree.delete(10);
    assert_eq!(tree.in_order(), vec![15, 20, 25, 30, 35]);

    // Delete node with two children
    tree.delete(30);
    assert_eq!(tree.in_order(), vec![15, 20, 25, 35]);

    // Build balanced BST from sorted array
    let balanced = Bst::from_sorted(&[1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(balanced.in_order(), vec![1, 2, 3, 4, 5, 6, 7]);

    println!("All assertions passed.");
}
```

---

## What Comes Next: AVL Trees

We have seen the BST's Achilles' heel: the degenerate case. Insert sorted data and your
O(log n) data structure becomes O(n). The fix is to **rebalance** the tree after every
insertion and deletion, keeping the height at O(log n) no matter what.

The simplest self-balancing BST is the **AVL tree** (named after Adelson-Velsky and
Landis, 1962). It maintains a balance invariant: for every node, the heights of the left
and right subtrees differ by at most 1. When an insertion or deletion violates this, the
tree performs **rotations** -- local restructuring operations that restore balance without
breaking the BST property.

That is the next lesson. For now, make sure you are comfortable with the unbalanced BST:
the property, search, insert, all three deletion cases, and in-order traversal. Everything
in AVL trees builds directly on top of this foundation.

---

## Key Takeaways

1. **The BST property**: for every node, left subtree values < node value < right subtree
   values. This single rule enables efficient search.

2. **All single-element operations are O(h)**, where h is the tree height. For a balanced
   tree h = O(log n); for a degenerate tree h = O(n).

3. **Insertion order determines tree shape.** Sorted input produces a linked list. Randomized
   input tends to produce a roughly balanced tree. You cannot rely on luck.

4. **Deletion has three cases**: leaf (remove), one child (replace with child), two children
   (replace with in-order successor, then delete successor).

5. **In-order traversal of a BST produces sorted output.** This is a direct consequence of
   the BST property and is useful for validation, iteration, and kth-element queries.

6. **Rust's BTreeMap is a B-tree, not a BST.** It stores multiple keys per node for cache
   efficiency. Use it for ordered maps in production Rust. Implement BSTs to learn the
   concepts.

7. **Self-balancing trees (AVL, Red-Black) fix the degenerate case.** Without them, a BST
   is only as good as its insertion order. The next lesson covers AVL trees.

---

## Exercises

1. **Build and traverse.** Insert these values into a BST in order: [50, 30, 70, 20, 40,
   60, 80]. Draw the resulting tree. Then do in-order, pre-order, and post-order traversals.
   Verify that in-order gives sorted output.

2. **Degenerate to balanced.** Given the degenerate BST formed by inserting [1, 2, 3, 4, 5,
   6, 7] in order, extract the values with in-order traversal, then rebuild a balanced BST
   using the `from_sorted` approach. What is the height of each tree?

3. **Delete all three cases.** Starting from the tree in Exercise 1, delete 20 (leaf), then
   delete 30 (one child after 20 is gone), then delete 50 (two children). Draw the tree
   after each deletion and verify the BST property.

4. **Validate BST.** Implement `is_valid_bst` and test it on both valid and invalid trees.
   Construct an invalid tree by manually setting node pointers to violate the BST property
   (e.g., put 25 as the right child of 10 when the root is 20).

5. **Kth smallest without collecting.** Implement the counting in-order traversal to find
   the kth smallest element. Test on a tree with values [1..100] built using `from_sorted`.
   Verify: `kth_smallest(1)` returns 1, `kth_smallest(50)` returns 50, `kth_smallest(100)`
   returns 100.

6. **Floor and ceiling.** Implement `floor(value)` (largest key <= value) and
   `ceiling(value)` (smallest key >= value). These are useful operations that BSTs
   support efficiently. For example, in a BST containing [10, 20, 30, 40, 50]:
   `floor(25)` returns 20, `ceiling(25)` returns 30.

<details>
<summary>Hint for Exercise 6</summary>

For `floor(value)`: traverse the tree. If the current node equals value, return it. If
the current node is greater than value, go left (the floor must be in the left subtree).
If the current node is less than value, it is a *candidate* for the floor -- but there
might be a closer value in the right subtree. Go right, and if you find a better candidate,
use it; otherwise, use the current node.

`ceiling` is the mirror image.

</details>

---

*Next up: [Lesson 18](./18_avl_trees.md) -- AVL Trees, where we guarantee O(log n) height
with rotations.*
