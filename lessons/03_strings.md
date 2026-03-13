# Lesson 03: Strings & String Manipulation

## What Is a String, Really?

At the hardware level, there is no such thing as text. There are only numbers.
A string is a sequence of bytes that we *agree* to interpret as human-readable
characters using an encoding scheme. The encoding is the Rosetta Stone that maps
numbers to glyphs.

Think of it like a bookshelf. Each slot holds a number. The encoding is the
catalog that says "number 72 means the letter H." Without the catalog, the shelf
is just numbers.

```
Bookshelf (bytes in memory):

  slot:  [ 0 ]  [ 1 ]  [ 2 ]  [ 3 ]  [ 4 ]
  byte:  | 72 | 101 | 108 | 108 | 111 |
  char:  | H  |  e  |  l  |  l  |  o  |    <-- ASCII interpretation
```

---

## ASCII and UTF-8: A Brief History

**ASCII** (1963) mapped 128 characters to 7-bit integers. It covered English
letters, digits, and control characters. Fine for American engineers, useless for
everyone else.

**UTF-8** (1993) solved the internationalization problem. It is a
variable-width encoding: each Unicode code point is encoded as 1 to 4 bytes.
Crucially, it is backwards-compatible with ASCII --- any valid ASCII byte
sequence is also valid UTF-8.

```
UTF-8 Encoding Scheme
======================

 Bytes   Bits available   Code point range         Example
 -----   --------------   ----------------------   ------------------
   1         7            U+0000  .. U+007F        'A'  = 0x41
   2        11            U+0080  .. U+07FF        'o'  = 0xC3 0xB6
   3        16            U+0800  .. U+FFFF        ''  = 0xE4 0xB8 0x96
   4        21            U+10000 .. U+10FFFF      ''  = 0xF0 0x9F 0x8C 0x8D

Byte layout:

  1-byte:  0xxxxxxx
  2-byte:  110xxxxx  10xxxxxx
  3-byte:  1110xxxx  10xxxxxx  10xxxxxx
  4-byte:  11110xxx  10xxxxxx  10xxxxxx  10xxxxxx
           ^^^^^^^
           Leading bits tell you how many bytes this character occupies.
```

**Why this matters for DSA:** You cannot assume 1 character = 1 byte. Indexing
into a UTF-8 string by byte position can land you in the middle of a multi-byte
character. This is why Rust refuses to let you do `s[3]` on a `String`.

---

## Rust's Two Main String Types

Rust has two string types you will use constantly: `String` and `&str`. They are
not interchangeable, but they are deeply related.

| Property       | `String`                       | `&str`                            |
|----------------|--------------------------------|-----------------------------------|
| Ownership      | Owned (lives on the heap)      | Borrowed (a view into bytes)      |
| Mutability     | Growable, mutable              | Immutable (read-only slice)       |
| Stored where   | Heap-allocated buffer          | Could be heap, stack, or binary   |
| Size known at  | Runtime (dynamic)              | Compile-time (fat pointer)        |
| Analogy        | You own the whole notebook     | You are reading over someone's shoulder |

### Memory Layout

```
   Stack                          Heap
  +-----------+                  +---+---+---+---+---+
  | ptr   ----+----------------->| H | e | l | l | o |   (5 bytes, UTF-8)
  | len:  5   |                  +---+---+---+---+---+
  | cap:  8   |                  |   |   |   |         <-- 3 bytes unused capacity
  +-----------+                  +---+---+---+
     String                      allocated buffer (capacity = 8)


   Stack
  +-----------+
  | ptr   ----+-------> (points into some existing UTF-8 bytes)
  | len:  5   |
  +-----------+
     &str                (no capacity field --- it cannot grow)
```

A `String` is a `Vec<u8>` that guarantees its contents are valid UTF-8. It has
three fields on the stack: a pointer, a length, and a capacity. The actual
character data lives on the heap.

An `&str` is a *fat pointer*: just a pointer and a length. It borrows UTF-8
bytes that live somewhere else (inside a `String`, in static program data, etc.).

### Converting Between Them

```rust
// &str -> String  (allocates)
let owned: String = "hello".to_string();
let also_owned: String = String::from("hello");

// String -> &str  (free, just borrows)
let borrowed: &str = &owned;
let also_borrowed: &str = owned.as_str();

// In function signatures, prefer &str for inputs:
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// This function accepts both String and &str thanks to deref coercion:
greet("world");                         // &str literal
greet(&String::from("world"));          // &String coerces to &str
```

Rule of thumb: accept `&str`, return `String`. This gives callers maximum
flexibility while making ownership transfer explicit.

---

## The Three Levels of a Rust String

A Rust string can be viewed at three different granularities. This is one of the
most important things to internalize.

```
     String: "noel"   (n + o + e + combining-diaeresis + l)
                        The "e" is actually two Unicode scalars!

  ┌─────────────────────────────────────────────────┐
  │  BYTES   (what the computer stores)              │
  │  [110, 111, 101, 204, 136, 108]                  │
  │   n    o    e    \xCC  \x88   l                   │
  │                  ^^^^^^^^^^^^                      │
  │                  combining diaeresis (U+0308)      │
  │   6 bytes total                                    │
  ├─────────────────────────────────────────────────┤
  │  CHARS   (Unicode scalar values)                  │
  │  ['n', 'o', 'e', '\u{0308}', 'l']                │
  │   5 chars total                                    │
  ├─────────────────────────────────────────────────┤
  │  GRAPHEME CLUSTERS  (what humans perceive)        │
  │  ["n", "o", "e\u{0308}", "l"]                     │
  │   4 graphemes total --- this is what you "see"     │
  └─────────────────────────────────────────────────┘
```

```rust
let s = "noe\u{0308}l";  // "noel" with combining diaeresis

println!("bytes:  {}", s.len());           // 6
println!("chars:  {}", s.chars().count()); // 5
// graphemes require the `unicode-segmentation` crate:
// println!("graphemes: {}", s.graphemes(true).count()); // 4
```

**Takeaway:** `.len()` gives you bytes, `.chars().count()` gives you Unicode
scalar values, and neither gives you what a human would call "the number of
characters." For most DSA problems on ASCII-only input, bytes and chars coincide,
but you should know the difference.

---

## Why You Can't Index Strings by Position

In Python you write `s[3]` and get the 4th character. In Rust, this is a
compile error:

```rust
let s = String::from("hello");
// let c = s[3];  // ERROR: `String` cannot be indexed by `usize`
```

The reason is fundamental: UTF-8 is variable-width. To find the *n*-th
character, you must scan from the beginning, counting multi-byte sequences. That
is O(n), not O(1). Rust refuses to hide O(n) work behind array-indexing syntax
that *looks* O(1).

**What you can do instead:**

```rust
let s = "hello, world";

// 1. Byte slicing (you must ensure you are on a char boundary):
let slice: &str = &s[0..5];  // "hello" -- panics if you slice mid-character!

// 2. Iterate over chars:
let fourth: Option<char> = s.chars().nth(3);  // Some('l'), but O(n)

// 3. Collect into a Vec<char> for O(1) random access (costs memory):
let chars: Vec<char> = s.chars().collect();
let fourth: char = chars[3];  // 'l', O(1) after the initial O(n) collect
```

For DSA problems, the `Vec<char>` approach is your friend when you need random
access. It trades memory for convenience.

---

## Slicing Strings Safely

String slicing in Rust operates on byte indices, not character indices. If your
slice boundary falls inside a multi-byte character, Rust panics at runtime.

```rust
let s = "Hello, world!";  // ASCII only, so byte index = char index

let hello: &str = &s[0..5];     // "Hello"
let world: &str = &s[7..12];    // "world"

// For non-ASCII:
let s = "cafe\u{0301}";  // "cafe" -- the e has an acute accent (2 bytes)
// &s[0..5] would panic because byte 5 is inside the combining accent.
// Use .char_indices() to find safe boundaries:

for (byte_idx, ch) in s.char_indices() {
    println!("byte {}: {:?}", byte_idx, ch);
}
// byte 0: 'c'
// byte 1: 'a'
// byte 2: 'f'
// byte 3: 'e'
// byte 4: '\u{301}'  <-- starts at byte 4, occupies 2 bytes
```

---

## Common String Operations in Rust

Here is a quick reference for the operations you will use constantly:

```rust
let mut s = String::from("Hello");

// Appending
s.push(' ');              // push a single char
s.push_str("world");     // push a string slice
s += "!";                // shorthand for push_str (via Add trait)

// Length and emptiness
s.len();                  // byte length (12)
s.is_empty();             // false

// Checking content
s.contains("world");     // true
s.starts_with("Hello");  // true
s.ends_with("!");         // true

// Searching
s.find("world");          // Some(6)  -- byte index of first occurrence
s.rfind('o');             // Some(7)  -- byte index of last occurrence

// Splitting
let words: Vec<&str> = s.split_whitespace().collect();  // ["Hello", "world!"]
let parts: Vec<&str> = "a,b,c".split(',').collect();    // ["a", "b", "c"]

// Trimming
"  padded  ".trim();              // "padded"
"  padded  ".trim_start();        // "padded  "
"  padded  ".trim_end();          // "  padded"

// Case conversion (returns new String)
"hello".to_uppercase();           // "HELLO"
"HELLO".to_lowercase();           // "hello"

// Replacing
"aabaa".replace("aa", "X");      // "XbX"
"aabaa".replacen("aa", "X", 1);  // "Xbaa"  (replace first N occurrences)
```

---

## The StringBuilder Pattern: `String::with_capacity`

In Java you have `StringBuilder`. In Rust, `String` *is* the builder --- but you
should be smart about allocation.

Every time a `String` outgrows its buffer, it allocates a new, larger buffer on
the heap and copies everything over. If you know (or can estimate) the final
size, pre-allocate:

```
Without pre-allocation (multiple reallocations):

  push "He" -> alloc 2  [H][e]
  push "llo" -> alloc 5  [H][e][l][l][o]          <-- copy + realloc
  push " wo" -> alloc 10 [H][e][l][l][o][ ][w][o] <-- copy + realloc
  push "rld" -> fits in existing capacity

With pre-allocation:

  with_capacity(11) -> alloc 11 [_][_][_][_][_][_][_][_][_][_][_]
  push "He"   -> [H][e][_][_][_][_][_][_][_][_][_]
  push "llo"  -> [H][e][l][l][o][_][_][_][_][_][_]
  push " wo"  -> [H][e][l][l][o][ ][w][o][_][_][_]
  push "rld"  -> [H][e][l][l][o][ ][w][o][r][l][d]
  Zero reallocs!
```

```rust
// Bad: many small allocations as the string grows
fn build_csv_bad(rows: &[Vec<&str>]) -> String {
    let mut out = String::new();
    for row in rows {
        out.push_str(&row.join(","));
        out.push('\n');
    }
    out
}

// Good: estimate total size, allocate once
fn build_csv_good(rows: &[Vec<&str>]) -> String {
    let estimated_size: usize = rows.iter()
        .map(|row| row.iter().map(|cell| cell.len() + 1).sum::<usize>())
        .sum();

    let mut out = String::with_capacity(estimated_size);
    for row in rows {
        out.push_str(&row.join(","));
        out.push('\n');
    }
    out
}
```

This is the same principle as `Vec::with_capacity`. When you are building a
string in a loop, especially inside a hot path, this is the single easiest
performance win.

---

## String Algorithms

Now for the DSA meat. These are patterns you will see over and over in coding
interviews and real codebases.

### 1. Reversing a String

The naive approach: collect chars, reverse, collect back into a String.

```rust
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

assert_eq!(reverse_string("hello"), "olleh");
assert_eq!(reverse_string("rust"), "tsur");
```

**In-place reversal** (when you have a `Vec<char>`):

```rust
fn reverse_in_place(chars: &mut Vec<char>) {
    let (mut left, mut right) = (0, chars.len().saturating_sub(1));
    while left < right {
        chars.swap(left, right);
        left += 1;
        right -= 1;
    }
}

let mut chars: Vec<char> = "hello".chars().collect();
reverse_in_place(&mut chars);
assert_eq!(chars, vec!['o', 'l', 'l', 'e', 'h']);
```

**Time:** O(n). **Space:** O(n) for the first version (new allocation), O(1)
extra for the in-place version.

**Gotcha:** Reversing a string with combining characters (like our "noel"
example) can produce garbled output. For DSA problems you can usually assume
ASCII, but in production code, use a grapheme-aware reversal.

---

### 2. Palindrome Check

A palindrome reads the same forwards and backwards. The two-pointer technique
is the classic approach:

```
    "racecar"
     ^     ^      'r' == 'r' -> move inward
      ^   ^       'a' == 'a' -> move inward
       ^ ^        'c' == 'c' -> move inward
        ^         pointers crossed -> palindrome!
```

```rust
fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();  // safe for ASCII input
    let (mut left, mut right) = (0, bytes.len().saturating_sub(1));

    while left < right {
        if bytes[left] != bytes[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

assert!(is_palindrome("racecar"));
assert!(is_palindrome("a"));
assert!(!is_palindrome("hello"));
```

**Case-insensitive, alphanumeric-only variant** (the classic LeetCode version):

```rust
fn is_palindrome_alphanumeric(s: &str) -> bool {
    let chars: Vec<char> = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let (mut left, mut right) = (0, chars.len().saturating_sub(1));
    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

assert!(is_palindrome_alphanumeric("A man, a plan, a canal: Panama"));
assert!(!is_palindrome_alphanumeric("race a car"));
```

**Time:** O(n). **Space:** O(n) for the filtered `Vec<char>`. You can reduce
space to O(1) by using two iterators (one forward, one reverse) and skipping
non-alphanumeric characters on the fly, but the code is less readable.

---

### 3. Anagram Detection

Two strings are anagrams if they contain exactly the same characters in any
order. "listen" and "silent" are anagrams.

**Approach 1: Sort and compare.** O(n log n) time, simple.

```rust
fn are_anagrams_sort(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut a_chars: Vec<char> = a.chars().collect();
    let mut b_chars: Vec<char> = b.chars().collect();
    a_chars.sort_unstable();
    b_chars.sort_unstable();
    a_chars == b_chars
}
```

**Approach 2: Character frequency with HashMap.** O(n) time.

```rust
use std::collections::HashMap;

fn are_anagrams(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut freq: HashMap<char, i32> = HashMap::new();

    for ch in a.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    for ch in b.chars() {
        *freq.entry(ch).or_insert(0) -= 1;
    }

    freq.values().all(|&count| count == 0)
}

assert!(are_anagrams("listen", "silent"));
assert!(!are_anagrams("hello", "world"));
```

**For ASCII-only input**, you can use a fixed-size array instead of a HashMap
for better cache performance:

```rust
fn are_anagrams_ascii(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut counts = [0i32; 128];  // one slot per ASCII value
    for byte in a.bytes() {
        counts[byte as usize] += 1;
    }
    for byte in b.bytes() {
        counts[byte as usize] -= 1;
    }
    counts.iter().all(|&c| c == 0)
}
```

**Time:** O(n). **Space:** O(1) for the fixed-array version (128 is constant),
O(k) for the HashMap version where k is the alphabet size.

---

### 4. Character Frequency Counting

This is a building block that appears in many string problems (anagrams, most
frequent character, minimum window substring, etc.).

```rust
use std::collections::HashMap;

fn char_frequency(s: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for ch in s.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}

let freq = char_frequency("mississippi");
// {'m': 1, 'i': 4, 's': 4, 'p': 2}

// Find the most frequent character:
if let Some((&ch, &count)) = freq.iter().max_by_key(|(_, &v)| v) {
    println!("Most frequent: '{}' appears {} times", ch, count);
}
```

The `entry` API is Rust's elegant way to handle the "insert or update" pattern.
`freq.entry(ch).or_insert(0)` returns a mutable reference to the value: if the
key does not exist, it inserts 0 first. Then `+= 1` increments it.

---

### 5. Substring Search (Naive)

Given a text `haystack` and a pattern `needle`, find the first occurrence.

```
  haystack: "hello world"
  needle:   "world"

  Try position 0: "hello" != "world" -> shift
  Try position 1: "ello " != "world" -> shift
  ...
  Try position 6: "world" == "world" -> found at index 6!
```

```rust
fn find_substring(haystack: &str, needle: &str) -> Option<usize> {
    let h = haystack.as_bytes();
    let n = needle.as_bytes();

    if n.is_empty() {
        return Some(0);
    }
    if n.len() > h.len() {
        return None;
    }

    for i in 0..=(h.len() - n.len()) {
        if &h[i..i + n.len()] == n {
            return Some(i);
        }
    }
    None
}

assert_eq!(find_substring("hello world", "world"), Some(6));
assert_eq!(find_substring("hello world", "xyz"), None);
```

**Time:** O(n * m) worst case where n = haystack length, m = needle length.

In practice, Rust's built-in `str::find()` uses a variant of the
Two-Way algorithm which runs in O(n + m) time. For interview purposes,
knowing the naive approach is expected; for production code, just use `.find()`.

There are better algorithms (KMP, Rabin-Karp, Boyer-Moore) that guarantee linear
time. Those are worth a dedicated lesson.

---

## Putting It Together: A Worked Example

Let's solve a classic problem: **group anagrams.** Given a list of strings,
group them by anagram equivalence class.

```
Input:  ["eat", "tea", "tan", "ate", "nat", "bat"]
Output: [["eat", "tea", "ate"], ["tan", "nat"], ["bat"]]
```

```rust
use std::collections::HashMap;

fn group_anagrams(words: &[&str]) -> Vec<Vec<String>> {
    let mut groups: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

    for &word in words {
        // Create a canonical key: sorted characters
        let mut key: Vec<u8> = word.bytes().collect();
        key.sort_unstable();

        groups.entry(key)
            .or_insert_with(Vec::new)
            .push(word.to_string());
    }

    groups.into_values().collect()
}

let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
let result = group_anagrams(&input);
// result contains three groups (order may vary):
// [["eat", "tea", "ate"], ["tan", "nat"], ["bat"]]
```

**What's happening:** We use the sorted bytes of each word as a HashMap key. All
anagrams produce the same sorted key. The HashMap groups them naturally.

**Time:** O(n * k log k) where n = number of words, k = max word length (due
to sorting each word). You could get O(n * k) by using a character-count array
as the key instead of sorting.

---

## Rust-Specific Gotchas: A Summary

1. **`String` is not `&str`.** You will hit ownership/borrowing errors until
   your fingers learn the conversions. Accept `&str` in function parameters,
   return `String` when you need to hand back owned data.

2. **No integer indexing.** `s[3]` does not compile. Use `.chars().nth(3)`,
   byte slicing `&s[3..4]` (if you know it is safe), or collect into
   `Vec<char>`.

3. **`.len()` returns bytes, not characters.** `"cafe\u{0301}".len()` is 6, not
   4. Use `.chars().count()` if you need Unicode scalar count.

4. **Slicing can panic.** `&s[0..3]` panics if byte 3 is in the middle of a
   multi-byte character. Use `.char_indices()` to find safe boundaries.

5. **Grapheme clusters are not built-in.** The character "e" (e + combining
   accent) is two `char` values but one visible glyph. The standard library
   does not provide grapheme segmentation; use the `unicode-segmentation`
   crate.

6. **`chars()` and `bytes()` are iterators, not collections.** They are lazy.
   If you need random access or multiple passes, `.collect::<Vec<_>>()` first.

7. **String concatenation with `+` moves the left operand:**
   ```rust
   let a = String::from("hello");
   let b = String::from(" world");
   let c = a + &b;  // `a` is moved into `c`; `a` is no longer usable
   ```
   Use `format!()` when combining multiple strings to avoid ownership juggling.

---

## Complexity Cheat Sheet

| Operation                        | Time     | Notes                                  |
|----------------------------------|----------|----------------------------------------|
| `s.len()`                        | O(1)     | Stored field, not computed             |
| `s.is_empty()`                   | O(1)     | Just checks `len == 0`                |
| `s.push(ch)`                     | O(1)*    | Amortized; may trigger realloc        |
| `s.push_str(t)`                  | O(m)     | m = length of t; may realloc          |
| `s.chars().nth(n)`               | O(n)     | Must scan from start                  |
| `s.contains(pat)`                | O(n)     | Linear scan                           |
| `s.find(pat)`                    | O(n)     | Uses Two-Way algorithm internally     |
| `&s[a..b]` (slicing)            | O(1)     | Just pointer arithmetic + bounds check|
| `s.chars().collect::<Vec<_>>()`  | O(n)     | Full scan + allocation                |
| `String::with_capacity(n)`       | O(1)     | Single allocation                     |
| `format!("{}{}", a, b)`          | O(n+m)   | Allocates new String                  |

*Amortized O(1): occasionally O(n) when the buffer doubles, but averaged over
many pushes it is O(1) per push.

---

## Exercises

Try these in `src/solutions/`. Each builds on concepts from this lesson.

1. **Reverse Words in a String:** Given `"the sky is blue"`, return
   `"blue is sky the"`. Handle leading/trailing/multiple spaces.

2. **Valid Palindrome II:** Given a string, determine if it can become a
   palindrome by removing at most one character.

3. **Longest Substring Without Repeating Characters:** Use the sliding window
   technique with a `HashMap<char, usize>` to track last-seen positions.

4. **String Compression:** `"aabcccccaaa"` becomes `"a2b1c5a3"`. Return the
   original if the compressed version is not shorter.

5. **Isomorphic Strings:** Determine if two strings are isomorphic (a bijective
   character mapping exists). Example: `"egg"` and `"add"` are isomorphic.

---

## Key Takeaways

- Strings are sequences of bytes interpreted through an encoding. In Rust, that
  encoding is always UTF-8.
- `String` owns heap-allocated UTF-8 data. `&str` borrows a view into UTF-8
  bytes.
- UTF-8 is variable-width, which is why indexing is O(n) and Rust does not
  allow `s[i]`.
- For DSA problems on ASCII input, working with `bytes()` or collecting into
  `Vec<char>` gives you the random access you are used to.
- Pre-allocate with `String::with_capacity` when building strings in loops.
- Character frequency counting with `HashMap` (or a fixed `[i32; 128]` array
  for ASCII) is a fundamental pattern that underpins anagram detection, sliding
  window problems, and more.
- Know the three levels: bytes, chars (Unicode scalars), and grapheme clusters.
  Most DSA problems only care about the first two.
