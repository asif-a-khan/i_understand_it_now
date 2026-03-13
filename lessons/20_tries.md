# Lesson 20: Tries (Prefix Trees)

## The Big Idea

Open your phone and start typing a text message. Type "hel" and your keyboard
immediately suggests "hello", "help", "helmet". Type one more letter -- "hell" --
and the suggestions narrow to "hello", "hell". Every keystroke prunes the
possibilities. The keyboard is not scanning every word in its dictionary from
scratch each time. It is walking down a tree structure, one letter at a time,
following the path your letters have carved.

That tree structure is a **trie** (rhymes with "try" -- it comes from "re**trie**val,"
though many people pronounce it "tree"; context usually makes it clear).

Another way to think about it: imagine a phone book organized not alphabetically
by full last name, but letter by letter. The "S" tab has sub-tabs for "Sm", "Sn",
"So", etc. Under "Sm" you find "Smi", "Smo". Under "Smi" you find "Smith",
"Smit". Each level narrows the search by one character. You never compare full
strings -- you just follow the path, character by character, until you arrive at
the entry or discover it does not exist.

A trie encodes this idea as a tree where:
- Each node represents a single character (or more precisely, an edge between
  nodes represents a character).
- The path from the root to any node spells out a prefix.
- Nodes can be marked as "end of word" to indicate that the path so far is a
  complete key, not just a prefix of one.

---

## Why Tries Exist

You might wonder: why not just use a `HashMap<String, V>` or a sorted `Vec` with
binary search? Those work fine for exact lookups. But consider these operations:

- "Give me every word that starts with `pre`."
- "What is the longest common prefix among a set of strings?"
- "As the user types each character, update the list of completions."

A `HashMap` can answer "does this exact key exist?" in O(1), but "give me
everything that starts with this prefix" requires scanning every key -- O(n * k)
where n is the number of keys and k is the average key length.

A trie answers prefix queries in **O(m)** where m is the length of the prefix,
**independent of how many keys are stored**. Whether you have 10 words or 10
million, looking up "pre" takes exactly 3 steps down the tree. That is the
fundamental win.

| Operation              | HashMap      | Trie         |
|------------------------|:------------:|:------------:|
| Exact lookup           | O(1) avg     | O(m)         |
| Insert                 | O(1) avg     | O(m)         |
| Delete                 | O(1) avg     | O(m)         |
| Prefix search          | O(n * k)     | O(m)         |
| Autocomplete (top k)   | O(n * k)     | O(m + results) |
| Longest common prefix  | O(n * k)     | O(m)         |

Where m = length of the key/prefix, n = number of stored keys, k = average key length.

The HashMap wins on raw exact-lookup speed (hash + compare vs. m character
traversals). The trie wins whenever you care about prefixes.

---

## Trie Structure: A Visual Tour

Let us build a trie containing the words: **"cat"**, **"car"**, **"card"**, **"care"**, **"do"**, **"dog"**.

```
                         (root)
                        /      \
                       c        d
                       |        |
                       a        o *
                      / \       |
                     t*  r*     g *
                        / \
                       d*  e*

  * = end-of-word marker
```

Read any path from root to a starred node and you get a stored word:
- root -> c -> a -> t = "cat"
- root -> c -> a -> r = "car"
- root -> c -> a -> r -> d = "card"
- root -> c -> a -> r -> e = "care"
- root -> d -> o = "do"
- root -> d -> o -> g = "dog"

Notice the **shared prefixes**:
- "cat", "car", "card", "care" all share the path root -> c -> a (the prefix "ca").
- "car", "card", "care" share root -> c -> a -> r (the prefix "car").
- "do" and "dog" share root -> d -> o (the prefix "do").

This sharing is what makes tries space-efficient for datasets with many common
prefixes (like English words, URLs, file paths, IP addresses).

---

## The Node Structure

Each trie node needs two things:
1. **Children** -- a mapping from character to child node. We will use
   `HashMap<char, TrieNode>` for generality. (If the alphabet is fixed and small,
   like lowercase a-z, you could use a `[Option<Box<TrieNode>>; 26]` array for
   better cache performance -- but the HashMap version handles Unicode and is
   simpler to reason about.)
2. **End-of-word flag** -- a boolean indicating whether the path from the root to
   this node represents a complete word, not just a prefix.

```rust
use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}
```

The `Default` derive gives us `TrieNode { children: HashMap::new(), is_end: false }`
for free, which is convenient when creating new nodes.

The trie itself is just a wrapper around a root node:

```rust
#[derive(Debug, Default)]
struct Trie {
    root: TrieNode,
}
```

---

## Insertion

To insert a word, walk the trie character by character. At each step, if the
child for the current character does not exist, create it. When you have consumed
every character, mark the final node as end-of-word.

### Step-by-Step: Inserting "car" then "card"

Starting with an empty trie, insert **"car"**:

```
  Step 0: root is empty
  (root)

  Step 1: 'c' -- no child 'c', create it
  (root)
     |
     c

  Step 2: 'a' -- no child 'a' under 'c', create it
  (root)
     |
     c
     |
     a

  Step 3: 'r' -- no child 'r' under 'a', create it. Mark end-of-word.
  (root)
     |
     c
     |
     a
     |
     r*
```

Now insert **"card"**:

```
  Step 1: 'c' -- child 'c' exists, follow it
  Step 2: 'a' -- child 'a' exists, follow it
  Step 3: 'r' -- child 'r' exists, follow it (already marked as end for "car")
  Step 4: 'd' -- no child 'd' under 'r', create it. Mark end-of-word.

  (root)
     |
     c
     |
     a
     |
     r*        <-- "car" ends here
     |
     d*        <-- "card" ends here
```

The key insight: inserting "card" reused the entire "car" path. No duplication.

### The Code

```rust
impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            // or_default() creates a new TrieNode if 'ch' is not present.
            node = node.children.entry(ch).or_default();
        }
        node.is_end = true;
    }
}
```

That is it. The `entry` API (from Lesson 06) does the heavy lifting: if the child
exists, return a mutable reference to it; if not, insert a default node and return
a mutable reference to that. Each character is processed once, so insertion is
**O(m)** where m is the word length.

---

## Search (Exact Match)

To check if a word exists in the trie, walk character by character. If at any
point the child for the current character is missing, the word is not stored.
If you reach the end of the word, check the `is_end` flag -- if it is `false`,
the word is only a prefix of some longer stored word, not a word itself.

### Visualizing Search

Search for **"car"** in this trie:

```
  (root)
     |
     c
     |
     a
    / \
   t*  r*
      / \
     d*  e*
```

```
  'c' -> found child 'c', move to it
  'a' -> found child 'a', move to it
  'r' -> found child 'r', move to it. End of search string.
  Check is_end: true. "car" IS in the trie.
```

Search for **"ca"**:

```
  'c' -> found, move
  'a' -> found, move. End of search string.
  Check is_end: false. "ca" is NOT a stored word (it is a prefix of "cat", "car", etc.).
```

Search for **"cab"**:

```
  'c' -> found, move
  'a' -> found, move
  'b' -> NOT found among children of 'a'. "cab" is NOT in the trie.
```

### The Code

```rust
impl Trie {
    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_end
    }
}
```

Time: **O(m)**. We visit each character exactly once, and each `HashMap::get` is
O(1) amortized.

---

## Prefix Search (starts_with)

This is where tries earn their keep. "Does any stored word start with this
prefix?" Walk the trie along the prefix. If you can walk the entire prefix without
hitting a dead end, the answer is yes -- regardless of the `is_end` flag.

### Visualizing Prefix Search

Does any word start with **"car"**?

```
  (root)
     |
     c
     |
     a
    / \
   t*  r*         <-- We reach here after consuming "car". Children exist.
      / \              At least "card" and "care" continue from here.
     d*  e*            Answer: YES.
```

Does any word start with **"cx"**?

```
  'c' -> found, move
  'x' -> NOT found among children of 'c'. Answer: NO.
```

### The Code

```rust
impl Trie {
    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return false,
            }
        }
        true  // We consumed the whole prefix without failing
    }
}
```

Notice the only difference from `search`: we return `true` at the end instead
of `node.is_end`. That single difference separates "is this a stored word?" from
"is this a prefix of any stored word?"

---

## A Helper: Walking to a Prefix Node

Both `search` and `starts_with` share the same traversal logic. We can factor it
out:

```rust
impl Trie {
    /// Walk the trie along the given string. Returns a reference to the final
    /// node if the full path exists, or None if we hit a dead end.
    fn find_node(&self, s: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for ch in s.chars() {
            node = node.children.get(&ch)?;
        }
        Some(node)
    }

    fn search(&self, word: &str) -> bool {
        self.find_node(word).map_or(false, |node| node.is_end)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }
}
```

The `?` operator short-circuits on `None` -- if `get` returns `None`, the whole
function returns `None`. Clean and idiomatic.

---

## Deletion

Deletion is trickier than insertion because after removing a word, you may need
to clean up nodes that are no longer part of any other word. The idea:

1. Walk to the end of the word.
2. Unmark `is_end`.
3. On the way back up (recursion makes this natural), remove child nodes that
   have no children of their own and are not end-of-word markers. These are "dead
   branches."

### Visualizing Deletion

Delete **"car"** from:

```
  (root)                            (root)
     |                                 |
     c           delete "car"          c
     |          ------------>          |
     a                                 a
    / \                               / \
   t*  r*                            t*  r       <-- r is no longer end-of-word
      / \                               / \
     d*  e*                            d*  e*

  "car" is gone, but "card" and "care" are preserved.
  The 'r' node stays because it still has children.
```

Delete **"card"** from the result above:

```
  (root)                            (root)
     |                                 |
     c           delete "card"         c
     |          ------------>          |
     a                                 a
    / \                               / \
   t*  r                             t*  r
      / \                               |
     d*  e*                             e*

  The 'd' node had no children and was no longer end-of-word,
  so it gets pruned.
```

### The Code

```rust
impl Trie {
    /// Remove a word from the trie. Returns true if the word was found and removed.
    fn remove(&mut self, word: &str) -> bool {
        Self::remove_recursive(&mut self.root, word, 0)
    }

    fn remove_recursive(node: &mut TrieNode, word: &str, depth: usize) -> bool {
        let chars: Vec<char> = word.chars().collect();

        // Base case: we have consumed all characters
        if depth == chars.len() {
            if !node.is_end {
                return false; // Word was not in the trie
            }
            node.is_end = false;
            return true;
        }

        let ch = chars[depth];

        // If the child does not exist, the word is not in the trie
        let Some(child) = node.children.get_mut(&ch) else {
            return false;
        };

        let found = Self::remove_recursive(child, word, depth + 1);

        // After recursive call, prune the child if it is now a dead branch:
        // no children of its own and not end-of-word.
        if found {
            if let Some(child) = node.children.get(&ch) {
                if child.children.is_empty() && !child.is_end {
                    node.children.remove(&ch);
                }
            }
        }

        found
    }
}
```

Time: **O(m)** for a word of length m. We walk down once and unwind once.

Note: The `chars().collect::<Vec<char>>()` call is needed here because `word.chars()`
produces an iterator we cannot index into directly (Rust strings are UTF-8, not
arrays of chars). For ASCII-only problems -- which covers most interview scenarios
-- you could alternatively work with `word.as_bytes()` and `u8` for simplicity.

---

## Putting It All Together

```rust
use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

#[derive(Debug, Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.is_end = true;
    }

    fn find_node(&self, s: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for ch in s.chars() {
            node = node.children.get(&ch)?;
        }
        Some(node)
    }

    fn search(&self, word: &str) -> bool {
        self.find_node(word).map_or(false, |node| node.is_end)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    fn remove(&mut self, word: &str) -> bool {
        Self::remove_recursive(&mut self.root, word, 0)
    }

    fn remove_recursive(node: &mut TrieNode, word: &str, depth: usize) -> bool {
        let chars: Vec<char> = word.chars().collect();
        if depth == chars.len() {
            if !node.is_end {
                return false;
            }
            node.is_end = false;
            return true;
        }
        let ch = chars[depth];
        let Some(child) = node.children.get_mut(&ch) else {
            return false;
        };
        let found = Self::remove_recursive(child, word, depth + 1);
        if found {
            if let Some(child) = node.children.get(&ch) {
                if child.children.is_empty() && !child.is_end {
                    node.children.remove(&ch);
                }
            }
        }
        found
    }
}

fn main() {
    let mut trie = Trie::new();

    for word in ["cat", "car", "card", "care", "do", "dog"] {
        trie.insert(word);
    }

    assert!(trie.search("car"));
    assert!(trie.search("card"));
    assert!(!trie.search("ca"));        // prefix only, not a word
    assert!(!trie.search("carbon"));    // not inserted

    assert!(trie.starts_with("ca"));    // "cat", "car", etc. start with "ca"
    assert!(trie.starts_with("car"));
    assert!(!trie.starts_with("cx"));

    assert!(trie.remove("car"));
    assert!(!trie.search("car"));       // gone
    assert!(trie.search("card"));       // still there
    assert!(trie.search("care"));       // still there

    println!("All assertions passed.");
}
```

---

## Collecting Words: Autocomplete

A trie really shines when you need to enumerate all words with a given prefix.
The approach: walk to the prefix node, then do a depth-first traversal to collect
all words reachable from there.

```rust
impl Trie {
    /// Return all stored words that start with the given prefix.
    fn autocomplete(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();
        if let Some(node) = self.find_node(prefix) {
            let mut current_word: String = prefix.to_string();
            Self::collect_words(node, &mut current_word, &mut results);
        }
        results
    }

    fn collect_words(node: &TrieNode, current: &mut String, results: &mut Vec<String>) {
        if node.is_end {
            results.push(current.clone());
        }
        // Sort keys for deterministic output order (optional but nice for testing)
        let mut children: Vec<_> = node.children.iter().collect();
        children.sort_by_key(|(ch, _)| *ch);

        for (&ch, child) in children {
            current.push(ch);
            Self::collect_words(child, current, results);
            current.pop();
        }
    }
}
```

Usage:

```rust
let mut trie = Trie::new();
for word in ["cat", "car", "card", "care", "cargo", "do", "dog"] {
    trie.insert(word);
}

let suggestions = trie.autocomplete("car");
// ["car", "card", "care", "cargo"]

let suggestions = trie.autocomplete("do");
// ["do", "dog"]

let suggestions = trie.autocomplete("z");
// [] -- no words start with 'z'
```

Visualizing what `autocomplete("car")` does:

```
  Walk to the "car" node:

  (root) --c--> (c) --a--> (a) --r--> (r)*    <-- prefix node found
                                       / | \
                                      d* e*  g
                                             |
                                             o*

  DFS from (r):
    "car"  -- (r) is end-of-word, collect it
    "card" -- (r)->d is end-of-word, collect it
    "care" -- (r)->e is end-of-word, collect it
    "cargo"-- (r)->g->o is end-of-word, collect it
```

Time: O(m + k) where m is the prefix length and k is the total number of
characters in all matching words. You pay for the traversal but only visit
relevant nodes.

---

## Complexity Analysis

### Time Complexity

| Operation        | Time     | Notes                                    |
|------------------|:--------:|------------------------------------------|
| Insert           | O(m)     | m = length of the word                   |
| Search           | O(m)     | m = length of the word                   |
| starts_with      | O(m)     | m = length of the prefix                 |
| Delete           | O(m)     | m = length of the word                   |
| Autocomplete     | O(m + k) | m = prefix length, k = output size       |

Every operation is independent of n (the total number of stored words). This is
the trie's defining characteristic.

### Space Complexity

Here is where tries get expensive. Each node contains a `HashMap` (or array) of
children. In the worst case -- storing n words of average length m with no shared
prefixes -- the trie uses **O(n * m)** nodes, each carrying the overhead of a
`HashMap`.

In practice, natural language and many real datasets have heavy prefix sharing,
which significantly reduces the actual space. But you should be aware that a trie
for a million short strings can use substantially more memory than a
`HashSet<String>` storing the same data.

**Rough memory comparison for storing n words of average length m:**

| Structure       | Space                            |
|-----------------|:---------------------------------|
| `HashSet`       | O(n * m) for the strings + hash table overhead |
| Trie (HashMap)  | O(total unique prefixes * per-node overhead) |
| Trie (array)    | O(total unique prefixes * alphabet_size) |

The per-node overhead of a `HashMap<char, TrieNode>` is significant (a HashMap
in Rust is ~48 bytes even when empty, plus allocations as it grows). For small
alphabets, an array-based node can be more compact:

```rust
// For lowercase ASCII only (26 letters)
struct ArrayTrieNode {
    children: [Option<Box<ArrayTrieNode>>; 26],
    is_end: bool,
}
```

Each node is a fixed 26 * 8 = 208 bytes (on 64-bit) plus 1 byte for `is_end`.
That is wasteful if most slots are empty, but it gives O(1) child lookup with no
hashing. The right choice depends on your alphabet size and prefix density.

---

## When to Use a Trie vs. a HashMap

This is a practical engineering question, not just an academic one.

**Use a HashMap when:**
- You only need exact key lookups and insertions.
- You do not care about prefix relationships.
- Memory is a concern and your keys do not share many prefixes.
- You want maximum raw speed for lookup.

**Use a Trie when:**
- You need prefix-based operations: autocomplete, prefix counting, starts-with
  checks.
- You want to find the longest common prefix.
- You are building a spell checker or dictionary with prefix-aware features.
- You are doing character-by-character matching (like in word search grids).
- Your dataset has heavy prefix sharing (URLs, file paths, IP addresses).

In interviews, if the problem mentions "prefix", "autocomplete", or "dictionary
with starts_with", a trie is almost certainly the intended data structure.

---

## Common Applications

### 1. Autocomplete Systems

This is the canonical use case. As the user types, walk the trie to the current
prefix node and enumerate (or rank) suggestions from there. Real autocomplete
systems augment tries with frequency counts, recency weighting, and
personalization, but the underlying traversal is the same.

### 2. Spell Checkers

Store a dictionary in a trie. To suggest corrections for a misspelled word, walk
the trie and allow a limited number of edit-distance deviations (insertions,
deletions, substitutions). This is more efficient than checking edit distance
against every word in the dictionary.

### 3. IP Routing (Longest Prefix Match)

Network routers use tries (specifically, radix trees) to match IP addresses
against routing tables. Given a destination IP, the router walks the trie bit by
bit and finds the longest matching prefix to determine the next hop. This must
happen billions of times per second in hardware.

### 4. Word Games (Boggle, Scrabble)

When searching a grid of letters for valid words, a trie lets you prune entire
branches of the search. If no word in the dictionary starts with "qz", you can
skip that path immediately rather than checking every possible word.

### 5. File Systems and URLs

Directory structures are inherently prefix-based. Routing in web frameworks
(matching URL paths to handlers) often uses a trie or radix tree internally.

---

## Interview Problems

### Problem 1: Implement a Trie (LeetCode 208)

This is the warm-up. Implement `insert`, `search`, and `starts_with` exactly as
we did above. If you have read this far, you can do this one from memory.

### Problem 2: Word Search II (LeetCode 212)

Given an m x n board of characters and a list of words, find all words that can
be formed by sequentially adjacent cells (horizontally or vertically). Each cell
can be used at most once per word.

**Why a trie?** The naive approach checks each word independently by running DFS
from every cell. With a trie, you build the dictionary into a trie, then run a
*single* DFS from each cell, walking the trie in parallel. At each step, if the
current path does not match any prefix in the trie, you prune immediately.

```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,  // Store complete word at terminal nodes
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { children: HashMap::new(), word: None }
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.word = Some(word.to_string());
    }
}

fn find_words(board: &[Vec<char>], words: &[&str]) -> Vec<String> {
    let mut root = TrieNode::new();
    for &word in words {
        root.insert(word);
    }

    let rows = board.len();
    let cols = board[0].len();
    let mut result = Vec::new();
    let mut visited = vec![vec![false; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            dfs(board, &mut root, r, c, &mut visited, &mut result);
        }
    }

    result
}

fn dfs(
    board: &[Vec<char>],
    node: &mut TrieNode,
    r: usize,
    c: usize,
    visited: &mut [Vec<bool>],
    result: &mut Vec<String>,
) {
    let ch = board[r][c];
    let Some(child) = node.children.get_mut(&ch) else {
        return; // No word in the trie continues with this character
    };

    // If this node marks a complete word, collect it
    if let Some(word) = child.word.take() {
        result.push(word);
        // Don't return -- there might be longer words continuing from here
    }

    visited[r][c] = true;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (dr, dc) in directions {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nr < board.len() as isize
            && nc >= 0 && nc < board[0].len() as isize
        {
            let (nr, nc) = (nr as usize, nc as usize);
            if !visited[nr][nc] {
                dfs(board, child, nr, nc, visited, result);
            }
        }
    }

    visited[r][c] = false;

    // Optimization: prune the trie node if it has no more children
    // (all words through this path have been found)
    if child.children.is_empty() {
        node.children.remove(&ch);
    }
}
```

The trie pruning trick (`children.remove` after exhausting a branch) is key to
performance on large boards with large dictionaries.

### Problem 3: Longest Common Prefix (LeetCode 14)

Given an array of strings, find the longest common prefix.

You can solve this without a trie (just compare characters column by column), but
the trie approach is instructive: insert all strings, then walk from the root
following single-child nodes until you hit a node with multiple children or an
end-of-word marker.

```
  Words: ["flower", "flow", "flight"]

  Trie:
  (root)
     |
     f          <-- single child, continue
     |
     l          <-- single child, continue
    / \
   o   i        <-- two children, STOP

  Longest common prefix: "fl"
```

```rust
impl Trie {
    fn longest_common_prefix(&self) -> String {
        let mut prefix = String::new();
        let mut node = &self.root;

        loop {
            // Stop if this node is end-of-word (one of the strings ends here)
            if node.is_end {
                break;
            }
            // Stop if there are zero or multiple children
            if node.children.len() != 1 {
                break;
            }
            // Exactly one child -- follow it
            let (&ch, child) = node.children.iter().next().unwrap();
            prefix.push(ch);
            node = child;
        }

        prefix
    }
}
```

### Problem 4: Design Autocomplete System

Combine a trie with frequency counting. Each terminal node stores a count (or a
priority). When the user types a prefix, walk to the prefix node, collect all
completions, and return the top-k by frequency. For production systems, you would
use a heap to avoid sorting the full result set, but the trie traversal is the
same.

---

## Space Optimization: Compressed Tries (Radix Trees)

Look at this trie for ["romane", "romanus", "romulus", "rubens", "ruber", "rubicon"]:

```
  Standard trie (simplified):

  (root) -> r -> o -> m -> a -> n -> e*
                                  -> u -> s*
                       -> u -> l -> u -> s*
            -> u -> b -> e -> n -> s*
                            -> r*
                    -> i -> c -> o -> n*
```

Many nodes have exactly one child. They form chains that could be collapsed:

```
  Compressed trie (radix tree):

  (root)
     |
     r
    / \
  om   ub
  / \   / \
an  ulus* en  icon*
/ \      / \
e*  us*  s*  r*
```

Each edge now carries a *string* (or a slice of one) rather than a single
character. Chains of single-child nodes are merged into one edge. This is a
**radix tree** (also called a Patricia tree, or compact prefix tree).

**Benefits:**
- Far fewer nodes. For sparse datasets, the savings can be dramatic.
- Fewer pointer dereferences during traversal.

**Trade-offs:**
- Insertion and deletion are more complex (you may need to split or merge edges).
- Implementation is significantly more involved.

In practice, radix trees are used in high-performance systems (Linux kernel
routing tables, many database indices, web framework routers). For interviews,
you rarely need to implement one, but you should know they exist and why.

Rust's ecosystem has the `radix_trie` crate if you need one in production.

---

## A Note on the Array-Based Variant

For problems where the alphabet is fixed and small (e.g., lowercase English
letters), you will often see this variant in interview solutions:

```rust
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: std::array::from_fn(|_| None),
            is_end: bool::default(),
        }
    }
}
```

Child lookup becomes `children[(ch as u8 - b'a') as usize]` -- a direct array
index, no hashing. This is faster and more cache-friendly than a HashMap when the
alphabet is small, but wastes space when most children slots are empty.

For interview settings, the HashMap variant is cleaner and handles any character
set. Use the array variant when the problem explicitly constrains the alphabet
and you want to squeeze out performance.

---

## Key Takeaways

1. A **trie** is a tree where each path from root to node represents a prefix.
   It answers prefix queries in O(m) regardless of how many strings are stored.

2. The core operations -- insert, search, starts_with -- are all simple walks
   from the root, following one character per step.

3. Tries beat HashMaps for **prefix-based operations** (autocomplete, prefix
   counting, longest common prefix). HashMaps beat tries for **exact lookups**
   (no prefix structure needed, O(1) vs O(m)).

4. Space is the trie's weakness. Each node carries the overhead of its children
   mapping. Compressed tries (radix trees) mitigate this by collapsing chains of
   single-child nodes.

5. The `HashMap<char, TrieNode>` children approach is the most flexible. For
   fixed small alphabets, an array of `Option<Box<TrieNode>>` is faster.

6. In interviews, the word "prefix" is your signal. If the problem involves
   prefix matching, prefix counting, or building dictionaries, reach for a trie.

---

## Practice Problems

Once you are comfortable with the implementation, try these:

- **Implement Trie** (LeetCode 208) -- The warm-up. Insert, search, starts_with.
- **Word Search II** (LeetCode 212) -- Grid search with a trie for pruning.
  This is a hard problem that combines DFS + backtracking + trie.
- **Design Add and Search Words** (LeetCode 211) -- Search with wildcards
  (`.` matches any character). Requires DFS through the trie branching at
  wildcard characters.
- **Longest Common Prefix** (LeetCode 14) -- Can be solved without a trie,
  but try both approaches.
- **Map Sum Pairs** (LeetCode 677) -- Trie where each node accumulates a sum,
  and you query the sum of all values whose keys start with a given prefix.
- **Replace Words** (LeetCode 648) -- Given a dictionary of roots and a
  sentence, replace each word with its shortest root prefix found in the
  dictionary.
