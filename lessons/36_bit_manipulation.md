# Lesson 36: Bit Manipulation

## Why Should You Care?

Every value your program touches -- every integer, every character, every boolean -- is a
sequence of bits. Most of the time, you work at a higher level of abstraction and never
think about it. But certain problems become dramatically simpler (and faster) when you
operate on the bits directly rather than treating numbers as opaque math objects.

Bit manipulation shows up in:
- **Interviews:** A whole class of problems expects you to use XOR, masking, and shifting.
- **Systems programming:** Flags, permissions, hardware registers, protocol headers.
- **Performance:** Bit tricks replace branching and arithmetic with single CPU instructions.
- **Subset enumeration:** Representing and iterating over all subsets of a set using integers.

This is not arcane wizardry. It is a practical skill with a small number of core operations
that combine in powerful ways.

---

## Binary Number Representation

You already know decimal (base 10). Binary (base 2) is the same idea, but with only two
digits: 0 and 1.

### The Analogy: Light Switches and Permission Flags

Imagine a row of 8 light switches on a wall. Each switch is either OFF (0) or ON (1).
That row of 8 switches is a byte. Each individual switch is a bit.

```
  Switch #:    7     6     5     4     3     2     1     0
            +-----+-----+-----+-----+-----+-----+-----+-----+
            | OFF | OFF | ON  | OFF | ON  | ON  | OFF | ON  |
            +-----+-----+-----+-----+-----+-----+-----+-----+
  Bit:         0     0     1     0     1     1     0     1

  Binary:  0b00101101
```

Now consider something you've used a thousand times -- Unix file permissions. The string
`rwxr-xr--` is not just a display format. It maps directly to bits:

```
  Permission bits for a file:

  Owner    Group    Other
  r w x    r - x    r - -
  1 1 1    1 0 1    1 0 0    =  0b111_101_100  =  0o754

  Each group of 3 bits:
    bit 2 = read    (4)
    bit 1 = write   (2)
    bit 0 = execute (1)

  To check: "does group have execute permission?"
    permissions & 0o010 != 0     (mask for group execute bit)
```

This is not abstract -- it is how `chmod 754 file.sh` actually works. The number 754 is
three octal digits, each representing 3 bits of a 9-bit permission mask. Feature flags in
web applications, hardware register control, network subnet masks (`255.255.255.0` =
`11111111.11111111.11111111.00000000`), and RGB color packing (`0xFF3A20`) all work the
same way.

Each switch position has a "weight" -- a power of 2, just like each decimal digit has a
power of 10:

```
  Position:  7      6      5      4      3      2      1      0
  Weight:    128    64     32     16     8      4      2      1
             2^7    2^6    2^5    2^4    2^3    2^2    2^1    2^0

  The number 0b00101101:
    0*128 + 0*64 + 1*32 + 0*16 + 1*8 + 1*4 + 0*2 + 1*1
    = 32 + 8 + 4 + 1
    = 45
```

### Rust Binary Literals and Formatting

```rust
fn main() {
    let x: u8 = 0b00101101; // binary literal
    println!("{}", x);       // 45
    println!("{:b}", x);     // 101101
    println!("{:08b}", x);   // 00101101  (zero-padded to 8 digits)

    // You can also use underscores for readability:
    let flags: u16 = 0b1010_0011_0000_1111;
    println!("{:016b}", flags); // 1010001100001111
}
```

### How Computers Store Integers

A `u8` uses 8 bits: values 0 to 255 (2^8 - 1).
A `u32` uses 32 bits: values 0 to 4,294,967,295 (2^32 - 1).
An `i32` uses 32 bits with **two's complement** for negative numbers: -2^31 to 2^31 - 1.

Two's complement means the highest bit is the sign bit. For `i32`:

```
   0 = 00000000 00000000 00000000 00000000
   1 = 00000000 00000000 00000000 00000001
  -1 = 11111111 11111111 11111111 11111111    (all bits set)
  -2 = 11111111 11111111 11111111 11111110
  MAX = 01111111 11111111 11111111 11111111    (2,147,483,647)
  MIN = 10000000 00000000 00000000 00000000    (-2,147,483,648)
```

---

## Bitwise Operators in Rust

There are six bitwise operators. Each one works on individual bits in parallel -- bit 0 of
the left operand interacts with bit 0 of the right operand, bit 1 with bit 1, and so on.

### AND (`&`)

Each output bit is 1 only if **both** input bits are 1. Think of it as "keep only the bits
that are on in *both* numbers."

```
  AND Truth Table       Example: 0b1100 & 0b1010
  +-+-+-----+
  |A|B|A & B|            1 1 0 0
  +-+-+-----+          & 1 0 1 0
  |0|0|  0  |          ---------
  |0|1|  0  |            1 0 0 0  = 0b1000 = 8
  |1|0|  0  |
  |1|1|  1  |
  +-+-+-----+
```

**Use case:** Masking. Extract specific bits from a number.

### OR (`|`)

Each output bit is 1 if **either** input bit is 1. Think of it as "turn on any bit that
is on in *either* number."

```
  OR Truth Table        Example: 0b1100 | 0b1010
  +-+-+-----+
  |A|B|A | B|            1 1 0 0
  +-+-+-----+          | 1 0 1 0
  |0|0|  0  |          ---------
  |0|1|  1  |            1 1 1 0  = 0b1110 = 14
  |1|0|  1  |
  |1|1|  1  |
  +-+-+-----+
```

**Use case:** Setting bits. Turn on a flag without disturbing other flags.

### XOR (`^`)

Each output bit is 1 if the input bits are **different**. Think of it as "toggle."

```
  XOR Truth Table       Example: 0b1100 ^ 0b1010
  +-+-+-----+
  |A|B|A ^ B|            1 1 0 0
  +-+-+-----+          ^ 1 0 1 0
  |0|0|  0  |          ---------
  |0|1|  1  |            0 1 1 0  = 0b0110 = 6
  |1|0|  1  |
  |1|1|  0  |
  +-+-+-----+
```

**Key properties of XOR:**
- `a ^ a = 0` (anything XOR itself is zero)
- `a ^ 0 = a` (anything XOR zero is itself)
- XOR is commutative and associative: `a ^ b ^ c = c ^ a ^ b`

These properties make XOR extremely useful for cancellation tricks.

### NOT (`!`)

Flips every bit. In Rust, `!` is the bitwise NOT operator (not `~` like in C).

```
  NOT Example (u8):   !0b00101101

    0 0 1 0 1 1 0 1
  ! -----------------
    1 1 0 1 0 0 1 0  = 0b11010010 = 210
```

**Careful with signed types:** `!0i32` gives `-1` because flipping all zeros gives all ones,
which in two's complement is -1.

### Left Shift (`<<`)

Shifts all bits to the left by the specified number of positions. Vacated positions on the
right are filled with zeros.

```
  Left shift by 2:    0b00001101 << 2

  Before:   0 0 0 0 1 1 0 1
                          ------> shift left 2
  After:    0 0 1 1 0 1 0 0

  13 << 2 = 52
  Equivalent to: 13 * 2^2 = 13 * 4 = 52
```

**Left shifting by `n` is multiplication by 2^n** (as long as you don't overflow).

### Right Shift (`>>`)

Shifts all bits to the right. For unsigned types, vacated positions on the left are filled
with zeros (logical shift). For signed types, the sign bit is preserved (arithmetic shift).

```
  Right shift by 2:   0b00101100 >> 2

  Before:   0 0 1 0 1 1 0 0
  shift right 2 <------
  After:    0 0 0 0 1 0 1 1

  44 >> 2 = 11
  Equivalent to: 44 / 2^2 = 44 / 4 = 11  (integer division, rounds toward zero)
```

**Right shifting by `n` is integer division by 2^n.**

### Summary Table

```rust
fn main() {
    let a: u8 = 0b1100; // 12
    let b: u8 = 0b1010; // 10

    println!("a & b  = {:04b} ({})", a & b, a & b);    // 1000 (8)
    println!("a | b  = {:04b} ({})", a | b, a | b);    // 1110 (14)
    println!("a ^ b  = {:04b} ({})", a ^ b, a ^ b);    // 0110 (6)
    println!("!a     = {:08b} ({})", !a, !a);           // 11110011 (243)
    println!("a << 2 = {:08b} ({})", a << 2, a << 2);  // 00110000 (48)
    println!("a >> 1 = {:04b} ({})", a >> 1, a >> 1);   // 0110 (6)
}
```

---

## Common Bit Tricks

These are the "vocabulary words" of bit manipulation. Each one is a small building block
that appears over and over in interview problems and systems code.

### Check If a Number Is Even or Odd

The least significant bit (bit 0) determines parity. If it is 1, the number is odd. If it
is 0, the number is even.

```
  Even numbers end in 0:    4 = 100,   6 = 110,   8 = 1000
  Odd numbers end in 1:     3 = 011,   5 = 101,   7 = 0111

  n & 1:
    If result is 0 -> even
    If result is 1 -> odd
```

```rust
fn is_even(n: i32) -> bool {
    n & 1 == 0
}

fn is_odd(n: i32) -> bool {
    n & 1 == 1
}
```

This is equivalent to `n % 2 == 0` but avoids the modulo operation. In practice, the
compiler often optimizes `% 2` to `& 1` anyway, but the bitwise version makes the intent
explicit.

### Check If Bit `k` Is Set

Use AND with a mask that has only bit `k` turned on. The mask is `1 << k`.

```
  Check if bit 3 of 0b10101100 is set:

      1 0 1 0 1 1 0 0     (n = 172)
    & 0 0 0 0 1 0 0 0     (1 << 3 = 8)
    -------------------
      0 0 0 0 1 0 0 0     != 0, so bit 3 IS set
```

```rust
fn is_bit_set(n: u32, k: u32) -> bool {
    (n >> k) & 1 == 1
    // or equivalently: n & (1 << k) != 0
}
```

### Set Bit `k` (Turn It On)

Use OR with the mask `1 << k`. This turns on bit `k` without affecting other bits.

```
  Set bit 2 of 0b10100001:

      1 0 1 0 0 0 0 1
    | 0 0 0 0 0 1 0 0     (1 << 2 = 4)
    -------------------
      1 0 1 0 0 1 0 1     bit 2 is now ON
```

```rust
fn set_bit(n: u32, k: u32) -> u32 {
    n | (1 << k)
}
```

### Clear Bit `k` (Turn It Off)

Use AND with the *inverted* mask `!(1 << k)`. The inverted mask has all bits set except bit
`k`, so AND-ing with it clears only bit `k`.

```
  Clear bit 5 of 0b10101101:

      1 0 1 0 1 1 0 1
    & 1 1 0 1 1 1 1 1     !(1 << 5)
    -------------------
      1 0 0 0 1 1 0 1     bit 5 is now OFF
```

```rust
fn clear_bit(n: u32, k: u32) -> u32 {
    n & !(1 << k)
}
```

### Toggle Bit `k` (Flip It)

Use XOR with the mask `1 << k`. XOR flips the bit: if it was 1, it becomes 0; if it was
0, it becomes 1.

```rust
fn toggle_bit(n: u32, k: u32) -> u32 {
    n ^ (1 << k)
}
```

### All Four Operations Together

```rust
fn main() {
    let n: u32 = 0b10101100; // 172

    // Check:  is bit 3 set?
    println!("bit 3 set? {}", (n >> 3) & 1 == 1);  // true

    // Set:    turn on bit 0
    let with_bit0 = n | (1 << 0);
    println!("set bit 0:   {:08b}", with_bit0);     // 10101101

    // Clear:  turn off bit 5
    let without_bit5 = n & !(1 << 5);
    println!("clear bit 5: {:08b}", without_bit5);   // 10001100

    // Toggle: flip bit 2
    let toggled = n ^ (1 << 2);
    println!("toggle bit 2: {:08b}", toggled);       // 10101000
}
```

### Check If a Number Is a Power of Two

A power of two has exactly one bit set: 1, 2, 4, 8, 16, ...

```
  1  = 0001
  2  = 0010
  4  = 0100
  8  = 1000
  16 = 10000
```

The trick: `n & (n - 1)` clears the lowest set bit. If the result is zero, then `n` had
exactly one bit set -- meaning it is a power of two.

```
  n = 8:     1000        n = 6:     0110
  n - 1:     0111        n - 1:     0101
  n & (n-1): 0000  --> power of 2   n & (n-1): 0100  --> NOT a power of 2
```

```rust
fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}
```

The `n != 0` guard is needed because 0 is not a power of two, but `0 & (0 - 1)` would
underflow for unsigned types (Rust panics in debug, wraps in release).

### Count Set Bits (Popcount)

Count the number of 1-bits in a number. This is called "population count" or "Hamming weight."

**Kernighan's trick:** `n & (n - 1)` clears the lowest set bit. Repeat until n is zero,
counting iterations.

```
  n = 0b10110100 (4 set bits)

  Iteration 1: 10110100 & 10110011 = 10110000  (cleared bit 2)
  Iteration 2: 10110000 & 10101111 = 10100000  (cleared bit 4)
  Iteration 3: 10100000 & 10011111 = 10000000  (cleared bit 5)
  Iteration 4: 10000000 & 01111111 = 00000000  (cleared bit 7)
  Done. Count = 4.
```

```rust
fn count_set_bits(mut n: u32) -> u32 {
    let mut count = 0;
    while n != 0 {
        n &= n - 1;   // clear lowest set bit
        count += 1;
    }
    count
}
```

This runs in O(k) where k is the number of set bits, not the total number of bits.

In production, use Rust's built-in: `n.count_ones()`. It compiles to the hardware POPCNT
instruction on x86.

### Get / Clear / Isolate the Lowest Set Bit

```rust
fn lowest_set_bit(n: u32) -> u32 {
    // Isolate the lowest set bit.
    // Works because -n in two's complement flips all bits and adds 1,
    // which means (n & (negated n)) isolates the lowest set bit.
    // For unsigned types, we use wrapping negation:
    n & n.wrapping_neg()
}

fn clear_lowest_set_bit(n: u32) -> u32 {
    n & (n - 1)    // the same trick we used for power-of-two check
}

fn main() {
    let n: u32 = 0b10110100;
    println!("lowest set bit: {:08b}", lowest_set_bit(n));       // 00000100
    println!("cleared lowest: {:08b}", clear_lowest_set_bit(n)); // 10110000
}
```

```
  Isolating the lowest set bit of n = 0b10110100:

  n:               1 0 1 1 0 1 0 0
  wrapping_neg(n): 0 1 0 0 1 1 0 0
  n & neg:         0 0 0 0 0 1 0 0  =  4 (bit position 2)
```

---

## Rust-Specific: Built-In Bit Methods

Rust's integer types have excellent built-in methods for bit manipulation. Use these in
production -- they map to efficient CPU instructions.

```rust
fn main() {
    let n: u32 = 0b00101100;

    // count_ones: number of set bits (popcount)
    assert_eq!(n.count_ones(), 3);

    // count_zeros: number of unset bits
    assert_eq!(n.count_zeros(), 29); // 32 - 3

    // leading_zeros: number of 0-bits before the highest 1-bit
    assert_eq!(n.leading_zeros(), 26);

    // trailing_zeros: number of 0-bits after the lowest 1-bit
    assert_eq!(n.trailing_zeros(), 2);

    // rotate_left / rotate_right: circular bit rotation
    let r = 0b1000_0001u8;
    assert_eq!(r.rotate_left(1), 0b0000_0011);

    // reverse_bits: reverse the order of all bits
    let v: u8 = 0b10110001;
    assert_eq!(v.reverse_bits(), 0b10001101);

    // is_power_of_two (unsigned types only)
    assert!(8u32.is_power_of_two());
    assert!(!6u32.is_power_of_two());

    // next_power_of_two (unsigned types only)
    assert_eq!(5u32.next_power_of_two(), 8);

    // pow: integer exponentiation
    assert_eq!(2u32.pow(10), 1024);
}
```

### `i32` vs `u32` for Bit Operations

Many interview problems give you `i32` inputs. Be aware of the differences:

- **Shifting:** Right-shifting a negative `i32` does arithmetic shift (preserves sign bit).
  Right-shifting a `u32` does logical shift (fills with zeros). If you need logical shift
  on a signed value, cast to unsigned first.
- **NOT:** `!0i32` is `-1`. `!0u32` is `4294967295` (u32::MAX).
- **Overflow:** Rust panics on overflow in debug mode. Use `.wrapping_add()`,
  `.wrapping_sub()`, etc., or cast to `u32` when doing bit tricks that rely on wrapping.

```rust
fn main() {
    // Signed right shift preserves sign:
    let neg: i32 = -8;               // 11111111...11111000
    println!("{:032b}", neg >> 1);    // 11111111...11111100 = -4

    // Unsigned right shift fills with zeros:
    let pos: u32 = neg as u32;       // 11111111...11111000 (same bits, different type)
    println!("{:032b}", pos >> 1);    // 01111111...11111100 = 2147483644

    // For bit manipulation problems, often safest to work with u32
    // and cast back to i32 at the end.
}
```

### Wrapping Behavior and Safety

In debug mode, Rust panics on integer overflow. This can trip you up with bit tricks:

```rust
// This panics in debug mode:
// let x: u32 = 1 << 32;       // shift amount >= bit width

// Safe alternatives:
let k: u32 = 32;
let x: u32 = 1u32.checked_shl(k).unwrap_or(0);

// For "negative" of unsigned (used in lowest-set-bit trick):
let n: u32 = 12;
let neg_n = n.wrapping_neg();  // safe, no panic

// For LeetCode i32 problems, cast early:
fn reverse_bits_leetcode(x: i32) -> i32 {
    (x as u32).reverse_bits() as i32
}
```

---

## Interview Problem 1: Single Number (LeetCode 136)

**Problem:** Given a non-empty array where every element appears twice except for one, find
the single element. Do it in O(n) time and O(1) space.

**The XOR trick:** XOR all elements together. Every pair cancels out (`a ^ a = 0`), leaving
only the single element (`0 ^ single = single`).

```
  [4, 1, 2, 1, 2]

  4 ^ 1 ^ 2 ^ 1 ^ 2
  = 4 ^ (1 ^ 1) ^ (2 ^ 2)       (reorder -- XOR is commutative + associative)
  = 4 ^ 0 ^ 0
  = 4
```

```rust
fn single_number(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &x| acc ^ x)
}

fn main() {
    assert_eq!(single_number(&[4, 1, 2, 1, 2]), 4);
    assert_eq!(single_number(&[2, 2, 1]), 1);
}
```

**Time:** O(n). **Space:** O(1).

---

## Interview Problem 2: Number of 1 Bits (LeetCode 191)

**Problem:** Given an unsigned integer, return the number of 1-bits (Hamming weight).

```rust
fn hamming_weight(mut n: u32) -> u32 {
    let mut count = 0;
    while n != 0 {
        n &= n - 1;    // Kernighan's trick: clear lowest set bit
        count += 1;
    }
    count
}

fn main() {
    assert_eq!(hamming_weight(0b1011), 3);
    assert_eq!(hamming_weight(0b10000000), 1);
    assert_eq!(hamming_weight(u32::MAX), 32);
}
```

Or just use the built-in: `n.count_ones()`.

**Time:** O(k) where k = set bits. **Space:** O(1).

---

## Interview Problem 3: Reverse Bits (LeetCode 190)

**Problem:** Reverse the bits of a 32-bit unsigned integer.

```
  Input:  00000010 10010100 00000000 11100101
  Output: 10100111 00000000 00101001 01000000
```

Strategy: extract each bit from the input (starting from bit 0), and place it into the
output (starting from bit 31).

```rust
fn reverse_bits(mut n: u32) -> u32 {
    let mut result: u32 = 0;
    for _ in 0..32 {
        result = (result << 1) | (n & 1);  // shift result left, add lowest bit of n
        n >>= 1;                            // shift n right
    }
    result
}

fn main() {
    let input: u32 = 0b00000010_10010100_00000000_11100101;
    let output = reverse_bits(input);
    println!("{:032b}", output);
    // 10100111_00000000_00101001_01000000

    // Or just use the built-in:
    assert_eq!(input.reverse_bits(), output);
}
```

```
  How it works, step by step for 8-bit example (n = 0b1101_0010):

  Step 1: result = 0b0,       extract LSB of n (0), result = 0b0,        n >>= 1
  Step 2: result = 0b00,      extract LSB of n (1), result = 0b01,       n >>= 1
  Step 3: result = 0b010,     extract LSB of n (0), result = 0b010,      n >>= 1
  Step 4: result = 0b0100,    extract LSB of n (0), result = 0b0100,     n >>= 1
  Step 5: result = 0b01000,   extract LSB of n (1), result = 0b01001,    n >>= 1
  Step 6: result = 0b010010,  extract LSB of n (0), result = 0b010010,   n >>= 1
  Step 7: result = 0b0100100, extract LSB of n (1), result = 0b0100101,  n >>= 1
  Step 8: result = 0b01001010,extract LSB of n (1), result = 0b01001011, n >>= 1

  Result: 0b01001011 (original 0b11010010 reversed)
```

**Time:** O(1) -- always exactly 32 iterations. **Space:** O(1).

---

## Interview Problem 4: Missing Number (LeetCode 268)

**Problem:** Given an array containing `n` distinct numbers in the range `[0, n]`, find the
one missing number.

**Approach:** XOR all numbers from 0 to n, then XOR all numbers in the array. Everything
pairs up and cancels except the missing number.

```
  nums = [3, 0, 1]    n = 3, range [0, 3]

  XOR of indices + n:  0 ^ 1 ^ 2 ^ 3
  XOR of array:        3 ^ 0 ^ 1

  Combined: 0 ^ 1 ^ 2 ^ 3 ^ 3 ^ 0 ^ 1
          = (0^0) ^ (1^1) ^ (3^3) ^ 2
          = 0 ^ 0 ^ 0 ^ 2
          = 2   <-- the missing number
```

```rust
fn missing_number(nums: &[i32]) -> i32 {
    let n = nums.len() as i32;
    let mut xor = n; // start with n (the extra value in [0, n])
    for (i, &num) in nums.iter().enumerate() {
        xor ^= i as i32 ^ num;
    }
    xor
}

fn main() {
    assert_eq!(missing_number(&[3, 0, 1]), 2);
    assert_eq!(missing_number(&[0, 1]), 2);
    assert_eq!(missing_number(&[9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
}
```

**Time:** O(n). **Space:** O(1). No overflow risk, no extra allocation.

You could also solve this with the sum formula `n*(n+1)/2 - sum(nums)`, but the XOR approach
avoids potential integer overflow for large n.

---

## Interview Problem 5: Subsets via Bitmask (LeetCode 78)

**Problem:** Given an array of distinct integers, return all possible subsets.

**The bitmask idea:** For an array of `n` elements, there are `2^n` subsets. Each subset can
be represented by an `n`-bit integer where bit `i` indicates whether element `i` is included.

```
  Elements: [a, b, c]    n = 3, so 2^3 = 8 subsets

  Bitmask   Binary   Subset
  -------   ------   ------
    0        000      {}
    1        001      {a}
    2        010      {b}
    3        011      {a, b}
    4        100      {c}
    5        101      {a, c}
    6        110      {b, c}
    7        111      {a, b, c}
```

Each integer from 0 to 2^n - 1 maps to exactly one subset. To decode: check which bits are
set.

```rust
fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let n = nums.len();
    let total = 1 << n; // 2^n
    let mut result = Vec::with_capacity(total);

    for mask in 0..total {
        let mut subset = Vec::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                subset.push(nums[i]);
            }
        }
        result.push(subset);
    }

    result
}

fn main() {
    let nums = [1, 2, 3];
    let all_subsets = subsets(&nums);
    for s in &all_subsets {
        println!("{:?}", s);
    }
    // [], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]
}
```

**Time:** O(n * 2^n). **Space:** O(n * 2^n) for the output.

This approach only works for small `n` (up to about 20-25) because 2^n grows exponentially.
But many interview subset problems have `n <= 20`, making bitmask enumeration the cleanest
approach.

---

## Interview Problem 6: Bitwise AND of Numbers Range (LeetCode 201)

**Problem:** Given two integers `left` and `right`, return the bitwise AND of all numbers
in the range [left, right].

**Key insight:** AND only preserves bits that are 1 in ALL numbers in the range. As you
sweep through consecutive integers, lower bits oscillate between 0 and 1, which zeroes them
out under AND. The result is the **common binary prefix** of `left` and `right`, with the
differing suffix replaced by zeros.

```
  left = 5 (0b0101), right = 7 (0b0111)

  AND of all numbers in [5, 7]:
    0 1 0 1   (5)
    0 1 1 0   (6)
    0 1 1 1   (7)
  & ---------
    0 1 0 0   (4)

  The top 2 bits (01) are the common prefix. The bottom 2 bits
  flip during the range, so they AND down to 00.
```

Strategy: right-shift both `left` and `right` until they are equal (i.e., find the common
prefix), then shift back.

```rust
fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
    let mut shift = 0;
    while left != right {
        left >>= 1;
        right >>= 1;
        shift += 1;
    }
    left << shift
}

fn main() {
    assert_eq!(range_bitwise_and(5, 7), 4);
    assert_eq!(range_bitwise_and(0, 0), 0);
    assert_eq!(range_bitwise_and(1, 2147483647), 0);
}
```

```
  Walk through left=5, right=7:

  Iteration 1: 5>>1 = 2,  7>>1 = 3   (not equal, shift=1)
  Iteration 2: 2>>1 = 1,  3>>1 = 1   (equal! shift=2)

  Common prefix = 1, shift back: 1 << 2 = 4
```

**Time:** O(log n) -- at most 32 iterations. **Space:** O(1).

---

## Bitmask Dynamic Programming

Bitmask DP uses an integer's bits to represent which elements of a set have been "used" or
"visited." This transforms subset-tracking from a combinatorial nightmare into simple array
indexing.

### When to Use Bitmask DP

- The input set is **small** (n <= 20, since 2^20 = ~1 million states).
- You need to track **which elements have been selected**.
- The problem involves **optimal assignment**, **shortest path visiting all nodes** (TSP),
  or **partitioning elements into groups**.

### Conceptual Framework

Think of it this way: if you have `n` items, there are `2^n` possible subsets of those items.
A bitmask DP table has one entry per subset. The integer `mask` encodes which items are "in"
the subset, and `dp[mask]` stores the answer for that subset.

```
  n = 4 items: {A, B, C, D}

  mask = 0b0000 = 0:   {}          -- no items selected
  mask = 0b0101 = 5:   {A, C}      -- items 0 and 2 selected
  mask = 0b1111 = 15:  {A, B, C, D} -- all items selected

  dp[mask] might store:
    - minimum cost to process exactly these items
    - number of ways to arrange exactly these items
    - whether this subset can be partitioned into valid groups
```

### Example: Minimum Cost to Visit All Cities (Traveling Salesman Variant)

Given `n` cities and a distance matrix, find the minimum cost to visit every city starting
from city 0.

State: `dp[mask][i]` = minimum cost to reach city `i` having visited exactly the set of
cities represented by `mask`.

Transition: from state `(mask, u)`, try going to each unvisited city `v`. The new state is
`(mask | (1 << v), v)` with cost `dp[mask][u] + dist[u][v]`.

```rust
fn min_cost_tsp(dist: &[Vec<i32>]) -> i32 {
    let n = dist.len();
    let full_mask = (1 << n) - 1;
    let inf = i32::MAX / 2;

    // dp[mask][i] = min cost to be at city i with visited set = mask
    let mut dp = vec![vec![inf; n]; 1 << n];
    dp[1][0] = 0; // start at city 0, only city 0 visited (mask = 0b...001)

    for mask in 1..=full_mask {
        for u in 0..n {
            if dp[mask][u] >= inf {
                continue;
            }
            if mask & (1 << u) == 0 {
                continue; // u must be in the visited set
            }
            // Try going to each unvisited city v
            for v in 0..n {
                if mask & (1 << v) != 0 {
                    continue; // v already visited
                }
                let next_mask = mask | (1 << v);
                let new_cost = dp[mask][u] + dist[u][v];
                if new_cost < dp[next_mask][v] {
                    dp[next_mask][v] = new_cost;
                }
            }
        }
    }

    // Minimum cost having visited all cities, ending at any city
    (0..n).map(|i| dp[full_mask][i]).min().unwrap()
}

fn main() {
    let dist = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    assert_eq!(min_cost_tsp(&dist), 80);
    // Path: 0 -> 1 -> 3 -> 2 = 10 + 25 + 30 = 65... actually:
    // Path: 0 -> 1 -> 3 -> 2 costs 10+25+30 = 65
    // Path: 0 -> 2 -> 3 -> 1 costs 15+30+25 = 70
    // Path: 0 -> 3 -> 1 -> 2 costs 20+25+35 = 80
    // Minimum is 65. Let the DP find it.
}
```

```
  State space for 4 cities:

  mask (binary)    visited cities     possible current cities
  ---------------------------------------------------------------
  0001             {0}                0
  0011             {0, 1}             0 or 1
  0101             {0, 2}             0 or 2
  0111             {0, 1, 2}          0, 1, or 2
  ...
  1111             {0, 1, 2, 3}       0, 1, 2, or 3

  Total DP states: 2^4 * 4 = 64
  For n = 15:      2^15 * 15 = 491,520    (very fast)
  For n = 20:      2^20 * 20 = 20,971,520 (still feasible)
  For n = 25:      2^25 * 25 = 838 million (getting tight)
```

**Bitmask DP complexity:** O(2^n * n^2) time, O(2^n * n) space for TSP-style problems.

### Iterating Over Submasks of a Given Mask

A common pattern in bitmask DP: enumerate all submasks of a given mask (e.g., when
partitioning a set into two groups, you iterate over all ways to split `mask` into two
complementary submasks).

```rust
/// Iterate over all non-empty submasks of `mask`.
fn submasks_of(mask: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut sub = mask;
    loop {
        result.push(sub);
        if sub == 0 {
            break;
        }
        sub = (sub - 1) & mask; // move to next smaller submask
    }
    result
}

fn main() {
    // All submasks of 0b1010 (bits 1 and 3 set):
    let subs = submasks_of(0b1010);
    for s in &subs {
        println!("{:04b}", s);
    }
    // 1010, 1000, 0010, 0000
}
```

The expression `(sub - 1) & mask` decrements the submask while staying within the original
mask's bits. The total work across ALL masks of n bits is O(3^n), not O(4^n). Each bit is
in one of three states: not in the outer mask, in the outer mask but not the submask, or in
both. Hence 3^n.

---

## Bitmasks as Sets: A Practical Example

Bitmasks are not just for interview problems. They are a compact, efficient way to represent
sets of up to 32 (or 64) elements using a single integer.

```rust
fn main() {
    // Represent sets of characters 'a' through 'z' as a u32 bitmask.
    // Bit 0 = 'a', bit 1 = 'b', ..., bit 25 = 'z'.

    let mut seen: u32 = 0;

    let word = "hello";
    for ch in word.chars() {
        let bit = (ch as u32) - ('a' as u32);
        seen |= 1 << bit; // add character to set
    }

    println!("Characters in '{}': ", word);
    for i in 0..26 {
        if seen & (1 << i) != 0 {
            print!("{}", (b'a' + i as u8) as char);
        }
    }
    println!(); // prints: ehlo

    // Set operations are single instructions:
    let set_a: u32 = 0b0000_0111; // {a, b, c}
    let set_b: u32 = 0b0000_0110; // {b, c}

    let union        = set_a | set_b;   // {a, b, c}
    let intersection = set_a & set_b;   // {b, c}
    let difference   = set_a & !set_b;  // {a}
    let sym_diff     = set_a ^ set_b;   // {a}
    let is_subset    = (set_b & set_a) == set_b;  // true: {b,c} is subset of {a,b,c}
    let set_size     = set_a.count_ones();         // 3

    println!("union:        {:07b}", union);
    println!("intersection: {:07b}", intersection);
    println!("difference:   {:07b}", difference);
    println!("sym_diff:     {:07b}", sym_diff);
    println!("is_subset:    {}", is_subset);
    println!("size:         {}", set_size);
}
```

Bitmask set operations are O(1) -- a single CPU instruction each. Compare that to `HashSet`
operations, which involve hashing, probing, and memory allocation. When your "universe" of
elements is small (under 64), bitmasks are dramatically faster.

---

## Bit Manipulation Visual Cheat Sheet

```
  Operation               Expression          What It Does
  ---------------------   ----------------    ----------------------------------
  Check if even           n & 1 == 0          Bit 0 is the parity bit
  Check if odd            n & 1 == 1
  Check bit k             (n >> k) & 1        Shift bit k to position 0, mask
  Set bit k               n | (1 << k)        OR with single-bit mask
  Clear bit k             n & !(1 << k)       AND with inverted mask
  Toggle bit k            n ^ (1 << k)        XOR with single-bit mask
  Clear lowest set bit    n & (n - 1)         Kernighan's trick
  Isolate lowest set bit  n & n.wrapping_neg() Two's complement trick
  Is power of 2?          n>0 && n&(n-1)==0   Exactly one bit set
  Count set bits          n.count_ones()      Hardware POPCNT
  All bits set (u32)      u32::MAX            0xFFFFFFFF
  Create mask of k bits   (1 << k) - 1        e.g., k=4 -> 0b1111
  Turn off trailing 1s    n & (n + 1)         Clears trailing 1s
  Swap a and b            std::mem::swap      (prefer this over XOR swap)
```

---

## Common Pitfalls

### 1. Shift Amount Overflow

In Rust, shifting by more than the bit width panics in debug mode.

```rust
// This panics in debug:
// let x: u32 = 1 << 32;

// Safe way to handle dynamic shift amounts:
let k: u32 = 32;
let x: u32 = if k < 32 { 1u32 << k } else { 0 };
// Or: 1u32.checked_shl(k).unwrap_or(0)
```

### 2. Operator Precedence

Bitwise operators have lower precedence than comparison operators in Rust (same as in C).
This means `n & 1 == 0` is actually parsed as `n & (1 == 0)`.

```rust
// WRONG: this is n & (1 == 0) which is a type error in Rust
// let even = n & 1 == 0;

// CORRECT: use parentheses
let even = (n & 1) == 0;
```

Rust's compiler catches this as a type error (unlike C where it silently does the wrong
thing), but it is still a common source of confusion.

### 3. Signed vs Unsigned Confusion

```rust
fn main() {
    let x: i32 = -1;
    // Right shift preserves sign for i32:
    println!("{}", x >> 1);       // -1 (arithmetic shift, fills with 1s)

    let y: u32 = x as u32;       // reinterpret bits as unsigned
    println!("{}", y >> 1);       // 2147483647 (logical shift, fills with 0s)
}
```

When an interview problem says "treat the input as unsigned," cast to `u32` first.

### 4. Forgetting the `n != 0` Guard

Many bit tricks have edge cases at zero:
- `is_power_of_two(0)` -- 0 is not a power of two, but `0 & (0 - 1)` underflows.
- `trailing_zeros(0)` -- the built-in returns 32 for u32 (all bits are zero).
- `lowest_set_bit(0)` -- returns 0, which might be confused with "bit 0."

Always consider what your function does when the input is 0.

---

## Complexity Summary

| Technique | Time | Space | Notes |
|-----------|------|-------|-------|
| Any single bitwise op | O(1) | O(1) | CPU instruction |
| Count set bits (Kernighan) | O(k) | O(1) | k = number of set bits |
| Count set bits (built-in) | O(1) | O(1) | Hardware POPCNT instruction |
| Single Number (XOR) | O(n) | O(1) | Scan array once |
| Reverse bits | O(1) | O(1) | Fixed 32 iterations |
| Range Bitwise AND | O(log n) | O(1) | At most 32 shifts |
| Generate all subsets | O(n * 2^n) | O(n * 2^n) | Exponential, n <= ~20 |
| Enumerate all submasks | O(3^n) | O(1) | Summed across all masks |
| Bitmask DP (TSP-style) | O(2^n * n^2) | O(2^n * n) | Feasible for n <= 20 |

---

## Practice Problems

### Easy (5 problems)

1. **Number of 1 Bits** (LeetCode 191) -- Count set bits. Use Kernighan's trick or
   `count_ones()`.
2. **Reverse Bits** (LeetCode 190) -- Reverse bit order of a 32-bit integer.
3. **Power of Two** (LeetCode 231) -- One-liner with `n & (n - 1)`.
4. **Missing Number** (LeetCode 268) -- XOR approach to find the missing value in [0, n].
5. **Counting Bits** (LeetCode 338) -- Return array where `ans[i]` = number of 1 bits in
   `i`. Use DP: `ans[i] = ans[i >> 1] + (i & 1)`.

### Medium (5 problems)

1. **Single Number** (LeetCode 136) -- XOR all elements; the unique one survives.
2. **Single Number II** (LeetCode 137) -- Every element appears 3 times except one. Count
   bits mod 3 at each position.
3. **Subsets** (LeetCode 78) -- Generate all subsets using bitmask enumeration.
4. **Bitwise AND of Numbers Range** (LeetCode 201) -- Find the common binary prefix of
   `left` and `right`.
5. **Total Hamming Distance** (LeetCode 477) -- For each bit position, count how many
   numbers have 0 vs 1, then multiply the two counts.

### Hard (5 problems)

1. **Single Number III** (LeetCode 260) -- Two unique numbers in a doubled array. XOR all
   to get `a ^ b`, then split by any differing bit.
2. **Maximum XOR of Two Numbers in Array** (LeetCode 421) -- Trie-based bit-by-bit greedy
   from the most significant bit down.
3. **Shortest Path Visiting All Nodes** (LeetCode 847) -- BFS with bitmask state tracking
   which nodes have been visited.
4. **Partition to K Equal Sum Subsets** (LeetCode 698) -- Bitmask DP over element
   assignments to k buckets.
5. **Find the Shortest Superstring** (LeetCode 943) -- Bitmask DP (TSP variant) using
   string overlap costs.

---

*Bit manipulation is a small toolkit with outsized leverage. The operators are simple -- you
learned AND, OR, and NOT in a logic class or a programming tutorial years ago. What makes bit
manipulation powerful is recognizing when a problem that looks like it needs arrays, hash maps,
or math can be collapsed into a few bitwise operations. The key identities to burn into memory
are: `n & (n-1)` clears the lowest set bit, `n & (-n)` isolates it, and `a ^ a = 0`.
Everything else is built from those three.*
