// Backtracking — Solution Templates
// Edit these functions to solve each problem. Run with: cargo run -- solve <problem_id>

// ── Easy ────────────────────────────────────────────────────────────────

/// Generate all subsets of the given array. Return sorted (each subset sorted,
/// list sorted lexicographically).
///
/// Example: nums=[1,2,3] => [[], [1], [1,2], [1,2,3], [1,3], [2], [2,3], [3]]
pub fn subsets(_nums: &[i32]) -> Vec<Vec<i32>> {
    todo!()
}

/// Generate all permutations of the given array. Return sorted lexicographically.
///
/// Example: nums=[1,2,3] => [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
pub fn permutations(_nums: &[i32]) -> Vec<Vec<i32>> {
    todo!()
}

/// Generate all combinations of k numbers chosen from [1, n]. Return sorted.
///
/// Example: n=4, k=2 => [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
pub fn combinations(_n: i32, _k: i32) -> Vec<Vec<i32>> {
    todo!()
}

/// Given a string of digits 2-9, return all possible letter combinations (phone keypad).
/// Return sorted. Return empty vec for empty input.
///
/// Mapping: 2->abc, 3->def, 4->ghi, 5->jkl, 6->mno, 7->pqrs, 8->tuv, 9->wxyz
///
/// Example: digits="23" => ["ad","ae","af","bd","be","bf","cd","ce","cf"]
pub fn letter_combinations(_digits: &str) -> Vec<String> {
    todo!()
}

/// Generate all binary strings of length n, sorted.
///
/// Example: n=2 => ["00", "01", "10", "11"]
/// Example: n=0 => [""]
pub fn binary_strings(_n: usize) -> Vec<String> {
    todo!()
}

// ── Medium ──────────────────────────────────────────────────────────────

/// Combination Sum: find all unique combinations in candidates that sum to target.
/// Each candidate can be used unlimited times. Return sorted.
///
/// Example: candidates=[2,3,6,7], target=7 => [[2,2,3],[7]]
pub fn combination_sum(_candidates: &[i32], _target: i32) -> Vec<Vec<i32>> {
    todo!()
}

/// Combination Sum II: each candidate used at most once, may contain duplicates.
/// Return unique combinations that sum to target, sorted.
///
/// Example: candidates=[10,1,2,7,6,1,5], target=8 => [[1,1,6],[1,2,5],[1,7],[2,6]]
pub fn combination_sum_ii(_candidates: &[i32], _target: i32) -> Vec<Vec<i32>> {
    todo!()
}

/// Palindrome Partitioning: partition s so every substring is a palindrome. Return sorted.
///
/// Example: s="aab" => [["a","a","b"],["aa","b"]]
pub fn palindrome_partition(_s: &str) -> Vec<Vec<String>> {
    todo!()
}

/// Generate all valid combinations of n pairs of parentheses. Return sorted.
///
/// Example: n=3 => ["((()))","(()())","(())()","()(())","()()()"]
pub fn generate_parentheses(_n: usize) -> Vec<String> {
    todo!()
}

/// Word Search: return true if the word exists in the grid (adjacent cells, no reuse).
///
/// Example: board=[['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], word="ABCCED" => true
pub fn word_search(_board: &[Vec<char>], _word: &str) -> bool {
    todo!()
}

// ── Hard ────────────────────────────────────────────────────────────────

/// N-Queens: return all solutions for placing n queens on an n x n board.
/// Each solution is Vec<String> where 'Q' is a queen and '.' is empty. Return sorted.
///
/// Example: n=4 => [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
pub fn n_queens(_n: usize) -> Vec<Vec<String>> {
    todo!()
}

/// Sudoku Solver: fill in the 9x9 grid (0 = empty) so each row, column,
/// and 3x3 box contains 1-9 exactly once.
///
/// Example: A valid Sudoku puzzle => its unique solution
pub fn sudoku_solver(_board: &[Vec<u8>]) -> Vec<Vec<u8>> {
    todo!()
}

/// Word Break II: add spaces to s so each word is in wordDict. Return all sentences, sorted.
///
/// Example: s="catsanddog", wordDict=["cat","cats","and","sand","dog"]
/// => ["cat sand dog","cats and dog"]
pub fn word_break_ii(_s: &str, _word_dict: &[String]) -> Vec<String> {
    todo!()
}

/// Restore IP Addresses: insert dots to form all valid IPv4 addresses. Return sorted.
///
/// Example: s="25525511135" => ["255.255.11.135","255.255.111.35"]
pub fn restore_ip(_s: &str) -> Vec<String> {
    todo!()
}

/// Expression Add Operators: insert +, -, * between digits to reach target. Return sorted.
/// No leading zeros on operands (except "0" itself).
///
/// Example: num="123", target=6 => ["1*2*3","1+2+3"]
/// Example: num="105", target=5 => ["1*0+5","10-5"]
pub fn expression_add_operators(_num: &str, _target: i64) -> Vec<String> {
    todo!()
}
