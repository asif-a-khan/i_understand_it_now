mod cli;
mod complexity;
mod problems;
mod progress;
mod solutions;
mod tracker;
mod tui;
mod visualizer;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "iuin",
    about = "I Understand It Now — learn data structures & algorithms in Rust"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Test a specific solution against randomized inputs
    Solve {
        /// Problem identifier (e.g. arrays_two_sum)
        problem: String,
        /// Also measure empirical complexity
        #[arg(long)]
        complexity: bool,
    },
    /// Watch a specific algorithm visualization
    Viz {
        /// Algorithm identifier (e.g. bubble_sort)
        algorithm: String,
        /// Playback mode: "step" or "auto"
        #[arg(long, default_value = "step")]
        mode: String,
        /// Delay between frames in auto mode (milliseconds)
        #[arg(long, default_value = "500")]
        delay: u64,
    },
    /// List available problems
    List {
        /// Filter by topic (e.g. arrays, binary_search)
        #[arg(long)]
        topic: Option<String>,
        /// Filter by difficulty (easy, medium, hard)
        #[arg(long)]
        difficulty: Option<String>,
    },
    /// Show your progress
    Status,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Solve {
            problem,
            complexity,
        }) => cli::solve(&problem, complexity),
        Some(Command::Viz {
            algorithm,
            mode,
            delay,
        }) => cli::viz(&algorithm, &mode, delay),
        Some(Command::List { topic, difficulty }) => {
            cli::list(topic.as_deref(), difficulty.as_deref());
        }
        Some(Command::Status) => cli::status(),
        None => {
            if let Err(e) = tui::run() {
                eprintln!("TUI error: {e}");
                std::process::exit(1);
            }
        }
    }
}
