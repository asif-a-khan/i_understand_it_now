# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0-alpha] - 2026-03-15

First alpha release of **I Understand It Now** (iuin) -- an interactive terminal app for learning data structures and algorithms by doing.

### Content

- **38 lessons** covering 6 parts of the DSA curriculum, from Big-O notation through advanced topics like segment trees, KMP, and monotonic stacks
- **573 practice problems** (191 easy, 191 medium, 191 hard) across 38 topics with randomized test inputs
- **636 algorithm visualizations** -- every problem has a step-by-step animated visualization showing how the canonical algorithm works

### Curriculum

| Part | Topics | Problems |
|------|--------|----------|
| 1 -- Foundations | Big-O, Arrays, Strings, Linked Lists, Stacks & Queues, Hash Maps, Recursion | 105 |
| 2 -- Sorting & Searching | Binary Search, Bubble/Selection/Insertion Sort, Merge Sort, Quick Sort, Heap Sort, Counting/Radix Sort, Two Pointers, Prefix Sum | 120 |
| 3 -- Trees | Binary Trees, BSTs, Heaps & Priority Queues, Balanced BSTs, Tries | 75 |
| 4 -- Graphs | Graph Representations, BFS/DFS, Grid Problems, Topological Sort, Shortest Path, MST, Union-Find | 105 |
| 5 -- Paradigms | Backtracking, Greedy, Dynamic Programming, Divide & Conquer, Intervals | ~93 |
| 6 -- Advanced | Segment/Fenwick Trees, Sparse Tables, Monotonic Stacks, Bit Manipulation, String Algorithms, Math & Geometry | ~75 |

### TUI Features

- **Lesson browser** -- read markdown lessons with syntax highlighting directly in the terminal
- **Problem runner** -- test solutions against randomized inputs, view pass/fail results
- **Visualization player** -- watch algorithm animations with play/pause, step forward/back, speed control
- **In-TUI code editor** -- edit solutions without leaving the app (or use `$EDITOR`)
- **Progress tracking** -- local `.iuin/progress.toml` tracks lessons read and problems solved
- **Dashboard** -- overview of completion progress across all topics
- **Empirical complexity analysis** -- run solutions at multiple input sizes and estimate Big-O

### Visualization System

- **5 renderer types**: Array (rounded boxes), Tree (spatial + indented), Graph (grid + adjacency sidebar), Grid (2D cells with walls/start/end), None (theory)
- **9 distinct highlight colors**: Comparing (yellow), Swapping (red), Sorted (green), Active (cyan), Pivot (light magenta), Found (light green), Reading (light cyan), Writing (magenta), Target (red + underline)
- **Semantic pointer labels**: 722 context-specific labels (`scan`, `fill`, `node`, `pick`, `lo`/`mid`/`hi`, `text`/`pat`, etc.) instead of generic `i`/`j`
- **Pointer labels above cells**: labels and arrows appear above the data, not below
- **Separate metrics and legend lines** for readability
- **Instrumented replay**: solutions using `Tracked<T>` automatically record operations and play back the user's execution step-by-step
- **7 reference visualizations**: Bubble Sort, Selection Sort, Insertion Sort, Merge Sort, Quick Sort, Heap Sort, Binary Search

### CLI Mode

- `cargo run` -- launch full TUI
- `cargo run -- solve <problem>` -- test a specific solution
- `cargo run -- viz <algorithm>` -- watch a reference visualization in the terminal

### Developer

- Zero clippy warnings enforced
- CI pipeline: check, clippy, fmt, test (GitHub Actions)
- Visualization style guide (`STYLE_GUIDE_VIZ.md`)
- Contributing guidelines (`CONTRIBUTING.md`)

### Known Limitations

- Trie visualizations render character paths as flat arrays rather than multi-way tree structures (the tree renderer only supports binary trees)
- No Windows terminal tested yet (crossterm should work but untested)
- In-TUI editor is basic -- complex editing is better done in `$EDITOR`
- No network features -- everything runs locally
