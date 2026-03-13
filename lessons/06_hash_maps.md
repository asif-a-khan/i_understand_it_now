# Lesson 06: Hash Maps & Hash Sets

## The Big Idea

Imagine you work at a coat check. A guest hands you their coat, you give them a
numbered ticket, and you hang the coat on that numbered hook. When they come back,
they hand you the ticket, you go straight to hook #47, and grab their coat. You
don't search through every coat in the room. That's a hash map.

A **hash map** (also called a hash table, dictionary, or associative array) stores
key-value pairs and gives you near-instant lookup by key. A **hash set** is the
same structure but it only cares about whether a key *exists* -- there's no
associated value.

These are arguably the most practically useful data structures in all of
programming. If arrays are your bread, hash maps are your butter.

---

## How It Actually Works

### Step 1: The Hash Function

A hash function takes an input of any size and produces a fixed-size integer.
Think of it as an address calculator.

```
   Key            Hash Function          Hash Value
 -------         -------------          -----------
 "alice"    -->   h("alice")    -->     4918720135
 "bob"      -->   h("bob")      -->     2283614902
 "charlie"  -->   h("charlie")  -->     7719283101
```

The hash value gets mapped to a bucket index via modulo:

```
bucket_index = hash_value % number_of_buckets
```

So if we have 8 buckets:

```
  Key         Hash Value     % 8    Bucket
 ---------   -----------    -----   ------
 "alice"     4918720135       7       [7]
 "bob"       2283614902       6       [6]
 "charlie"   7719283101       5       [5]
```

Visually, the table looks like this:

```
  Buckets
  +-------+
0 |       |
  +-------+
1 |       |
  +-------+
2 |       |
  +-------+
3 |       |
  +-------+
4 |       |
  +-------+
5 | charlie: "..." |
  +-------+
6 | bob: "..."     |
  +-------+
7 | alice: "..."   |
  +-------+
```

**A good hash function has three properties:**
1. **Deterministic** -- the same input always produces the same output.
2. **Uniform** -- outputs are spread evenly across the range. No clustering.
3. **Fast** -- it should be cheap to compute.

It does NOT need to be reversible. You can't recover the key from the hash. This
is a one-way mapping, not encryption.

---

### Step 2: Collisions Are Inevitable

With a finite number of buckets and an infinite universe of possible keys, two
different keys will eventually hash to the same bucket. This is called a
**collision**. It's not a bug; it's a mathematical certainty (pigeonhole
principle).

```
  "alice"   --hash--> 7
  "dave"    --hash--> 7    <-- collision!
```

There are two main strategies for dealing with this.

---

### Collision Resolution: Chaining

Each bucket holds a linked list (or similar collection). Colliding entries just
get appended.

```
  Buckets
  +-------+
0 |  ---  |
  +-------+
1 |  ---  |
  +-------+
2 |  ---  |
  +-------+
3 |   *---+--> ("eve", 42) -> ("frank", 77) -> null
  +-------+
4 |  ---  |
  +-------+
5 |   *---+--> ("charlie", 10) -> null
  +-------+
6 |   *---+--> ("bob", 55) -> null
  +-------+
7 |   *---+--> ("alice", 30) -> ("dave", 99) -> null
  +-------+

  Each bucket is the head of a chain.
  Lookup: hash the key, go to the bucket, walk the chain comparing keys.
```

**Pros:** Simple. Gracefully handles high load.
**Cons:** Pointer chasing is cache-unfriendly. Extra memory for the list nodes.

---

### Collision Resolution: Open Addressing

Instead of a chain, we probe for the next available slot *within the array
itself*. The simplest version is **linear probing**: if bucket `i` is taken, try
`i+1`, then `i+2`, and so on.

```
  Insert "dave" -> hash = 7, but bucket 7 is taken by "alice"

  +-------+
5 | charlie|
  +-------+
6 | bob    |
  +-------+
7 | alice  |  <-- occupied, probe forward
  +-------+
0 | dave   |  <-- found empty slot (wraps around)
  +-------+
1 |        |
  +-------+
```

**Pros:** Cache-friendly (data is contiguous in memory). No extra allocations.
**Cons:** Clustering -- runs of occupied slots build up, degrading performance.
Deletion is tricky (you can't just empty the slot; you need tombstones or
back-shifting).

> Rust's standard `HashMap` uses a variant of open addressing called
> **SwissTable** (from Google's Abseil library). It uses SIMD instructions to
> probe multiple slots at once. Extremely fast in practice.

---

### Load Factor and Rehashing

The **load factor** is the ratio of stored entries to total buckets:

```
  load_factor = num_entries / num_buckets
```

As it grows, collisions become more frequent and performance degrades:

```
  Load Factor ~0.25 (mostly empty, fast)
  +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
  |   |   | X |   |   |   | X |   |   |   |   | X |   |   | X |   |
  +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+

  Load Factor ~0.50 (healthy)
  +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
  | X |   | X |   | X |   | X | X |   |   | X | X |   | X | X |   |
  +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+

  Load Factor ~0.90 (too full, lots of collisions)
  +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
  | X | X | X |   | X | X | X | X | X | X | X | X | X | X | X |   |
  +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
```

When the load factor exceeds a threshold (commonly ~0.75 for chaining, ~0.87 for
Rust's SwissTable), the map **rehashes**: it allocates a larger array (typically
2x) and re-inserts every entry. This is O(n), but it happens infrequently enough
that the *amortized* cost of insertion remains O(1).

This is the same amortization argument as `Vec::push` -- you pay a big cost
occasionally, but spread over all operations it averages out to constant time.

---

## Complexity

| Operation       | Average Case | Worst Case |
|-----------------|:------------:|:----------:|
| Insert          | O(1)*        | O(n)       |
| Lookup          | O(1)         | O(n)       |
| Delete          | O(1)         | O(n)       |
| Iteration       | O(n)         | O(n)       |

*amortized, accounting for occasional rehashes.

The worst case hits when every key hashes to the same bucket (a degenerate hash
function, or an adversarial input attack). This is why Rust defaults to
**SipHash** -- more on that below.

---

## Rust's HashMap and HashSet

### Basic Usage

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Insert
    scores.insert("Alice".to_string(), 100);
    scores.insert("Bob".to_string(), 85);
    scores.insert("Charlie".to_string(), 92);

    // Lookup -- returns Option<&V>
    if let Some(score) = scores.get("Alice") {
        println!("Alice scored {score}");
    }

    // Contains check
    assert!(scores.contains_key("Bob"));

    // Remove -- returns Option<V>
    let removed = scores.remove("Charlie");
    assert_eq!(removed, Some(92));

    // Iteration (order is NOT guaranteed)
    for (name, score) in &scores {
        println!("{name}: {score}");
    }
}
```

### Pre-allocating Capacity

If you know roughly how many entries you'll have, avoid repeated rehashing:

```rust
// Allocates enough buckets for at least 1000 entries without rehashing
let mut map: HashMap<String, Vec<u8>> = HashMap::with_capacity(1000);
```

### HashSet

A `HashSet<T>` is literally `HashMap<T, ()>` under the hood. It's for when you
only care about membership, not associated values.

```rust
use std::collections::HashSet;

fn has_duplicates(nums: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &n in nums {
        // .insert() returns false if the value was already present
        if !seen.insert(n) {
            return true;
        }
    }
    false
}

fn main() {
    assert!(has_duplicates(&[1, 2, 3, 2]));
    assert!(!has_duplicates(&[1, 2, 3, 4]));
}
```

Set operations work as you'd expect from math:

```rust
use std::collections::HashSet;

fn main() {
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5, 6].into_iter().collect();

    let union: HashSet<_> = a.union(&b).copied().collect();
    // {1, 2, 3, 4, 5, 6}

    let intersection: HashSet<_> = a.intersection(&b).copied().collect();
    // {3, 4}

    let difference: HashSet<_> = a.difference(&b).copied().collect();
    // {1, 2}  (in a but not in b)

    let symmetric_diff: HashSet<_> = a.symmetric_difference(&b).copied().collect();
    // {1, 2, 5, 6}  (in one but not both)
}
```

---

## The Entry API

This is one of Rust's best ergonomic wins for hash maps. The **entry API** lets
you look up a key and decide what to do based on whether it exists, all in a
single lookup (no double hashing).

The problem it solves:

```rust
// Naive approach: two lookups (get + insert)
if !map.contains_key(&key) {
    map.insert(key, default_value);
}
// Or worse, when you want to modify:
if let Some(v) = map.get_mut(&key) {
    *v += 1;
} else {
    map.insert(key, 1);
}
```

The entry API collapses this into one operation:

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    let words = ["hello", "world", "hello", "rust", "hello", "world"];

    for word in &words {
        // or_insert: if key is absent, insert the given value, then return &mut V
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }

    // {"hello": 3, "world": 2, "rust": 1}
    println!("{word_counts:?}");
}
```

The `entry()` method returns an `Entry` enum with two variants:
- `Occupied(OccupiedEntry)` -- key exists, gives you mutable access to the value
- `Vacant(VacantEntry)` -- key is missing, lets you insert

Common patterns:

```rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();

    // or_insert_with: lazily compute default (avoids allocation if key exists)
    map.entry("scores".to_string())
        .or_insert_with(Vec::new)
        .push(95);

    // or_default: uses Default::default() -- Vec::new() for Vec, 0 for numbers
    map.entry("grades".to_string())
        .or_default()
        .push(88);

    // and_modify + or_insert: update if exists, insert if not
    let mut counters: HashMap<&str, i32> = HashMap::new();
    for key in ["a", "b", "a", "c", "b", "a"] {
        counters.entry(key)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    // {"a": 3, "b": 2, "c": 1}
}
```

---

## Using Custom Types as Keys

To use your own type as a `HashMap` key, it must implement `Hash` and `Eq`.
In practice, you derive both along with `PartialEq`:

```rust
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut visited: HashMap<Point, bool> = HashMap::new();
    visited.insert(Point { x: 0, y: 0 }, true);
    visited.insert(Point { x: 1, y: 3 }, true);

    assert!(visited.contains_key(&Point { x: 0, y: 0 }));
}
```

**The contract:** if two values are equal (`a == b`), they must produce the same
hash (`hash(a) == hash(b)`). The reverse doesn't need to hold -- collisions are
fine. But violating the forward rule leads to silent, maddening bugs where you
insert a key and then can't find it.

This means you generally **cannot** use `f32` or `f64` as keys, because floating
point equality is weird (`NaN != NaN`). If you need float-keyed maps, you'll
typically convert to an integer representation first (e.g., ordered bits via
`f64::to_bits()`).

---

## Rust-Specific Details

### SipHash: Safety by Default

Rust's `HashMap` defaults to **SipHash-1-3** as its hash function. SipHash is a
cryptographically-influenced (not cryptographic) hash designed to be resistant
to **HashDoS attacks** -- where an attacker crafts inputs that all hash to the
same bucket, turning O(1) lookups into O(n) and causing denial of service.

SipHash achieves this by using a random seed (generated once per `HashMap`
instance). Even if an attacker knows the hash algorithm, they can't predict bucket
assignments without the seed.

**The tradeoff:** SipHash is slower than simpler hashes (like FxHash or
AHash) for small keys. If you're building a compiler or game engine where
HashDoS isn't a concern and you need raw speed, you can swap the hasher:

```rust
// In Cargo.toml: ahash = "0.8"
use ahash::AHashMap;

let mut map: AHashMap<String, i32> = AHashMap::new();
map.insert("fast".to_string(), 1);
```

Or use the `rustc_hash` crate for `FxHashMap`, which is what the Rust compiler
itself uses internally.

For DSA practice and most applications, the default hasher is the right choice.

### Iteration Order

`HashMap` does NOT maintain insertion order. If you need ordered iteration, use:
- `BTreeMap` -- sorted by key (O(log n) operations)
- `IndexMap` (from the `indexmap` crate) -- preserves insertion order with O(1) lookups

---

## HashMap vs BTreeMap -- When to Use Which

| Criteria                    | HashMap            | BTreeMap            |
|-----------------------------|--------------------|---------------------|
| Average lookup/insert       | O(1)               | O(log n)            |
| Worst-case lookup/insert    | O(n)               | O(log n)            |
| Ordered iteration           | No                 | Yes (sorted by key) |
| Range queries (`range()`)   | No                 | Yes                 |
| Min/max key                 | O(n) scan          | O(log n)            |
| Key trait requirement       | `Hash + Eq`        | `Ord`               |
| Memory layout               | Flat array + SIMD  | Tree of nodes       |
| Cache performance           | Good               | Moderate            |

**Rules of thumb:**
- Default to `HashMap`. It's faster for almost everything.
- Use `BTreeMap` when you need sorted keys, range queries, or a guaranteed
  O(log n) worst case (e.g., in adversarial/real-time contexts where amortized
  O(1) with occasional O(n) spikes is unacceptable).
- Use `BTreeMap` when your key type implements `Ord` but not `Hash` (uncommon
  but possible).

---

## Common Patterns

### 1. Frequency Counting

The bread and butter of hash map usage in interviews and real code.

```rust
use std::collections::HashMap;

fn char_frequencies(s: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    freq
}

fn most_frequent_char(s: &str) -> Option<(char, usize)> {
    char_frequencies(s)
        .into_iter()
        .max_by_key(|&(_, count)| count)
}

fn main() {
    let freq = char_frequencies("abracadabra");
    // {'a': 5, 'b': 2, 'r': 2, 'c': 1, 'd': 1}
    println!("{freq:?}");

    let top = most_frequent_char("abracadabra");
    println!("{top:?}"); // Some(('a', 5))
}
```

### 2. Two Sum (the classic)

Given an array of integers and a target, find two indices whose values sum to the
target. This is the canonical "use a hash map" problem.

```rust
use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Map from value -> index
    let mut seen: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return Some((j, i));
        }
        seen.insert(num, i);
    }

    None
}

fn main() {
    let nums = [2, 7, 11, 15];
    let result = two_sum(&nums, 9);
    assert_eq!(result, Some((0, 1))); // nums[0] + nums[1] = 2 + 7 = 9
}
```

**Why this works:** for each element, we ask "have I already seen the number that
would complete the pair?" The hash map answers that question in O(1). Total time:
O(n). Without the map, you'd need a nested loop at O(n^2).

### 3. Grouping / Bucketing

Group items by some computed property.

```rust
use std::collections::HashMap;

fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<&str>> {
    let mut groups: HashMap<usize, Vec<&str>> = HashMap::new();
    for &word in words {
        groups.entry(word.len()).or_default().push(word);
    }
    groups
}

fn group_anagrams(words: &[String]) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for word in words {
        // Sort the characters to create a canonical key for anagrams
        let mut key: Vec<char> = word.chars().collect();
        key.sort();
        let key: String = key.into_iter().collect();

        groups.entry(key).or_default().push(word.clone());
    }
    groups.into_values().collect()
}

fn main() {
    let words = ["eat", "tea", "tan", "ate", "nat", "bat"];
    let words: Vec<String> = words.iter().map(|s| s.to_string()).collect();
    let groups = group_anagrams(&words);
    for group in &groups {
        println!("{group:?}");
    }
    // ["eat", "tea", "ate"]
    // ["tan", "nat"]
    // ["bat"]
}
```

### 4. Caching / Memoization

Store previously computed results to avoid redundant work.

```rust
use std::collections::HashMap;

fn fib_memo(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    let result = fib_memo(n - 1, cache) + fib_memo(n - 2, cache);
    cache.insert(n, result);
    result
}

fn main() {
    let mut cache = HashMap::new();
    println!("fib(50) = {}", fib_memo(50, &mut cache)); // instant
}
```

### 5. Tracking First/Last Occurrence

```rust
use std::collections::HashMap;

fn first_non_repeating_char(s: &str) -> Option<char> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    // Second pass: find first char with count 1
    // (iteration over the string preserves order; iteration over the map does not)
    s.chars().find(|c| counts[c] == 1)
}

fn main() {
    assert_eq!(first_non_repeating_char("aabbcdd"), Some('c'));
    assert_eq!(first_non_repeating_char("aabb"), None);
}
```

---

## Common Pitfalls

**1. Assuming ordered iteration.**
`HashMap` iteration order is arbitrary and can change between runs. If your
test passes today but you're relying on iteration order, it will break
eventually. Use `BTreeMap` or sort the output if order matters.

**2. Cloning keys unnecessarily.**
If your keys are `&str` and the data lives long enough, use `HashMap<&str, V>`
instead of `HashMap<String, V>` to avoid allocations. Lifetimes will keep you
honest.

**3. Forgetting that `.entry()` borrows the map mutably.**
You can't call `map.entry(key)` while holding another mutable reference into
the same map. The borrow checker will catch this, but it can require
restructuring your code.

**4. Using types with bad `Hash` implementations.**
If your custom `Hash` implementation produces the same value for many distinct
inputs, you've created a degenerate map. Prefer deriving `Hash` over writing it
by hand.

---

## Mental Model Recap

```
  Phone book analogy:

  Key    = person's name
  Value  = phone number
  Hash   = "go to the section starting with the first letter"
  Bucket = one page of names in that section

  Looking up "Smith":
    1. Hash("Smith") -> "S" section       (O(1) jump)
    2. Scan the S page for "Smith"         (short chain if load is low)
    3. Read the phone number               (done)

  Without the phone book structure you'd scan every name: O(n).
  With it, you jump to the right section and scan a tiny subset: O(1) amortized.
```

---

## Key Takeaways

1. Hash maps give O(1) average-case lookup, insert, and delete. They achieve this
   by computing an array index from the key via a hash function.

2. Collisions are handled by chaining (linked lists per bucket) or open addressing
   (probing within the array). Rust uses SwissTable, an open-addressing scheme.

3. The **load factor** determines when to rehash. Rehashing is O(n) but happens
   rarely enough that insertion remains amortized O(1).

4. Rust's **Entry API** is the idiomatic way to insert-or-update. Learn
   `or_insert`, `or_default`, `or_insert_with`, and `and_modify`.

5. Use `HashMap` when you need fast key-value lookups. Use `BTreeMap` when you
   need sorted keys or range queries. Use `HashSet` when you only care about
   membership.

6. Rust uses SipHash by default for DoS resistance. Swap it out only when you've
   measured and you know you need the speed.

---

## Practice Problems

Once you're comfortable with the concepts, try these:

- **Contains Duplicate** -- given `&[i32]`, return whether any value appears twice.
- **Two Sum** -- find indices of two numbers that add to a target.
- **Group Anagrams** -- group strings by their sorted-character signature.
- **Valid Sudoku** -- use sets to track seen digits per row, column, and box.
- **LRU Cache** -- combine a `HashMap` with a doubly-linked list (or use `LinkedHashMap`).

Each of these lives or dies on your comfort with hash maps.
