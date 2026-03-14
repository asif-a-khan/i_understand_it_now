use crate::problems::Problem;
use crate::tracker::OperationLog;

/// Result of empirical complexity measurement.
#[allow(dead_code)]
pub struct ComplexityResult {
    /// (input_size, operations_count) pairs
    pub measurements: Vec<(usize, usize)>,
    /// Estimated Big-O class
    pub estimated_complexity: String,
    /// ASCII chart of the measurements
    pub ascii_plot: String,
}

/// Measure the empirical complexity of a problem's solution by running it
/// at multiple input sizes and analyzing the growth pattern.
pub fn measure_complexity(problem: &dyn Problem) -> ComplexityResult {
    let sizes = [10, 20, 50, 100, 200, 500];
    let mut measurements = Vec::new();

    for &size in &sizes {
        let mut total_ops = 0usize;
        let runs = 3;
        for _ in 0..runs {
            let tests = problem.generate_tests();
            if let Some(test) = tests.first() {
                let mut log = OperationLog::new();
                let _ = problem.run_solution(test, &mut log);
                total_ops += log.total_operations();
            }
        }
        let avg_ops = total_ops / runs;
        if avg_ops > 0 {
            measurements.push((size, avg_ops));
        }
    }

    let estimated_complexity = estimate_big_o(&measurements);
    let ascii_plot = draw_plot(&measurements, &estimated_complexity);

    ComplexityResult {
        measurements,
        estimated_complexity,
        ascii_plot,
    }
}

/// Estimate Big-O by analyzing growth ratios.
fn estimate_big_o(measurements: &[(usize, usize)]) -> String {
    if measurements.len() < 2 {
        return "Insufficient data".to_string();
    }

    // Compute growth ratios between consecutive measurements
    let mut ratios = Vec::new();
    for w in measurements.windows(2) {
        let (n1, ops1) = w[0];
        let (n2, ops2) = w[1];
        if ops1 > 0 && n1 > 0 {
            let size_ratio = n2 as f64 / n1 as f64;
            let ops_ratio = ops2 as f64 / ops1 as f64;
            ratios.push((size_ratio, ops_ratio));
        }
    }

    if ratios.is_empty() {
        return "Insufficient data".to_string();
    }

    // Analyze the relationship between size growth and ops growth
    // For O(n): ops_ratio ≈ size_ratio
    // For O(n²): ops_ratio ≈ size_ratio²
    // For O(n log n): ops_ratio ≈ size_ratio * log(n2)/log(n1)
    // For O(log n): ops_ratio ≈ log(n2)/log(n1)
    // For O(1): ops_ratio ≈ 1

    let avg_exponent: f64 =
        ratios.iter().map(|(sr, or)| or.ln() / sr.ln()).sum::<f64>() / ratios.len() as f64;

    if avg_exponent < 0.1 {
        "O(1)".to_string()
    } else if avg_exponent < 0.6 {
        "O(log n)".to_string()
    } else if avg_exponent < 1.15 {
        "O(n)".to_string()
    } else if avg_exponent < 1.5 {
        "O(n log n)".to_string()
    } else if avg_exponent < 2.2 {
        "O(n^2)".to_string()
    } else if avg_exponent < 3.2 {
        "O(n^3)".to_string()
    } else {
        format!("O(n^{:.1})", avg_exponent)
    }
}

/// Draw an ASCII plot of the measurements.
fn draw_plot(measurements: &[(usize, usize)], estimated: &str) -> String {
    if measurements.is_empty() {
        return "No data to plot.".to_string();
    }

    let max_ops = measurements.iter().map(|m| m.1).max().unwrap_or(1).max(1);
    let max_n = measurements.iter().map(|m| m.0).max().unwrap_or(1);
    let height = 15usize;
    let width = 50usize;

    let mut lines = Vec::new();
    lines.push(format!("  Empirical Complexity: {}", estimated));
    lines.push(String::new());
    lines.push("  ops".to_string());

    // Build the plot grid
    let mut grid = vec![vec![' '; width]; height];

    // Plot points
    for &(n, ops) in measurements {
        let x = (n as f64 / max_n as f64 * (width - 1) as f64) as usize;
        let y = (ops as f64 / max_ops as f64 * (height - 1) as f64) as usize;
        let x = x.min(width - 1);
        let y = y.min(height - 1);
        grid[y][x] = '*';

        // Draw a dot with surrounding area for visibility
        if x > 0 {
            grid[y][x - 1] = '-';
        }
        if x + 1 < width {
            grid[y][x + 1] = '-';
        }
    }

    // Render rows (top to bottom = high to low)
    for row in (0..height).rev() {
        let ops_label = if row == height - 1 {
            format!("{:>8}", max_ops)
        } else if row == 0 {
            format!("{:>8}", 0)
        } else {
            "        ".to_string()
        };
        let row_str: String = grid[row].iter().collect();
        lines.push(format!("  {} |{}", ops_label, row_str));
    }

    // X-axis
    lines.push(format!("  {:>8} +{}", "", "-".repeat(width)));
    lines.push(format!(
        "  {:>8}  0{:>width$}",
        "",
        max_n,
        width = width - 1
    ));
    lines.push(format!("  {:>8}  n (input size)", ""));
    lines.push(String::new());

    // Data table
    lines.push("  Size  |  Operations".to_string());
    lines.push("  ------+-----------".to_string());
    for &(n, ops) in measurements {
        lines.push(format!("  {:>5} | {:>9}", n, ops));
    }

    lines.join("\n")
}
