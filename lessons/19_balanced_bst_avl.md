# Lesson 19: Balanced BSTs (AVL Trees)

## The Problem with Ordinary BSTs

In Lesson 17 we built a binary search tree from scratch. We saw that search, insertion,
and deletion all run in O(h) time, where h is the height of the tree. And we said
"in the best case, h = log n." That qualifier -- "in the best case" -- is doing a lot
of heavy lifting. This lesson is about removing it.

Consider what happens when you insert sorted data into a plain BST:

```
Insert 1, 2, 3, 4, 5 in order:

    1
     \
      2
       \
        3
         \
          4
           \
            5

Height: 5 (same as n)
Search for 5: visit every single node
```

That is not a tree. That is a linked list wearing a tree costume. Every operation is
O(n). You paid the complexity cost of a tree-based structure and got none of the benefits.

Now compare the same five values arranged in a balanced BST:

```
Balanced BST with 1, 2, 3, 4, 5:

        3
       / \
      2   4
     /     \
    1       5

Height: 3 (which is floor(log2(5)) + 1)
Search for 5: visit 3 -> 4 -> 5 (three steps)
```

Same data, same number of nodes, wildly different performance. The difference is
**balance**. A balanced BST guarantees h = O(log n), which means every operation --
search, insert, delete -- is O(log n) in the *worst* case. Not the best case. Not the
average case. The worst case.

---

## A Real-World Analogy: The Self-Leveling Bookshelf

Imagine a bookshelf where you organize books by title, left to right. You can binary
search the shelf -- check the middle, go left or right. But if you keep adding books
only to the right end (because they happen to be alphabetically later), the shelf gets
lopsided. Eventually one side has 50 books and the other has 2, and your "binary search"
is barely better than scanning the whole shelf.

Now imagine the shelf is mechanical. Every time you add a book, it checks whether the
left side and right side are roughly balanced. If one side gets more than one level
deeper than the other, the shelf physically rotates a section of books to restore
balance. A few books shift position, but the invariant holds: no side is ever more than
one level deeper than the other. That is an AVL tree.

The shelf does a small amount of extra work on each insertion (the rebalancing), but it
*guarantees* that every future search stays fast. You trade a small constant-factor
overhead on writes for a hard guarantee on all operations.

---

## Balance Factor and the AVL Invariant

An AVL tree (named after its inventors Adelson-Velsky and Landis, 1962) is a BST with
one additional rule.

**Balance factor** of a node = height(left subtree) - height(right subtree)

The height of an empty subtree is -1 (or 0, depending on convention -- we will use -1
so that a leaf node has height 0).

**The AVL invariant**: for every node in the tree, the balance factor is -1, 0, or 1.
In other words: |balance factor| <= 1.

That is the entire definition. A BST where every node's subtrees differ in height by
at most 1.

```
Balanced (AVL-valid):            Unbalanced (AVL-violated):

        8  (bf=0)                       8  (bf=-2)  <-- violation!
       / \                               \
      4   12 (bf=0)                      12  (bf=-1)
     / \                                   \
    2   6                                  15  (bf=0)

  bf(8)  = h(left) - h(right)      bf(8)  = h(left) - h(right)
         = 1 - 1 = 0                      = (-1) - 1 = -2
  bf(4)  = 0 - 0 = 0               bf(12) = (-1) - 0 = -1
  bf(12) = (-1) - (-1) = 0         bf(15) = (-1) - (-1) = 0
```

Here "bf" means balance factor, and h(left)/h(right) are the heights of the left and
right subtrees. When a subtree is empty, its height is -1.

### Why |bf| <= 1 Guarantees O(log n) Height

An AVL tree of height h contains at least F(h+3) - 1 nodes, where F is the Fibonacci
sequence. This means that for n nodes, the height is at most ~1.44 * log2(n). So AVL
trees are not *perfectly* balanced (like a complete binary tree), but they are balanced
*enough* that the height stays O(log n). The constant factor of 1.44 versus 1.0 is
small enough that it does not change the complexity class.

---

## The Four Rotation Cases

When you insert (or delete) a node and the AVL invariant breaks, you fix it with
**rotations**. A rotation is a local restructuring of two or three nodes that restores
balance without breaking the BST ordering.

There are exactly four cases. Each is identified by *where* the imbalance occurs and
*which direction* the heavy subtree leans.

### Case 1: Left-Left (Right Rotation)

The left child's left subtree is too tall. The fix: rotate the imbalanced node to the
right.

```
Before (left-left imbalance at z):

          z  (bf=+2)
         / \
        y   T4
       / \
      x   T3
     / \
    T1  T2

After (single right rotation):

        y  (bf=0)
       / \
      x   z
     / \ / \
    T1 T2 T3 T4
```

Node y becomes the new root of this subtree. Node z becomes y's right child. The
subtree T3 (which was y's right child) moves to become z's left child. This is valid
because T3's values are between y and z in the BST ordering.

### Case 2: Right-Right (Left Rotation)

The mirror image of Case 1. The right child's right subtree is too tall. Rotate left.

```
Before (right-right imbalance at z):

    z  (bf=-2)
   / \
  T1   y
      / \
     T2   x
         / \
        T3  T4

After (single left rotation):

        y  (bf=0)
       / \
      z   x
     / \ / \
    T1 T2 T3 T4
```

Same idea, mirrored. Node y becomes the new root. Node z becomes y's left child. T2
moves from y's left to z's right.

### Case 3: Left-Right (Left-Right Double Rotation)

The left child's *right* subtree is too tall. A single rotation will not fix this --
the "zig-zag" shape needs two rotations. First rotate the left child to the left, then
rotate the imbalanced node to the right.

```
Before (left-right imbalance at z):

        z  (bf=+2)
       / \
      y   T4
     / \
    T1   x
        / \
       T2  T3

Step 1: Left-rotate y (transforms into left-left case):

        z
       / \
      x   T4
     / \
    y   T3
   / \
  T1  T2

Step 2: Right-rotate z:

        x  (bf=0)
       / \
      y   z
     / \ / \
    T1 T2 T3 T4
```

Two rotations, and the tree is balanced. The final structure is the same shape as the
other cases -- three nodes in a perfectly balanced arrangement with four subtrees.

### Case 4: Right-Left (Right-Left Double Rotation)

The mirror of Case 3. The right child's *left* subtree is too tall.

```
Before (right-left imbalance at z):

    z  (bf=-2)
   / \
  T1   y
      / \
     x   T4
    / \
   T2  T3

Step 1: Right-rotate y (transforms into right-right case):

    z
   / \
  T1   x
      / \
     T2   y
         / \
        T3  T4

Step 2: Left-rotate z:

        x  (bf=0)
       / \
      z   y
     / \ / \
    T1 T2 T3 T4
```

### The Pattern

Notice that all four cases end in the same shape: a balanced subtree with the median
value on top. The rotations are just different paths to get there. In code, you detect
which case you are in, apply one or two rotations, and update the heights.

---

## Implementing an AVL Tree in Rust

Let us build this piece by piece. We will use `Box` for heap-allocated nodes and
`Option<Box<Node>>` for possibly-empty children.

### The Node Structure

```rust
struct AvlNode<K: Ord, V> {
    key: K,
    value: V,
    height: i32,
    left: Option<Box<AvlNode<K, V>>>,
    right: Option<Box<AvlNode<K, V>>>,
}

impl<K: Ord, V> AvlNode<K, V> {
    fn new(key: K, value: V) -> Self {
        AvlNode {
            key,
            value,
            height: 0,  // a leaf has height 0
            left: None,
            right: None,
        }
    }
}
```

We store the height directly in each node rather than recomputing it every time. This
turns height queries into O(1) lookups at the cost of maintaining the value on
mutations.

### Height and Balance Factor Helpers

```rust
/// Returns the height of a subtree. An empty subtree has height -1.
fn height<K: Ord, V>(node: &Option<Box<AvlNode<K, V>>>) -> i32 {
    match node {
        Some(n) => n.height,
        None => -1,
    }
}

/// Returns the balance factor: height(left) - height(right).
fn balance_factor<K: Ord, V>(node: &AvlNode<K, V>) -> i32 {
    height(&node.left) - height(&node.right)
}

/// Recalculates the height of a node from its children.
fn update_height<K: Ord, V>(node: &mut AvlNode<K, V>) {
    node.height = 1 + std::cmp::max(
        height(&node.left),
        height(&node.right),
    );
}
```

### Rotations

Rotations are the mechanical heart of the AVL tree. Each one is a pointer-swap
operation that runs in O(1) time.

```rust
/// Right rotation (fixes left-left imbalance).
///
///       z              y
///      / \            / \
///     y   T4  -->   x    z
///    / \               / \
///   x   T3           T3  T4
///
fn rotate_right<K: Ord, V>(mut z: Box<AvlNode<K, V>>) -> Box<AvlNode<K, V>> {
    let mut y = z.left.take().expect("rotate_right requires a left child");
    z.left = y.right.take(); // T3 moves from y's right to z's left
    update_height(&mut z);
    y.right = Some(z);
    update_height(&mut y);
    y
}

/// Left rotation (fixes right-right imbalance).
///
///   z                  y
///  / \                / \
/// T1   y    -->     z    x
///     / \          / \
///    T2   x      T1  T2
///
fn rotate_left<K: Ord, V>(mut z: Box<AvlNode<K, V>>) -> Box<AvlNode<K, V>> {
    let mut y = z.right.take().expect("rotate_left requires a right child");
    z.right = y.left.take(); // T2 moves from y's left to z's right
    update_height(&mut z);
    y.left = Some(z);
    update_height(&mut y);
    y
}
```

The key insight in the Rust implementation: `take()` moves the child out of the
`Option`, leaving `None` behind. This lets us restructure the pointers without
violating ownership rules. There is no `unsafe` code here -- Rust's ownership model
handles the pointer manipulation naturally.

### Rebalancing

After an insertion or deletion changes the tree, we call this on each ancestor of the
modified node as we walk back up the recursion:

```rust
/// Rebalances a node if needed. Returns the (possibly new) root of this subtree.
fn rebalance<K: Ord, V>(mut node: Box<AvlNode<K, V>>) -> Box<AvlNode<K, V>> {
    update_height(&mut node);
    let bf = balance_factor(&node);

    if bf > 1 {
        // Left-heavy
        if balance_factor(node.left.as_ref().unwrap()) < 0 {
            // Left-Right case: first rotate left child left
            let left = node.left.take().unwrap();
            node.left = Some(rotate_left(left));
        }
        // Left-Left case (or Left-Right after first rotation)
        rotate_right(node)
    } else if bf < -1 {
        // Right-heavy
        if balance_factor(node.right.as_ref().unwrap()) > 0 {
            // Right-Left case: first rotate right child right
            let right = node.right.take().unwrap();
            node.right = Some(rotate_right(right));
        }
        // Right-Right case (or Right-Left after first rotation)
        rotate_left(node)
    } else {
        // Already balanced (-1, 0, or 1)
        node
    }
}
```

Notice how the double-rotation cases (Left-Right, Right-Left) decompose into two single
rotations. The first rotation converts them into the simpler single-rotation case, and
then the second rotation fixes the imbalance. This keeps the code compact.

### Insertion

Insertion in an AVL tree is insertion in a BST, plus rebalancing on the way back up.

```rust
fn insert<K: Ord, V>(
    root: Option<Box<AvlNode<K, V>>>,
    key: K,
    value: V,
) -> Box<AvlNode<K, V>> {
    let mut node = match root {
        None => return Box::new(AvlNode::new(key, value)),
        Some(node) => node,
    };

    match key.cmp(&node.key) {
        std::cmp::Ordering::Less => {
            let left = node.left.take();
            node.left = Some(insert(left, key, value));
        }
        std::cmp::Ordering::Greater => {
            let right = node.right.take();
            node.right = Some(insert(right, key, value));
        }
        std::cmp::Ordering::Equal => {
            // Key already exists -- update the value
            node.value = value;
            return node;
        }
    }

    rebalance(node)
}
```

The recursion walks down to find the insertion point, creates the new node, then
rebalances *every ancestor* on the way back up. In practice, at most one or two
rotations are needed per insertion (this is a provable property of AVL trees), but
we check every ancestor because the height updates must propagate up regardless.

### Walking Through an Example

Let us insert 1, 2, 3, 4, 5, 6, 7 into an AVL tree step by step. This is the
worst-case input for a plain BST, but the AVL tree handles it gracefully.

```
Insert 1:       Insert 2:        Insert 3 (triggers rotation):

    1               1                 1           rotate_left(1)
                     \                 \           ------------>
                      2                 2                2
                                         \              / \
                                          3            1   3

bf(1) after inserting 3 = -2, right child bf(2) = -1
--> Right-Right case, left rotation at 1.
```

```
Insert 4:              Insert 5 (triggers rotation):

      2                      2                       2
     / \                    / \                     / \
    1   3                  1   3                   1   4
         \                      \     rotate       / \
          4                      4   -------->    3   5
                                  \
                                   5

bf(3) = -2, right child bf(4) = -1
--> Right-Right case, left rotation at 3.
```

```
Insert 6 (triggers rotation):      Insert 7 (triggers rotation):

      2                                   4
     / \                                 / \
    1   4         rotate_left(2)        2   6
       / \        ------------>        / \ / \
      3   5                           1  3 5  7
           \
            6

bf(2) = -2, right child bf(4) = -1
--> Right-Right case, left rotation at 2.

Then insert 7 to the right of 6 -- bf(5)=-2, left rotation at 5 gives us:

            4
           / \
          2   6
         / \ / \
        1  3 5  7
```

Seven sequential values, and the tree is perfectly balanced. Height is 2 instead of 6.
Every search takes at most 3 comparisons instead of 7.

---

## Deletion (The Concept)

Deletion in an AVL tree follows the same pattern as deletion in a regular BST (from
Lesson 17), with rebalancing on the way back up. The three BST deletion cases still
apply:

1. **Leaf node**: just remove it.
2. **One child**: replace the node with its child.
3. **Two children**: swap with the in-order successor (or predecessor), then delete
   from the subtree.

After any deletion, you walk back up the tree rebalancing each ancestor. The difference
from insertion: deletion can cause rotations at *multiple* levels (up to O(log n)
rotations in the worst case), whereas insertion causes at most two.

We will not write out the full deletion code here -- it follows the same structural
pattern as insertion but with more cases to handle. The important point is that the
rebalancing logic (the `rebalance` function above) is identical. You just call it on
each ancestor after the structural change.

---

## Time Complexity Summary

Because the AVL invariant guarantees h = O(log n), all operations inherit that bound:

```
Operation       Average     Worst Case    Space
─────────────────────────────────────────────────
Search          O(log n)    O(log n)      O(1)
Insert          O(log n)    O(log n)      O(1)*
Delete          O(log n)    O(log n)      O(1)*
Min / Max       O(log n)    O(log n)      O(1)
In-order walk   O(n)        O(n)          O(log n)
```

(*) O(1) amortized extra space for the operation itself, not counting the node storage.
The recursive implementation uses O(log n) stack space; an iterative version with parent
pointers can achieve O(1).

Compare this to a plain BST, where the worst-case column reads O(n) for search, insert,
and delete. That is the entire value proposition of balanced BSTs: you trade a constant-
factor overhead on mutations (rebalancing) for the *elimination* of the worst case.

---

## AVL Trees vs. Red-Black Trees

You will encounter red-black trees in the wild more often than AVL trees. Both are
self-balancing BSTs with O(log n) guarantees. The difference is in the details.

**AVL trees:**
- Stricter balance: height difference of at most 1 at every node.
- Slightly faster lookups: the tree is more tightly balanced, so the height is lower
  (~1.44 * log2(n) vs ~2 * log2(n) for red-black trees).
- Slightly slower insertions and deletions: stricter balance means more rotations on
  average.

**Red-Black trees:**
- Looser balance: uses a "coloring" scheme (red and black nodes) with rules that
  guarantee the longest path is at most twice the shortest path.
- Slightly slower lookups: the tree can be taller than an AVL tree.
- Slightly faster insertions and deletions: looser balance means fewer rotations on
  average. Insertion requires at most 2 rotations; deletion requires at most 3.
- Used in many standard libraries: Java's `TreeMap`, C++'s `std::map`, Linux kernel's
  `rbtree`.

**The trade-off in one sentence**: AVL trees are better when you read more than you
write; red-black trees are better when writes are frequent. In practice the difference
is small -- both are O(log n) for everything.

We are not going to implement a red-black tree in this course. The concept matters more
than the implementation: *multiple balancing strategies exist, all achieving O(log n),
with different constant-factor trade-offs*.

---

## Other Self-Balancing Trees Worth Knowing About

AVL and red-black trees are not the only options. Here are three more you will encounter
in real systems:

**B-Trees (and B+ Trees)**
- Instead of binary (two children), B-tree nodes hold multiple keys and have multiple
  children (a "branching factor" of hundreds or thousands).
- Optimized for systems where reading a large block of data is cheap but seeking to a
  new location is expensive -- i.e., disks and SSDs. Each node is sized to fit one disk
  page.
- Used in virtually every database index (PostgreSQL, MySQL, SQLite) and in filesystems
  (NTFS, ext4, Btrfs).
- This is what Rust's `BTreeMap` and `BTreeSet` are based on. Despite the name
  suggesting "binary tree," the B stands for the original authors (Bayer and McCreight).
  Rust's B-tree uses a branching factor of ~11 and is cache-friendly, which often makes
  it faster than a binary tree in practice.

**Splay Trees**
- Instead of maintaining a strict balance invariant, splay trees move every accessed
  node to the root using rotations ("splaying").
- No guaranteed O(log n) for a *single* operation -- individual operations can be O(n).
  But they achieve O(log n) *amortized* over a sequence of operations.
- Their strength: frequently accessed elements naturally migrate to the root, giving
  them cache-like behavior. If you have a working set of "hot" keys, splay trees can
  outperform stricter trees.
- Used in some memory allocators, caching systems, and network routers.

**Treaps (Tree + Heap)**
- Each node has a BST key and a random priority. The tree is a BST with respect to keys
  and a heap with respect to priorities.
- The random priorities make the tree probabilistically balanced -- expected O(log n)
  height, like a randomized BST.
- Simple to implement and used in competitive programming for their clean split/merge
  operations.

---

## In Practice: You Rarely Write Your Own

Here is the honest truth: in production Rust code, you will almost never implement a
balanced BST yourself. The standard library gives you:

- **`BTreeMap<K, V>`** and **`BTreeSet<K>`** -- balanced tree-based ordered collections,
  backed by a B-tree. O(log n) search, insert, delete. Supports ordered iteration,
  range queries, and `entry` API.
- **`HashMap<K, V>`** and **`HashSet<K>`** -- for when you do not need ordering and want
  O(1) average-case performance.

```rust
use std::collections::BTreeMap;

fn main() {
    let mut scores = BTreeMap::new();

    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Carol", 92);
    scores.insert("Dave", 78);
    scores.insert("Eve", 99);

    // O(log n) lookup
    if let Some(score) = scores.get("Carol") {
        println!("Carol scored {score}");
    }

    // Ordered iteration -- this is the killer feature of BTreeMap over HashMap
    for (name, score) in &scores {
        println!("{name}: {score}");
    }
    // Output is alphabetical: Alice, Bob, Carol, Dave, Eve

    // Range queries -- get everyone scoring 85 or above
    for (name, score) in scores.range("B".."D") {
        println!("{name}: {score}");
    }
    // Output: Bob: 87, Carol: 92
}
```

So why study AVL trees at all?

1. **Understanding guarantees.** When you choose `BTreeMap` over `HashMap`, you should
   know *why* it guarantees O(log n). When someone asks "why not just use a sorted
   array?" you should be able to explain why insertion into a sorted array is O(n) while
   insertion into a balanced tree is O(log n).

2. **Interview and problem-solving fluency.** Many problems hinge on understanding that
   balanced BSTs give you O(log n) search, insert, delete, *and* ordered access. You
   need the mental model even if you never write the rotations by hand.

3. **Debugging and performance reasoning.** If a `BTreeMap` operation is slower than you
   expect, understanding the internal structure helps you reason about why. Knowing what
   rotations cost helps you understand write amplification in databases that use B-trees.

4. **It makes everything else easier.** Understanding AVL rotations makes red-black
   trees, B-trees, and other balanced structures less mysterious. The core idea --
   "detect imbalance, rotate to fix it" -- is the same everywhere.

---

## Key Takeaways

1. **Unbalanced BSTs degenerate to O(n).** Sorted or nearly-sorted input creates a
   linked list. This is not a theoretical curiosity -- it happens in practice whenever
   data has order (timestamps, auto-incrementing IDs, alphabetical names).

2. **AVL trees maintain |balance factor| <= 1 at every node.** This guarantees height
   <= 1.44 * log2(n), which means all operations are O(log n) worst-case.

3. **Four rotation cases** handle all imbalances: LL (single right rotation), RR
   (single left rotation), LR (double: left then right), RL (double: right then left).
   All four produce the same balanced three-node subtree shape.

4. **Rotations are O(1) local operations.** They rearrange two or three nodes and their
   subtrees. The rest of the tree is unaffected.

5. **AVL trees favor reads; red-black trees favor writes.** Both are O(log n). The
   constant factors differ slightly. In practice, this rarely matters.

6. **B-trees win on real hardware.** By packing many keys per node, B-trees minimize
   cache misses and disk seeks. That is why Rust uses `BTreeMap`, databases use B-tree
   indexes, and filesystems use B-tree variants.

7. **Use `BTreeMap` in production.** The value of this lesson is not "now go implement
   your own AVL tree in production code." It is "now you understand *why* ordered map
   operations are O(log n), and you can make informed decisions about which data
   structure to reach for."

---

## What's Next

With balanced BSTs understood, we have the foundation for more advanced tree structures.
In upcoming lessons we will look at heaps (a different kind of tree with different
invariants), tries (trees specialized for string operations), and graph representations
that build on everything we have covered so far.
