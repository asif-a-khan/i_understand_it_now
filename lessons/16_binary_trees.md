# Lesson 16: Binary Trees & Traversals (BFS / DFS)

## Why Trees?

Every data structure you have seen so far has been *linear*: arrays, linked lists, stacks,
queues. Elements live in a sequence -- one after another. That model works for many problems,
but the real world is full of *hierarchical* relationships.

- A file system: directories contain subdirectories which contain files.
- An HTML document: `<html>` contains `<body>` which contains `<div>` which contains `<p>`.
- A family tree: grandparents have children who have children.
- A tournament bracket: two players compete, the winner advances to the next round.
- An organizational chart: CEO has VPs, VPs have directors, directors have managers.
- A decision process: yes/no at each step, branching into different outcomes.

These are all trees. A tree captures the idea of "one thing connects to several things, each
of which connects to several more things, but there is no going back up without retracing
your steps." Once you see the pattern, you will notice it everywhere.

Trees are also the backbone of many algorithms and data structures you will encounter later:
binary search trees, heaps (you saw these in Lesson 12), tries, segment trees, and expression
parsers. This lesson builds the foundation.

---

## What Is a Binary Tree?

A **tree** is a connected, acyclic graph. In plainer terms: a collection of **nodes**
connected by **edges**, where there is exactly one path between any two nodes, and no loops.

A **binary tree** is a tree where each node has **at most two children**, conventionally
called **left** and **right**.

```
              [10]             <-- root (the single entry point)
             /    \
          [5]      [15]        <-- children of root
         /   \        \
       [3]   [7]      [20]    <-- leaves are nodes with no children
       /
     [1]
```

### Terminology

Here is the vocabulary you need. Refer back to the diagram above.

| Term           | Definition                                                        | Example from diagram     |
|----------------|-------------------------------------------------------------------|--------------------------|
| **Node**       | An element in the tree, holding a value                           | `[10]`, `[5]`, `[3]`    |
| **Edge**       | A connection between a parent and child                           | `[10]--[5]`, `[5]--[3]` |
| **Root**       | The topmost node (no parent)                                      | `[10]`                   |
| **Leaf**       | A node with no children                                           | `[7]`, `[20]`, `[1]`    |
| **Parent**     | The node directly above (connected by one edge)                   | `[5]` is parent of `[3]`|
| **Child**      | A node directly below                                             | `[3]` is child of `[5]` |
| **Subtree**    | A node and all its descendants                                    | `[5]` with `[3],[7],[1]` |
| **Depth**      | Distance (edges) from the root to a node                          | Depth of `[3]` = 2      |
| **Height**     | Distance (edges) from a node to its deepest descendant            | Height of `[5]` = 2     |
| **Height of tree** | Height of the root node                                       | 3 (root to `[1]`)       |
| **Level**      | All nodes at the same depth                                       | Level 1: `[5]`, `[15]`  |

**Depth** counts *down* from the root. **Height** counts *up* from the leaves. The root has
depth 0. A leaf has height 0. The height of the tree is the height of its root -- equivalently,
the maximum depth of any node.

### The Family Tree Analogy

A binary tree maps naturally to a family tree (with a simplification: each person has at most
two children).

```
                [Grandparent]          depth 0
                /            \
        [Parent A]        [Parent B]   depth 1
        /       \               \
  [Child 1] [Child 2]      [Child 3]  depth 2
```

The root is the eldest ancestor. Leaves are the people with no children. Depth is the number
of generations below the founding ancestor. An "uncle" is your parent's sibling (the other
child of the grandparent node). This analogy is imperfect -- real family trees are not binary
-- but it gives you the right intuition for parent-child relationships in trees.

### The File System Analogy

Your file system is a tree (usually not binary, but the concept is the same):

```
  /
  ├── home/
  │   ├── ak/
  │   │   ├── Documents/
  │   │   └── .config/
  │   └── guest/
  └── etc/
      ├── nginx/
      └── ssh/
```

The root directory `/` is the root node. Each directory is a node whose children are its
subdirectories and files. When you run `find /home -name "*.rs"`, you are doing a tree
traversal -- visiting every node under `/home` and checking a condition. That is exactly
what we will implement in this lesson.

---

## Types of Binary Trees

Not all binary trees are created equal. These distinctions matter for understanding algorithm
complexities and data structure guarantees.

```
  FULL                 COMPLETE              PERFECT              BALANCED
  Every node has       All levels filled     All levels fully     Heights of left
  0 or 2 children.     except possibly       filled. Every leaf   and right subtrees
  No "only child"      the last, which is    at the same depth.   differ by at most 1
  nodes.               filled left-to-right.                      at every node.

      [A]                  [A]                  [A]                  [A]
     /   \                /   \                /   \                /   \
   [B]   [C]            [B]   [C]            [B]   [C]            [B]   [C]
         / \           / \   /              / \   / \            / \
       [D] [E]       [D] [E][F]           [D][E][F][G]        [D] [E]
```

| Type         | Key property                                           | Why it matters                                |
|--------------|--------------------------------------------------------|-----------------------------------------------|
| **Full**     | Every node has 0 or 2 children                         | Simplifies some proofs and algorithms         |
| **Complete** | Filled level by level, left to right                   | Can be stored in an array (heaps use this)    |
| **Perfect**  | All levels completely filled                            | Has exactly 2^h - 1 nodes at height h         |
| **Balanced** | No subtree is much deeper than its sibling             | Guarantees O(log n) height                    |

The most important distinction in practice is **balanced vs unbalanced**. A balanced binary
tree with n nodes has height O(log n). An unbalanced tree can degrade to a linked list with
height O(n), which destroys the performance guarantees of tree-based algorithms.

```
  Balanced (height = 3):              Degenerate / Unbalanced (height = 6):

          [4]                         [1]
         /   \                          \
       [2]   [6]                        [2]
      / \   / \                           \
    [1] [3][5] [7]                        [3]
                                            \
  n = 7, height = 2                         [4]
  (log2(7) ≈ 2.8)                             \
                                              [5]
                                                \
                                                [6]
                                                  \
                                                  [7]

                                        n = 7, height = 6
                                        (basically a linked list)
```

---

## Representing a Binary Tree in Rust

In Lesson 07, we previewed a tree definition. Now let's build it out properly.

### Approach 1: Enum-Based (Clean, Idiomatic)

```rust
#[derive(Debug)]
enum Tree<T> {
    Empty,
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}
```

This is a direct encoding: a tree is either empty, or it is a node with a value and two
subtrees. The `Box` is required because `Tree` is recursive -- without indirection, the
compiler cannot determine the size of the type (it would be infinite).

Building a tree:

```rust
use Tree::*;

fn leaf<T>(value: T) -> Tree<T> {
    Node {
        value,
        left: Box::new(Empty),
        right: Box::new(Empty),
    }
}

// Build the tree:
//        10
//       /  \
//      5    15
//     / \     \
//    3   7    20

let tree = Node {
    value: 10,
    left: Box::new(Node {
        value: 5,
        left: Box::new(leaf(3)),
        right: Box::new(leaf(7)),
    }),
    right: Box::new(Node {
        value: 15,
        left: Box::new(Empty),
        right: Box::new(leaf(20)),
    }),
};
```

### Approach 2: Struct with Option (Common in Practice)

```rust
#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn with_children(
        value: T,
        left: Option<Box<TreeNode<T>>>,
        right: Option<Box<TreeNode<T>>>,
    ) -> Self {
        TreeNode { value, left, right }
    }
}
```

This is closer to what you see in LeetCode-style problems. A `None` child means "no subtree
here." The struct approach separates "a node exists" from "a node's children," while the enum
approach bakes the distinction into the type itself.

**Which should you use?** Both work. The enum approach is more idiomatic Rust -- pattern
matching is cleaner, and `Empty` is a first-class variant rather than an `Option` wrapper. The
struct approach is more familiar if you are coming from other languages and maps more directly
to LeetCode problem templates. This lesson uses the **enum approach** for the core examples and
shows the struct approach where relevant.

### A Helper for Building Test Trees

Building trees by hand is verbose. A small helper makes examples much more readable:

```rust
impl<T> Tree<T> {
    fn leaf(value: T) -> Self {
        Node {
            value,
            left: Box::new(Empty),
            right: Box::new(Empty),
        }
    }

    fn new(value: T, left: Tree<T>, right: Tree<T>) -> Self {
        Node {
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

// Now the same tree reads like this:
let tree = Tree::new(
    10,
    Tree::new(5, Tree::leaf(3), Tree::leaf(7)),
    Tree::new(15, Empty, Tree::leaf(20)),
);
```

---

## Tree Traversals: The Big Picture

A traversal is a systematic way to visit every node in a tree exactly once. Unlike arrays
(where you iterate left to right) or linked lists (where you follow the chain), trees have
*branching structure*, so there are multiple meaningful orderings.

The two families of traversal:

1. **Depth-First Search (DFS)**: Go as deep as possible down one path before backtracking.
   Like exploring a cave system -- you follow one tunnel to its end before coming back and
   trying the next tunnel.

2. **Breadth-First Search (BFS)**: Visit all nodes at the current depth before going deeper.
   Like a ripple spreading from a stone dropped in a pond -- level by level outward.

DFS has three standard orderings depending on *when* you process the current node relative
to its children:

| Traversal       | Order                               | Mnemonic                        |
|-----------------|-------------------------------------|---------------------------------|
| **Pre-order**   | Node, Left, Right                   | **N**LR -- process Node first   |
| **In-order**    | Left, Node, Right                   | L**N**R -- Node in the middle   |
| **Post-order**  | Left, Right, Node                   | LR**N** -- process Node last    |
| **Level-order** | Level 0, Level 1, Level 2, ...      | BFS -- breadth first            |

Let's trace all four on the same tree:

```
           [1]
          /   \
        [2]   [3]
       /  \     \
     [4]  [5]   [6]
```

---

## DFS: Pre-Order Traversal (Node, Left, Right)

Visit the node first, then recurse into the left subtree, then the right subtree.

### Step-by-Step Trace

```
Step  Action              Visit order so far
----  ------              ------------------
 1    Visit [1]           [1]
 2    Go left to [2]
 3    Visit [2]           [1, 2]
 4    Go left to [4]
 5    Visit [4]           [1, 2, 4]
 6    [4] has no left
 7    [4] has no right
 8    Back to [2], go right to [5]
 9    Visit [5]           [1, 2, 4, 5]
10    [5] has no left
11    [5] has no right
12    Back to [1], go right to [3]
13    Visit [3]           [1, 2, 4, 5, 3]
14    [3] has no left
15    Go right to [6]
16    Visit [6]           [1, 2, 4, 5, 3, 6]

Result: [1, 2, 4, 5, 3, 6]
```

### Recursive Implementation

```rust
fn preorder<T: std::fmt::Debug>(tree: &Tree<T>, result: &mut Vec<&T>) {
    match tree {
        Empty => {}
        Node { value, left, right } => {
            result.push(value);       // visit Node
            preorder(left, result);   // then Left
            preorder(right, result);  // then Right
        }
    }
}
```

### Iterative Implementation (Explicit Stack)

Recall from Lesson 07: any recursive function can be made iterative by managing your own stack.
For pre-order, push right first, then left, so that left is popped (and processed) first.

```rust
fn preorder_iterative<T>(tree: &Tree<T>) -> Vec<&T> {
    let mut result = Vec::new();
    let mut stack = vec![tree];

    while let Some(node) = stack.pop() {
        match node {
            Empty => {}
            Node { value, left, right } => {
                result.push(value);
                stack.push(right);   // push right first
                stack.push(left);    // so left is popped first
            }
        }
    }
    result
}
```

**When to use pre-order**: When you need to process a node *before* its descendants.
Serializing a tree (saving its structure to a string/file), cloning a tree, or printing a
tree's structure are all pre-order tasks. You need to know the parent before you can place
the children.

---

## DFS: In-Order Traversal (Left, Node, Right)

Recurse into the left subtree first, then visit the node, then recurse right.

### Step-by-Step Trace

```
Step  Action              Visit order so far
----  ------              ------------------
 1    Go left from [1] to [2]
 2    Go left from [2] to [4]
 3    [4] has no left
 4    Visit [4]           [4]
 5    [4] has no right
 6    Back to [2]
 7    Visit [2]           [4, 2]
 8    Go right to [5]
 9    [5] has no left
10    Visit [5]           [4, 2, 5]
11    [5] has no right
12    Back to [1]
13    Visit [1]           [4, 2, 5, 1]
14    Go right to [3]
15    [3] has no left
16    Visit [3]           [4, 2, 5, 1, 3]
17    Go right to [6]
18    [6] has no left
19    Visit [6]           [4, 2, 5, 1, 3, 6]

Result: [4, 2, 5, 1, 3, 6]
```

### Recursive Implementation

```rust
fn inorder<T>(tree: &Tree<T>, result: &mut Vec<&T>) {
    match tree {
        Empty => {}
        Node { value, left, right } => {
            inorder(left, result);    // Left first
            result.push(value);       // then Node
            inorder(right, result);   // then Right
        }
    }
}
```

### Iterative Implementation

In-order iterative is trickier than pre-order. You need to push all left descendants first,
then visit, then move right.

```rust
fn inorder_iterative<T>(tree: &Tree<T>) -> Vec<&T> {
    let mut result = Vec::new();
    let mut stack: Vec<&Tree<T>> = Vec::new();
    let mut current = tree;

    loop {
        // Push all left children onto the stack
        while let Node { left, .. } = current {
            stack.push(current);
            current = left;
        }
        // Pop from stack, visit, move right
        match stack.pop() {
            None => break,
            Some(Node { value, right, .. }) => {
                result.push(value);
                current = right;
            }
            Some(Empty) => unreachable!(),
        }
    }
    result
}
```

The idea: keep going left until you hit `Empty`. Then backtrack (pop from stack), visit that
node, and move to its right child. Repeat until the stack is empty and there is nothing left
to explore.

**When to use in-order**: In a **Binary Search Tree** (BST), in-order traversal visits nodes
in sorted order. This is its signature use case. If you have a BST and need the elements
sorted, do an in-order traversal. We will cover BSTs in the next lesson.

---

## DFS: Post-Order Traversal (Left, Right, Node)

Recurse into both subtrees first, then visit the node.

### Step-by-Step Trace

```
Step  Action              Visit order so far
----  ------              ------------------
 1    Go left from [1] to [2]
 2    Go left from [2] to [4]
 3    [4] has no left
 4    [4] has no right
 5    Visit [4]           [4]
 6    Back to [2], go right to [5]
 7    [5] has no left
 8    [5] has no right
 9    Visit [5]           [4, 5]
10    Back to [2]
11    Visit [2]           [4, 5, 2]
12    Back to [1], go right to [3]
13    [3] has no left
14    Go right to [6]
15    [6] has no left
16    [6] has no right
17    Visit [6]           [4, 5, 2, 6]
18    Back to [3]
19    Visit [3]           [4, 5, 2, 6, 3]
20    Back to [1]
21    Visit [1]           [4, 5, 2, 6, 3, 1]

Result: [4, 5, 2, 6, 3, 1]
```

### Recursive Implementation

```rust
fn postorder<T>(tree: &Tree<T>, result: &mut Vec<&T>) {
    match tree {
        Empty => {}
        Node { value, left, right } => {
            postorder(left, result);    // Left first
            postorder(right, result);   // then Right
            result.push(value);         // then Node
        }
    }
}
```

### Iterative Implementation

Post-order iterative is the most complex of the three DFS traversals. A clean approach is
to do a modified pre-order (Node, Right, Left) and reverse the result.

```rust
fn postorder_iterative<T>(tree: &Tree<T>) -> Vec<&T> {
    let mut result = Vec::new();
    let mut stack = vec![tree];

    while let Some(node) = stack.pop() {
        match node {
            Empty => {}
            Node { value, left, right } => {
                result.push(value);
                stack.push(left);    // push left first (opposite of pre-order)
                stack.push(right);   // so right is popped first
            }
        }
    }
    result.reverse();  // reverse NRL -> LRN
    result
}
```

Why does this work? Pre-order is NLR. If we swap the push order to get NRL, then reversing
the output gives LRN, which is post-order. This is a common interview trick worth remembering.

**When to use post-order**: When you need to process children *before* their parent. Deleting
a tree (free children before freeing the parent), computing the size of a subtree (need the
sizes of the children first), or evaluating an expression tree (evaluate operands before
applying the operator).

---

## BFS: Level-Order Traversal

Visit every node at depth 0, then depth 1, then depth 2, and so on. This uses a **queue**
instead of a stack.

### Step-by-Step Trace

```
Queue state                                Visit order so far
-----------                                ------------------
[1]                                        (start)
  Dequeue [1], enqueue children [2, 3]
[2, 3]                                     [1]
  Dequeue [2], enqueue children [4, 5]
[3, 4, 5]                                  [1, 2]
  Dequeue [3], enqueue children [6]
[4, 5, 6]                                  [1, 2, 3]
  Dequeue [4], no children
[5, 6]                                     [1, 2, 3, 4]
  Dequeue [5], no children
[6]                                         [1, 2, 3, 4, 5]
  Dequeue [6], no children
[]                                          [1, 2, 3, 4, 5, 6]

Result: [1, 2, 3, 4, 5, 6]
```

Visualized on the tree:

```
           [1]          <-- Level 0: visit first
          /   \
        [2]   [3]       <-- Level 1: visit second
       /  \     \
     [4]  [5]   [6]     <-- Level 2: visit third
```

### Implementation Using VecDeque

```rust
use std::collections::VecDeque;

fn level_order<T>(tree: &Tree<T>) -> Vec<&T> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(tree);

    while let Some(node) = queue.pop_front() {
        match node {
            Empty => {}
            Node { value, left, right } => {
                result.push(value);
                queue.push_back(left);
                queue.push_back(right);
            }
        }
    }
    result
}
```

The `VecDeque` (from Lesson 05) provides O(1) push to the back and O(1) pop from the front --
exactly the queue behavior BFS requires.

### Level-Order With Grouping by Level

Many problems ask for the output grouped by level: `[[1], [2, 3], [4, 5, 6]]`. The trick is
to process all nodes in the current level before moving on:

```rust
fn level_order_grouped<T>(tree: &Tree<T>) -> Vec<Vec<&T>> {
    let mut levels = Vec::new();
    let mut queue = VecDeque::new();

    if let Node { .. } = tree {
        queue.push_back(tree);
    }

    while !queue.is_empty() {
        let level_size = queue.len();  // number of nodes at this level
        let mut current_level = Vec::new();

        for _ in 0..level_size {
            if let Some(Node { value, left, right }) = queue.pop_front() {
                current_level.push(value);
                if let Node { .. } = left.as_ref() {
                    queue.push_back(left);
                }
                if let Node { .. } = right.as_ref() {
                    queue.push_back(right);
                }
            }
        }
        levels.push(current_level);
    }
    levels
}
```

**When to use BFS / level-order**: When the problem involves levels, layers, or shortest
distance from the root. Finding the minimum depth of a tree, printing zigzag level-order,
connecting nodes at the same level, or finding the rightmost node at each level are all BFS
problems.

---

## Traversal Summary: All Four Side by Side

Using the tree:

```
           [1]
          /   \
        [2]   [3]
       /  \     \
     [4]  [5]   [6]
```

| Traversal    | Order rule       | Result               | Data structure |
|------------- |------------------|----------------------|----------------|
| Pre-order    | Node, Left, Right| `[1, 2, 4, 5, 3, 6]`| Stack (or recursion) |
| In-order     | Left, Node, Right| `[4, 2, 5, 1, 3, 6]`| Stack (or recursion) |
| Post-order   | Left, Right, Node| `[4, 5, 2, 6, 3, 1]`| Stack (or recursion) |
| Level-order  | Level by level   | `[1, 2, 3, 4, 5, 6]`| Queue          |

The mnemonic for the DFS orderings: the name tells you where **Node** goes.
- **Pre**-order: Node comes first (*pre*fix)
- **In**-order: Node goes in the middle (*in*fix)
- **Post**-order: Node comes last (*post*fix)

---

## When to Use Which Traversal

This is the practical question. Here is a guide:

| Problem pattern                                | Best traversal  | Why                                     |
|------------------------------------------------|-----------------|-----------------------------------------|
| Serialize / clone / copy a tree                | Pre-order       | Need parent info before children        |
| BST: get elements in sorted order              | In-order        | Left < Node < Right in a BST            |
| Delete / free a tree                           | Post-order      | Must free children before parent        |
| Evaluate expression tree                       | Post-order      | Operands before operator                |
| Compute height / size / max depth              | Post-order      | Need children's answers first           |
| Find minimum depth                             | BFS             | Stop at first leaf encountered          |
| Print level by level                           | BFS             | Natural level grouping                  |
| Shortest path from root to target              | BFS             | BFS visits closest nodes first          |
| Check if tree is balanced                      | Post-order      | Compare subtree heights bottom-up       |
| Path from root to a node                       | Pre-order/DFS   | Build path as you descend               |

---

## Common Patterns and Problems

Now that you have the traversals, let's apply them. These patterns appear repeatedly in
interview problems and real-world code.

### Pattern 1: Height of a Tree

The height of a tree is 0 for an empty tree (or -1, depending on convention -- we use 0 here
for Empty), and `1 + max(left_height, right_height)` otherwise. This is post-order: you need
the heights of both subtrees before you can compute the current node's height.

```rust
fn height<T>(tree: &Tree<T>) -> usize {
    match tree {
        Empty => 0,
        Node { left, right, .. } => {
            1 + height(left).max(height(right))
        }
    }
}
```

Trace on the example tree:

```
           [1]           height = 1 + max(2, 2) = 3
          /   \
        [2]   [3]        [2]: 1 + max(1,1) = 2    [3]: 1 + max(0,1) = 2
       /  \     \
     [4]  [5]   [6]      all leaves: 1 + max(0,0) = 1
```

Time: O(n) -- every node visited once. Space: O(h) for the recursive call stack, where h is
the height.

### Pattern 2: Size of a Tree (Count Nodes)

```rust
fn size<T>(tree: &Tree<T>) -> usize {
    match tree {
        Empty => 0,
        Node { left, right, .. } => {
            1 + size(left) + size(right)
        }
    }
}
```

### Pattern 3: Maximum Value

```rust
fn max_value(tree: &Tree<i32>) -> Option<i32> {
    match tree {
        Empty => None,
        Node { value, left, right } => {
            let left_max = max_value(left);
            let right_max = max_value(right);
            let mut best = *value;
            if let Some(l) = left_max {
                best = best.max(l);
            }
            if let Some(r) = right_max {
                best = best.max(r);
            }
            Some(best)
        }
    }
}
```

### Pattern 4: Path Sum (Root to Leaf)

"Does any root-to-leaf path have a sum equal to a target value?"

This is a pre-order/DFS pattern: carry the running sum down, check at each leaf.

```rust
fn has_path_sum(tree: &Tree<i32>, target: i32) -> bool {
    match tree {
        Empty => false,
        Node { value, left, right } => {
            let remaining = target - value;
            // Check if this is a leaf
            let is_leaf = matches!(left.as_ref(), Empty)
                       && matches!(right.as_ref(), Empty);
            if is_leaf {
                return remaining == 0;
            }
            has_path_sum(left, remaining) || has_path_sum(right, remaining)
        }
    }
}
```

Trace: target = 8 on this tree:

```
           [5]
          /   \
        [3]   [7]
       /
     [1]

  Path [5] -> [3] -> [1]: sum = 9, not 8
  Path [5] -> [7]:        sum = 12, not 8
  Result: false

  If target = 9: path [5] -> [3] -> [1] sums to 9, return true.
```

### Pattern 5: Invert (Mirror) a Binary Tree

The famous interview question. Swap every node's left and right children.

```rust
fn invert<T>(tree: Tree<T>) -> Tree<T> {
    match tree {
        Empty => Empty,
        Node { value, left, right } => {
            Node {
                value,
                left: Box::new(invert(*right)),   // swap: old right becomes new left
                right: Box::new(invert(*left)),    // swap: old left becomes new right
            }
        }
    }
}
```

Before and after:

```
  Before:          After:
      [1]              [1]
     /   \            /   \
   [2]   [3]        [3]   [2]
  / \      \       /      / \
[4] [5]   [6]   [6]    [5] [4]
```

### Pattern 6: Check if Two Trees Are Identical

```rust
fn is_same_tree<T: PartialEq>(a: &Tree<T>, b: &Tree<T>) -> bool {
    match (a, b) {
        (Empty, Empty) => true,
        (
            Node { value: v1, left: l1, right: r1 },
            Node { value: v2, left: l2, right: r2 },
        ) => v1 == v2 && is_same_tree(l1, l2) && is_same_tree(r1, r2),
        _ => false,  // one is Empty and the other is not
    }
}
```

### Pattern 7: Is Balanced?

A tree is balanced if, at every node, the heights of the left and right subtrees differ by
at most 1. The naive approach computes height at every node (O(n^2)). The better approach
computes height and checks balance in a single pass:

```rust
/// Returns Some(height) if balanced, None if not.
fn check_balanced<T>(tree: &Tree<T>) -> Option<usize> {
    match tree {
        Empty => Some(0),
        Node { left, right, .. } => {
            let left_h = check_balanced(left)?;   // ? propagates None
            let right_h = check_balanced(right)?;
            if left_h.abs_diff(right_h) > 1 {
                None  // not balanced
            } else {
                Some(1 + left_h.max(right_h))
            }
        }
    }
}

fn is_balanced<T>(tree: &Tree<T>) -> bool {
    check_balanced(tree).is_some()
}
```

This is O(n) time and O(h) space. The `?` operator short-circuits: if any subtree is
unbalanced (returns `None`), we stop immediately.

---

## Complexity Analysis

For a binary tree with n nodes and height h:

| Operation/Traversal | Time   | Space    | Notes                                    |
|---------------------|--------|----------|------------------------------------------|
| Any DFS traversal   | O(n)   | O(h)    | Visit every node; stack depth = height   |
| BFS traversal       | O(n)   | O(w)    | Visit every node; queue width = max level width |
| Height              | O(n)   | O(h)    | Must check every node                    |
| Size                | O(n)   | O(h)    | Must count every node                    |
| Search (unsorted)   | O(n)   | O(h)    | May need to check every node             |

Where:
- h = height of the tree. For a balanced tree, h = O(log n). For a degenerate tree, h = O(n).
- w = maximum width (max number of nodes at any level). For a complete tree, w = O(n/2) = O(n).

The space difference matters: DFS uses space proportional to *height* (deep but narrow), while
BFS uses space proportional to *width* (shallow but wide). For a balanced tree with a million
nodes, DFS uses ~20 stack frames, while BFS might have ~500,000 nodes in the queue at the last
level. For a degenerate (linked list) tree, DFS uses O(n) stack space, while BFS uses O(1)
queue space. Choose based on the tree shape you expect.

---

## Putting It All Together: A Complete Module

Here is a self-contained Rust module that ties everything together:

```rust
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Tree<T> {
    Empty,
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

use Tree::*;

impl<T> Tree<T> {
    fn leaf(value: T) -> Self {
        Node {
            value,
            left: Box::new(Empty),
            right: Box::new(Empty),
        }
    }

    fn new(value: T, left: Tree<T>, right: Tree<T>) -> Self {
        Node {
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

// ----- Traversals -----

fn preorder<T>(tree: &Tree<T>, result: &mut Vec<&T>) {
    if let Node { value, left, right } = tree {
        result.push(value);
        preorder(left, result);
        preorder(right, result);
    }
}

fn inorder<T>(tree: &Tree<T>, result: &mut Vec<&T>) {
    if let Node { value, left, right } = tree {
        inorder(left, result);
        result.push(value);
        inorder(right, result);
    }
}

fn postorder<T>(tree: &Tree<T>, result: &mut Vec<&T>) {
    if let Node { value, left, right } = tree {
        postorder(left, result);
        postorder(right, result);
        result.push(value);
    }
}

fn level_order<T>(tree: &Tree<T>) -> Vec<&T> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(tree);
    while let Some(node) = queue.pop_front() {
        if let Node { value, left, right } = node {
            result.push(value);
            queue.push_back(left);
            queue.push_back(right);
        }
    }
    result
}

// ----- Common operations -----

fn height<T>(tree: &Tree<T>) -> usize {
    match tree {
        Empty => 0,
        Node { left, right, .. } => 1 + height(left).max(height(right)),
    }
}

fn size<T>(tree: &Tree<T>) -> usize {
    match tree {
        Empty => 0,
        Node { left, right, .. } => 1 + size(left) + size(right),
    }
}

fn main() {
    //        1
    //       / \
    //      2   3
    //     / \   \
    //    4   5   6
    let tree = Tree::new(
        1,
        Tree::new(2, Tree::leaf(4), Tree::leaf(5)),
        Tree::new(3, Empty, Tree::leaf(6)),
    );

    let mut pre = Vec::new();
    preorder(&tree, &mut pre);
    println!("Pre-order:   {:?}", pre);        // [1, 2, 4, 5, 3, 6]

    let mut ino = Vec::new();
    inorder(&tree, &mut ino);
    println!("In-order:    {:?}", ino);         // [4, 2, 5, 1, 3, 6]

    let mut post = Vec::new();
    postorder(&tree, &mut post);
    println!("Post-order:  {:?}", post);        // [4, 5, 2, 6, 3, 1]

    let lvl = level_order(&tree);
    println!("Level-order: {:?}", lvl);         // [1, 2, 3, 4, 5, 6]

    println!("Height: {}", height(&tree));      // 3
    println!("Size:   {}", size(&tree));         // 6
}
```

---

## DFS vs BFS: A Mental Model

Think of tree traversal like exploring a building.

**DFS (depth-first)** is like taking the stairs all the way to the top floor, checking every
room on each floor as you go, then coming back down and trying the next staircase. You fully
explore one branch before trying another.

**BFS (breadth-first)** is like checking every room on floor 1, then every room on floor 2,
then floor 3, and so on. You fully explore one level before going deeper.

```
     DFS explores:                  BFS explores:
     1 -> 2 -> 4 (dead end)        1
     back to 2 -> 5 (dead end)     2, 3
     back to 1 -> 3 -> 6           4, 5, 6

     Goes deep first.              Goes wide first.
```

In code, the only structural difference is the data structure holding nodes to visit:

- **Stack** (LIFO) gives you DFS: you always explore the most recently discovered node.
- **Queue** (FIFO) gives you BFS: you always explore the earliest discovered node.

The recursive DFS implementations use the call stack as their implicit stack. The iterative
DFS implementations use a `Vec` as an explicit stack. BFS always uses a `VecDeque` as a queue.

---

## Exercises

1. **Count Leaves**: Write a function that counts the number of leaf nodes (nodes with no
   children) in a binary tree. What traversal does this naturally use?

2. **Minimum Depth**: Write a function that returns the minimum depth of a tree -- the
   shortest path from root to any leaf. Hint: BFS naturally finds this. Can you also solve
   it with DFS?

3. **Level Averages**: Given a binary tree of `f64` values, return a `Vec<f64>` where each
   element is the average of the values at that level. Use level-order traversal.

4. **All Root-to-Leaf Paths**: Return a `Vec<Vec<&T>>` containing every path from the root
   to a leaf. For the example tree, the result would be `[[1,2,4], [1,2,5], [1,3,6]]`.

5. **Iterative In-Order**: Implement in-order traversal iteratively using the struct-based
   `TreeNode<T>` representation (with `Option<Box<TreeNode<T>>>`). This is a classic
   interview question.

6. **Is Symmetric**: Write a function that checks if a binary tree is a mirror image of
   itself (symmetric around its center). The tree `[1, [2, [3], [4]], [2, [4], [3]]]`
   is symmetric.

7. **Diameter of a Tree**: The diameter is the length of the longest path between any two
   nodes (this path may or may not pass through the root). Hint: at each node, the longest
   path through that node is `left_height + right_height`. Track the maximum across all nodes.

8. **Right Side View**: Given a binary tree, return the values visible from the right side
   -- the last node at each level. Use BFS with level grouping.

---

## Key Takeaways

1. **A binary tree is a recursive data structure.** Each node contains a value and two
   subtrees, each of which is itself a tree. In Rust, represent this with either an `enum`
   (`Empty` / `Node`) or a `struct` with `Option<Box<...>>` children.

2. **There are four standard traversals.** Pre-order (NLR), in-order (LNR), and post-order
   (LRN) are depth-first. Level-order is breadth-first. The name tells you when the node
   is visited relative to its children.

3. **DFS uses a stack; BFS uses a queue.** Recursive DFS uses the call stack implicitly.
   Iterative DFS uses a `Vec`. BFS uses a `VecDeque`.

4. **Most tree problems follow one of a few patterns**: compute height, compute size, search
   for a value, check a property (balanced, symmetric), accumulate along a path, or transform
   the tree (invert, prune). All of these reduce to "visit every node and combine results."

5. **Recursion is the natural tool for trees.** The data structure is recursive, so the code
   should be too. Iterative versions exist and are useful when stack depth is a concern, but
   recursive solutions are almost always clearer.

6. **Balanced trees have O(log n) height; degenerate trees have O(n) height.** This
   distinction affects the space complexity of DFS (which is O(h)) and the time complexity
   of operations on binary search trees (next lesson).

7. **Choose DFS when** the problem involves paths, subtree properties, or depth. **Choose
   BFS when** the problem involves levels, layers, or shortest distance from the root.

---

Next lesson: [17 - Binary Search Trees](./17_binary_search_trees.md)
