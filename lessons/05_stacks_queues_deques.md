# Lesson 05: Stacks & Queues

Two of the most fundamental data structures in computer science, and two that show up
*constantly* in real code and interview problems. If arrays are the bread of data
structures, stacks and queues are the butter. They are both simple in concept but
surprisingly powerful when you recognize the patterns they solve.

---

## The Core Idea: Ordering Discipline

Both stacks and queues are **linear collections** where the defining characteristic is
the *order in which you remove elements*. They both support two primary operations: put
something in, take something out. The difference is *which* element comes out.

| Structure | Discipline | Mnemonic |
|-----------|-----------|----------|
| **Stack** | Last In, First Out (LIFO) | A stack of plates |
| **Queue** | First In, First Out (FIFO) | A line at the grocery store |

That's it. That's the whole difference. Everything else flows from this.

---

## Stacks (LIFO)

### The Mental Model

Think of a stack of plates in a cafeteria. You can only interact with the top plate:

- **Push**: place a new plate on top
- **Pop**: remove the top plate
- **Peek**: look at the top plate without removing it

You never reach into the middle. You never pull from the bottom. Only the top.

```
    Push 10       Push 20       Push 30       Pop (returns 30)

   +------+     +------+     +------+       +------+
   |  10  |     |  20  |     |  30  | <-top |  20  | <-top
   +------+     +------+     +------+       +------+
                |  10  |     |  20  |       |  10  |
                +------+     +------+       +------+
                             |  10  |
                             +------+
```

### Real-World Stack Examples

- **Browser back button**: Each page you visit gets pushed onto a history stack.
  Hitting "back" pops the most recent page.
- **Undo in a text editor**: Each edit is pushed. Ctrl+Z pops the last edit.
- **Function call stack**: When `main()` calls `foo()` which calls `bar()`, each
  call frame is pushed onto the call stack. When `bar()` returns, its frame is popped.
  This is literally why a "stack overflow" is called that -- too many frames pushed
  without popping.

### Stacks in Rust: `Vec<T>`

Rust has no dedicated `Stack` type. You don't need one. `Vec<T>` already is one:

```rust
fn main() {
    let mut stack: Vec<i32> = Vec::new();

    // Push onto the stack
    stack.push(10);
    stack.push(20);
    stack.push(30);

    // Peek at the top element
    assert_eq!(stack.last(), Some(&30));

    // Pop from the stack
    assert_eq!(stack.pop(), Some(30));  // returns Option<T>
    assert_eq!(stack.pop(), Some(20));
    assert_eq!(stack.pop(), Some(10));
    assert_eq!(stack.pop(), None);      // empty stack returns None
}
```

Why does `Vec` work perfectly as a stack? Because `push` and `pop` operate on the
**end** of the vector, which is O(1) amortized. No shifting of elements required.
The end of the `Vec` *is* the top of the stack.

### Operation Complexity (Vec as Stack)

| Operation | Time | Notes |
|-----------|------|-------|
| `push`    | O(1) amortized | Occasional resize, but amortized constant |
| `pop`     | O(1) | Just decrements the length |
| `last` (peek) | O(1) | Returns a reference to the last element |
| `len`     | O(1) | |
| `is_empty`| O(1) | |

### How It Looks in Memory

A `Vec`-based stack is just a contiguous block of memory with a pointer to the end:

```
  Vec { ptr, len: 4, capacity: 8 }
       |
       v
  +----+----+----+----+----+----+----+----+
  | 10 | 20 | 30 | 40 |    |    |    |    |
  +----+----+----+----+----+----+----+----+
    0    1    2    3    4    5    6    7
                        ^
                        |
                   top of stack (len = 4)
                   next push goes here
```

This is *great* for cache performance -- everything is contiguous in memory.

### Building a Stack Struct (Wrapper Pattern)

Sometimes you want a dedicated type to make intent clear and restrict the API.
Wrapping a `Vec` is the idiomatic way:

```rust
/// A stack backed by a Vec. Only exposes stack operations.
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut s = Stack::new();
    s.push("hello");
    s.push("world");
    assert_eq!(s.peek(), Some(&"world"));
    assert_eq!(s.pop(), Some("world"));
    assert_eq!(s.len(), 1);
}
```

This is a thin wrapper. Zero overhead -- the compiler will inline everything. The
benefit is *semantic clarity*: when a function takes a `Stack<T>`, you know exactly
what operations are expected.

---

## Queues (FIFO)

### The Mental Model

A queue is a line of people. The first person in line is the first person served:

- **Enqueue** (push_back): join the back of the line
- **Dequeue** (pop_front): the person at the front is served and leaves
- **Peek** (front): look at who's next without serving them

```
  Enqueue A    Enqueue B    Enqueue C    Dequeue (returns A)

  front->      front->      front->      front->
  +---+        +---+---+    +---+---+---+  +---+---+
  | A |        | A | B |    | A | B | C |  | B | C |
  +---+        +---+---+    +---+---+---+  +---+---+
      <-back       <-back           <-back      <-back
```

### Real-World Queue Examples

- **Printer queue**: Documents print in the order they were submitted.
- **Task scheduling**: OS schedulers use queues to give processes CPU time in order.
- **Message queues**: Kafka, RabbitMQ, SQS -- all fundamentally FIFO queues.
- **Breadth-first search**: BFS explores nodes level by level using a queue.

### Why Not `Vec<T>` for Queues?

You *could* use a `Vec` as a queue, but you shouldn't:

```rust
// DON'T DO THIS
let mut bad_queue: Vec<i32> = Vec::new();
bad_queue.push(1);      // enqueue at back: O(1) -- fine
bad_queue.push(2);
bad_queue.remove(0);    // dequeue from front: O(n) -- BAD
```

`remove(0)` shifts every element left by one position. On a queue with a million
items, that's a million copies on every dequeue. That's O(n) per operation, which
defeats the purpose.

### Queues in Rust: `VecDeque<T>`

The right tool is `std::collections::VecDeque` -- a **double-ended queue** backed
by a ring buffer:

```rust
use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();

    // Enqueue (push to back)
    queue.push_back(10);
    queue.push_back(20);
    queue.push_back(30);

    // Peek at the front
    assert_eq!(queue.front(), Some(&10));

    // Dequeue (pop from front)
    assert_eq!(queue.pop_front(), Some(10));
    assert_eq!(queue.pop_front(), Some(20));
    assert_eq!(queue.pop_front(), Some(30));
    assert_eq!(queue.pop_front(), None);
}
```

### How a Ring Buffer Works

`VecDeque` uses a contiguous array but tracks a `head` and `tail` index that wrap
around. This avoids ever shifting elements:

```
  Initial state after enqueue(A), enqueue(B), enqueue(C):

  +---+---+---+---+---+---+---+---+
  | A | B | C |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
    ^           ^
    head        tail

  After dequeue() returns A, dequeue() returns B:

  +---+---+---+---+---+---+---+---+
  |   |   | C |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
            ^   ^
           head tail

  After enqueue(D), enqueue(E), ..., enqueue(H) -- wraps around:

  +---+---+---+---+---+---+---+---+
  | H |   | C | D | E | F | G |   |
  +---+---+---+---+---+---+---+---+
        ^   ^
       tail head

  The "front" is at index 2 (C), and it wraps around through D,E,F,G,H.
```

No copying. No shifting. Just moving two indices. This is why both `push_back`
and `pop_front` are O(1) amortized.

### Operation Complexity (VecDeque)

| Operation | Time | Notes |
|-----------|------|-------|
| `push_back` | O(1) amortized | Enqueue |
| `pop_front` | O(1) | Dequeue |
| `push_front` | O(1) amortized | Can also push to front (it's a deque) |
| `pop_back` | O(1) | Can also pop from back |
| `front` / `back` | O(1) | Peek either end |
| `len` | O(1) | |
| Index access `[i]` | O(1) | Unlike linked-list-based queues |

The "deque" in `VecDeque` means "double-ended queue" -- you get O(1) operations on
*both* ends. This is strictly more powerful than a simple queue, but you can use it
as one by only using `push_back` and `pop_front`.

---

## Side-by-Side Comparison

```
         STACK (LIFO)                    QUEUE (FIFO)
   +---+                          +---+---+---+---+
   | 3 | <-- top (push/pop)       | 1 | 2 | 3 |   |
   +---+                          +---+---+---+---+
   | 2 |                            ^               ^
   +---+                          front            back
   | 1 |                        (dequeue)        (enqueue)
   +---+

   Last in = first out            First in = first out

   Rust: Vec<T>                   Rust: VecDeque<T>
   push() / pop()                 push_back() / pop_front()
```

---

## When to Use Each

### Use a Stack When:

- You need to **reverse** things (LIFO naturally reverses insertion order)
- You need to process **nested structures** (parentheses, HTML tags, JSON)
- You're implementing **DFS** (depth-first search)
- You need **backtracking** (maze solving, undo systems)
- You're converting recursion to iteration (the call stack *is* a stack)

### Use a Queue When:

- You need to process things **in order of arrival**
- You're implementing **BFS** (breadth-first search)
- You're building a **producer/consumer** pipeline
- You need **fair scheduling** (first come, first served)
- You're doing **level-order traversal** of a tree

---

## Classic Problem 1: Balanced Parentheses

The quintessential stack problem. Given a string of brackets, determine if they're
properly nested.

```
  "(())"     -> valid
  "({[]})"   -> valid
  "(]"       -> invalid
  "(()"      -> invalid
```

The insight: every closing bracket must match the *most recent* unmatched opening
bracket. "Most recent" screams stack.

```rust
fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {} // ignore non-bracket characters
        }
    }

    stack.is_empty() // any unmatched openers left?
}

fn main() {
    assert!(is_balanced("({[]})"));
    assert!(is_balanced(""));
    assert!(!is_balanced("(]"));
    assert!(!is_balanced("(()"));
    println!("All balanced-parentheses tests passed.");
}
```

Walk through `({[]})`:

```
  char   stack (top on right)    action
  ----   --------------------    ------
  '('    ['(']                   push
  '{'    ['(', '{']              push
  '['    ['(', '{', '[']         push
  ']'    ['(', '{']              pop '[' -- matches ']'
  '}'    ['(']                   pop '{' -- matches '}'
  ')'    []                      pop '(' -- matches ')'
  END    [] -- empty => valid
```

**Time**: O(n), one pass. **Space**: O(n) worst case (all openers).

---

## Classic Problem 2: Reverse a Sequence

Because a stack is LIFO, pushing then popping reverses order:

```rust
fn reverse_with_stack<T: Clone>(items: &[T]) -> Vec<T> {
    let mut stack: Vec<T> = Vec::new();

    for item in items {
        stack.push(item.clone());
    }

    let mut reversed = Vec::with_capacity(items.len());
    while let Some(item) = stack.pop() {
        reversed.push(item);
    }

    reversed
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(reverse_with_stack(&nums), vec![5, 4, 3, 2, 1]);

    let word: Vec<char> = "hello".chars().collect();
    let reversed: String = reverse_with_stack(&word).into_iter().collect();
    assert_eq!(reversed, "olleh");
}
```

In practice you'd just call `.rev()` or `[..].reverse()` in Rust, but the
stack-based reversal is the foundational technique behind many algorithms.

---

## Classic Problem 3: BFS Uses a Queue

Breadth-first search explores a graph level by level. The queue ensures you visit
all neighbors at distance `d` before any neighbor at distance `d+1`.

```rust
use std::collections::VecDeque;

/// BFS on an adjacency list graph. Returns distances from `start` to all
/// reachable nodes, or -1 if unreachable.
fn bfs(adj: &[Vec<usize>], start: usize) -> Vec<i32> {
    let n = adj.len();
    let mut dist = vec![-1i32; n];
    let mut queue = VecDeque::new();

    dist[start] = 0;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adj[node] {
            if dist[neighbor] == -1 {
                dist[neighbor] = dist[node] + 1;
                queue.push_back(neighbor);
            }
        }
    }

    dist
}

fn main() {
    //   0 -- 1 -- 3
    //   |         |
    //   2 ------- 4
    let adj = vec![
        vec![1, 2],    // 0's neighbors
        vec![0, 3],    // 1's neighbors
        vec![0, 4],    // 2's neighbors
        vec![1, 4],    // 3's neighbors
        vec![2, 3],    // 4's neighbors
    ];

    let distances = bfs(&adj, 0);
    assert_eq!(distances, vec![0, 1, 1, 2, 2]);
    // Node 0: distance 0 (start)
    // Node 1: distance 1 (0 -> 1)
    // Node 2: distance 1 (0 -> 2)
    // Node 3: distance 2 (0 -> 1 -> 3)
    // Node 4: distance 2 (0 -> 2 -> 4)
}
```

Step-by-step trace:

```
  distances = [0, 1, 1, 2, 2]

  BFS order from node 0:

    Queue: [0]           Visit 0 (dist=0), enqueue neighbors 1,2
    Queue: [1, 2]        Visit 1 (dist=1), enqueue neighbor 3
    Queue: [2, 3]        Visit 2 (dist=1), enqueue neighbor 4
    Queue: [3, 4]        Visit 3 (dist=2), neighbor 4 already queued
    Queue: [4]           Visit 4 (dist=2), neighbors already visited
    Queue: []            Done.
```

The queue guarantees we process all distance-1 nodes before any distance-2 nodes.
If you replaced the queue with a stack, you'd get DFS instead.

---

## DFS Uses a Stack

Depth-first search plunges as deep as possible before backtracking. Recursive DFS
uses the call stack implicitly. Iterative DFS uses an explicit stack:

```rust
fn dfs_iterative(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut order = Vec::new();
    let mut stack = Vec::new();

    stack.push(start);

    while let Some(node) = stack.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        order.push(node);

        // Push neighbors in reverse order so we visit them
        // in the original order (stack reverses things)
        for &neighbor in adj[node].iter().rev() {
            if !visited[neighbor] {
                stack.push(neighbor);
            }
        }
    }

    order
}
```

Stack -> DFS. Queue -> BFS. Swap the data structure, swap the traversal order.
This is one of those elegant symmetries worth committing to memory.

---

## Introduction to Monotonic Stacks

A monotonic stack is a stack that maintains a specific ordering invariant -- either
strictly increasing or strictly decreasing from bottom to top. It is a *technique*,
not a separate data structure. You use a regular `Vec`, but enforce the ordering
by popping elements that violate it before pushing.

### The Problem It Solves

The classic application: **"For each element, find the next greater element."**

Given `[2, 1, 4, 3]`, the answer is `[4, 4, -1, -1]`:
- 2's next greater element is 4
- 1's next greater element is 4
- 4 has no greater element to its right -> -1
- 3 has no greater element to its right -> -1

Brute force is O(n^2). A monotonic stack solves it in O(n).

### How It Works

Maintain a stack of *indices* whose values are in decreasing order. When you encounter
a value larger than the stack's top, that value is the "next greater element" for the
top. Pop and record the answer, then keep checking.

```rust
fn next_greater_elements(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![-1i32; n];
    let mut stack: Vec<usize> = Vec::new(); // stack of indices

    for i in 0..n {
        // Pop all elements that nums[i] is greater than
        while let Some(&top) = stack.last() {
            if nums[i] > nums[top] {
                result[top] = nums[i];
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    result
}

fn main() {
    assert_eq!(next_greater_elements(&[2, 1, 4, 3]), vec![4, 4, -1, -1]);
    assert_eq!(next_greater_elements(&[1, 2, 3, 4]), vec![2, 3, 4, -1]);
    assert_eq!(next_greater_elements(&[4, 3, 2, 1]), vec![-1, -1, -1, -1]);
    println!("All next-greater-element tests passed.");
}
```

Walk through `[2, 1, 4, 3]`:

```
  i=0, nums[0]=2:  stack=[]       -> push 0.          stack=[0]
  i=1, nums[1]=1:  1 < nums[0]=2  -> push 1.          stack=[0,1]
  i=2, nums[2]=4:  4 > nums[1]=1  -> result[1]=4, pop. stack=[0]
                    4 > nums[0]=2  -> result[0]=4, pop. stack=[]
                    push 2.                             stack=[2]
  i=3, nums[3]=3:  3 < nums[2]=4  -> push 3.          stack=[2,3]

  Remaining in stack (indices 2,3) have no next greater -> stay as -1.
  result = [4, 4, -1, -1]
```

**Time**: O(n). Each element is pushed once and popped at most once.
**Space**: O(n) for the stack.

Monotonic stacks show up in problems like:
- Next greater/smaller element
- Largest rectangle in a histogram
- Trapping rain water
- Stock span problems
- Sliding window maximum (combined with deque)

Don't worry about mastering these now. Just know the pattern exists: when you need
to efficiently find "the next element satisfying some comparison," think monotonic stack.

---

## Summary: The Cheat Sheet

```
  STACK                              QUEUE
  -----                              -----
  LIFO (Last In, First Out)          FIFO (First In, First Out)

  Rust type: Vec<T>                  Rust type: VecDeque<T>
    .push(val)                         .push_back(val)
    .pop() -> Option<T>                .pop_front() -> Option<T>
    .last() -> Option<&T>              .front() -> Option<&T>

  All O(1) amortized                 All O(1) amortized

  Use for:                           Use for:
    - DFS                              - BFS
    - Backtracking                     - Level-order traversal
    - Undo/redo                        - Fair scheduling
    - Parsing (brackets, etc.)         - Producer/consumer
    - Monotonic stack problems         - Streaming/buffering
    - Reversing sequences              - Order-preserving processing
```

### Key Takeaways

1. **Rust has no built-in `Stack` type** -- `Vec<T>` already has the right API.
   `push` and `pop` on a `Vec` operate on the end, giving you O(1) LIFO behavior.

2. **Never use `Vec` as a queue.** `remove(0)` is O(n). Use `VecDeque` which uses a
   ring buffer for O(1) operations on both ends.

3. **Stack = DFS, Queue = BFS.** This mapping is fundamental. Swapping the data
   structure in a graph traversal swaps the traversal order.

4. **Monotonic stacks** are a powerful technique for "next greater/smaller" problems,
   running in O(n) where brute force takes O(n^2).

5. Both structures are building blocks. You'll see them embedded inside more complex
   algorithms constantly -- priority queues build on queues conceptually, expression
   evaluation uses stacks, BFS-based shortest paths use queues, etc.

---

## Exercises

Try these in `dsa-forge` to solidify the concepts:

1. **Balanced Parentheses**: Implement the checker shown above. Then extend it to
   handle edge cases: empty strings, strings with non-bracket characters mixed in.

2. **Implement a Queue Using Two Stacks**: Use two `Vec<T>`s to simulate a queue.
   Enqueue pushes to stack A. Dequeue pops from stack B; if B is empty, transfer
   all of A into B first. What's the amortized time complexity?

3. **Min Stack**: Design a stack that supports `push`, `pop`, `peek`, and
   `get_min` -- all in O(1) time. Hint: use an auxiliary stack.

4. **Next Greater Element**: Implement the monotonic stack solution above, then try
   the circular variant (the array wraps around).

5. **BFS Shortest Path**: Given an unweighted graph, find the shortest path between
   two nodes using BFS. Return the actual path, not just the distance.
