# Visualization Style Guide

Rules for writing visualization functions in `src/visualizer/problem_viz.rs`.

## Layout

```
┌─ Annotation (centered, bold) ─────────────────────────────┐
│  ━━━━━━━━━━━━━━━━━━━━─────── ⏸ 5/20 500ms ──────────     │
│                                                            │
│   scan               match              (labels above)    │
│    ▼                   ▼                 (arrows down)    │
│    0    1    2    3    4    5    6        (indices)        │
│   ╭──╮ ╭──╮ ╭──╮ ╭──╮ ╭──╮ ╭──╮ ╭──╮                    │
│   │ 7│ │ 2│ │11│ │ 4│ │ 8│ │ 3│ │ 5│   (values)         │
│   ╰──╯ ╰──╯ ╰──╯ ╰──╯ ╰──╯ ╰──╯ ╰──╯                    │
│    ✓                   ★                 (status below)   │
│                                                            │
│  cmp 4  swp 0  read 2  write 0          (metrics line)   │
│  █ cmp █ swp █ done █ active █ read     (legend line)    │
│  [←/→] step [Space] play/pause [?] help [Esc] back       │
└────────────────────────────────────────────────────────────┘
```

Key layout details:
- **Pointer labels above arrows**: labels → ▼ → index → cells → status symbols
- **Metrics and legend on separate lines** (not combined)
- **Space = play/pause**, `→`/`Enter` = step forward, `a` also toggles play

## Pointer Naming

Every frame with active algorithm state MUST use named pointers via `v.ptrs()`.

### Conventions by Category

| Category | Pattern | Pointer Names | Example |
|----------|---------|---------------|---------|
| Arrays (hash) | Hash scan + match | `scan`, `match` | Two Sum, Contains Duplicate |
| Arrays (two-ptr) | Two pointers inward | `L`, `R` | Trapping Rain Water |
| Strings | Linear scan | `scan` | First Unique Char |
| Strings | Two pointers | `L`, `R` | Valid Palindrome |
| Linked Lists | Traversal | `curr`, `prev`, `next` | Reverse Linked List |
| Linked Lists | Fast/slow | `slow`, `fast` | Has Cycle |
| Stacks | Scan + stack top | `scan`, `top` | Valid Parentheses |
| Stacks | Popped element | `pop` | Next Greater Element |
| Hash Maps | Hash scan + match | `scan`, `match` | Contains Duplicate |
| Recursion | Table filling | `fill` | Fibonacci |
| Recursion | Depth tracking | `depth` | Max Depth Tree |
| Recursion | Choice/pick | `pick` | Permutations |
| Binary Search | Halving | `lo`, `mid`, `hi` | Binary Search |
| Basic Sorts | Inner/outer loop | `i`, `j` | Bubble Sort (canonical) |
| Quick Sort | Partition | `i`, `j`, `pivot` | Quick Sort (canonical) |
| Merge Sort | Merge step | `L`, `R`, `out` | Merge Two Sorted |
| Heap Sort | Heap operations | `parent`, `child` | Heap Sort |
| Counting/Radix | Input → output | `scan`, `out` | Counting Sort |
| Two Pointers | Converging | `L`, `R` | Two Sum Sorted |
| Prefix Sum | Build / query | `fill`, `scan` | Range Sum Query |
| Binary Trees | Node traversal | `node`, `root` | Max Depth |
| BST | Search | `node`, `cmp` | BST Search |
| Heaps (PQ) | Heap node | `node` | Kth Largest |
| Tries | Trie node | `node` | Implement Trie |
| Graphs | Node + neighbor | `node`, `nb` | BFS/DFS |
| Graphs | Source + dest | `src`, `dst` | Dijkstra |
| Grid/Matrix | Current cell | `cell` | Flood Fill |
| Backtracking | Current choice | `pick` | Subsets |
| Greedy | Scan + best | `scan`, `best` | Jump Game |
| DP | Table filling | `fill`, `prev` | Climbing Stairs |
| Divide & Conquer | Range bounds | `lo`, `hi`, `mid` | Max Subarray |
| Intervals | Interval scan | `scan`, `curr` | Merge Intervals |
| Segment/Fenwick | Tree node + range | `node`, `L`, `R` | Range Sum Query |
| Sparse Table | Build + query | `fill`, `L`, `R` | Range Min Query |
| Monotonic Stack | Scan + pop | `scan`, `pop` | Next Greater |
| Bit Manipulation | Bit / number | `bit`, `num` | Single Number |
| String Algorithms | Text + pattern | `text`, `pat` | KMP |
| Math/Geometry | Number + step | `num`, `step` | GCD |

### Rules

- Labels are 1-5 characters, lowercase except `L`/`R`
- Same pointer name = same color throughout the visualization
- Every highlighted index should have a pointer label unless it's a "done" (Sorted) highlight
- `"i"` and `"j"` are ONLY used in basic_sorts and quick_sort (canonical loop variables)

## Highlight Semantics

| HighlightKind | Meaning | TUI Color (bg) | CLI Color (fg) | When to use |
|---------------|---------|-----------------|-----------------|-------------|
| `Active` | Currently examining | Cyan | DarkCyan | Default for "looking at this" |
| `Comparing` | Two elements compared | Yellow | Yellow | Explicit comparisons (>, <, ==) |
| `Swapping` | Two elements swapped | Red | Red | Swap operations |
| `Pivot` | Anchor / reference | LightMagenta | Magenta | Partition pivots, anchors |
| `Found` | Answer discovered | LightGreen | Green | "Aha" moment (1-2 frames) |
| `Sorted` | In final position | Green | DarkGreen | Done, won't be touched again |
| `Reading` | Element being read | LightCyan | Cyan | Read operations |
| `Writing` | Element being written | Magenta | DarkMagenta | Write operations |
| `Target` | Search target | Red+Underline | Red | Target element, visible throughout |

### Rules

- Each color is visually distinct — no two kinds share the same color
- Comparisons always highlight EXACTLY 2 elements
- Swaps always highlight EXACTLY 2 elements
- `Sorted` is permanent — once marked, keep it in subsequent frames
- `Found` is for the "aha" moment — use sparingly (1-2 frames)
- `Target` should persist across ALL frames so the user always knows what they're looking for
- Don't highlight more than 3-4 indices per frame (visual noise)

## Annotations

The annotation appears centered at the top of the visualization.

### Opening Frame

**Must start with `"Goal:"`** and include the strategy:

```
"Goal: find two numbers that add up to 9. Strategy: hash map stores seen values, check complement each step. O(n)."
```

### Step Annotations

**Format:** `{action}: {detail with concrete values}`

Examples:
- `"Compare [2]=5 vs [4]=3 — swap needed"`
- `"Scan [0]=7, need 2 in map — not found, store 7"`
- `"Pivot=5 at [3], partition [0..6]"`
- `"Window [2..5] sum=12 > target, shrink left"`

### Final Frame

Summarize the result:
- `"Result: [1, 3] — indices whose values sum to 9. O(n) time, O(n) space."`

### Rules

- Lead with action verb (Compare, Swap, Check, Scan, Found, Skip, Merge, Split, Insert)
- Include concrete values, not just indices: `"[2]=5"` not `"[2]"`
- Keep under 80 characters
- Opening = `"Goal: ..."`, Final = `"Result: ..."`

## Input Generation

Every visualization generates random inputs using `rand::rng()`.

- Array size: 6-10 elements (enough to show the algorithm, small enough to read)
- Values: small positive integers (1-20) for most problems
- Guarantee the problem is solvable (e.g., Two Sum must have a valid pair)
- Edge cases are for tests, not visualizations — show the "normal" case
- Use `rand_arr()` for random arrays, `rand_unique()` for distinct values

## Frame Count

- Target: 15-40 frames per visualization
- Minimum: 8 frames (too few = not educational)
- Maximum: 60 frames (too many = tedious even on auto-play)
- Skip redundant frames (don't show every "still scanning..." if nothing changes)
- Always include: initial state (frame 1), key decision points, final result

## VizData Types

Choose the correct renderer for the data structure:

| Data | VizData | Conversion | Use For |
|------|---------|------------|---------|
| Flat array | `Array` | `v.into_frames()` | Sorting, arrays, stacks, hash maps |
| Characters | `Array` | `v.into_labeled_frames(labels)` | Strings, brackets |
| Linked list | `Array` | `v.into_labeled_frames(labels)` | Linked lists (with `→` arrows) |
| Binary tree | `Tree` | `v.into_tree_frames()` | Trees, BSTs, heaps, balanced BSTs |
| 2D grid | `Grid` | `v.into_grid_frames(rows, cols)` | Grids, graphs, matrices |
| Graph | `Graph` | Construct `VizData::Graph` directly | Graph algorithms |
| None | `None` | N/A | Theory / scalar problems |

## Animation

- Auto-play starts at 500ms per frame
- User can adjust 50ms-2000ms with `+`/`-`
- Space toggles play/pause (media player convention)
- `→`/`Enter` = step forward, `←` = step backward
- `Home`/`End` = jump to first/last frame
- `?` = toggle help overlay with keybindings + color legend

## Structure of a viz function

```rust
fn viz_problem_name() -> Vec<VizFrame> {
    let mut rng = rand::rng();
    // 1. Generate random input (guarantee solvability)
    let n = rng.random_range(6..=8);
    let nums = rand_arr(&mut rng, n, 1, 15);

    // 2. Create VizLog
    let mut v = VizLog::new(nums.clone());

    // 3. First frame: Goal + Strategy
    v.ptrs(&[], &[], format!(
        "Goal: find two numbers that sum to {}. Strategy: hash map. O(n).", target
    ));

    // 4. Algorithm steps with ptrs()
    for (i, &num) in nums.iter().enumerate() {
        v.ptrs(
            &[(i, HighlightKind::Active)],
            &[(i, "scan")],
            format!("Scan [{}]={}, need {} in map", i, num, target - num),
        );
    }

    // 5. Final frame: result
    v.ptrs(
        &[(a, HighlightKind::Found), (b, HighlightKind::Found)],
        &[(a, "ans"), (b, "ans")],
        format!("Result: [{}, {}] — sum = {}. O(n) time.", a, b, target),
    );

    v.into_frames()
}
```

## Adding a new topic

1. Write `viz_xxx()` functions for all 15 problems in the topic
2. Add entries to `get_problem_viz()` match arm
3. Use `v.ptrs()` for every frame (not `v.step()`)
4. Follow the pointer naming conventions for your category (see table above)
5. Opening annotation: `"Goal: ... Strategy: ... O(...)"`
6. Run `cargo clippy -- -D warnings` — zero warnings
7. Run `cargo fmt` — clean formatting
8. Test: `cargo run`, navigate to any problem in the topic, press `[V]`
