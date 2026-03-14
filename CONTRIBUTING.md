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
- **Add visualizations** — implement `ReferenceViz` for algorithms that don't have one yet. See `src/visualizer/reference.rs` for examples.
- **Improve the TUI** — bug fixes, UX improvements, accessibility.
- **Fix bugs** — check the issue tracker.

## Development Workflow

1. Fork the repo and create a branch from `main`
2. Make your changes
3. Run `cargo check` and `cargo clippy` — ensure zero errors and zero warnings
4. Open a pull request with a clear description of what you changed and why

## Code Style

- Follow standard Rust conventions (`rustfmt` defaults)
- Keep solutions in `src/solutions/` as stubs (`todo!()`) — these are for the user to fill in
- Problems should generate 10 randomized test cases via `rand::rng()`
- Lessons should include real-world analogies and ASCII-art visuals where helpful

## Project Layout

```
src/
├── main.rs             # CLI dispatch
├── cli.rs              # CLI commands
├── tui/                # interactive terminal app
├── visualizer/         # algorithm animations
├── tracker/            # Tracked<T> operation recording
├── problems/           # problem definitions + test generators
├── solutions/          # user-editable solution stubs
├── progress.rs         # local progress tracking
└── complexity.rs       # empirical Big-O estimation
```
