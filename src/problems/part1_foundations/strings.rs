use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part1_foundations::strings as solutions;
use crate::tracker::{track_string, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(ReverseString),
        Box::new(ValidPalindrome),
        Box::new(IsAnagram),
        Box::new(FirstUniqueChar),
        Box::new(LongestCommonPrefix),
        Box::new(LongestPalindromicSubstring),
        Box::new(GroupAnagrams),
        Box::new(StringToInteger),
        Box::new(ZigzagConversion),
        Box::new(CountAndSay),
        Box::new(LongestSubstringNoRepeat),
        Box::new(MinimumWindowSubstring),
        Box::new(RegexMatching),
        Box::new(EditDistance),
        Box::new(WildcardMatching),
    ]
}

// ── Helper: generate a random lowercase string ─────────────────────────

fn random_lower(rng: &mut impl Rng, len: usize) -> String {
    (0..len)
        .map(|_| (b'a' + rng.random_range(0..26u8)) as char)
        .collect()
}

fn random_alpha_mixed(rng: &mut impl Rng, len: usize) -> String {
    (0..len)
        .map(|_| {
            let kind = rng.random_range(0..4u8);
            match kind {
                0 => (b'a' + rng.random_range(0..26u8)) as char,
                1 => (b'A' + rng.random_range(0..26u8)) as char,
                2 => (b'0' + rng.random_range(0..10u8)) as char,
                _ => {
                    let specials = b" ,.!@#";
                    specials[rng.random_range(0..specials.len())] as char
                }
            }
        })
        .collect()
}

// ── Easy 1: Reverse String ─────────────────────────────────────────────

struct ReverseString;
struct ReverseTest {
    s: String,
}

impl Problem for ReverseString {
    fn id(&self) -> &str {
        "strings_reverse"
    }
    fn name(&self) -> &str {
        "Reverse String"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a string `s`, return the string reversed.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 30\n\
         - `s` consists of lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=30);
                let s = random_lower(&mut rng, len);
                TestCase {
                    data: Box::new(ReverseTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReverseTest>().unwrap();
        let expected: String = t.s.chars().rev().collect();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::reverse(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}", t.s),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 2: Valid Palindrome ───────────────────────────────────────────

struct ValidPalindrome;
struct ValidPalindromeTest {
    s: String,
}

impl Problem for ValidPalindrome {
    fn id(&self) -> &str {
        "strings_valid_palindrome"
    }
    fn name(&self) -> &str {
        "Valid Palindrome"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a string `s`, return `true` if it is a palindrome considering only \
         alphanumeric characters and ignoring case.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 50\n\
         - `s` may contain letters, digits, spaces, and punctuation."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|i| {
                let s = if i < 5 {
                    // Generate a palindrome ~50% of the time
                    let half_len = rng.random_range(1..=10);
                    let half: String = (0..half_len)
                        .map(|_| (b'a' + rng.random_range(0..26u8)) as char)
                        .collect();
                    let rev: String = half.chars().rev().collect();
                    if rng.random_range(0..2u8) == 0 {
                        // Even palindrome
                        format!("{half}{rev}")
                    } else {
                        // Odd palindrome with a middle char
                        let mid = (b'a' + rng.random_range(0..26u8)) as char;
                        format!("{half}{mid}{rev}")
                    }
                } else {
                    // Random mixed string (likely not a palindrome)
                    let len = rng.random_range(1..=30);
                    random_alpha_mixed(&mut rng, len)
                };
                TestCase {
                    data: Box::new(ValidPalindromeTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ValidPalindromeTest>().unwrap();
        let expected = ref_is_palindrome(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::is_palindrome(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_is_palindrome(s: &str) -> bool {
    let filtered: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let rev: Vec<char> = filtered.iter().copied().rev().collect();
    filtered == rev
}

// ── Easy 3: Is Anagram ─────────────────────────────────────────────────

struct IsAnagram;
struct IsAnagramTest {
    s: String,
    t: String,
}

impl Problem for IsAnagram {
    fn id(&self) -> &str {
        "strings_is_anagram"
    }
    fn name(&self) -> &str {
        "Valid Anagram"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`.\n\n\
         Constraints:\n\
         - 1 <= s.len(), t.len() <= 50\n\
         - `s` and `t` consist of lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=20);
                let s = random_lower(&mut rng, len);
                let t = if rng.random_range(0..2u8) == 0 {
                    // Make an anagram by shuffling
                    let mut chars: Vec<char> = s.chars().collect();
                    for i in (1..chars.len()).rev() {
                        let j = rng.random_range(0..=i);
                        chars.swap(i, j);
                    }
                    chars.into_iter().collect()
                } else {
                    // Random string (probably not an anagram)
                    let t_len = rng.random_range(1..=20);
                    random_lower(&mut rng, t_len)
                };
                TestCase {
                    data: Box::new(IsAnagramTest { s, t }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IsAnagramTest>().unwrap();
        let expected = ref_is_anagram(&t.s, &t.t);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_s = track_string(&t.s, shared_log.clone());
        let tracked_t = track_string(&t.t, shared_log.clone());
        let actual = solutions::is_anagram(&tracked_s, &tracked_t);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}, t={:?}", t.s, t.t),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_is_anagram(s: &str, t: &str) -> bool {
    let mut s_sorted: Vec<char> = s.chars().collect();
    let mut t_sorted: Vec<char> = t.chars().collect();
    s_sorted.sort();
    t_sorted.sort();
    s_sorted == t_sorted
}

// ── Easy 4: First Unique Character ─────────────────────────────────────

struct FirstUniqueChar;
struct FirstUniqueCharTest {
    s: String,
}

impl Problem for FirstUniqueChar {
    fn id(&self) -> &str {
        "strings_first_unique_char"
    }
    fn name(&self) -> &str {
        "First Unique Character in a String"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a string `s` consisting of lowercase English letters, return the index \
         of the first non-repeating character. If no such character exists, return -1.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 50"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=30);
                // Use a small alphabet to increase duplicate chance
                let s: String = (0..len)
                    .map(|_| (b'a' + rng.random_range(0..8u8)) as char)
                    .collect();
                TestCase {
                    data: Box::new(FirstUniqueCharTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FirstUniqueCharTest>().unwrap();
        let expected = ref_first_unique_char(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::first_unique_char(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_first_unique_char(s: &str) -> i32 {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    for (i, c) in s.chars().enumerate() {
        if counts[&c] == 1 {
            return i as i32;
        }
    }
    -1
}

// ── Easy 5: Longest Common Prefix ──────────────────────────────────────

struct LongestCommonPrefix;
struct LCPTest {
    strs: Vec<String>,
}

impl Problem for LongestCommonPrefix {
    fn id(&self) -> &str {
        "strings_longest_common_prefix"
    }
    fn name(&self) -> &str {
        "Longest Common Prefix"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array of strings `strs`, return the longest common prefix.\n\
         If there is no common prefix, return an empty string.\n\n\
         Constraints:\n\
         - 1 <= strs.len() <= 20\n\
         - 0 <= strs[i].len() <= 30\n\
         - `strs[i]` consists of lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let count = rng.random_range(1..=10);
                let prefix_len = rng.random_range(0..=5);
                let prefix = random_lower(&mut rng, prefix_len);
                let strs: Vec<String> = (0..count)
                    .map(|_| {
                        let suffix_len = rng.random_range(0..=10);
                        let suffix = random_lower(&mut rng, suffix_len);
                        format!("{prefix}{suffix}")
                    })
                    .collect();
                TestCase {
                    data: Box::new(LCPTest { strs }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LCPTest>().unwrap();
        let expected = ref_longest_common_prefix(&t.strs);
        let actual = solutions::longest_common_prefix(&t.strs);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("strs={:?}", t.strs),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let first = strs[0].as_bytes();
    let mut prefix_len = first.len();
    for s in &strs[1..] {
        prefix_len = prefix_len.min(s.len());
        for (i, &byte) in first.iter().enumerate().take(prefix_len) {
            if s.as_bytes()[i] != byte {
                prefix_len = i;
                break;
            }
        }
    }
    strs[0][..prefix_len].to_string()
}

// ── Medium 1: Longest Palindromic Substring ────────────────────────────

struct LongestPalindromicSubstring;
struct LPSTest {
    s: String,
}

impl Problem for LongestPalindromicSubstring {
    fn id(&self) -> &str {
        "strings_longest_palindromic_substring"
    }
    fn name(&self) -> &str {
        "Longest Palindromic Substring"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a string `s`, return the longest palindromic substring in `s`.\n\
         If there are multiple answers of the same length, return any one.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 100\n\
         - `s` consists of lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                // Use small alphabet to increase palindrome chance
                let len = rng.random_range(1..=40);
                let s: String = (0..len)
                    .map(|_| (b'a' + rng.random_range(0..5u8)) as char)
                    .collect();
                TestCase {
                    data: Box::new(LPSTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LPSTest>().unwrap();
        let expected = ref_longest_palindromic_substring(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::longest_palindromic_substring(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }

        // Validate: actual must be a palindrome and same length as expected
        let actual_chars: Vec<char> = actual.chars().collect();
        let is_palindrome = actual_chars
            .iter()
            .copied()
            .eq(actual_chars.iter().copied().rev());
        let is_substring = t.s.contains(&actual);
        let correct = is_palindrome && is_substring && actual.len() == expected.len();

        SolutionResult {
            is_correct: correct,
            input_description: format!("s={:?}", t.s),
            expected: format!("{expected:?} (len={})", expected.len()),
            actual: format!("{actual:?} (len={})", actual.len()),
        }
    }
}

fn ref_longest_palindromic_substring(s: &str) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n == 0 {
        return String::new();
    }
    let mut best_start = 0;
    let mut best_len = 1;

    let expand = |mut l: usize, mut r: usize| -> (usize, usize) {
        while l < n && r < n && bytes[l] == bytes[r] {
            if l == 0 {
                return (0, r + 1);
            }
            l -= 1;
            r += 1;
        }
        (l + 1, r - l - 1)
    };

    for i in 0..n {
        // Odd-length palindrome centered at i
        let (start, len) = expand(i, i);
        if len > best_len {
            best_start = start;
            best_len = len;
        }
        // Even-length palindrome centered between i and i+1
        if i + 1 < n {
            let (start, len) = expand(i, i + 1);
            if len > best_len {
                best_start = start;
                best_len = len;
            }
        }
    }
    s[best_start..best_start + best_len].to_string()
}

// ── Medium 2: Group Anagrams ───────────────────────────────────────────

struct GroupAnagrams;
struct GroupAnagramsTest {
    strs: Vec<String>,
}

impl Problem for GroupAnagrams {
    fn id(&self) -> &str {
        "strings_group_anagrams"
    }
    fn name(&self) -> &str {
        "Group Anagrams"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array of strings `strs`, group the anagrams together.\n\
         Return the groups where each group is sorted alphabetically, and \
         the groups themselves are sorted by their first element.\n\n\
         Constraints:\n\
         - 1 <= strs.len() <= 50\n\
         - 0 <= strs[i].len() <= 20\n\
         - `strs[i]` consists of lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let count = rng.random_range(1..=20);
                let strs: Vec<String> = (0..count)
                    .map(|_| {
                        let len = rng.random_range(1..=8);
                        // Small alphabet to create anagram groups
                        (0..len)
                            .map(|_| (b'a' + rng.random_range(0..4u8)) as char)
                            .collect()
                    })
                    .collect();
                TestCase {
                    data: Box::new(GroupAnagramsTest { strs }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<GroupAnagramsTest>().unwrap();
        let expected = ref_group_anagrams(&t.strs);
        let actual = solutions::group_anagrams(&t.strs);

        // Normalize for comparison: sort each group, then sort groups by first element
        let normalize = |mut groups: Vec<Vec<String>>| -> Vec<Vec<String>> {
            for g in &mut groups {
                g.sort();
            }
            groups.sort_by(|a, b| a.first().cmp(&b.first()));
            groups
        };
        let exp_norm = normalize(expected);
        let act_norm = normalize(actual);

        SolutionResult {
            is_correct: exp_norm == act_norm,
            input_description: format!("strs={:?}", t.strs),
            expected: format!("{exp_norm:?}"),
            actual: format!("{act_norm:?}"),
        }
    }
}

fn ref_group_anagrams(strs: &[String]) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let mut key: Vec<char> = s.chars().collect();
        key.sort();
        let key: String = key.into_iter().collect();
        map.entry(key).or_default().push(s.clone());
    }
    let mut groups: Vec<Vec<String>> = map.into_values().collect();
    for g in &mut groups {
        g.sort();
    }
    groups.sort_by(|a, b| a.first().cmp(&b.first()));
    groups
}

// ── Medium 3: String to Integer (atoi) ─────────────────────────────────

struct StringToInteger;
struct AtoiTest {
    s: String,
}

impl Problem for StringToInteger {
    fn id(&self) -> &str {
        "strings_string_to_integer"
    }
    fn name(&self) -> &str {
        "String to Integer (atoi)"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Implement `atoi` which converts a string to a 32-bit signed integer.\n\n\
         Rules:\n\
         1. Discard leading whitespace.\n\
         2. Check for optional '+' or '-' sign.\n\
         3. Read digits until a non-digit or end of string.\n\
         4. Clamp to [i32::MIN, i32::MAX] on overflow.\n\
         5. Return 0 if no digits are read.\n\n\
         Constraints:\n\
         - 0 <= s.len() <= 50"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests = Vec::new();
        for i in 0..10 {
            let s = match i {
                0 => "42".to_string(),
                1 => "   -42".to_string(),
                2 => "4193 with words".to_string(),
                3 => "words and 987".to_string(),
                4 => "".to_string(),
                5 => "-91283472332".to_string(),
                6 => "+1".to_string(),
                _ => {
                    // Random: optional whitespace + optional sign + digits + optional junk
                    let ws_len = rng.random_range(0..=3);
                    let ws: String = " ".repeat(ws_len);
                    let sign = match rng.random_range(0..3u8) {
                        0 => "+",
                        1 => "-",
                        _ => "",
                    };
                    let digit_len = rng.random_range(0..=10);
                    let digits: String = (0..digit_len)
                        .map(|_| (b'0' + rng.random_range(0..10u8)) as char)
                        .collect();
                    let junk_len = rng.random_range(0..=5);
                    let junk = random_lower(&mut rng, junk_len);
                    format!("{ws}{sign}{digits}{junk}")
                }
            };
            tests.push(TestCase {
                data: Box::new(AtoiTest { s }),
            });
        }
        tests
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AtoiTest>().unwrap();
        let expected = ref_string_to_integer(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::string_to_integer(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_string_to_integer(s: &str) -> i32 {
    let s = s.trim_start();
    if s.is_empty() {
        return 0;
    }
    let mut chars = s.chars().peekable();
    let sign: i64 = match chars.peek() {
        Some('+') => {
            chars.next();
            1
        }
        Some('-') => {
            chars.next();
            -1
        }
        _ => 1,
    };
    let mut result: i64 = 0;
    for c in chars {
        if !c.is_ascii_digit() {
            break;
        }
        result = result * 10 + (c as i64 - '0' as i64);
        if sign * result > i32::MAX as i64 {
            return i32::MAX;
        }
        if sign * result < i32::MIN as i64 {
            return i32::MIN;
        }
    }
    (sign * result) as i32
}

// ── Medium 4: Zigzag Conversion ────────────────────────────────────────

struct ZigzagConversion;
struct ZigzagTest {
    s: String,
    num_rows: usize,
}

impl Problem for ZigzagConversion {
    fn id(&self) -> &str {
        "strings_zigzag_conversion"
    }
    fn name(&self) -> &str {
        "Zigzag Conversion"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "The string is written in a zigzag pattern on a given number of rows, \
         then read line by line.\n\n\
         Example: \"PAYPALISHIRING\" with 3 rows:\n\
         P   A   H   N\n\
         A P L S I I G\n\
         Y   I   R\n\
         Result: \"PAHNAPLSIIGYIR\"\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 100\n\
         - 1 <= num_rows <= 10"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(1..=50);
                let s = random_lower(&mut rng, len);
                let num_rows = rng.random_range(1..=6);
                TestCase {
                    data: Box::new(ZigzagTest { s, num_rows }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ZigzagTest>().unwrap();
        let expected = ref_zigzag_convert(&t.s, t.num_rows);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::zigzag_convert(&tracked, t.num_rows);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}, num_rows={}", t.s, t.num_rows),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_zigzag_convert(s: &str, num_rows: usize) -> String {
    if num_rows <= 1 || num_rows >= s.len() {
        return s.to_string();
    }
    let mut rows: Vec<String> = vec![String::new(); num_rows];
    let mut cur_row: usize = 0;
    let mut going_down = false;

    for c in s.chars() {
        rows[cur_row].push(c);
        if cur_row == 0 || cur_row == num_rows - 1 {
            going_down = !going_down;
        }
        if going_down {
            cur_row += 1;
        } else {
            cur_row -= 1;
        }
    }
    rows.concat()
}

// ── Medium 5: Count and Say ────────────────────────────────────────────

struct CountAndSay;
struct CountAndSayTest {
    n: usize,
}

impl Problem for CountAndSay {
    fn id(&self) -> &str {
        "strings_count_and_say"
    }
    fn name(&self) -> &str {
        "Count and Say"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "The count-and-say sequence is defined as follows:\n\
         - Term 1: \"1\"\n\
         - Each subsequent term describes the previous term by counting consecutive \
           identical digits.\n\n\
         Example: 1 -> 11 -> 21 -> 1211 -> 111221 -> ...\n\n\
         Given `n` (1-indexed), return the nth term.\n\n\
         Constraints:\n\
         - 1 <= n <= 20"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=15);
                TestCase {
                    data: Box::new(CountAndSayTest { n }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountAndSayTest>().unwrap();
        let expected = ref_count_and_say(t.n);
        let actual = solutions::count_and_say(t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("n={}", t.n),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_count_and_say(n: usize) -> String {
    let mut result = "1".to_string();
    for _ in 1..n {
        let chars: Vec<char> = result.chars().collect();
        let mut next = String::new();
        let mut i = 0;
        while i < chars.len() {
            let ch = chars[i];
            let mut count = 1;
            while i + count < chars.len() && chars[i + count] == ch {
                count += 1;
            }
            next.push_str(&count.to_string());
            next.push(ch);
            i += count;
        }
        result = next;
    }
    result
}

// ── Hard 1: Longest Substring Without Repeating Characters ─────────────

struct LongestSubstringNoRepeat;
struct LSNRTest {
    s: String,
}

impl Problem for LongestSubstringNoRepeat {
    fn id(&self) -> &str {
        "strings_longest_substring_no_repeat"
    }
    fn name(&self) -> &str {
        "Longest Substring Without Repeating Characters"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a string `s`, find the length of the longest substring without \
         repeating characters.\n\n\
         Constraints:\n\
         - 0 <= s.len() <= 200\n\
         - `s` consists of English letters, digits, symbols, and spaces."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len = rng.random_range(0..=50);
                // Small alphabet to force repeats
                let s: String = (0..len)
                    .map(|_| (b'a' + rng.random_range(0..10u8)) as char)
                    .collect();
                TestCase {
                    data: Box::new(LSNRTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LSNRTest>().unwrap();
        let expected = ref_longest_substring_no_repeat(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::longest_substring_no_repeat(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_longest_substring_no_repeat(s: &str) -> i32 {
    use std::collections::HashSet;
    let chars: Vec<char> = s.chars().collect();
    let mut set = HashSet::new();
    let mut best = 0i32;
    let mut left = 0;
    for right in 0..chars.len() {
        while set.contains(&chars[right]) {
            set.remove(&chars[left]);
            left += 1;
        }
        set.insert(chars[right]);
        best = best.max((right - left + 1) as i32);
    }
    best
}

// ── Hard 2: Minimum Window Substring ───────────────────────────────────

struct MinimumWindowSubstring;
struct MWSTest {
    s: String,
    t: String,
}

impl Problem for MinimumWindowSubstring {
    fn id(&self) -> &str {
        "strings_minimum_window_substring"
    }
    fn name(&self) -> &str {
        "Minimum Window Substring"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given two strings `s` and `t`, return the minimum window substring of `s` \
         such that every character in `t` (including duplicates) is included in the window. \
         If there is no such window, return the empty string.\n\n\
         Constraints:\n\
         - 1 <= s.len(), t.len() <= 200\n\
         - `s` and `t` consist of uppercase and lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let s_len = rng.random_range(5..=40);
                let s: String = (0..s_len)
                    .map(|_| (b'a' + rng.random_range(0..6u8)) as char)
                    .collect();
                // t is a subset of characters from s (usually)
                let t_len = rng.random_range(1..=s_len.min(10));
                let s_chars: Vec<char> = s.chars().collect();
                let t: String = (0..t_len)
                    .map(|_| {
                        if rng.random_range(0..4u8) == 0 {
                            // Occasionally pick a char not in s
                            (b'a' + rng.random_range(0..8u8)) as char
                        } else {
                            s_chars[rng.random_range(0..s_chars.len())]
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(MWSTest { s, t }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MWSTest>().unwrap();
        let expected = ref_min_window(&t.s, &t.t);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_s = track_string(&t.s, shared_log.clone());
        let tracked_t = track_string(&t.t, shared_log.clone());
        let actual = solutions::min_window(&tracked_s, &tracked_t);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}, t={:?}", t.s, t.t),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_min_window(s: &str, t: &str) -> String {
    if t.is_empty() || s.len() < t.len() {
        return String::new();
    }
    let mut need: HashMap<char, i32> = HashMap::new();
    for c in t.chars() {
        *need.entry(c).or_insert(0) += 1;
    }
    let required = need.len();
    let mut formed = 0;
    let mut window_counts: HashMap<char, i32> = HashMap::new();
    let s_chars: Vec<char> = s.chars().collect();

    let mut best_len = usize::MAX;
    let mut best_start = 0;
    let mut left = 0;

    for right in 0..s_chars.len() {
        let c = s_chars[right];
        *window_counts.entry(c).or_insert(0) += 1;
        if let Some(&needed) = need.get(&c) {
            if window_counts[&c] == needed {
                formed += 1;
            }
        }
        while formed == required {
            let window_size = right - left + 1;
            if window_size < best_len {
                best_len = window_size;
                best_start = left;
            }
            let lc = s_chars[left];
            *window_counts.get_mut(&lc).unwrap() -= 1;
            if let Some(&needed) = need.get(&lc) {
                if window_counts[&lc] < needed {
                    formed -= 1;
                }
            }
            left += 1;
        }
    }
    if best_len == usize::MAX {
        String::new()
    } else {
        s_chars[best_start..best_start + best_len].iter().collect()
    }
}

// ── Hard 3: Regex Matching ─────────────────────────────────────────────

struct RegexMatching;
struct RegexTest {
    s: String,
    p: String,
}

impl Problem for RegexMatching {
    fn id(&self) -> &str {
        "strings_regex_matching"
    }
    fn name(&self) -> &str {
        "Regular Expression Matching"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Implement regular expression matching with support for '.' and '*'.\n\n\
         - '.' matches any single character.\n\
         - '*' matches zero or more of the preceding element.\n\
         - The matching must cover the entire input string (not partial).\n\n\
         Constraints:\n\
         - 0 <= s.len() <= 30\n\
         - 0 <= p.len() <= 30\n\
         - `s` contains only lowercase English letters.\n\
         - `p` contains only lowercase English letters, '.', and '*'.\n\
         - Each '*' has a valid preceding character to repeat."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests = Vec::new();
        // A few fixed edge cases
        let fixed = vec![
            ("aa", "a"),
            ("aa", "a*"),
            ("ab", ".*"),
            ("", ".*"),
            ("", ""),
            ("abc", "a.c"),
            ("aab", "c*a*b"),
        ];
        for (s, p) in &fixed {
            tests.push(TestCase {
                data: Box::new(RegexTest {
                    s: s.to_string(),
                    p: p.to_string(),
                }),
            });
        }
        // Random tests
        while tests.len() < 10 {
            let s_len = rng.random_range(0..=10);
            let s: String = (0..s_len)
                .map(|_| (b'a' + rng.random_range(0..3u8)) as char)
                .collect();
            // Build a pattern: sequence of (char or '.') optionally followed by '*'
            let parts = rng.random_range(0..=5);
            let mut p = String::new();
            for _ in 0..parts {
                let ch = match rng.random_range(0..4u8) {
                    0 => '.',
                    _ => (b'a' + rng.random_range(0..3u8)) as char,
                };
                p.push(ch);
                if rng.random_range(0..3u8) == 0 {
                    p.push('*');
                }
            }
            tests.push(TestCase {
                data: Box::new(RegexTest { s, p }),
            });
        }
        tests
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RegexTest>().unwrap();
        let expected = ref_is_match_regex(&t.s, &t.p);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_s = track_string(&t.s, shared_log.clone());
        let tracked_p = track_string(&t.p, shared_log.clone());
        let actual = solutions::is_match_regex(&tracked_s, &tracked_p);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}, p={:?}", t.s, t.p),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_is_match_regex(s: &str, p: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let m = s.len();
    let n = p.len();
    // dp[i][j] = does s[0..i] match p[0..j]?
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;
    // Handle patterns like a*, a*b*, .* matching empty string
    for j in 1..=n {
        if p[j - 1] == '*' && j >= 2 {
            dp[0][j] = dp[0][j - 2];
        }
    }
    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                // Zero occurrences of the preceding element
                dp[i][j] = j >= 2 && dp[i][j - 2];
                // One or more occurrences
                if j >= 2 && (p[j - 2] == '.' || p[j - 2] == s[i - 1]) {
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
                }
            } else if p[j - 1] == '.' || p[j - 1] == s[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    dp[m][n]
}

// ── Hard 4: Edit Distance ──────────────────────────────────────────────

struct EditDistance;
struct EditDistanceTest {
    word1: String,
    word2: String,
}

impl Problem for EditDistance {
    fn id(&self) -> &str {
        "strings_edit_distance"
    }
    fn name(&self) -> &str {
        "Edit Distance"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given two strings `word1` and `word2`, return the minimum number of operations \
         required to convert `word1` into `word2`.\n\n\
         Allowed operations: insert a character, delete a character, replace a character.\n\n\
         Constraints:\n\
         - 0 <= word1.len(), word2.len() <= 100\n\
         - Both consist of lowercase English letters."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let len1 = rng.random_range(0..=15);
                let len2 = rng.random_range(0..=15);
                let word1 = random_lower(&mut rng, len1);
                let word2 = random_lower(&mut rng, len2);
                TestCase {
                    data: Box::new(EditDistanceTest { word1, word2 }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<EditDistanceTest>().unwrap();
        let expected = ref_edit_distance(&t.word1, &t.word2);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_w1 = track_string(&t.word1, shared_log.clone());
        let tracked_w2 = track_string(&t.word2, shared_log.clone());
        let actual = solutions::edit_distance(&tracked_w1, &tracked_w2);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("word1={:?}, word2={:?}", t.word1, t.word2),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_edit_distance(word1: &str, word2: &str) -> i32 {
    let w1: Vec<char> = word1.chars().collect();
    let w2: Vec<char> = word2.chars().collect();
    let m = w1.len();
    let n = w2.len();
    let mut dp = vec![vec![0i32; n + 1]; m + 1];
    for (i, row) in dp.iter_mut().enumerate().take(m + 1) {
        row[0] = i as i32;
    }
    for j in 0..=n {
        dp[0][j] = j as i32;
    }
    for i in 1..=m {
        for j in 1..=n {
            if w1[i - 1] == w2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}

// ── Hard 5: Wildcard Matching ──────────────────────────────────────────

struct WildcardMatching;
struct WildcardTest {
    s: String,
    p: String,
}

impl Problem for WildcardMatching {
    fn id(&self) -> &str {
        "strings_wildcard_matching"
    }
    fn name(&self) -> &str {
        "Wildcard Matching"
    }
    fn topic(&self) -> &str {
        "strings"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Implement wildcard pattern matching with support for '?' and '*'.\n\n\
         - '?' matches any single character.\n\
         - '*' matches any sequence of characters (including the empty sequence).\n\
         - The matching must cover the entire input string (not partial).\n\n\
         Constraints:\n\
         - 0 <= s.len() <= 50\n\
         - 0 <= p.len() <= 50\n\
         - `s` contains only lowercase English letters.\n\
         - `p` contains only lowercase English letters, '?', and '*'."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests = Vec::new();
        // Fixed edge cases
        let fixed = vec![
            ("aa", "a"),
            ("aa", "*"),
            ("cb", "?a"),
            ("", ""),
            ("", "*"),
            ("abc", "a?c"),
            ("adceb", "*a*b"),
        ];
        for (s, p) in &fixed {
            tests.push(TestCase {
                data: Box::new(WildcardTest {
                    s: s.to_string(),
                    p: p.to_string(),
                }),
            });
        }
        // Random tests
        while tests.len() < 10 {
            let s_len = rng.random_range(0..=12);
            let s: String = (0..s_len)
                .map(|_| (b'a' + rng.random_range(0..4u8)) as char)
                .collect();
            let p_len = rng.random_range(0..=12);
            let p: String = (0..p_len)
                .map(|_| match rng.random_range(0..6u8) {
                    0 => '?',
                    1 => '*',
                    _ => (b'a' + rng.random_range(0..4u8)) as char,
                })
                .collect();
            tests.push(TestCase {
                data: Box::new(WildcardTest { s, p }),
            });
        }
        tests
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WildcardTest>().unwrap();
        let expected = ref_is_match_wildcard(&t.s, &t.p);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked_s = track_string(&t.s, shared_log.clone());
        let tracked_p = track_string(&t.p, shared_log.clone());
        let actual = solutions::is_match_wildcard(&tracked_s, &tracked_p);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}, p={:?}", t.s, t.p),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_is_match_wildcard(s: &str, p: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let m = s.len();
    let n = p.len();
    // dp[i][j] = does s[0..i] match p[0..j]?
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;
    // '*' can match empty sequence
    for j in 1..=n {
        if p[j - 1] == '*' {
            dp[0][j] = dp[0][j - 1];
        } else {
            break;
        }
    }
    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                // '*' matches empty (dp[i][j-1]) or one more char (dp[i-1][j])
                dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
            } else if p[j - 1] == '?' || p[j - 1] == s[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    dp[m][n]
}
