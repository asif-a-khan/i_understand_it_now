use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part6_advanced::string_algorithms as solutions;
use crate::tracker::{track_string, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy (5)
        Box::new(StringAlgoPatternMatch),
        Box::new(StringAlgoCountOccurrences),
        Box::new(StringAlgoIsRotation),
        Box::new(StringAlgoLongestPrefixSuffix),
        Box::new(StringAlgoRepeatedStringMatch),
        // Medium (5)
        Box::new(StringAlgoKmp),
        Box::new(StringAlgoRabinKarp),
        Box::new(StringAlgoZFunction),
        Box::new(StringAlgoLongestDuplicateSubstring),
        Box::new(StringAlgoShortestPalindrome),
        // Hard (5)
        Box::new(StringAlgoSuffixArray),
        Box::new(StringAlgoLcpArray),
        Box::new(StringAlgoDistinctSubstrings),
        Box::new(StringAlgoPalindromePartitioningMin),
        Box::new(StringAlgoLongestCommonSubstring),
    ]
}

// ── Reference implementations ────────────────────────────────────────

fn ref_pattern_match(text: &str, pattern: &str) -> i32 {
    if pattern.is_empty() {
        return 0;
    }
    match text.find(pattern) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

fn ref_count_occurrences(text: &str, pattern: &str) -> i32 {
    if pattern.is_empty() {
        return 0;
    }
    text.match_indices(pattern).count() as i32
}

fn ref_is_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    if s1.is_empty() {
        return true;
    }
    let doubled = format!("{s1}{s1}");
    doubled.contains(s2)
}

fn ref_longest_prefix_suffix(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n <= 1 {
        return 0;
    }
    // KMP failure function
    let mut lps = vec![0usize; n];
    let mut len = 0;
    let mut i = 1;
    while i < n {
        if bytes[i] == bytes[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    lps[n - 1] as i32
}

fn ref_repeated_string_match(a: &str, b: &str) -> i32 {
    if a.is_empty() {
        return -1;
    }
    let mut repeated = String::new();
    let mut count = 0;
    while repeated.len() < b.len() + a.len() * 2 {
        repeated.push_str(a);
        count += 1;
        if repeated.contains(b) {
            return count;
        }
    }
    -1
}

fn ref_kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let n = t.len();
    let m = p.len();
    if m == 0 || m > n {
        return vec![];
    }
    // Build failure function
    let mut lps = vec![0usize; m];
    let mut len = 0;
    let mut i = 1;
    while i < m {
        if p[i] == p[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            i += 1;
        }
    }
    // Search
    let mut results = Vec::new();
    let mut ti = 0;
    let mut pi = 0;
    while ti < n {
        if t[ti] == p[pi] {
            ti += 1;
            pi += 1;
        }
        if pi == m {
            results.push(ti - m);
            pi = lps[pi - 1];
        } else if ti < n && t[ti] != p[pi] {
            if pi != 0 {
                pi = lps[pi - 1];
            } else {
                ti += 1;
            }
        }
    }
    results
}

fn ref_z_function(s: &str) -> Vec<usize> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut z = vec![0usize; n];
    if n == 0 {
        return z;
    }
    z[0] = n;
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && bytes[z[i]] == bytes[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

fn ref_longest_duplicate_substring(s: &str) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut best = String::new();
    for len in (1..n).rev() {
        let mut found = false;
        let mut seen = std::collections::HashSet::new();
        for i in 0..=n - len {
            let sub = &bytes[i..i + len];
            if !seen.insert(sub.to_vec()) {
                best = String::from_utf8(sub.to_vec()).unwrap();
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
    best
}

fn ref_shortest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let rev: String = s.chars().rev().collect();
    let combined = format!("{s}#{rev}");
    let bytes = combined.as_bytes();
    let n = bytes.len();
    // KMP failure function
    let mut lps = vec![0usize; n];
    let mut len = 0;
    let mut i = 1;
    while i < n {
        if bytes[i] == bytes[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            i += 1;
        }
    }
    let longest_pal_prefix = lps[n - 1];
    let suffix_to_add: String = s[longest_pal_prefix..].chars().rev().collect();
    format!("{suffix_to_add}{s}")
}

fn ref_suffix_array(s: &str) -> Vec<usize> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&a, &b| bytes[a..].cmp(&bytes[b..]));
    sa
}

fn ref_lcp_array(s: &str) -> Vec<usize> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n == 0 {
        return vec![];
    }
    let sa = ref_suffix_array(s);
    let mut rank = vec![0usize; n];
    for (i, &s_idx) in sa.iter().enumerate() {
        rank[s_idx] = i;
    }
    let mut lcp = vec![0usize; n.saturating_sub(1)];
    let mut h = 0usize;
    for i in 0..n {
        if rank[i] > 0 {
            let j = sa[rank[i] - 1];
            while i + h < n && j + h < n && bytes[i + h] == bytes[j + h] {
                h += 1;
            }
            lcp[rank[i] - 1] = h;
            h = h.saturating_sub(1);
        } else {
            h = 0;
        }
    }
    lcp
}

fn ref_distinct_substrings(s: &str) -> i64 {
    let n = s.len() as i64;
    let total = n * (n + 1) / 2;
    let lcp = ref_lcp_array(s);
    let lcp_sum: i64 = lcp.iter().map(|&x| x as i64).sum();
    total - lcp_sum
}

fn ref_palindrome_partitioning_min(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n == 0 {
        return 0;
    }
    // is_pal[i][j] = true if s[i..=j] is palindrome
    let mut is_pal = vec![vec![false; n]; n];
    for (i, row) in is_pal.iter_mut().enumerate().take(n) {
        row[i] = true;
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if bytes[i] == bytes[j] {
                if len == 2 {
                    is_pal[i][j] = true;
                } else {
                    is_pal[i][j] = is_pal[i + 1][j - 1];
                }
            }
        }
    }
    // dp[i] = min cuts for s[0..=i]
    let mut dp = vec![0i32; n];
    for i in 0..n {
        if is_pal[0][i] {
            dp[i] = 0;
        } else {
            dp[i] = i32::MAX;
            for j in 1..=i {
                if is_pal[j][i] {
                    dp[i] = dp[i].min(dp[j - 1] + 1);
                }
            }
        }
    }
    dp[n - 1]
}

fn ref_longest_common_substring(s1: &str, s2: &str) -> String {
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    let n = b1.len();
    let m = b2.len();
    let mut max_len = 0;
    let mut end_idx = 0; // end index in s1
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if b1[i - 1] == b2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                if dp[i][j] > max_len {
                    max_len = dp[i][j];
                    end_idx = i;
                }
            }
        }
    }
    if max_len == 0 {
        String::new()
    } else {
        s1[end_idx - max_len..end_idx].to_string()
    }
}

// ── Easy 1: Pattern Match ────────────────────────────────────────────

struct StringAlgoPatternMatch;

struct PatternMatchTest {
    text: String,
    pattern: String,
}

impl Problem for StringAlgoPatternMatch {
    fn id(&self) -> &str {
        "string_algo_pattern_match"
    }
    fn name(&self) -> &str {
        "First Pattern Occurrence"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Find the first occurrence of pattern in text. Return -1 if not found.\n\n\
         Input: (text: String, pattern: String)\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let text_len = rng.random_range(5..=30);
                let text =
                    crate::problems::helpers::random_string_from(&mut rng, text_len, b"abcd");
                let pat_len = rng.random_range(1..=5);
                let pattern = if rng.random_range(0..3) == 0 {
                    // Sometimes use a substring of text
                    let start = rng.random_range(0..text_len.saturating_sub(pat_len).max(1));
                    let end = (start + pat_len).min(text_len);
                    text[start..end].to_string()
                } else {
                    crate::problems::helpers::random_string_from(&mut rng, pat_len, b"abcd")
                };
                TestCase {
                    data: Box::new(PatternMatchTest { text, pattern }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PatternMatchTest>().unwrap();
        let expected = ref_pattern_match(&t.text, &t.pattern);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_text = track_string(&t.text, shared_log.clone());
        let tracked_pattern = track_string(&t.pattern, shared_log.clone());
        let actual = solutions::pattern_match(&tracked_text, &tracked_pattern);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("text=\"{}\", pattern=\"{}\"", t.text, t.pattern),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 2: Count Occurrences ────────────────────────────────────────

struct StringAlgoCountOccurrences;

struct CountOccTest {
    text: String,
    pattern: String,
}

impl Problem for StringAlgoCountOccurrences {
    fn id(&self) -> &str {
        "string_algo_count_occurrences"
    }
    fn name(&self) -> &str {
        "Count Pattern Occurrences"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Count the number of (possibly overlapping) occurrences of pattern in text.\n\n\
         Input: (text: String, pattern: String)\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let text_len = rng.random_range(5..=30);
                let text = crate::problems::helpers::random_string_from(&mut rng, text_len, b"abc");
                let pat_len = rng.random_range(1..=3);
                let pattern =
                    crate::problems::helpers::random_string_from(&mut rng, pat_len, b"abc");
                TestCase {
                    data: Box::new(CountOccTest { text, pattern }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountOccTest>().unwrap();
        let expected = ref_count_occurrences(&t.text, &t.pattern);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_text = track_string(&t.text, shared_log.clone());
        let tracked_pattern = track_string(&t.pattern, shared_log.clone());
        let actual = solutions::count_occurrences(&tracked_text, &tracked_pattern);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("text=\"{}\", pattern=\"{}\"", t.text, t.pattern),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 3: Is Rotation ──────────────────────────────────────────────

struct StringAlgoIsRotation;

struct RotationTest {
    s1: String,
    s2: String,
}

impl Problem for StringAlgoIsRotation {
    fn id(&self) -> &str {
        "string_algo_is_rotation"
    }
    fn name(&self) -> &str {
        "String Rotation Check"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Check if s2 is a rotation of s1.\n\
         Example: \"abcde\" rotated -> \"cdeab\"\n\n\
         Input: (s1: String, s2: String)\n\
         Output: bool"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(3..=15);
                let s1 = crate::problems::helpers::random_string_from(&mut rng, len, b"abcd");
                let s2 = if rng.random_range(0..2) == 0 {
                    // Make a rotation
                    let rot = rng.random_range(0..len);
                    format!("{}{}", &s1[rot..], &s1[..rot])
                } else {
                    crate::problems::helpers::random_string_from(&mut rng, len, b"abcd")
                };
                TestCase {
                    data: Box::new(RotationTest { s1, s2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RotationTest>().unwrap();
        let expected = ref_is_rotation(&t.s1, &t.s2);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_s1 = track_string(&t.s1, shared_log.clone());
        let tracked_s2 = track_string(&t.s2, shared_log.clone());
        let actual = solutions::is_rotation(&tracked_s1, &tracked_s2);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s1=\"{}\", s2=\"{}\"", t.s1, t.s2),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 4: Longest Proper Prefix-Suffix ─────────────────────────────

struct StringAlgoLongestPrefixSuffix;

struct LpsTest {
    s: String,
}

impl Problem for StringAlgoLongestPrefixSuffix {
    fn id(&self) -> &str {
        "string_algo_longest_prefix_suffix"
    }
    fn name(&self) -> &str {
        "Longest Proper Prefix = Suffix"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Find the length of the longest proper prefix of the string which is also a suffix.\n\
         A proper prefix is not the entire string.\n\n\
         Input: String\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(2..=20);
                let s = crate::problems::helpers::random_string_from(&mut rng, len, b"abc");
                TestCase {
                    data: Box::new(LpsTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LpsTest>().unwrap();
        let expected = ref_longest_prefix_suffix(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::longest_prefix_suffix(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 5: Repeated String Match ────────────────────────────────────

struct StringAlgoRepeatedStringMatch;

struct RepeatedMatchTest {
    a: String,
    b: String,
}

impl Problem for StringAlgoRepeatedStringMatch {
    fn id(&self) -> &str {
        "string_algo_repeated_string_match"
    }
    fn name(&self) -> &str {
        "Repeated String Match"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Find the minimum number of times you need to repeat string a so that b is a \
         substring of the repeated string. Return -1 if impossible.\n\n\
         Input: (a: String, b: String)\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let a_len = rng.random_range(1..=6);
                let a = crate::problems::helpers::random_string_from(&mut rng, a_len, b"abc");
                let b_len = rng.random_range(1..=10);
                let b = if rng.random_range(0..3) == 0 {
                    // Make b from repeated a
                    let rep = a.repeat(3);
                    let start = rng.random_range(0..rep.len().saturating_sub(b_len).max(1));
                    let end = (start + b_len).min(rep.len());
                    rep[start..end].to_string()
                } else {
                    crate::problems::helpers::random_string_from(&mut rng, b_len, b"abc")
                };
                TestCase {
                    data: Box::new(RepeatedMatchTest { a, b }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RepeatedMatchTest>().unwrap();
        let expected = ref_repeated_string_match(&t.a, &t.b);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_a = track_string(&t.a, shared_log.clone());
        let tracked_b = track_string(&t.b, shared_log.clone());
        let actual = solutions::repeated_string_match(&tracked_a, &tracked_b);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("a=\"{}\", b=\"{}\"", t.a, t.b),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 1: KMP ────────────────────────────────────────────────────

struct StringAlgoKmp;

struct KmpTest {
    text: String,
    pattern: String,
}

impl Problem for StringAlgoKmp {
    fn id(&self) -> &str {
        "string_algo_kmp"
    }
    fn name(&self) -> &str {
        "KMP Pattern Matching"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Find all occurrences of pattern in text using KMP algorithm.\n\
         Return starting indices of all matches.\n\n\
         Input: (text: String, pattern: String)\n\
         Output: Vec<usize>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let text_len = rng.random_range(10..=40);
                let text = crate::problems::helpers::random_string_from(&mut rng, text_len, b"abc");
                let pat_len = rng.random_range(1..=5);
                let pattern =
                    crate::problems::helpers::random_string_from(&mut rng, pat_len, b"abc");
                TestCase {
                    data: Box::new(KmpTest { text, pattern }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KmpTest>().unwrap();
        let expected = ref_kmp_search(&t.text, &t.pattern);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_text = track_string(&t.text, shared_log.clone());
        let tracked_pattern = track_string(&t.pattern, shared_log.clone());
        let actual = solutions::kmp_search(&tracked_text, &tracked_pattern);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("text=\"{}\", pattern=\"{}\"", t.text, t.pattern),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 2: Rabin-Karp ─────────────────────────────────────────────

struct StringAlgoRabinKarp;

struct RabinKarpTest {
    text: String,
    pattern: String,
}

impl Problem for StringAlgoRabinKarp {
    fn id(&self) -> &str {
        "string_algo_rabin_karp"
    }
    fn name(&self) -> &str {
        "Rabin-Karp Pattern Matching"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Find all occurrences of pattern in text using Rabin-Karp (rolling hash).\n\
         Return starting indices of all matches.\n\n\
         Input: (text: String, pattern: String)\n\
         Output: Vec<usize>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let text_len = rng.random_range(10..=40);
                let text =
                    crate::problems::helpers::random_string_from(&mut rng, text_len, b"abcd");
                let pat_len = rng.random_range(1..=5);
                let pattern =
                    crate::problems::helpers::random_string_from(&mut rng, pat_len, b"abcd");
                TestCase {
                    data: Box::new(RabinKarpTest { text, pattern }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RabinKarpTest>().unwrap();
        let expected = ref_kmp_search(&t.text, &t.pattern); // same result
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_text = track_string(&t.text, shared_log.clone());
        let tracked_pattern = track_string(&t.pattern, shared_log.clone());
        let actual = solutions::rabin_karp(&tracked_text, &tracked_pattern);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("text=\"{}\", pattern=\"{}\"", t.text, t.pattern),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 3: Z-Function ─────────────────────────────────────────────

struct StringAlgoZFunction;

struct ZFuncTest {
    s: String,
}

impl Problem for StringAlgoZFunction {
    fn id(&self) -> &str {
        "string_algo_z_function"
    }
    fn name(&self) -> &str {
        "Z-Function Array"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Compute the Z-function array. z[i] = length of the longest substring starting \
         at position i which is also a prefix of the string. z[0] = len(s) by convention.\n\n\
         Input: String\n\
         Output: Vec<usize>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(3..=20);
                let s = crate::problems::helpers::random_string_from(&mut rng, len, b"abc");
                TestCase {
                    data: Box::new(ZFuncTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ZFuncTest>().unwrap();
        let expected = ref_z_function(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::z_function(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 4: Longest Duplicate Substring ────────────────────────────

struct StringAlgoLongestDuplicateSubstring;

struct LongestDupTest {
    s: String,
}

impl Problem for StringAlgoLongestDuplicateSubstring {
    fn id(&self) -> &str {
        "string_algo_longest_duplicate_substring"
    }
    fn name(&self) -> &str {
        "Longest Duplicate Substring"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Find the longest substring that appears at least twice.\n\
         Return empty string if none.\n\n\
         Input: String\n\
         Output: String"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(4..=15);
                let s = crate::problems::helpers::random_string_from(&mut rng, len, b"abc");
                TestCase {
                    data: Box::new(LongestDupTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LongestDupTest>().unwrap();
        let expected = ref_longest_duplicate_substring(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::longest_duplicate_substring(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        // Accept any answer of the same length that occurs at least twice
        let valid = actual.len() == expected.len()
            && (actual.is_empty() || t.s.match_indices(&actual).count() >= 2);
        SolutionResult {
            is_correct: valid,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("\"{}\" (len={})", expected, expected.len()),
            actual: format!("\"{}\" (len={})", actual, actual.len()),
        }
    }
}

// ── Medium 5: Shortest Palindrome ────────────────────────────────────

struct StringAlgoShortestPalindrome;

struct ShortPalTest {
    s: String,
}

impl Problem for StringAlgoShortestPalindrome {
    fn id(&self) -> &str {
        "string_algo_shortest_palindrome"
    }
    fn name(&self) -> &str {
        "Shortest Palindrome"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Find the shortest palindrome by adding characters to the front of s.\n\n\
         Input: String\n\
         Output: String"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(2..=15);
                let s = crate::problems::helpers::random_string_from(&mut rng, len, b"abc");
                TestCase {
                    data: Box::new(ShortPalTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ShortPalTest>().unwrap();
        let expected = ref_shortest_palindrome(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::shortest_palindrome(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("\"{expected}\""),
            actual: format!("\"{actual}\""),
        }
    }
}

// ── Hard 1: Suffix Array ─────────────────────────────────────────────

struct StringAlgoSuffixArray;

struct SuffixArrayTest {
    s: String,
}

impl Problem for StringAlgoSuffixArray {
    fn id(&self) -> &str {
        "string_algo_suffix_array"
    }
    fn name(&self) -> &str {
        "Build Suffix Array"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Build the suffix array for a string. The suffix array is the array of starting \
         indices of all suffixes, sorted lexicographically.\n\n\
         Input: String\n\
         Output: Vec<usize>"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(3..=15);
                let s = crate::problems::helpers::random_string_from(&mut rng, len, b"abcd");
                TestCase {
                    data: Box::new(SuffixArrayTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SuffixArrayTest>().unwrap();
        let expected = ref_suffix_array(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::suffix_array(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 2: LCP Array ────────────────────────────────────────────────

struct StringAlgoLcpArray;

struct LcpArrayTest {
    s: String,
}

impl Problem for StringAlgoLcpArray {
    fn id(&self) -> &str {
        "string_algo_lcp_array"
    }
    fn name(&self) -> &str {
        "LCP Array from Suffix Array"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Build the LCP (Longest Common Prefix) array from the suffix array.\n\
         lcp[i] = length of longest common prefix between suffix sa[i] and sa[i+1].\n\n\
         Input: String\n\
         Output: Vec<usize> (length n-1)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(3..=15);
                let s = crate::problems::helpers::random_string_from(&mut rng, len, b"abcd");
                TestCase {
                    data: Box::new(LcpArrayTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LcpArrayTest>().unwrap();
        let expected = ref_lcp_array(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::lcp_array(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 3: Distinct Substrings ──────────────────────────────────────

struct StringAlgoDistinctSubstrings;

struct DistinctSubTest {
    s: String,
}

impl Problem for StringAlgoDistinctSubstrings {
    fn id(&self) -> &str {
        "string_algo_distinct_substrings"
    }
    fn name(&self) -> &str {
        "Count Distinct Substrings"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Count the number of distinct non-empty substrings of a string.\n\
         Use suffix array + LCP array for efficient computation.\n\n\
         Input: String\n\
         Output: i64"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(3..=12);
                let s = crate::problems::helpers::random_string_from(&mut rng, len, b"abc");
                TestCase {
                    data: Box::new(DistinctSubTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DistinctSubTest>().unwrap();
        let expected = ref_distinct_substrings(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::distinct_substrings(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 4: Palindrome Partitioning (Min Cuts) ───────────────────────

struct StringAlgoPalindromePartitioningMin;

struct PalPartTest {
    s: String,
}

impl Problem for StringAlgoPalindromePartitioningMin {
    fn id(&self) -> &str {
        "string_algo_palindrome_partitioning_min"
    }
    fn name(&self) -> &str {
        "Min Cuts for Palindrome Partitioning"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find the minimum number of cuts to partition the string so every part is a palindrome.\n\n\
         Input: String\n\
         Output: i32"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(2..=12);
                let s = crate::problems::helpers::random_string_from(&mut rng, len, b"abcde");
                TestCase {
                    data: Box::new(PalPartTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PalPartTest>().unwrap();
        let expected = ref_palindrome_partitioning_min(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::palindrome_partitioning_min(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 5: Longest Common Substring ─────────────────────────────────

struct StringAlgoLongestCommonSubstring;

struct LcsTest {
    s1: String,
    s2: String,
}

impl Problem for StringAlgoLongestCommonSubstring {
    fn id(&self) -> &str {
        "string_algo_longest_common_substring"
    }
    fn name(&self) -> &str {
        "Longest Common Substring"
    }
    fn topic(&self) -> &str {
        "string_algorithms"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Find the longest substring that appears in both s1 and s2.\n\
         Return empty string if none.\n\n\
         Input: (s1: String, s2: String)\n\
         Output: String"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len1 = rng.random_range(3..=15);
                let len2 = rng.random_range(3..=15);
                let s1 = crate::problems::helpers::random_string_from(&mut rng, len1, b"abcd");
                let s2 = crate::problems::helpers::random_string_from(&mut rng, len2, b"abcd");
                TestCase {
                    data: Box::new(LcsTest { s1, s2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LcsTest>().unwrap();
        let expected = ref_longest_common_substring(&t.s1, &t.s2);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_s1 = track_string(&t.s1, shared_log.clone());
        let tracked_s2 = track_string(&t.s2, shared_log.clone());
        let actual = solutions::longest_common_substring(&tracked_s1, &tracked_s2);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        // Accept any answer of the same length that is a substring of both
        let valid = actual.len() == expected.len()
            && (actual.is_empty() || (t.s1.contains(&actual) && t.s2.contains(&actual)));
        SolutionResult {
            is_correct: valid,
            input_description: format!("s1=\"{}\", s2=\"{}\"", t.s1, t.s2),
            expected: format!("\"{}\" (len={})", expected, expected.len()),
            actual: format!("\"{}\" (len={})", actual, actual.len()),
        }
    }
}
