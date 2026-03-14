# Visualization Style Guide

Rules for writing visualization functions in `src/visualizer/problem_viz.rs`.

## Layout

```
┌─ Problem Name ─────────────────── ▶ Step 5/20  500ms ─┐
│  ━━━━━━━━━━━━━━━━━━━━─────────────────────────────     │
│                                                         │
│   ███  ██  ████  ███  ████ ██  ███      (bars)         │
│   ███  ██  ████  ███  ████ ██  ███                     │
│    7    2   11    4    8    3    5       (values)       │
│    ▲                   ▲                (arrows)       │
│   scan               match             (labels)       │
│                                                         │
│  nums[0]=7, need 2 — found in map at index 3           │
│                                                         │
│  cmp 4  swp 0      █ cmp █ swp █ done █ ptr █ pivot   │
│  [←/→] step [A] play [+/-] speed [Esc] back            │
└─────────────────────────────────────────────────────────┘
```

## Pointer Naming

Every frame with active algorithm state MUST use named pointers via `v.ptrs()`.

### Conventions

| Pattern | Pointer Names | Example |
|---------|---------------|---------|
| Linear scan | `i` or `scan` | Two Sum, Contains Duplicate |
| Two pointers (same dir) | `slow`, `fast` | Remove Duplicates |
| Two pointers (inward) | `L`, `R` | Trapping Rain Water, Two Sum II |
| Sliding window | `L`, `R` | Max Subarray, Min Window |
| Read/write | `read`, `write` | Remove Duplicates Sorted |
| Binary search | `lo`, `mid`, `hi` | Binary Search |
| Sorting inner/outer | `i`, `j` | Bubble Sort, Selection Sort |
| Pivot-based | `pivot`, `i`, `j` | Quick Sort |
| Key insertion | `key`, `j` | Insertion Sort |
| Parent/child (heap) | `parent`, `child` | Heap Sort |
| Merge | `L`, `R`, `out` | Merge Sort merge step |
| Buy/sell | `buy`, `sell` | Best Time to Buy/Sell Stock |
| Min/max tracking | `min`, `max` or `best` | Various |
| Hash lookup | `scan`, `match` | Two Sum, Duplicates |
| Stack-based | `i`, `top` | Monotonic stack problems |

**Rules:**
- Labels are 1-5 characters, always lowercase except `L`/`R`
- Same pointer name = same color throughout the visualization
- Every highlighted index should have a pointer label unless it's a "done" (Sorted) highlight

## Highlight Semantics

| HighlightKind | Meaning | Color | When to use |
|---------------|---------|-------|-------------|
| `Active` | Currently examining / scanning | Cyan | Default for "looking at this element" |
| `Comparing` | Two elements being compared | Yellow | Explicit comparisons (>, <, ==) |
| `Swapping` | Two elements being swapped | Red | Swap operations |
| `Pivot` | Pivot / anchor / reference point | Magenta | Partition pivots, min trackers, anchors |
| `Found` | Target found / answer element | Green | Final answer discovery |
| `Sorted` | In final position / processed | Green | Element is done, won't be touched again |

**Rules:**
- Comparisons always highlight EXACTLY 2 elements
- Swaps always highlight EXACTLY 2 elements
- `Sorted` is permanent — once marked, keep it in subsequent frames
- `Found` is for the "aha" moment — use sparingly (1-2 frames)
- Don't highlight more than 3-4 indices per frame (visual noise)

## Annotations

The annotation is a single line explaining what's happening.

**Format:** `{action}: {detail}`

**Examples:**
- `"Compare [2]=5 vs [4]=3 — swap needed"`
- `"nums[0]=7, need 2 in map"`
- `"Pivot=5 at [3], partition [0..6]"`
- `"Window [2..5] sum=12 > target, shrink left"`
- `"Merge: take 3 from left half"`

**Rules:**
- Lead with the action verb (Compare, Swap, Check, Scan, Found, Skip, Merge, Split, Insert)
- Include concrete values, not just indices: `"[2]=5"` not `"[2]"`
- Keep under 80 characters
- Final frame annotation summarizes the result: `"Result: [1, 3] — sum = 9"`

## Input Generation

Every visualization generates random inputs using `rand::rng()`.

**Rules:**
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

## Animation

- Auto-play starts at 500ms per frame
- User can adjust 50ms-2000ms with +/-
- First frame should set context (problem name, parameters, initial state)
- Last frame should show the answer clearly with `Sorted`/`Found` highlights

## Structure of a viz function

```rust
fn viz_problem_name() -> Vec<VizFrame> {
    let mut rng = rand::rng();
    // 1. Generate random input (guarantee solvability)
    let n = rng.random_range(6..=8);
    let nums = rand_arr(&mut rng, n, 1, 15);

    // 2. Create VizLog
    let mut v = VizLog::new(nums.clone());

    // 3. First frame: context
    v.ptrs(&[], &[], format!("Problem Name — target={}", target));

    // 4. Algorithm steps with ptrs()
    for (i, &num) in nums.iter().enumerate() {
        v.ptrs(
            &[(i, HighlightKind::Active)],
            &[(i, "scan")],
            format!("Check [{}]={}", i, num),
        );
    }

    // 5. Final frame: result
    v.ptrs(
        &[(a, HighlightKind::Found), (b, HighlightKind::Found)],
        &[(a, "ans"), (b, "ans")],
        format!("Result: {:?}", answer),
    );

    v.into_frames()
}
```

## Adding a new topic

1. Write `viz_xxx()` functions for all 15 problems in the topic
2. Add entries to `get_problem_viz()` match arm
3. Use `v.ptrs()` for every frame (not `v.step()`)
4. Run `cargo clippy -- -D warnings` — zero warnings
5. Test: `cargo run`, navigate to any problem in the topic, press `[V]`
