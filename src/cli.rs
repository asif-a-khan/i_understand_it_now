use crossterm::style::Stylize;

use crate::complexity;
use crate::problems;
use crate::progress::{ProblemProgress, Progress};
use crate::tracker::OperationLog;
use crate::visualizer;

pub fn solve(problem_name: &str, show_complexity: bool) {
    let Some(problem) = problems::get_problem(problem_name) else {
        eprintln!("{}", "Unknown problem".red().bold());
        eprintln!("  {problem_name}\n");
        suggest_problems(problem_name);
        std::process::exit(1);
    };

    println!("{}", format!("=== {} ===", problem.name()).bold());
    println!("Difficulty: {}", colorize_difficulty(problem.difficulty()));
    println!("Topic: {}", problem.topic());
    println!("{}\n", problem.description());

    let test_cases = problem.generate_tests();
    let total = test_cases.len();
    let mut passed = 0;
    let mut total_comparisons = 0;
    let mut total_swaps = 0;
    let mut total_ops = 0;

    for (i, test) in test_cases.iter().enumerate() {
        let mut log = OperationLog::new();
        let result = problem.run_solution(test, &mut log);

        let comps = log.comparisons();
        let swaps = log.swaps();
        let ops = log.total_operations();
        total_comparisons += comps;
        total_swaps += swaps;
        total_ops += ops;

        let status = if result.is_correct {
            passed += 1;
            "PASS".green().bold()
        } else {
            "FAIL".red().bold()
        };

        let metrics = if ops > 0 {
            format!(" | ops: {} (cmp: {}, swap: {})", ops, comps, swaps)
        } else {
            String::new()
        };

        println!("  Test {}/{}: {}{}", i + 1, total, status, metrics,);

        if !result.is_correct {
            println!("    Input:    {}", result.input_description);
            println!("    Expected: {}", result.expected);
            println!("    Got:      {}", result.actual);
        }
    }

    println!();
    if passed == total {
        println!(
            "{}",
            format!("Result: {passed}/{total} tests passed")
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            format!("Result: {passed}/{total} tests passed")
                .red()
                .bold()
        );
    }

    if total_ops > 0 {
        println!(
            "Total operations: {} (comparisons: {}, swaps: {})",
            total_ops, total_comparisons, total_swaps
        );
    }

    // Save progress
    let mut progress = Progress::load();
    let solved = passed == total;
    let entry = progress
        .problems
        .entry(problem_name.to_string())
        .or_insert(ProblemProgress {
            solved: false,
            best_comparisons: None,
            best_swaps: None,
            best_total_ops: None,
        });
    if solved {
        entry.solved = true;
    }
    if total_ops > 0 {
        entry.best_total_ops = Some(entry.best_total_ops.map_or(total_ops, |b| b.min(total_ops)));
        entry.best_comparisons = Some(
            entry
                .best_comparisons
                .map_or(total_comparisons, |b| b.min(total_comparisons)),
        );
        entry.best_swaps = Some(entry.best_swaps.map_or(total_swaps, |b| b.min(total_swaps)));
    }
    let _ = progress.save();

    // Empirical complexity measurement
    if show_complexity {
        println!("\n{}", "Measuring empirical complexity...".bold());
        let result = complexity::measure_complexity(problem.as_ref());
        println!("{}", result.ascii_plot);
    }
}

pub fn viz(algorithm_name: &str, mode: &str, delay_ms: u64) {
    let Some(viz) = visualizer::get_reference(algorithm_name) else {
        eprintln!("{}", "Unknown algorithm".red().bold());
        eprintln!("  {algorithm_name}\n");
        eprintln!("Available visualizations:");
        for name in visualizer::list_references() {
            eprintln!("  {}", name);
        }
        std::process::exit(1);
    };

    println!("{}", format!("=== {} ===", viz.name()).bold());
    println!("{}\n", viz.description());

    let input = viz.default_input();
    println!("Input: {:?}\n", input);

    let frames = viz.generate_frames(&input);
    println!(
        "Generated {} frames. Launching visualizer...\n",
        frames.len()
    );

    let playback_mode = match mode {
        "auto" => visualizer::engine::PlaybackMode::AutoPlay,
        _ => visualizer::engine::PlaybackMode::StepByStep,
    };

    if let Err(e) = visualizer::engine::play(&frames, playback_mode, delay_ms) {
        eprintln!("Visualization error: {e}");
    }
}

pub fn list(topic_filter: Option<&str>, difficulty_filter: Option<&str>) {
    let all = problems::all_problems();
    let progress = Progress::load();

    let filtered: Vec<_> = all
        .iter()
        .filter(|p| {
            if let Some(topic) = topic_filter {
                if p.topic() != topic {
                    return false;
                }
            }
            if let Some(diff) = difficulty_filter {
                let d = format!("{}", p.difficulty()).to_lowercase();
                if d != diff.to_lowercase() {
                    return false;
                }
            }
            true
        })
        .collect();

    if filtered.is_empty() {
        println!("No problems found matching the filter.");
        if topic_filter.is_some() || difficulty_filter.is_some() {
            println!("\nAvailable topics:");
            let mut topics: Vec<_> = all.iter().map(|p| p.topic().to_string()).collect();
            topics.sort();
            topics.dedup();
            for t in &topics {
                println!("  {t}");
            }
        }
        return;
    }

    let mut current_topic = String::new();
    for p in &filtered {
        if p.topic() != current_topic {
            current_topic = p.topic().to_string();
            println!("\n{}", format!("── {} ──", current_topic).bold());
        }

        let check = if progress.problems.get(p.id()).is_some_and(|pp| pp.solved) {
            "[x]".green()
        } else {
            "[ ]".dark_grey()
        };

        let diff = colorize_difficulty(p.difficulty());
        println!("  {} {} {} — {}", check, diff, p.id(), p.name());
    }

    let solved = filtered
        .iter()
        .filter(|p| progress.problems.get(p.id()).is_some_and(|pp| pp.solved))
        .count();
    println!(
        "\n{}",
        format!("{solved}/{} problems solved", filtered.len()).bold()
    );
}

pub fn status() {
    let progress = Progress::load();
    let all = problems::all_problems();
    let total = all.len();
    let solved = all
        .iter()
        .filter(|p| progress.problems.get(p.id()).is_some_and(|pp| pp.solved))
        .count();

    println!("{}", "=== I Understand It Now — Progress ===".bold());
    println!(
        "Overall: {solved}/{total} problems solved ({:.0}%)\n",
        if total > 0 {
            solved as f64 / total as f64 * 100.0
        } else {
            0.0
        }
    );

    // Group by topic
    let mut topics: Vec<String> = all.iter().map(|p| p.topic().to_string()).collect();
    topics.sort();
    topics.dedup();

    for topic in &topics {
        let topic_problems: Vec<_> = all.iter().filter(|p| p.topic() == topic).collect();
        let topic_solved = topic_problems
            .iter()
            .filter(|p| progress.problems.get(p.id()).is_some_and(|pp| pp.solved))
            .count();
        let topic_total = topic_problems.len();
        let pct = if topic_total > 0 {
            topic_solved as f64 / topic_total as f64 * 100.0
        } else {
            0.0
        };

        let bar_width = 20;
        let filled = (pct / 100.0 * bar_width as f64) as usize;
        let bar = format!("[{}{}]", "#".repeat(filled), "-".repeat(bar_width - filled));

        let bar_colored = if topic_solved == topic_total && topic_total > 0 {
            bar.green()
        } else if topic_solved > 0 {
            bar.yellow()
        } else {
            bar.dark_grey()
        };

        println!(
            "  {:>25}  {} {}/{}",
            topic, bar_colored, topic_solved, topic_total
        );
    }

    // Lessons read
    let lessons_read = progress.lessons_read.values().filter(|&&v| v).count();
    println!("\nLessons read: {lessons_read}/38");
}

fn colorize_difficulty(d: problems::Difficulty) -> crossterm::style::StyledContent<String> {
    match d {
        problems::Difficulty::Easy => format!("{d}").green(),
        problems::Difficulty::Medium => format!("{d}").yellow(),
        problems::Difficulty::Hard => format!("{d}").red(),
    }
}

fn suggest_problems(query: &str) {
    let all = problems::list_problems();
    let matches: Vec<_> = all
        .iter()
        .filter(|name| name.contains(query) || query.contains(name.as_str()))
        .take(5)
        .collect();

    if !matches.is_empty() {
        eprintln!("Did you mean:");
        for name in matches {
            eprintln!("  {name}");
        }
    } else {
        eprintln!("Available problems (first 10):");
        for name in all.iter().take(10) {
            eprintln!("  {name}");
        }
        if all.len() > 10 {
            eprintln!(
                "  ... and {} more (use 'cargo run -- list' to see all)",
                all.len() - 10
            );
        }
    }
}
