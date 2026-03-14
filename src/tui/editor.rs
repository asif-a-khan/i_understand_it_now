use std::io;
use std::path::PathBuf;
use std::process::Command;

use crossterm::terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

/// Map a problem's topic to its solution file path.
pub fn solution_file_path(topic: &str) -> PathBuf {
    // Determine which part directory contains this topic
    let (part_dir, file_stem) = match topic {
        "big_o" => ("part1_foundations", "big_o"),
        "arrays" => ("part1_foundations", "arrays"),
        "strings" => ("part1_foundations", "strings"),
        "linked_lists" => ("part1_foundations", "linked_lists"),
        "stacks_queues" => ("part1_foundations", "stacks_queues"),
        "hash_maps" => ("part1_foundations", "hash_maps"),
        "recursion" => ("part1_foundations", "recursion"),
        "binary_search" => ("part2_sorting", "binary_search"),
        "basic_sorts" => ("part2_sorting", "basic_sorts"),
        "merge_sort" => ("part2_sorting", "merge_sort"),
        "quick_sort" => ("part2_sorting", "quick_sort"),
        "heap_sort" => ("part2_sorting", "heap_sort"),
        "counting_radix" => ("part2_sorting", "counting_radix"),
        "two_pointers" => ("part2_sorting", "two_pointers"),
        "prefix_sum" => ("part2_sorting", "prefix_sum"),
        "binary_trees" => ("part3_trees", "binary_trees"),
        "bst" => ("part3_trees", "bst"),
        "heaps_priority_queues" => ("part3_trees", "heaps_priority_queues"),
        "balanced_bst" => ("part3_trees", "balanced_bst"),
        "tries" => ("part3_trees", "tries"),
        "graph_representations" => ("part4_graphs", "graph_representations"),
        "graph_bfs_dfs" => ("part4_graphs", "graph_bfs_dfs"),
        "matrix_grid" => ("part4_graphs", "matrix_grid"),
        "topological_sort" => ("part4_graphs", "topological_sort"),
        "shortest_path" => ("part4_graphs", "shortest_path"),
        "mst" => ("part4_graphs", "mst"),
        "union_find" => ("part4_graphs", "union_find"),
        "backtracking" => ("part5_paradigms", "backtracking"),
        "greedy" => ("part5_paradigms", "greedy"),
        "dynamic_programming" => ("part5_paradigms", "dynamic_programming"),
        "divide_conquer" => ("part5_paradigms", "divide_conquer"),
        "intervals" => ("part5_paradigms", "intervals"),
        "segment_fenwick" => ("part6_advanced", "segment_fenwick"),
        "sparse_tables" => ("part6_advanced", "sparse_tables"),
        "monotonic" => ("part6_advanced", "monotonic"),
        "bit_manipulation" => ("part6_advanced", "bit_manipulation"),
        "string_algorithms" => ("part6_advanced", "string_algorithms"),
        "math_geometry" => ("part6_advanced", "math_geometry"),
        _ => ("part1_foundations", topic),
    };

    PathBuf::from("src/solutions")
        .join(part_dir)
        .join(format!("{}.rs", file_stem))
}

/// Suspend the TUI, launch $EDITOR on the given file, then resume the TUI.
pub fn launch_external_editor(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    path: &str,
) -> io::Result<()> {
    // Restore terminal to normal state
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), terminal::LeaveAlternateScreen)?;

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let status = Command::new(&editor).arg(path).status();

    // Re-enter TUI mode regardless of editor result
    crossterm::execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    terminal.clear()?;

    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Editor exited with status: {}", s),
        )),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to launch editor '{}': {}", editor, e),
        )),
    }
}
