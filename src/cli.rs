use crate::problems;
use crate::tracker::OperationLog;

pub fn solve(problem_name: &str) {
    let Some(problem) = problems::get_problem(problem_name) else {
        eprintln!("Unknown problem: {problem_name}");
        eprintln!("\nAvailable problems:");
        for name in problems::list_problems() {
            eprintln!("  {name}");
        }
        std::process::exit(1);
    };

    println!("=== {} ===", problem.name());
    println!("Difficulty: {:?}", problem.difficulty());
    println!("{}\n", problem.description());

    let test_cases = problem.generate_tests();
    let total = test_cases.len();
    let mut passed = 0;

    for (i, test) in test_cases.iter().enumerate() {
        let mut log = OperationLog::new();
        let result = problem.run_solution(test, &mut log);

        let status = if result.is_correct {
            passed += 1;
            "PASS"
        } else {
            "FAIL"
        };

        println!(
            "Test {}/{}: {} | comparisons: {} | swaps: {}",
            i + 1,
            total,
            status,
            log.comparisons(),
            log.swaps(),
        );

        if !result.is_correct {
            println!("  Input:    {:?}", result.input_description);
            println!("  Expected: {:?}", result.expected);
            println!("  Got:      {:?}", result.actual);
        }
    }

    println!("\nResult: {passed}/{total} tests passed");
}

pub fn viz(algorithm_name: &str) {
    let Some(viz) = crate::visualizer::get_reference(algorithm_name) else {
        eprintln!("Unknown algorithm: {algorithm_name}");
        eprintln!("\nAvailable visualizations:");
        for name in crate::visualizer::list_references() {
            eprintln!("  {name}");
        }
        std::process::exit(1);
    };

    println!("=== Visualizing: {} ===\n", viz.name());
    println!("(Visualization playback coming soon)");
}
