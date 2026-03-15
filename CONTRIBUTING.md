# Contributing

Thanks for your interest in contributing to **I Understand It Now**!

## Getting Started

```bash
git clone https://github.com/asif-a-khan/i_understand_it_now.git
cd i_understand_it_now
cargo check
cargo run
```

## Ways to Contribute

- **Add practice problems** — each topic has 15 problems (5 easy, 5 medium, 5 hard). New problems go in `src/problems/`.
- **Add lesson content** — improve or expand the markdown lessons in `lessons/`.
- **Add visualizations** — write viz functions for problems. See [STYLE_GUIDE_VIZ.md](STYLE_GUIDE_VIZ.md) for the full specification.
- **Improve the TUI** — bug fixes, UX improvements, accessibility.
- **Fix bugs** — check the issue tracker.

## Development Workflow

1. Fork the repo and create a branch from `main`
2. Make your changes
3. Run `cargo check`, `cargo clippy -- -D warnings`, and `cargo fmt --check` — ensure zero errors, zero warnings, clean formatting
4. Open a pull request with a clear description of what you changed and why

## Code Style

- Follow standard Rust conventions (`rustfmt` defaults)
- Keep solutions in `src/solutions/` as stubs (`todo!()`) — these are for the user to fill in
- Problems should generate 10 randomized test cases via `rand::rng()`
- Lessons should include real-world analogies and ASCII-art visuals where helpful

## Visualization Guidelines

When writing visualization functions, follow [STYLE_GUIDE_VIZ.md](STYLE_GUIDE_VIZ.md). Key rules:

- **Opening annotation** must start with `"Goal:"` and include strategy + complexity
- **Pointer labels** must be semantic — use the naming table in the style guide (e.g., `"scan"` not `"i"` for hash-scan, `"node"` for tree traversal)
- **Highlights** must use the correct `HighlightKind` for the operation — all 9 kinds have distinct colors
- **Frame count** should be 15-40 (8 minimum, 60 maximum)
- **VizData type** must match the data structure — `into_tree_frames()` for trees, `into_grid_frames()` for grids
- Use `v.ptrs()` for every frame (not `v.step()`)

Quick checklist before submitting a viz PR:
```
[ ] Opening frame: "Goal: ... Strategy: ... O(...)."
[ ] Pointer labels: semantic names per style guide table
[ ] Highlights: correct HighlightKind, <=4 per frame
[ ] Final frame: "Result: ..." with Found/Sorted highlights
[ ] Frame count: 8-60 range
[ ] cargo clippy -- -D warnings: zero warnings
[ ] cargo fmt --check: clean
```

## Project Layout

```
src/
├── main.rs             # CLI dispatch
├── cli.rs              # CLI commands
├── tui/                # interactive terminal app
│   ├── viz_player.rs   # visualization renderer (5 renderer types)
│   ├── keybindings.rs  # help overlay with color legend
│   └── theme.rs        # color constants
├── visualizer/         # algorithm animations
│   ├── problem_viz.rs  # 636 per-problem visualizations
│   ├── reference.rs    # canonical algorithm demos
│   ├── instrumented.rs # replay from Tracked<T> operation logs
│   └── engine.rs       # CLI-mode renderer
├── tracker/            # Tracked<T> operation recording
├── problems/           # problem definitions + test generators
├── solutions/          # user-editable solution stubs
├── progress.rs         # local progress tracking
└── complexity.rs       # empirical Big-O estimation
```
