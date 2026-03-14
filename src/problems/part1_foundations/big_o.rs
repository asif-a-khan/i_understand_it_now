use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part1_foundations::big_o as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy (5)
        Box::new(BigOSingleLoop),
        Box::new(BigOConstant),
        Box::new(BigONestedLoop),
        Box::new(BigOBinarySearch),
        Box::new(BigOSequential),
        // Medium (5)
        Box::new(BigOTripleNested),
        Box::new(BigOLogLinear),
        Box::new(BigOSqrt),
        Box::new(BigOTwoInputs),
        Box::new(BigOHalfEachTime),
        // Hard (5)
        Box::new(BigOFibonacciNaive),
        Box::new(BigOPermutations),
        Box::new(BigOMatrixMultiply),
        Box::new(BigOAmortizedDynamicArray),
        Box::new(BigOBfsGraph),
    ]
}

/// Shared test data for conceptual Big-O problems.
/// Each problem has exactly one correct answer, so we store a simple identifier.
#[allow(dead_code)]
struct BigOTest {
    pattern_id: u32,
}

/// Helper: build one test case carrying the given pattern id.
fn single_test(id: u32) -> Vec<TestCase> {
    vec![TestCase {
        data: Box::new(BigOTest { pattern_id: id }),
    }]
}

/// Helper: compare a user answer to the expected answer, normalizing whitespace.
fn check(test: &TestCase, expected: &str, actual: &str, snippet: &str) -> SolutionResult {
    let _t = test.data.downcast_ref::<BigOTest>().unwrap();
    let norm_expected = expected.replace(' ', "");
    let norm_actual = actual.replace(' ', "");
    SolutionResult {
        is_correct: norm_expected == norm_actual,
        input_description: snippet.to_string(),
        expected: expected.to_string(),
        actual: actual.to_string(),
    }
}

// ── Easy 1: Single Loop ─────────────────────────────────────────────────

struct BigOSingleLoop;

impl Problem for BigOSingleLoop {
    fn id(&self) -> &str {
        "big_o_single_loop"
    }
    fn name(&self) -> &str {
        "Single Loop Complexity"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }

    fn description(&self) -> &str {
        "What is the time complexity of a single for-loop iterating from 0 to n?\n\n\
         ```\n\
         for i in 0..n {\n\
         \x20   // O(1) work per iteration\n\
         }\n\
         ```\n\n\
         Return a string like \"O(n)\", \"O(1)\", \"O(n^2)\", etc."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(1)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(n)";
        let actual = solutions::single_loop();
        check(test, expected, actual, "for i in 0..n { O(1) work }")
    }
}

// ── Easy 2: Constant Time ───────────────────────────────────────────────

struct BigOConstant;

impl Problem for BigOConstant {
    fn id(&self) -> &str {
        "big_o_constant"
    }
    fn name(&self) -> &str {
        "Constant Time Access"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }

    fn description(&self) -> &str {
        "What is the time complexity of accessing an array element by index?\n\n\
         ```\n\
         let x = arr[42];\n\
         ```\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(2)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(1)";
        let actual = solutions::constant();
        check(test, expected, actual, "let x = arr[42];")
    }
}

// ── Easy 3: Nested Loop ────────────────────────────────────────────────

struct BigONestedLoop;

impl Problem for BigONestedLoop {
    fn id(&self) -> &str {
        "big_o_nested_loop"
    }
    fn name(&self) -> &str {
        "Nested Loop Complexity"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }

    fn description(&self) -> &str {
        "What is the time complexity of two nested loops, each iterating from 0 to n?\n\n\
         ```\n\
         for i in 0..n {\n\
         \x20   for j in 0..n {\n\
         \x20       // O(1) work\n\
         \x20   }\n\
         }\n\
         ```\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(3)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(n^2)";
        let actual = solutions::nested_loop();
        check(
            test,
            expected,
            actual,
            "for i in 0..n { for j in 0..n { O(1) } }",
        )
    }
}

// ── Easy 4: Binary Search ──────────────────────────────────────────────

struct BigOBinarySearch;

impl Problem for BigOBinarySearch {
    fn id(&self) -> &str {
        "big_o_binary_search"
    }
    fn name(&self) -> &str {
        "Binary Search Complexity"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }

    fn description(&self) -> &str {
        "What is the time complexity of binary search on a sorted array of n elements?\n\n\
         ```\n\
         let (mut lo, mut hi) = (0, n);\n\
         while lo < hi {\n\
         \x20   let mid = lo + (hi - lo) / 2;\n\
         \x20   if arr[mid] == target { return mid; }\n\
         \x20   else if arr[mid] < target { lo = mid + 1; }\n\
         \x20   else { hi = mid; }\n\
         }\n\
         ```\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(4)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(log n)";
        let actual = solutions::binary_search();
        check(
            test,
            expected,
            actual,
            "Binary search: halve the range each step",
        )
    }
}

// ── Easy 5: Sequential Loops ───────────────────────────────────────────

struct BigOSequential;

impl Problem for BigOSequential {
    fn id(&self) -> &str {
        "big_o_sequential"
    }
    fn name(&self) -> &str {
        "Sequential Loops"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }

    fn description(&self) -> &str {
        "What is the time complexity when two sequential (non-nested) loops each run O(n) times?\n\n\
         ```\n\
         for i in 0..n {\n\
         \x20   // O(1) work\n\
         }\n\
         for j in 0..n {\n\
         \x20   // O(1) work\n\
         }\n\
         ```\n\n\
         Hint: O(n) + O(n) = ?\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(5)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(n)";
        let actual = solutions::sequential();
        check(
            test,
            expected,
            actual,
            "for i in 0..n { O(1) }; for j in 0..n { O(1) }",
        )
    }
}

// ── Medium 1: Triple Nested Loop ───────────────────────────────────────

struct BigOTripleNested;

impl Problem for BigOTripleNested {
    fn id(&self) -> &str {
        "big_o_triple_nested"
    }
    fn name(&self) -> &str {
        "Triple Nested Loop"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }

    fn description(&self) -> &str {
        "What is the time complexity of three nested loops, each iterating up to n?\n\n\
         ```\n\
         for i in 0..n {\n\
         \x20   for j in 0..n {\n\
         \x20       for k in 0..n {\n\
         \x20           // O(1) work\n\
         \x20       }\n\
         \x20   }\n\
         }\n\
         ```\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(6)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(n^3)";
        let actual = solutions::triple_nested();
        check(
            test,
            expected,
            actual,
            "for i in 0..n { for j in 0..n { for k in 0..n { O(1) } } }",
        )
    }
}

// ── Medium 2: Log-Linear ──────────────────────────────────────────────

struct BigOLogLinear;

impl Problem for BigOLogLinear {
    fn id(&self) -> &str {
        "big_o_log_linear"
    }
    fn name(&self) -> &str {
        "Log-Linear Complexity"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }

    fn description(&self) -> &str {
        "What is the time complexity of a loop that runs n times, where each iteration \
         performs a binary search on an array of size n?\n\n\
         ```\n\
         for i in 0..n {\n\
         \x20   binary_search(&arr, some_target); // O(log n)\n\
         }\n\
         ```\n\n\
         Think: n iterations * cost per iteration.\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(7)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(n log n)";
        let actual = solutions::log_linear();
        check(
            test,
            expected,
            actual,
            "for i in 0..n { binary_search (O(log n)) }",
        )
    }
}

// ── Medium 3: Square Root ─────────────────────────────────────────────

struct BigOSqrt;

impl Problem for BigOSqrt {
    fn id(&self) -> &str {
        "big_o_sqrt"
    }
    fn name(&self) -> &str {
        "Square Root Loop"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }

    fn description(&self) -> &str {
        "What is the time complexity of a loop that iterates from 1 to sqrt(n)?\n\n\
         ```\n\
         let mut i = 1;\n\
         while i * i <= n {\n\
         \x20   // O(1) work\n\
         \x20   i += 1;\n\
         }\n\
         ```\n\n\
         This pattern appears in trial division for primality testing.\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(8)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(sqrt(n))";
        let actual = solutions::sqrt_loop();
        check(test, expected, actual, "while i * i <= n { O(1); i += 1; }")
    }
}

// ── Medium 4: Two Inputs ──────────────────────────────────────────────

struct BigOTwoInputs;

impl Problem for BigOTwoInputs {
    fn id(&self) -> &str {
        "big_o_two_inputs"
    }
    fn name(&self) -> &str {
        "Two Different Inputs"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }

    fn description(&self) -> &str {
        "What is the time complexity of nested loops with TWO different input sizes?\n\n\
         ```\n\
         for i in 0..m {\n\
         \x20   for j in 0..n {\n\
         \x20       // O(1) work\n\
         \x20   }\n\
         }\n\
         ```\n\n\
         Careful: the outer loop runs m times, the inner loop runs n times.\n\
         These are independent variables.\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(9)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(m*n)";
        let actual = solutions::two_inputs();
        check(
            test,
            expected,
            actual,
            "for i in 0..m { for j in 0..n { O(1) } }",
        )
    }
}

// ── Medium 5: Halving Each Time ───────────────────────────────────────

struct BigOHalfEachTime;

impl Problem for BigOHalfEachTime {
    fn id(&self) -> &str {
        "big_o_half_each_time"
    }
    fn name(&self) -> &str {
        "Halving Loop"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }

    fn description(&self) -> &str {
        "What is the time complexity of a loop that halves the input each iteration?\n\n\
         ```\n\
         let mut x = n;\n\
         while x > 1 {\n\
         \x20   // O(1) work\n\
         \x20   x /= 2;\n\
         }\n\
         ```\n\n\
         How many times can you halve n before reaching 1?\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(10)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(log n)";
        let actual = solutions::half_each_time();
        check(test, expected, actual, "while x > 1 { O(1); x /= 2; }")
    }
}

// ── Hard 1: Naive Fibonacci ───────────────────────────────────────────

struct BigOFibonacciNaive;

impl Problem for BigOFibonacciNaive {
    fn id(&self) -> &str {
        "big_o_fibonacci_naive"
    }
    fn name(&self) -> &str {
        "Naive Recursive Fibonacci"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }

    fn description(&self) -> &str {
        "What is the time complexity of the naive recursive Fibonacci implementation?\n\n\
         ```\n\
         fn fib(n: u64) -> u64 {\n\
         \x20   if n <= 1 { return n; }\n\
         \x20   fib(n - 1) + fib(n - 2)\n\
         }\n\
         ```\n\n\
         Each call branches into TWO recursive calls, and the recursion tree\n\
         has depth n. The exact bound is O(phi^n) where phi ~ 1.618, but the\n\
         standard Big-O answer is the exponential upper bound.\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(11)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(2^n)";
        let actual = solutions::fibonacci_naive();
        check(
            test,
            expected,
            actual,
            "fn fib(n) { if n<=1 {n} else { fib(n-1) + fib(n-2) } }",
        )
    }
}

// ── Hard 2: Permutations ──────────────────────────────────────────────

struct BigOPermutations;

impl Problem for BigOPermutations {
    fn id(&self) -> &str {
        "big_o_permutations"
    }
    fn name(&self) -> &str {
        "Generating All Permutations"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }

    fn description(&self) -> &str {
        "What is the time complexity of generating all permutations of n elements?\n\n\
         ```\n\
         fn permute(arr: &mut [i32], l: usize, results: &mut Vec<Vec<i32>>) {\n\
         \x20   if l == arr.len() {\n\
         \x20       results.push(arr.to_vec());\n\
         \x20       return;\n\
         \x20   }\n\
         \x20   for i in l..arr.len() {\n\
         \x20       arr.swap(l, i);\n\
         \x20       permute(arr, l + 1, results);\n\
         \x20       arr.swap(l, i);\n\
         \x20   }\n\
         }\n\
         ```\n\n\
         How many permutations of n distinct elements exist?\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(12)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(n!)";
        let actual = solutions::permutations();
        check(
            test,
            expected,
            actual,
            "Generate all permutations via backtracking",
        )
    }
}

// ── Hard 3: Matrix Multiplication ─────────────────────────────────────

struct BigOMatrixMultiply;

impl Problem for BigOMatrixMultiply {
    fn id(&self) -> &str {
        "big_o_matrix_multiply"
    }
    fn name(&self) -> &str {
        "Standard Matrix Multiplication"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }

    fn description(&self) -> &str {
        "What is the time complexity of standard (naive) multiplication of two n x n matrices?\n\n\
         ```\n\
         for i in 0..n {\n\
         \x20   for j in 0..n {\n\
         \x20       let mut sum = 0;\n\
         \x20       for k in 0..n {\n\
         \x20           sum += a[i][k] * b[k][j];\n\
         \x20       }\n\
         \x20       c[i][j] = sum;\n\
         \x20   }\n\
         }\n\
         ```\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(13)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(n^3)";
        let actual = solutions::matrix_multiply();
        check(
            test,
            expected,
            actual,
            "Three nested loops (i, j, k) each 0..n for matrix multiply",
        )
    }
}

// ── Hard 4: Amortized Dynamic Array ───────────────────────────────────

struct BigOAmortizedDynamicArray;

impl Problem for BigOAmortizedDynamicArray {
    fn id(&self) -> &str {
        "big_o_amortized_dynamic_array"
    }
    fn name(&self) -> &str {
        "Amortized Dynamic Array Push"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }

    fn description(&self) -> &str {
        "What is the AMORTIZED time complexity of a single push operation on a dynamic\n\
         array (like Rust's Vec) that doubles its capacity when full?\n\n\
         Most pushes are O(1), but occasionally the array must be reallocated\n\
         and all elements copied. However, these expensive operations become\n\
         rarer as the array grows.\n\n\
         If you perform n pushes in total, the total cost is O(n). What is the\n\
         amortized cost PER push?\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(14)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(1)";
        let actual = solutions::amortized_dynamic_array();
        check(
            test,
            expected,
            actual,
            "Vec::push with doubling — amortized cost per push",
        )
    }
}

// ── Hard 5: BFS on a Graph ────────────────────────────────────────────

struct BigOBfsGraph;

impl Problem for BigOBfsGraph {
    fn id(&self) -> &str {
        "big_o_bfs_graph"
    }
    fn name(&self) -> &str {
        "BFS Graph Traversal"
    }
    fn topic(&self) -> &str {
        "big_o"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }

    fn description(&self) -> &str {
        "What is the time complexity of BFS on a graph with V vertices and E edges\n\
         (adjacency list representation)?\n\n\
         ```\n\
         fn bfs(graph: &[Vec<usize>], start: usize) {\n\
         \x20   let mut visited = vec![false; graph.len()];\n\
         \x20   let mut queue = VecDeque::new();\n\
         \x20   visited[start] = true;\n\
         \x20   queue.push_back(start);\n\
         \x20   while let Some(node) = queue.pop_front() {\n\
         \x20       for &neighbor in &graph[node] {\n\
         \x20           if !visited[neighbor] {\n\
         \x20               visited[neighbor] = true;\n\
         \x20               queue.push_back(neighbor);\n\
         \x20           }\n\
         \x20       }\n\
         \x20   }\n\
         }\n\
         ```\n\n\
         Every vertex is enqueued at most once, and for each vertex we examine\n\
         all its edges.\n\n\
         Return the Big-O as a string."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        single_test(15)
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let expected = "O(V+E)";
        let actual = solutions::bfs_graph();
        check(
            test,
            expected,
            actual,
            "BFS: visit each vertex once, examine each edge once",
        )
    }
}
