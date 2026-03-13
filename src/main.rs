mod cli;
mod problems;
mod progress;
mod solutions;
mod tracker;
mod visualizer;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dsa-forge", about = "Learn data structures & algorithms in Rust")]
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
    },
    /// Watch a specific algorithm visualization
    Viz {
        /// Algorithm identifier (e.g. merge_sort)
        algorithm: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Solve { problem }) => cli::solve(&problem),
        Some(Command::Viz { algorithm }) => cli::viz(&algorithm),
        None => {
            println!("dsa-forge TUI coming soon!");
            println!("For now, try:");
            println!("  cargo run -- solve <problem>");
            println!("  cargo run -- viz <algorithm>");
        }
    }
}
