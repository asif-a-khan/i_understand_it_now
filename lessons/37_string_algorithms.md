# Lesson 37: String Algorithms

## Finding Needles in Haystacks -- at Scale

You already know how strings work in memory (Lesson 03). Now we tackle one of
the oldest and most practical problems in computer science: **finding patterns
inside text**.

Here is the real-world version of this problem. You are a researcher with a
2-billion-character human genome stored as a text file. You need to find every
occurrence of a 20-character DNA motif. The naive approach -- slide the motif
across every position -- would do roughly 2 billion x 20 = 40 billion character
comparisons. That is an afternoon wasted. The algorithms in this lesson cut
that to roughly 2 billion comparisons (linear time), finishing in seconds.

The same core problem appears everywhere:
- **Ctrl+F in your editor** -- finding a search term in a file.
- **Plagiarism detection** -- checking if chunks of one document appear in another.
- **Log analysis** -- scanning gigabytes of logs for error patterns.
- **Intrusion detection** -- matching network packets against known attack signatures.
- **Bioinformatics** -- locating gene sequences in DNA strings.

This lesson builds from the simplest approach (brute force) to three
progressively cleverer algorithms, then covers palindromes and string hashing.
Every implementation is complete, compilable Rust.

---

## Rust String Refresher for Algorithm Work

Before we dive into algorithms, a quick tactical reminder. If you have done
Lesson 03 this is review, but it matters for every implementation below.

```
  String          &str              &[u8]
  --------        --------          --------
  Owned,          Borrowed,         Borrowed,
  heap-allocated  UTF-8 slice       raw byte slice
  growable        immutable view    no UTF-8 guarantee
```

**For interview-style string matching problems on ASCII input**, you almost
always want to work with byte slices:

```rust
let text = "hello world";
let bytes: &[u8] = text.as_bytes();  // &[104, 101, 108, 108, 111, ...]
```

Why? Because `&str` indexing by position is O(n) -- Rust strings are UTF-8,
and characters can be 1-4 bytes, so you cannot jump to the i-th character in
constant time. But `&[u8]` indexing is O(1), just like a C array. Every
algorithm below uses `.as_bytes()` for this reason.

**When does this break?** When the input contains multi-byte Unicode characters
and you need to match at character boundaries. For competitive programming and
most interview problems, the input is ASCII and `.as_bytes()` is safe.

---

## Part 1: Naive String Matching (Brute Force)

### The Idea

Slide the pattern across the text one position at a time. At each position,
compare character by character.

```
Text:    [A][B][A][B][A][C]
Pattern: [A][B][A][C]

Step 1: position 0
  Text:    A  B  A  B  A  C
  Pattern: A  B  A  C
           ^  ^  ^  X        -- mismatch at index 3

Step 2: position 1
  Text:    A  B  A  B  A  C
  Pattern:    A  B  A  C
              X              -- mismatch at index 0

Step 3: position 2
  Text:    A  B  A  B  A  C
  Pattern:       A  B  A  C
                 ^  ^  ^  ^  -- full match at position 2!
```

### Rust Implementation

```rust
/// Returns all starting indices where `pattern` occurs in `text`.
/// Time:  O(n * m) worst case, where n = text.len(), m = pattern.len()
/// Space: O(1) extra (output excluded)
fn naive_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut results = Vec::new();
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let n = t.len();
    let m = p.len();

    if m == 0 || m > n {
        return results;
    }

    for i in 0..=(n - m) {
        let mut matched = true;
        for j in 0..m {
            if t[i + j] != p[j] {
                matched = false;
                break;
            }
        }
        if matched {
            results.push(i);
        }
    }
    results
}

fn main() {
    let text = "ABABAC";
    let pattern = "ABAC";
    let matches = naive_search(text, pattern);
    println!("Pattern found at positions: {:?}", matches);
    // Output: Pattern found at positions: [2]
}
```

### Complexity

| | Best | Worst | Space |
|---|---|---|---|
| Naive | O(n) | O(n * m) | O(1) |

The worst case hits when both text and pattern are repetitive, like
text = "AAAAAAAAB" and pattern = "AAAAB". You compare almost all of pattern
at every position before the mismatch.

**When is naive good enough?** For short patterns (m < 10) on moderate text,
the simplicity wins. The constant factors are tiny. But when m is large or the
alphabet is small (DNA: 4 characters), you need something smarter.

---

## Part 2: KMP (Knuth-Morris-Pratt)

### The Key Insight

When a mismatch happens, the naive algorithm throws away everything it learned
and starts over from the next position. That is wasteful. KMP asks: "I already
matched some prefix of the pattern. Is there a shorter prefix of the pattern
that is also a suffix of what I matched? If so, I can skip ahead."

### The Analogy

Imagine you are reading a long book looking for the phrase "abracadabra". You
have matched "abracad" and then hit a wrong letter. Instead of going back to
the start, you notice that the "a" at the end of "abracad" could be the start
of a new match. You slide the pattern forward so that "a" aligns, without
re-reading any of the text you already scanned.

### The Failure Function (Partial Match Table)

The magic of KMP is a preprocessed table called the **failure function** (or
"partial match table" or "prefix function"). For each position `j` in the
pattern, `fail[j]` stores the length of the longest proper prefix of
`pattern[0..=j]` that is also a suffix.

```
Pattern:  A  B  A  B  A  C
Index:    0  1  2  3  4  5
fail[]:   0  0  1  2  3  0

Let's trace it:

  j=0: "A"         -- no proper prefix/suffix possible  -> 0
  j=1: "AB"        -- prefixes: "A"
                      suffixes: "B"       no match      -> 0
  j=2: "ABA"       -- prefixes: "A", "AB"
                      suffixes: "A", "BA"
                      "A" == "A"                        -> 1
  j=3: "ABAB"      -- prefixes: "A", "AB", "ABA"
                      suffixes: "B", "AB", "BAB"
                      "AB" == "AB"                      -> 2
  j=4: "ABABA"     -- prefixes: "A", "AB", "ABA", "ABAB"
                      suffixes: "A", "BA", "ABA", "BABA"
                      "ABA" == "ABA"                    -> 3
  j=5: "ABABAC"    -- no prefix ending in C matches     -> 0
```

### How Matching Works

```
Text:     A  B  A  B  A  B  A  B  A  C
Pattern:  A  B  A  B  A  C
                         ^
                    mismatch at j=5 (text[5]='B', pattern[5]='C')
                    fail[5-1] = fail[4] = 3
                    so we set j=3 and continue without moving i back

Text:     A  B  A  B  A  B  A  B  A  C
Pattern:           A  B  A  B  A  C
                            ^
                    now comparing from j=3 onward
                    (we skipped re-checking the first 3 chars)
```

### Rust Implementation

```rust
/// Build the KMP failure function for a pattern.
/// fail[j] = length of longest proper prefix of pattern[0..=j]
///           that is also a suffix of pattern[0..=j].
fn build_kmp_table(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let mut fail = vec![0usize; m];

    // k tracks the length of the current matching prefix
    let mut k = 0;
    for i in 1..m {
        // While we have a partial match that doesn't extend, fall back
        while k > 0 && pattern[k] != pattern[i] {
            k = fail[k - 1];
        }
        if pattern[k] == pattern[i] {
            k += 1;
        }
        fail[i] = k;
    }

    fail
}

/// KMP search: returns all starting indices of pattern in text.
/// Time:  O(n + m) -- each character of text is visited at most twice
/// Space: O(m) for the failure table
fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut results = Vec::new();
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let n = t.len();
    let m = p.len();

    if m == 0 || m > n {
        return results;
    }

    let fail = build_kmp_table(p);

    let mut j = 0; // position in pattern
    for i in 0..n {
        while j > 0 && p[j] != t[i] {
            j = fail[j - 1];
        }
        if p[j] == t[i] {
            j += 1;
        }
        if j == m {
            results.push(i + 1 - m);
            j = fail[j - 1]; // look for overlapping matches
        }
    }
    results
}

fn main() {
    let text = "ABABABABAC";
    let pattern = "ABABAC";
    let matches = kmp_search(text, pattern);
    println!("KMP matches at: {:?}", matches);
    // Output: KMP matches at: [4]
}
```

### Why O(n + m)?

The failure table is built in O(m) -- the variable `k` can only increase m
times total, so it can only decrease m times total. The matching phase is O(n)
by the same argument on `j`. Total: O(n + m).

---

## Part 3: Rabin-Karp (Rolling Hash)

### The Key Insight

Instead of comparing characters, compare **hashes**. Compute a hash of the
pattern, then slide a window across the text, updating the hash in O(1) at each
step. Only do a full character comparison when the hashes match.

### The Analogy

Imagine you are a librarian checking if any page in a 1000-page book contains a
plagiarized paragraph. Instead of reading every paragraph word-by-word, you
assign each paragraph a numeric "fingerprint" (its hash). You pre-compute the
fingerprint of the suspect paragraph, then scan through the book comparing
fingerprints -- a single number comparison instead of a string comparison.
When fingerprints match, you read the actual text to confirm (because two
different paragraphs could have the same fingerprint -- a collision).

### Rolling Hash Mechanics

We treat each substring as a number in base `B` modulo a prime `M`.

```
Hash("ABAC") with B=256, M=101 (small prime for illustration):

  hash = (A*256^3 + B*256^2 + A*256^1 + C*256^0) mod 101

Sliding window -- remove leftmost char, add rightmost:

  Text:  ... [A][B][A][C][D] ...
         old window: A B A C   hash_old
         new window: B A C D   hash_new

  hash_new = (hash_old - A * 256^3) * 256 + D   (all mod M)
             ^^^^^^^^^^^^^^^^^^^^    ^^^   ^^^
             remove old left char    shift  add new right char
```

### ASCII-Art: Rolling Hash in Action

```
Text:    B  A  N  A  N  A  S
Pattern: A  N  A           hash_p = hash("ANA")

Step 1: window = "BAN"     hash_w = hash("BAN") != hash_p  -> skip
Step 2: window = "ANA"     hash_w = hash("ANA") == hash_p  -> verify -> MATCH at 1
Step 3: window = "NAN"     hash_w = hash("NAN") != hash_p  -> skip
Step 4: window = "ANA"     hash_w = hash("ANA") == hash_p  -> verify -> MATCH at 3
Step 5: window = "NAS"     hash_w = hash("NAS") != hash_p  -> skip
```

### Rust Implementation

```rust
/// Rabin-Karp search using a rolling polynomial hash.
/// Time:  O(n + m) expected, O(n * m) worst case (many hash collisions)
/// Space: O(1) extra
fn rabin_karp_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut results = Vec::new();
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let n = t.len();
    let m = p.len();

    if m == 0 || m > n {
        return results;
    }

    let base: u64 = 256;
    let modulus: u64 = 1_000_000_007; // large prime reduces collisions

    // Precompute base^(m-1) mod modulus  (the "high digit" multiplier)
    let mut h = 1u64;
    for _ in 0..(m - 1) {
        h = (h * base) % modulus;
    }

    // Compute initial hashes for pattern and first window of text
    let mut hash_p: u64 = 0;
    let mut hash_w: u64 = 0;
    for i in 0..m {
        hash_p = (hash_p * base + p[i] as u64) % modulus;
        hash_w = (hash_w * base + t[i] as u64) % modulus;
    }

    for i in 0..=(n - m) {
        // If hashes match, verify character by character (avoid false positives)
        if hash_w == hash_p {
            if t[i..(i + m)] == p[..] {
                results.push(i);
            }
        }

        // Roll the hash forward: remove t[i], add t[i + m]
        if i < n - m {
            // Add modulus before subtraction to avoid underflow
            hash_w = (hash_w + modulus - (t[i] as u64 * h) % modulus) % modulus;
            hash_w = (hash_w * base + t[i + m] as u64) % modulus;
        }
    }
    results
}

fn main() {
    let text = "BANANAS";
    let pattern = "ANA";
    let matches = rabin_karp_search(text, pattern);
    println!("Rabin-Karp matches at: {:?}", matches);
    // Output: Rabin-Karp matches at: [1, 3]
}
```

### Multi-Pattern Matching

Rabin-Karp shines when you need to search for **many patterns at once**. Compute
hashes for all patterns, store them in a `HashSet`, then roll one hash across
the text and check set membership at each position. This is O(n * k) for k
patterns of the same length, versus O(n * m * k) for naive.

```rust
use std::collections::HashSet;

/// Search for multiple patterns of the SAME length in text.
fn rabin_karp_multi(text: &str, patterns: &[&str]) -> Vec<(usize, String)> {
    let mut results = Vec::new();
    if patterns.is_empty() {
        return results;
    }

    let t = text.as_bytes();
    let n = t.len();
    let m = patterns[0].len(); // all patterns same length
    let base: u64 = 256;
    let modulus: u64 = 1_000_000_007;

    // Build set of pattern hashes
    let mut pattern_hashes = HashSet::new();
    for &pat in patterns {
        let p = pat.as_bytes();
        let mut h: u64 = 0;
        for &b in p {
            h = (h * base + b as u64) % modulus;
        }
        pattern_hashes.insert(h);
    }

    // Precompute h = base^(m-1) mod modulus
    let mut h_pow = 1u64;
    for _ in 0..(m - 1) {
        h_pow = (h_pow * base) % modulus;
    }

    // Hash first window
    let mut hash_w: u64 = 0;
    for i in 0..m {
        hash_w = (hash_w * base + t[i] as u64) % modulus;
    }

    for i in 0..=(n - m) {
        if pattern_hashes.contains(&hash_w) {
            // Verify against all patterns (collision check)
            let window = &text[i..i + m];
            for &pat in patterns {
                if window == pat {
                    results.push((i, pat.to_string()));
                }
            }
        }
        if i < n - m {
            hash_w = (hash_w + modulus - (t[i] as u64 * h_pow) % modulus) % modulus;
            hash_w = (hash_w * base + t[i + m] as u64) % modulus;
        }
    }
    results
}
```

---

## Part 4: Z-Function (Z-Array)

### The Key Insight

For a string `s`, `z[i]` is the length of the longest substring starting at
position `i` that matches a prefix of `s`. The Z-array encodes all the
prefix-matching information in O(n) time, and you can solve pattern matching
by concatenating `pattern + "$" + text` and computing one Z-array.

### Building the Z-Array: ASCII-Art Walkthrough

```
String:  a  a  b  x  a  a  b
Index:   0  1  2  3  4  5  6
Z:      [7, 1, 0, 0, 3, 1, 0]

z[0] = 7  (by convention, length of the whole string, or sometimes undefined)
z[1] = 1  "a" matches prefix "a", but "ab..." != "aa..."
z[2] = 0  "b" != "a" (first char of string)
z[3] = 0  "x" != "a"
z[4] = 3  "aab" matches prefix "aab"
z[5] = 1  "a" matches prefix "a", but "ab" position would go past end
z[6] = 0  "b" != "a"
```

### Using Z-Array for Pattern Matching

Concatenate: `pattern + "$" + text`, then build the Z-array. Any position `i`
where `z[i] == pattern.len()` is a match in the text.

```
Pattern: "ABA"    Text: "ABABAC"
Combined: "ABA$ABABAC"

Index:     0  1  2  3  4  5  6  7  8  9
Char:      A  B  A  $  A  B  A  B  A  C
Z:        [10, 0, 1, 0, 3, 0, 3, 0, 1, 0]
                       ^        ^
                       z[4]=3   z[6]=3
                       (= len("ABA") = 3, so match!)

Match positions in original text: 4-3-1=0 and 6-3-1=2
  (subtract pattern.len() + 1 for the "$" separator)
```

### Rust Implementation

```rust
/// Build the Z-array for string `s`.
/// z[i] = length of longest substring starting at i that matches a prefix of s.
/// Time:  O(n)
/// Space: O(n)
fn build_z_array(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0usize; n];
    z[0] = n; // by convention

    let mut l = 0; // left boundary of the current Z-box
    let mut r = 0; // right boundary (exclusive) of the current Z-box

    for i in 1..n {
        if i < r {
            // We are inside a known matching region.
            // z[i - l] tells us the match length from the mirrored position.
            z[i] = z[i - l].min(r - i);
        }

        // Try to extend the match beyond what we know
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }

        // Update the Z-box if we extended past r
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

/// Z-function based pattern search.
/// Time:  O(n + m)
/// Space: O(n + m)
fn z_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut results = Vec::new();
    let p = pattern.as_bytes();
    let t = text.as_bytes();
    let m = p.len();

    if m == 0 || m > t.len() {
        return results;
    }

    // Build combined string: pattern + '$' + text
    let mut combined = Vec::with_capacity(m + 1 + t.len());
    combined.extend_from_slice(p);
    combined.push(b'$'); // separator that appears in neither input
    combined.extend_from_slice(t);

    let z = build_z_array(&combined);

    // Any index i in the combined string where z[i] == m is a match.
    // The match starts at position (i - m - 1) in the original text.
    for i in (m + 1)..combined.len() {
        if z[i] == m {
            results.push(i - m - 1);
        }
    }
    results
}

fn main() {
    let text = "ABABAC";
    let pattern = "ABA";
    let matches = z_search(text, pattern);
    println!("Z-function matches at: {:?}", matches);
    // Output: Z-function matches at: [0, 2]
}
```

### Why Z-Array is Useful Beyond Matching

The Z-array has several direct applications:
- **Counting distinct substrings** of a given length.
- **Finding the shortest period** of a string (smallest p such that s is a
  repetition of s[0..p]).
- **String compression**: if `n % (n - z[i]) == 0` for some i, the string has
  a repeating unit.

---

## Part 5: Longest Palindromic Substring (Expand from Center)

### The Idea

A palindrome reads the same forwards and backwards. Every palindrome has a
center. For odd-length palindromes the center is one character; for even-length
it falls between two characters. We can try every possible center and expand
outward while characters match.

```
String:  b  a  b  a  d
         0  1  2  3  4

Center at index 2 (odd expansion):
  Expand: s[2]='b'                           -> "b"   (length 1)
  Expand: s[1]='a' == s[3]='a'?  YES         -> "aba" (length 3)
  Expand: s[0]='b' == s[4]='d'?  NO          -> stop

  Longest palindrome centered at 2: "aba" (length 3, starts at 1)

Center between index 0 and 1 (even expansion):
  Expand: s[0]='b' == s[1]='a'?  NO          -> stop (length 0)
```

### Rust Implementation

```rust
/// Find the longest palindromic substring.
/// Time:  O(n^2) worst case, but fast in practice
/// Space: O(1) extra (just indices)
fn longest_palindrome(s: &str) -> &str {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return s;
    }

    let mut best_start = 0;
    let mut best_len = 1;

    // Expand around center, return length of palindrome found
    let expand = |mut left: isize, mut right: isize| -> (usize, usize) {
        while left >= 0 && (right as usize) < n && b[left as usize] == b[right as usize]
        {
            left -= 1;
            right += 1;
        }
        // After the loop, left and right are one step past the palindrome bounds
        let start = (left + 1) as usize;
        let length = (right - left - 1) as usize;
        (start, length)
    };

    for i in 0..n {
        // Odd-length palindrome centered at i
        let (start1, len1) = expand(i as isize, i as isize);
        if len1 > best_len {
            best_start = start1;
            best_len = len1;
        }

        // Even-length palindrome centered between i and i+1
        let (start2, len2) = expand(i as isize, (i + 1) as isize);
        if len2 > best_len {
            best_start = start2;
            best_len = len2;
        }
    }

    &s[best_start..best_start + best_len]
}

fn main() {
    println!("{}", longest_palindrome("babad"));   // "bab" or "aba"
    println!("{}", longest_palindrome("cbbd"));    // "bb"
    println!("{}", longest_palindrome("racecar")); // "racecar"
}
```

---

## Part 6: String Hashing for Fast Comparison

### The Idea

Sometimes you need to compare many substrings for equality. Direct comparison
is O(length), but if you precompute prefix hashes with a polynomial rolling
hash, you can compare any two substrings in O(1).

This is the backbone of many competitive programming solutions and powers
algorithms like Rabin-Karp internally.

```
String:   a  b  c  a  b
Index:    0  1  2  3  4

prefix_hash[0] = a
prefix_hash[1] = a*B + b
prefix_hash[2] = a*B^2 + b*B + c
prefix_hash[3] = a*B^3 + b*B^2 + c*B + a
prefix_hash[4] = a*B^4 + b*B^3 + c*B^2 + a*B + b

Hash of substring s[2..=4] = "cab":
  = prefix_hash[4] - prefix_hash[1] * B^3   (all mod M)
```

### Rust Implementation

```rust
struct StringHasher {
    prefix: Vec<u64>,
    power: Vec<u64>,
    modulus: u64,
}

impl StringHasher {
    const BASE: u64 = 31;

    /// Build prefix hashes for the given string.
    fn new(s: &str, modulus: u64) -> Self {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut prefix = vec![0u64; n + 1];
        let mut power = vec![1u64; n + 1];

        for i in 0..n {
            prefix[i + 1] = (prefix[i] * Self::BASE + bytes[i] as u64) % modulus;
            power[i + 1] = (power[i] * Self::BASE) % modulus;
        }

        Self { prefix, power, modulus }
    }

    /// Get the hash of s[l..r] (0-indexed, exclusive end) in O(1).
    fn query(&self, l: usize, r: usize) -> u64 {
        let raw = self.prefix[r] + self.modulus
            - (self.prefix[l] * self.power[r - l]) % self.modulus;
        raw % self.modulus
    }
}

fn main() {
    let s = "abcab";
    let hasher = StringHasher::new(s, 1_000_000_007);

    // Compare "ab" (indices 0..2) with "ab" (indices 3..5)
    let h1 = hasher.query(0, 2);
    let h2 = hasher.query(3, 5);
    println!("hash(s[0..2]) = {}, hash(s[3..5]) = {}", h1, h2);
    println!("Equal? {}", h1 == h2); // true

    // Compare "ab" (0..2) with "bc" (1..3)
    let h3 = hasher.query(1, 3);
    println!("hash(s[0..2]) = {}, hash(s[1..3]) = {}", h1, h3);
    println!("Equal? {}", h1 == h3); // false
}
```

**Collision warning**: Two different substrings can produce the same hash. For
contest/interview safety, either (a) use two different moduli and compare both
hashes, or (b) verify with direct comparison on hash match. Option (a) makes
collision probability roughly 1/10^18.

---

## Complexity Summary

```
+---------------------+-----------+-----------+----------+-------------------+
| Algorithm           | Preprocess| Search    | Space    | Notes             |
+---------------------+-----------+-----------+----------+-------------------+
| Naive               | O(1)      | O(n*m)    | O(1)     | Simple, no setup  |
| KMP                 | O(m)      | O(n)      | O(m)     | Guaranteed linear |
| Rabin-Karp          | O(m)      | O(n) avg  | O(1)     | O(n*m) worst      |
|                     |           | O(n*m) wc |          |   (hash collisions|
| Z-function          | O(n+m)    | O(n+m)    | O(n+m)   | Simple, versatile |
| Expand from center  | O(1)      | O(n^2)    | O(1)     | Palindromes       |
| Prefix hashing      | O(n)      | O(1)/query| O(n)     | Substring compare |
+---------------------+-----------+-----------+----------+-------------------+

n = text length,  m = pattern length
```

---

## When to Use Which Algorithm

This is the most important section if you are preparing for interviews.

**Use naive when:**
- Pattern is very short (< ~10 characters).
- You are asked to implement string search and simplicity is more important than
  asymptotic speed.
- The problem constraints guarantee small input (n * m < 10^7).

**Use KMP when:**
- You need **guaranteed** O(n + m) worst-case time.
- The problem involves searching in a text with low-entropy / repetitive content
  (like DNA strings or binary strings), where naive degrades.
- You also need to find the **period** of a string or the failure function
  itself has structural meaning.

**Use Rabin-Karp when:**
- You need to search for **multiple patterns** of the same length.
- You are doing **2D pattern matching** (matching a sub-grid in a grid).
- The problem involves detecting **duplicate substrings** (plagiarism style).

**Use Z-function when:**
- You want a clean, easy-to-code linear-time search alternative to KMP.
- The problem specifically asks about **prefix matching** properties.
- You need the Z-array for downstream computation (periods, distinct substrings).

**Use expand-from-center when:**
- The problem is about **palindromes** (longest palindromic substring, count of
  palindromic substrings).
- Input size is moderate (n up to ~10,000). For n up to 10^6, consider Manacher's
  algorithm (O(n), but harder to implement).

**Use prefix hashing when:**
- You need to compare many pairs of substrings efficiently.
- The problem involves checking if rotations of a string are equal, or finding
  the longest common substring between two strings.

---

## Common Pitfalls in Rust

1. **Off-by-one with `.as_bytes()` and slicing**: Remember `s[i..j]` is
   exclusive on `j`. The substring has length `j - i`.

2. **Modular arithmetic underflow**: When subtracting hashes, always add the
   modulus before subtracting to prevent u64 underflow. The pattern is:
   `(a + modulus - b % modulus) % modulus`.

3. **`String` vs `&str` ownership in results**: If the function signature
   returns `&str`, the lifetime ties to the input. If you need owned results,
   return `String` (with `.to_string()` or `String::from()`).

4. **Char vs byte iteration**: `s.chars()` iterates Unicode scalar values
   (variable width). `s.bytes()` or `s.as_bytes()` gives raw bytes. For ASCII
   problems, always use bytes.

5. **Pattern length 0**: Always handle the empty pattern case explicitly. Most
   algorithms break or behave unexpectedly with m = 0.

---

## Practice Problems

### Easy (Warm-up and fundamentals)

| # | Problem | Key Technique |
|---|---------|---------------|
| 1 | [Implement strStr()](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/) | Naive or KMP |
| 2 | [Repeated Substring Pattern](https://leetcode.com/problems/repeated-substring-pattern/) | KMP failure function |
| 3 | [Is Subsequence](https://leetcode.com/problems/is-subsequence/) | Two pointers on strings |
| 4 | [Valid Palindrome](https://leetcode.com/problems/valid-palindrome/) | Two pointers |
| 5 | [Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/) | Vertical scan or binary search |

### Medium (Core algorithm application)

| # | Problem | Key Technique |
|---|---------|---------------|
| 1 | [Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/) | Expand from center |
| 2 | [Repeated DNA Sequences](https://leetcode.com/problems/repeated-dna-sequences/) | Rabin-Karp / rolling hash |
| 3 | [Longest Happy Prefix](https://leetcode.com/problems/longest-happy-prefix/) | KMP failure function or Z-array |
| 4 | [Shortest Palindrome](https://leetcode.com/problems/shortest-palindrome/) | KMP on reversed + original |
| 5 | [Distinct Echo Substrings](https://leetcode.com/problems/distinct-echo-substrings/) | String hashing |

### Hard (Deep understanding required)

| # | Problem | Key Technique |
|---|---------|---------------|
| 1 | [Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/) | Sliding window + hash map |
| 2 | [Longest Duplicate Substring](https://leetcode.com/problems/longest-duplicate-substring/) | Binary search + Rabin-Karp |
| 3 | [Palindrome Pairs](https://leetcode.com/problems/palindrome-pairs/) | Trie + palindrome check |
| 4 | [Count of Substrings Containing Every Vowel and K Consonants II](https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii/) | Sliding window + hashing |
| 5 | [Sum of Scores of Built Strings](https://leetcode.com/problems/sum-of-scores-of-built-strings/) | Z-function directly |

---

## Key Takeaways

1. **Naive is O(nm) and that is often fine.** Do not over-engineer when
   constraints are small. But know when it is not enough.

2. **KMP and Z-function both achieve O(n + m)** for single-pattern matching.
   KMP uses the failure function; Z-function uses the Z-array. They encode
   the same structural information differently. Learn whichever clicks for you;
   know both exist.

3. **Rabin-Karp trades deterministic guarantees for flexibility.** Its expected
   time is O(n + m) but worst case is O(nm) due to hash collisions. Its
   superpower is multi-pattern matching and 2D matching.

4. **Prefix hashing gives O(1) substring comparison** after O(n) preprocessing.
   This is a Swiss Army knife for string problems.

5. **In Rust, work with `&[u8]` for ASCII string algorithms.** It gives you
   O(1) indexing and avoids UTF-8 boundary issues. Convert early with
   `.as_bytes()` and work in byte-land throughout.

6. **The failure function / Z-array are useful data structures in their own
   right**, not just subroutines of search algorithms. Many problems reduce to
   "build this array and read off the answer."
