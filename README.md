# I Understand It Now

An interactive terminal application for learning data structures and algorithms in Rust. Features step-by-step algorithm visualizations, 570 practice problems with randomized test inputs, and a full TUI with an integrated code editor.

## Features

- **38 lessons** covering Big-O through advanced topics (segment trees, string algorithms, graph theory)
- **570 practice problems** — 15 per topic (5 easy, 5 medium, 5 hard) with fresh randomized inputs every run
- **Algorithm visualizations** — step-by-step and auto-play animated bar charts for 7 sorting/searching algorithms
- **Instrumented replay** — watch your own solution's comparisons and swaps animated after running tests
- **Empirical complexity measurement** — automatically estimates your solution's Big-O by running it at multiple input sizes
- **Operation tracking** — transparent `Tracked<T>` wrapper counts comparisons, swaps, reads, and writes with zero extra code from you
- **Full TUI** — browse lessons, run problems, watch visualizations, and edit solutions without leaving the terminal
- **In-TUI code editor** — basic editor with syntax coloring, or launch your `$EDITOR`
- **Progress tracking** — checkmarks, completion bars, and best operation counts saved locally

## Curriculum

| Part | Topics | Problems |
|------|--------|----------|
| 1. Foundations | Big-O, Arrays, Strings, Linked Lists, Stacks/Queues, Hash Maps, Recursion | 105 |
| 2. Sorting & Searching | Binary Search, Bubble/Selection/Insertion Sort, Merge Sort, Quick Sort, Heap Sort, Counting/Radix Sort, Two Pointers, Prefix Sum | 120 |
| 3. Trees | Binary Trees, BSTs, Heaps, Balanced BSTs, Tries | 75 |
| 4. Graphs | Representations, BFS/DFS, Grid Problems, Topological Sort, Shortest Path, MST, Union-Find | 105 |
| 5. Algorithm Paradigms | Backtracking, Greedy, Dynamic Programming, Divide & Conquer, Intervals | 75 |
| 6. Advanced | Segment/Fenwick Trees, Sparse Tables, Monotonic Stacks, Bit Manipulation, String Algorithms, Math & Geometry | 90 |

## Install

```bash
git clone https://github.com/asif-a-khan/i_understand_it_now.git
cd i_understand_it_now
cargo run
```

## Usage

### TUI (default)

```bash
cargo run
```

Launches the interactive terminal app. Navigate with the keyboard:

- **Dashboard** — `L` lessons, `P` problems, `V` visualizations, `?` help
- **Lesson browser** — `j/k` navigate, `Enter` open, `m` mark read
- **Problem runner** — `r` run tests, `e` open in $EDITOR, `i` in-TUI editor
- **Viz player** — `Left/Right` step, `a` auto-play, `+/-` speed
- **Results** — `w` watch replay, `c` measure complexity

### CLI

```bash
cargo run -- solve arrays_two_sum          # test a solution
cargo run -- solve merge_sort --complexity # with Big-O estimation
cargo run -- viz bubble_sort               # watch algorithm animation
cargo run -- viz heap_sort --mode auto     # auto-play mode
cargo run -- list                          # browse all problems
cargo run -- list --topic graphs           # filter by topic
cargo run -- status                        # view progress
```

## How It Works

1. Read a lesson in the TUI or open the markdown directly from `lessons/`
2. Watch the reference visualization for the algorithm
3. Edit your solution in `src/solutions/` (via $EDITOR, the in-TUI editor, or your preferred editor)
4. Run tests — see pass/fail results with operation counts
5. Watch the instrumented replay of your solution's execution
6. Check empirical complexity to see if you hit the target Big-O

### Tracked\<T\>

Solutions use `Tracked<T>` wrappers instead of raw types. You write normal Rust — operators like `<`, `>`, `==` work as expected — while every comparison, swap, read, and write is automatically recorded. This powers both the replay visualizations and the metrics display.

```rust
pub fn bubble_sort(arr: &mut [Tracked<i32>]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {       // comparison recorded
                tracked_swap(arr, j, j + 1); // swap recorded
            }
        }
    }
}
```

## Project Structure

```
iuin/
├── lessons/           # 38 markdown lecture notes
├── src/
│   ├── main.rs        # entry point (TUI or CLI)
│   ├── cli.rs         # CLI commands
│   ├── tui/           # full interactive terminal app
│   ├── visualizer/    # algorithm animations + instrumented replay
│   ├── tracker/       # Tracked<T> wrapper + operation recording
│   ├── problems/      # 570 problem definitions + test generators
│   ├── solutions/     # user-editable solution files
│   ├── progress.rs    # local progress tracking
│   └── complexity.rs  # empirical Big-O estimation
└── .iuin/             # local progress file (git-ignored)
```

## License

[MIT](LICENSE)
