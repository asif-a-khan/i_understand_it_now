# Lesson 04: Linked Lists (Singly & Doubly)

## What Problem Do Linked Lists Solve?

You already know arrays (and Rust's `Vec<T>`). They store elements contiguously in memory --
one element right after the next, like books lined up on a shelf. That contiguity is powerful:
you can jump to any index in O(1) because you just do pointer arithmetic
(`base_address + index * element_size`).

But that contiguity comes with a cost. What happens when you need to insert an element at the
front of an array?

```
Before:  [10, 20, 30, 40, 50]
Insert 5 at front:
  Step 1: shift everything right  [__, 10, 20, 30, 40, 50]
  Step 2: write the new value     [ 5, 10, 20, 30, 40, 50]
```

Every element has to move. That is O(n). Same story for deletion at the front. If your workload
is "frequently insert/remove at the beginning or middle," arrays fight you.

A **linked list** takes the opposite trade-off. Instead of storing elements next to each other,
each element lives *wherever it wants* in memory and holds a pointer to the next one.

Think of it like a **scavenger hunt**: each clue tells you where to find the next clue.
You cannot jump straight to clue #7 -- you have to follow the chain from clue #1.
But if you want to insert a new clue between #3 and #4, you just rewrite #3's "next location"
to point to the new clue, and point the new clue at #4. No one else moves.

---

## The Singly Linked List

### Anatomy of a Node

A singly linked list is a chain of **nodes**. Each node holds two things:

1. A **value** (the data you care about).
2. A **next pointer** (the address of the next node, or nothing if this is the tail).

```
  Node             Node             Node
 +--------+---+  +--------+---+  +--------+------+
 | data: 5| *--->| data:12| *--->| data: 8| None |
 +--------+---+  +--------+---+  +--------+------+
   ^
   |
  head
```

The **head** pointer is your entry point. It is the only thing you hold on to. Lose the head,
lose the whole list.

### The Train Analogy

Picture a train where each car is bolted to the next one with a single coupling:

```
  LOCOMOTIVE --> [CAR A] --> [CAR B] --> [CAR C] --> (end of train)
```

- You can only walk forward through the cars.
- To add a new car at the front, you couple it to the locomotive and re-couple the old front car
  behind it. No other cars need to move.
- To find a specific car, you start at the locomotive and walk through each one.

That is a singly linked list.

---

## Building a Singly Linked List in Rust

This is where it gets interesting. Linked lists are **famously hard** in Rust, and for good
reason. Rust's ownership model says every value has exactly one owner. A linked list is a chain
of nodes where each node owns the next one -- that part is fine. But what about operations that
need to mutate a node while also holding a reference to its neighbor? That is where things get
tricky.

Let's start with the idiomatic approach.

### The Core Types

```rust
// The idiomatic Rust linked list type alias.
// Read it as: "A link is either Some(boxed node) or None (end of list)."
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,  // Each node OWNS the next node
}

pub struct SinglyLinkedList<T> {
    head: Link<T>,
    length: usize,
}
```

Let's unpack this:

- **`Box<Node<T>>`** -- A heap-allocated node. We need `Box` because `Node` is a recursive type
  (it contains another `Node`). Without indirection, the compiler cannot determine the size of
  `Node` at compile time -- it would be infinite. `Box` gives us a fixed-size pointer (8 bytes
  on 64-bit) that points to the actual node on the heap.

- **`Option<...>`** -- Represents "there might be a next node, or this might be the end."
  `None` means end of list. This replaces the null pointers you would use in C.

- **`type Link<T> = Option<Box<Node<T>>>`** -- Just a type alias to save us from writing
  `Option<Box<Node<T>>>` everywhere. You will see this pattern in virtually every Rust linked
  list implementation.

### Why Not Just a Raw Pointer?

In C, you would write `struct Node* next;` and call it a day. In Rust, raw pointers exist but
using them requires `unsafe`. The `Box`+`Option` approach gives us:

- Automatic memory cleanup (when a node is dropped, its `Box`ed child is dropped too, and so on
  down the chain).
- Null safety at compile time (`Option` forces you to handle the "no next node" case).
- Clear ownership (each node owns exactly one child).

### Implementation

```rust
impl<T> SinglyLinkedList<T> {
    /// Create an empty list.
    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            length: 0,
        }
    }

    /// Push a value onto the front of the list. O(1).
    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),  // steal the old head
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    /// Pop a value from the front of the list. O(1).
    pub fn pop_front(&mut self) -> Option<T> {
        // .take() replaces self.head with None and gives us the old value.
        // .map() transforms Some(node) -> Some(node.data), passes through None.
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.data
        })
    }

    /// Peek at the front value without removing it. O(1).
    pub fn peek_front(&self) -> Option<&T> {
        // .as_ref() converts &Option<Box<Node>> to Option<&Box<Node>>
        // so we borrow instead of moving.
        self.head.as_ref().map(|node| &node.data)
    }

    /// Return the number of elements. O(1) because we track it.
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}
```

**Key Rust move: `Option::take()`**. This method replaces the Option in-place with `None` and
returns whatever was there. It is the bread and butter of linked list manipulation in Rust
because it lets you "steal" ownership of a node without angering the borrow checker. Memorize
this one.

### Visualizing push_front

```
Before:
  head -> [A] -> [B] -> [C] -> None

push_front(X):
  1. take() steals the current head:
       head -> None       (stolen) -> [A] -> [B] -> [C] -> None

  2. New node takes ownership of the stolen chain:
       new_node -> [X] -> [A] -> [B] -> [C] -> None

  3. head points to new node:
       head -> [X] -> [A] -> [B] -> [C] -> None
```

### Visualizing pop_front

```
Before:
  head -> [X] -> [A] -> [B] -> [C] -> None

pop_front():
  1. take() steals the head:
       head -> None       (stolen) -> [X] -> [A] -> [B] -> [C] -> None

  2. head = node.next (re-link):
       head -> [A] -> [B] -> [C] -> None
       (stolen) -> [X]    (next was moved out, node is partial)

  3. Return X. The Box for X is dropped (freed).
```

### Iteration

```rust
/// An iterator that borrows the list.
pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_deref(),
            // as_deref() converts Option<&Box<Node<T>>> to Option<&Node<T>>
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.data
        })
    }
}
```

Now you can write:

```rust
let mut list = SinglyLinkedList::new();
list.push_front(3);
list.push_front(2);
list.push_front(1);

for val in list.iter() {
    println!("{val}");
}
// prints: 1, 2, 3
```

### Implementing Drop

By default, Rust will recursively drop the list: dropping the head drops its child, which drops
its child, and so on. For a very long list, this blows the stack because each drop is a nested
function call. The fix is an iterative `Drop`:

```rust
impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
            // node is dropped here, but its next is already None,
            // so it doesn't recurse.
        }
    }
}
```

---

## The Doubly Linked List

### What Changes

In a singly linked list, each node points forward. A doubly linked list adds a **prev** pointer
so each node also points backward:

```
         +------+--------+---+    +---+--------+---+    +---+--------+------+
  None <-| prev | data:5 | *---->-| * | data:12| *---->-| * | data:8 | None |
         +------+--------+---+    +---+--------+---+    +---+--------+------+
           ^                                                           ^
           |                                                           |
          head                                                        tail
```

Think of it as a **subway line**: you can travel in either direction. Or a browser's
back/forward buttons -- each page knows both the previous page and the next one.

### Why Bother?

| Operation                  | Singly Linked | Doubly Linked |
|---------------------------|:-------------:|:-------------:|
| Insert at head            |     O(1)      |     O(1)      |
| Insert at tail            |     O(n)*     |     O(1)      |
| Delete from head          |     O(1)      |     O(1)      |
| Delete from tail          |     O(n)      |     O(1)      |
| Delete a given node       |     O(n)**    |     O(1)      |
| Search                    |     O(n)      |     O(n)      |
| Access by index           |     O(n)      |     O(n)      |

\* O(1) if you maintain a tail pointer, but you still cannot delete the tail in O(1) because
you cannot reach the second-to-last node without traversal.

\** O(n) because you need to find the *previous* node to re-link around the deleted one.

A doubly linked list makes O(1) insertion and deletion at *both* ends trivial, which is why it
is the backbone of structures like LRU caches and deques.

### The Rust Ownership Problem

Here is where Rust makes you earn it. In a doubly linked list, who *owns* a node?

```
  ... <- [prev | A | next] -> [prev | B | next] -> ...
```

Node A's `next` pointer references B. Node B's `prev` pointer references A. That is a
**cycle of references**. Rust's ownership model says: one owner. `Box` gives exclusive
ownership. You cannot have two `Box`es pointing to the same node. Period.

You have a few options, from safest to most powerful:

1. **`Rc<RefCell<Node<T>>>`** -- Reference-counted shared ownership with interior mutability.
   Safe but has runtime overhead (reference counting, borrow checking at runtime). This is the
   typical "safe Rust" approach.

2. **`unsafe` with raw pointers** -- What the standard library's `LinkedList` actually uses
   internally. You manage the memory yourself. Powerful, but you are back to C-style "hope you
   got it right."

3. **Arena allocation / index-based** -- Store all nodes in a `Vec<Node<T>>` and use indices
   instead of pointers. Nodes refer to each other by index. This sidesteps ownership entirely
   and is surprisingly practical.

For learning purposes, know that **safe doubly-linked lists in Rust are verbose and slow
compared to their C equivalents**. This is a deliberate trade-off. The Rust team considers it
a feature, not a bug: the language is showing you that shared mutable state is fundamentally
complex, and it refuses to let you ignore that complexity.

Here is a sketch of the `Rc<RefCell<>>` approach (simplified):

```rust
use std::cell::RefCell;
use std::rc::Rc;

type DLink<T> = Option<Rc<RefCell<DNode<T>>>>;

struct DNode<T> {
    data: T,
    next: DLink<T>,
    prev: DLink<T>,   // Weak<RefCell<DNode<T>>> is better here
                       // to avoid reference cycles / memory leaks.
}
```

In production Rust code, you would almost certainly use `std::collections::LinkedList` (which is
a doubly linked list implemented with `unsafe` internally) or, more likely, just use a `VecDeque`.

---

## Operation Complexity Summary

| Operation              | Singly Linked | Doubly Linked | Vec / Array |
|-----------------------|:-------------:|:-------------:|:-----------:|
| Push front             |     O(1)      |     O(1)      |    O(n)     |
| Pop front              |     O(1)      |     O(1)      |    O(n)     |
| Push back              |     O(n)*     |     O(1)      |    O(1)**   |
| Pop back               |     O(n)      |     O(1)      |    O(1)     |
| Insert at position k   |     O(k)      |     O(k)      |    O(n-k)   |
| Delete at position k   |     O(k)      |     O(k)      |    O(n-k)   |
| Search (unsorted)      |     O(n)      |     O(n)      |    O(n)     |
| Access by index        |     O(n)      |     O(n)      |    O(1)     |
| Space per element      |   data+ptr    |  data+2*ptr   |   data      |

\* O(1) with a tail pointer, but deleting the tail is still O(n).
\** Amortized O(1); occasionally O(n) when the backing array resizes.

---

## Common Interview Patterns

These patterns come up constantly. Even if you never use a linked list at work, you need them
for interviews and they train good recursive/pointer-manipulation thinking.

### 1. Fast and Slow Pointers (Floyd's Tortoise and Hare)

Use two pointers: **slow** advances one step at a time, **fast** advances two steps. This
technique solves several problems:

**Find the middle of a list:**

```
  slow
   v
  [1] -> [2] -> [3] -> [4] -> [5] -> None
   v
  fast

  Step 1: slow -> [2], fast -> [3]
  Step 2: slow -> [3], fast -> [5]
  Fast hit the end. Slow is at the middle.
```

```rust
fn find_middle<T>(head: &Link<T>) -> Option<&T> {
    let mut slow = head;
    let mut fast = head;

    loop {
        // Advance fast by two
        let next_fast = match fast {
            Some(node) => &node.next,
            None => break,
        };
        fast = match next_fast {
            Some(node) => &node.next,
            None => break,
        };
        // Advance slow by one
        slow = match slow {
            Some(node) => &node.next,
            None => break,
        };
    }

    slow.as_ref().map(|node| &node.data)
}
```

**Detect a cycle:**

If the list has a cycle, fast will eventually lap slow. If fast reaches `None`, there is no
cycle.

```
  Normal list (no cycle):
  [1] -> [2] -> [3] -> None         Fast reaches None. No cycle.

  List with cycle:
  [1] -> [2] -> [3] -> [4]
                  ^      |
                  |      v
                 [6] <- [5]          Fast and slow will eventually meet.
```

### 2. Reverse a Linked List

The classic. Walk through the list and flip each pointer backward.

```
  Before:  [1] -> [2] -> [3] -> None

  Step 1:  None <- [1]    [2] -> [3] -> None
                    ^      ^
                   prev   curr

  Step 2:  None <- [1] <- [2]    [3] -> None
                           ^      ^
                          prev   curr

  Step 3:  None <- [1] <- [2] <- [3]
                                  ^
                                 prev  (new head)
```

```rust
impl<T> SinglyLinkedList<T> {
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next = node.next.take();  // save the next node
            node.next = prev;             // point current backward
            prev = Some(node);            // advance prev
            current = next;               // advance current
        }

        self.head = prev;
    }
}
```

This is O(n) time, O(1) space. Notice how `take()` does the heavy lifting -- it lets us
disassemble and reassemble the chain without fighting the borrow checker.

### 3. Merge Two Sorted Lists

Given two sorted linked lists, merge them into one sorted list:

```
  List A:  1 -> 3 -> 5 -> None
  List B:  2 -> 4 -> 6 -> None
  Merged:  1 -> 2 -> 3 -> 4 -> 5 -> 6 -> None
```

The algorithm: compare the heads, take the smaller one, advance that list, repeat.

### 4. Remove the N-th Node from the End

Use two pointers separated by a gap of n. When the leading pointer hits the end, the trailing
pointer is at the node to remove.

```
  Remove 2nd from end in: [1] -> [2] -> [3] -> [4] -> [5]

  Set gap = 2:
  trail                    lead
   v                        v
  [1] -> [2] -> [3] -> [4] -> [5]

  Advance both until lead hits end:
                  trail                    lead
                   v                        v
  [1] -> [2] -> [3] -> [4] -> [5] -> None

  trail.next = trail.next.next  (skip [4])
  Result: [1] -> [2] -> [3] -> [5] -> None
```

---

## The Honest Truth: When to Actually Use Linked Lists

Linked lists are critical to *understand* but rarely the best *choice* in practice. Here is why:

### Cache Locality

Modern CPUs do not just read the byte you ask for. They read an entire **cache line** (usually
64 bytes). When you iterate an array, each access loads neighboring elements into cache for
free. The next access is likely a cache hit.

Linked list nodes live at random heap addresses. Every `node.next` dereference is a potential
**cache miss** -- the CPU has to go all the way to main memory. This is 10-100x slower than
a cache hit.

```
  Array in memory (contiguous -- cache-friendly):
  [...|10|20|30|40|50|60|70|80|...]
       ^^^^^^^^^^^^^^^^^^^^^^^^
       one or two cache lines

  Linked list in memory (scattered -- cache-hostile):
  [...|Node3|.............|Node1|.......|Node5|......|Node2|...]
       here         way over here    somewhere else    and here
```

### When Linked Lists Win

Despite the above, linked lists are the right tool when:

- You need **O(1) insertion/deletion at arbitrary known positions** (you already have a pointer
  to the node). Example: an LRU cache backed by a doubly linked list + hash map.
- You need a **persistent data structure** where old versions remain valid after modification
  (functional programming style).
- You are implementing other data structures (many trees and graphs are linked structures).
- Memory fragmentation makes large contiguous allocations impossible (rare in practice with
  modern allocators).

### What to Use Instead

| Need                          | Use This Instead       |
|-------------------------------|------------------------|
| Dynamic array                 | `Vec<T>`               |
| Queue / Deque                 | `VecDeque<T>`          |
| Stack                         | `Vec<T>` (push/pop)    |
| Sorted collection             | `BTreeSet<T>`          |
| Frequent front insert/remove  | `VecDeque<T>`          |

`VecDeque<T>` is a ring buffer backed by a contiguous array. It gives O(1) amortized push/pop
at both ends with excellent cache behavior. For most use cases where you think "linked list,"
`VecDeque` is the better answer.

---

## Rust's Standard Library: `std::collections::LinkedList`

Rust does provide a doubly linked list in the standard library. Its documentation literally says:

> "It is almost always better to use `Vec` or `VecDeque` instead of `LinkedList`. In general,
> array-based containers are faster, more memory-efficient, and make better use of CPU cache."

That is the Rust team telling you what this lesson is telling you. But the type exists for the
cases where you genuinely need it, and it is implemented with `unsafe` internally so you do not
have to deal with `Rc<RefCell<>>`.

---

## Exercises

1. **Implement `push_back` and `pop_back`** for the singly linked list above. You will need to
   walk to the end of the list. What is the time complexity?

2. **Implement `reverse`** iteratively (shown above) and then try it recursively. Which feels
   more natural to you? Which is more Rust-friendly?

3. **Detect a cycle** using the fast/slow pointer technique. You will need to use raw pointers
   or node IDs for comparison since Rust references don't easily support the `==` check you
   would do in other languages.

4. **Build a simple stack** using `SinglyLinkedList` as the backing store. Implement `push`,
   `pop`, and `peek`. Compare its performance against a `Vec<T>`-backed stack on large inputs.

5. **Think about this**: Why does Rust's `SinglyLinkedList` type not exist in the standard
   library, but `LinkedList` (doubly linked) does? What operations does a doubly linked list
   support that make it useful enough to include?

---

## Key Takeaways

- A **singly linked list** is a chain of heap-allocated nodes, each pointing to the next.
  Insertion and deletion at the head are O(1). Everything else is O(n).

- A **doubly linked list** adds backward pointers, enabling O(1) operations at both ends and
  O(1) deletion of any node you already have a reference to.

- In Rust, **`type Link<T> = Option<Box<Node<T>>>`** is the idiomatic building block for a
  singly linked list. `Option::take()` is your best friend for rearranging pointers.

- Doubly linked lists are hard in Rust because they require shared ownership.
  Use `Rc<RefCell<>>`, `unsafe`, or index-based approaches.

- Linked lists have **poor cache locality** compared to arrays. In practice, prefer `Vec` or
  `VecDeque` unless you have a specific reason to use a linked list.

- Learn the classic patterns anyway: **reverse a list**, **fast/slow pointers**, **merge sorted
  lists**, and **remove n-th from end**. They build fundamental pointer-manipulation intuition
  that transfers to trees, graphs, and systems programming.
