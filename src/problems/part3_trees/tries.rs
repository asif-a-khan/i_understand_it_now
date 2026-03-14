use rand::Rng;
use std::collections::{HashMap, HashSet};

use crate::problems::helpers;
use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part3_trees::tries as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(ImplementTrie),
        Box::new(SearchWord),
        Box::new(LongestCommonPrefix),
        Box::new(CountPrefix),
        Box::new(AutoComplete),
        Box::new(WordSearchII),
        Box::new(ReplaceWords),
        Box::new(MapSum),
        Box::new(SearchSuggestions),
        Box::new(AddSearchWord),
        Box::new(PalindromePairs),
        Box::new(WordBreakII),
        Box::new(ConcatenatedWords),
        Box::new(MaximumXor),
        Box::new(StreamOfCharacters),
    ]
}

// ── helpers ──────────────────────────────────────────────────────────────

fn random_word(rng: &mut impl Rng, max_len: usize) -> String {
    let len = rng.random_range(1..=max_len);
    helpers::random_string(rng, len)
}

fn random_word_list(rng: &mut impl Rng, count: usize, max_len: usize) -> Vec<String> {
    (0..count).map(|_| random_word(rng, max_len)).collect()
}

/// Simple trie for reference solutions.
struct RefTrie {
    children: HashMap<char, RefTrie>,
    is_end: bool,
    count: usize, // number of words passing through this node
}

impl RefTrie {
    fn new() -> Self {
        Self { children: HashMap::new(), is_end: false, count: 0 }
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(RefTrie::new);
            node.count += 1;
        }
        node.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return false,
            }
        }
        true
    }

    #[allow(dead_code)]
    fn count_prefix(&self, prefix: &str) -> i32 {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return 0,
            }
        }
        node.count as i32
    }

    #[allow(dead_code)]
    fn collect_words(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        self.collect_helper(prefix, &mut result);
        result
    }

    fn collect_helper(&self, current: &str, result: &mut Vec<String>) {
        if self.is_end {
            result.push(current.to_string());
        }
        let mut keys: Vec<char> = self.children.keys().copied().collect();
        keys.sort();
        for c in keys {
            let child = &self.children[&c];
            let next = format!("{current}{c}");
            child.collect_helper(&next, result);
        }
    }

    fn search_with_wildcard(&self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        self.wildcard_helper(&chars, 0)
    }

    fn wildcard_helper(&self, chars: &[char], idx: usize) -> bool {
        if idx == chars.len() {
            return self.is_end;
        }
        if chars[idx] == '.' {
            for child in self.children.values() {
                if child.wildcard_helper(chars, idx + 1) {
                    return true;
                }
            }
            false
        } else {
            match self.children.get(&chars[idx]) {
                Some(child) => child.wildcard_helper(chars, idx + 1),
                None => false,
            }
        }
    }
}

// ── Easy 1: Implement Trie ──────────────────────────────────────────────

struct ImplementTrie;
struct ImplementTrieTest {
    ops: Vec<(String, String)>, // (operation, argument)
}

impl Problem for ImplementTrie {
    fn id(&self) -> &str { "tries_implement_trie" }
    fn name(&self) -> &str { "Implement Trie" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Implement a trie that supports insert, search, and starts_with operations.\n\n\
         Input is a Vec of (operation, argument) pairs where operation is one of:\n\
         - \"insert\" — insert the word\n\
         - \"search\" — return true if the exact word exists\n\
         - \"starts_with\" — return true if any word has this prefix\n\n\
         Return a Vec<bool> with one entry per search/starts_with operation.\n\n\
         Constraints:\n\
         - 1 <= ops.len() <= 1000\n\
         - Words consist of lowercase English letters"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n_inserts = rng.random_range(1..=10);
            let n_queries = rng.random_range(1..=10);
            let mut ops = Vec::new();
            let words = random_word_list(&mut rng, n_inserts, 8);
            for w in &words {
                ops.push(("insert".to_string(), w.clone()));
            }
            for _ in 0..n_queries {
                let op = if rng.random_range(0..2) == 0 { "search" } else { "starts_with" };
                let word = if rng.random_range(0..2) == 0 {
                    words[rng.random_range(0..words.len())].clone()
                } else {
                    random_word(&mut rng, 8)
                };
                ops.push((op.to_string(), word));
            }
            TestCase { data: Box::new(ImplementTrieTest { ops }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ImplementTrieTest>().unwrap();
        let expected = ref_implement_trie(&t.ops);
        let actual = solutions::implement_trie(&t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("ops={:?}", t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_implement_trie(ops: &[(String, String)]) -> Vec<bool> {
    let mut trie = RefTrie::new();
    let mut result = Vec::new();
    for (op, arg) in ops {
        match op.as_str() {
            "insert" => { trie.insert(arg); }
            "search" => { result.push(trie.search(arg)); }
            "starts_with" => { result.push(trie.starts_with(arg)); }
            _ => {}
        }
    }
    result
}

// ── Easy 2: Search Word in Dictionary ───────────────────────────────────

struct SearchWord;
struct SearchWordTest { dict: Vec<String>, queries: Vec<String> }

impl Problem for SearchWord {
    fn id(&self) -> &str { "tries_search_word" }
    fn name(&self) -> &str { "Search Word in Dictionary" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a dictionary of words and a list of queries, for each query return whether \
         the word exists in the dictionary.\n\n\
         Use a trie for efficient lookup.\n\n\
         Constraints:\n\
         - 1 <= dict.len() <= 1000\n\
         - 1 <= queries.len() <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=15);
            let dict = random_word_list(&mut rng, n, 8);
            let q = rng.random_range(1..=10);
            let queries: Vec<String> = (0..q).map(|_| {
                if rng.random_range(0..2) == 0 {
                    dict[rng.random_range(0..dict.len())].clone()
                } else {
                    random_word(&mut rng, 8)
                }
            }).collect();
            TestCase { data: Box::new(SearchWordTest { dict, queries }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SearchWordTest>().unwrap();
        let set: HashSet<&String> = t.dict.iter().collect();
        let expected: Vec<bool> = t.queries.iter().map(|q| set.contains(q)).collect();
        let actual = solutions::search_word(&t.dict, &t.queries);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("dict={:?}, queries={:?}", t.dict, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 3: Longest Common Prefix ───────────────────────────────────────

struct LongestCommonPrefix;
struct LCPTest { words: Vec<String> }

impl Problem for LongestCommonPrefix {
    fn id(&self) -> &str { "tries_longest_common_prefix" }
    fn name(&self) -> &str { "Longest Common Prefix" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Find the longest common prefix string amongst an array of strings using a trie.\n\n\
         If there is no common prefix, return an empty string.\n\n\
         Constraints:\n\
         - 1 <= words.len() <= 200\n\
         - 0 <= words[i].len() <= 200\n\
         - words[i] consists of lowercase English letters"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=10);
            let prefix_len = rng.random_range(0..=4);
            let prefix = helpers::random_string(&mut rng, prefix_len);
            let words: Vec<String> = (0..n).map(|_| {
                let suffix_len = rng.random_range(0..=5);
                let suffix = helpers::random_string(&mut rng, suffix_len);
                format!("{prefix}{suffix}")
            }).collect();
            TestCase { data: Box::new(LCPTest { words }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LCPTest>().unwrap();
        let expected = ref_longest_common_prefix(&t.words);
        let actual = solutions::longest_common_prefix(&t.words);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("words={:?}", t.words),
            expected: format!("\"{}\"", expected),
            actual: format!("\"{}\"", actual),
        }
    }
}

fn ref_longest_common_prefix(words: &[String]) -> String {
    if words.is_empty() { return String::new(); }
    let first = &words[0];
    let mut prefix_len = first.len();
    for word in &words[1..] {
        prefix_len = prefix_len.min(word.len());
        for (i, (a, b)) in first.chars().zip(word.chars()).enumerate() {
            if a != b {
                prefix_len = prefix_len.min(i);
                break;
            }
        }
    }
    first[..prefix_len].to_string()
}

// ── Easy 4: Count Words with Prefix ─────────────────────────────────────

struct CountPrefix;
struct CountPrefixTest { words: Vec<String>, prefix: String }

impl Problem for CountPrefix {
    fn id(&self) -> &str { "tries_count_prefix" }
    fn name(&self) -> &str { "Count Words with Prefix" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a list of words and a prefix, count how many words start with the given prefix.\n\n\
         Constraints:\n\
         - 1 <= words.len() <= 1000\n\
         - 1 <= prefix.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            let words = random_word_list(&mut rng, n, 8);
            let prefix = if rng.random_range(0..3) == 0 {
                // Use prefix from existing word
                let w = &words[rng.random_range(0..words.len())];
                let plen = rng.random_range(1..=w.len());
                w[..plen].to_string()
            } else {
                let plen = rng.random_range(1..=3);
                helpers::random_string(&mut rng, plen)
            };
            TestCase { data: Box::new(CountPrefixTest { words, prefix }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountPrefixTest>().unwrap();
        let expected = t.words.iter().filter(|w| w.starts_with(&t.prefix)).count() as i32;
        let actual = solutions::count_prefix(&t.words, &t.prefix);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("words={:?}, prefix=\"{}\"", t.words, t.prefix),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 5: Auto Complete ───────────────────────────────────────────────

struct AutoComplete;
struct AutoCompleteTest { words: Vec<String>, prefix: String }

impl Problem for AutoComplete {
    fn id(&self) -> &str { "tries_auto_complete" }
    fn name(&self) -> &str { "Auto Complete" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a list of words and a prefix, return all words that start with the prefix, \
         sorted lexicographically.\n\n\
         Constraints:\n\
         - 1 <= words.len() <= 1000\n\
         - 0 <= prefix.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=15);
            let words = random_word_list(&mut rng, n, 6);
            let prefix = if rng.random_range(0..3) == 0 {
                let w = &words[rng.random_range(0..words.len())];
                let plen = rng.random_range(1..=w.len());
                w[..plen].to_string()
            } else {
                let plen = rng.random_range(1..=2);
                helpers::random_string(&mut rng, plen)
            };
            TestCase { data: Box::new(AutoCompleteTest { words, prefix }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AutoCompleteTest>().unwrap();
        let mut expected: Vec<String> = t.words.iter()
            .filter(|w| w.starts_with(&t.prefix))
            .cloned()
            .collect();
        expected.sort();
        expected.dedup();
        let actual = solutions::auto_complete(&t.words, &t.prefix);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("words={:?}, prefix=\"{}\"", t.words, t.prefix),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 1: Word Search II ────────────────────────────────────────────

struct WordSearchII;
struct WordSearchIITest { board: Vec<Vec<char>>, words: Vec<String> }

impl Problem for WordSearchII {
    fn id(&self) -> &str { "tries_word_search_ii" }
    fn name(&self) -> &str { "Word Search II" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an m x n board of characters and a list of words, return all words that can \
         be constructed from letters of sequentially adjacent cells (horizontally or vertically). \
         The same cell may not be used more than once in a word.\n\n\
         Return the found words sorted lexicographically.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 6\n\
         - 1 <= words.len() <= 100\n\
         - 1 <= word.len() <= 10"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(2..=4);
            let cols = rng.random_range(2..=4);
            let board: Vec<Vec<char>> = (0..rows)
                .map(|_| (0..cols).map(|_| (b'a' + rng.random_range(0..6u8)) as char).collect())
                .collect();
            let n_words = rng.random_range(1..=8);
            let words: Vec<String> = (0..n_words).map(|_| {
                let len = rng.random_range(1..=4);
                let w = helpers::random_string_from(&mut rng, len, b"abcdef");
                w
            }).collect();
            TestCase { data: Box::new(WordSearchIITest { board, words }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WordSearchIITest>().unwrap();
        let expected = ref_word_search_ii(&t.board, &t.words);
        let actual = solutions::word_search_ii(&t.board, &t.words);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("board={:?}, words={:?}", t.board, t.words),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_word_search_ii(board: &[Vec<char>], words: &[String]) -> Vec<String> {
    let rows = board.len();
    let cols = board[0].len();
    let mut found = HashSet::new();

    for word in words {
        if found.contains(word) { continue; }
        let chars: Vec<char> = word.chars().collect();
        'outer: for r in 0..rows {
            for c in 0..cols {
                let mut visited = vec![vec![false; cols]; rows];
                if dfs_word(board, &chars, 0, r, c, &mut visited) {
                    found.insert(word.clone());
                    break 'outer;
                }
            }
        }
    }
    let mut result: Vec<String> = found.into_iter().collect();
    result.sort();
    result
}

fn dfs_word(board: &[Vec<char>], chars: &[char], idx: usize, r: usize, c: usize, visited: &mut [Vec<bool>]) -> bool {
    if idx == chars.len() { return true; }
    if r >= board.len() || c >= board[0].len() || visited[r][c] || board[r][c] != chars[idx] {
        return false;
    }
    visited[r][c] = true;
    let dirs = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
    for (dr, dc) in dirs {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nc >= 0 {
            if dfs_word(board, chars, idx + 1, nr as usize, nc as usize, visited) {
                visited[r][c] = false;
                return true;
            }
        }
    }
    visited[r][c] = false;
    false
}

// ── Medium 2: Replace Words ─────────────────────────────────────────────

struct ReplaceWords;
struct ReplaceWordsTest { dict: Vec<String>, sentence: String }

impl Problem for ReplaceWords {
    fn id(&self) -> &str { "tries_replace_words" }
    fn name(&self) -> &str { "Replace Words" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a dictionary of root words and a sentence, replace each word in the sentence \
         with the shortest root that is a prefix of that word. If no root matches, keep the \
         original word.\n\n\
         Constraints:\n\
         - 1 <= dict.len() <= 1000\n\
         - sentence consists of lowercase letters and spaces"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n_roots = rng.random_range(1..=5);
            let dict: Vec<String> = (0..n_roots)
                .map(|_| {
                    let len = rng.random_range(1..=3);
                    helpers::random_string(&mut rng, len)
                })
                .collect();
            let n_words = rng.random_range(1..=8);
            let sentence_words: Vec<String> = (0..n_words).map(|_| {
                if rng.random_range(0..3) == 0 {
                    // Build word from root
                    let root = &dict[rng.random_range(0..dict.len())];
                    let slen = rng.random_range(0..=4);
                    let suffix = helpers::random_string(&mut rng, slen);
                    format!("{root}{suffix}")
                } else {
                    let wlen = rng.random_range(1..=6);
                    helpers::random_string(&mut rng, wlen)
                }
            }).collect();
            let sentence = sentence_words.join(" ");
            TestCase { data: Box::new(ReplaceWordsTest { dict, sentence }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReplaceWordsTest>().unwrap();
        let expected = ref_replace_words(&t.dict, &t.sentence);
        let actual = solutions::replace_words(&t.dict, &t.sentence);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("dict={:?}, sentence=\"{}\"", t.dict, t.sentence),
            expected: format!("\"{}\"", expected),
            actual: format!("\"{}\"", actual),
        }
    }
}

fn ref_replace_words(dict: &[String], sentence: &str) -> String {
    let mut trie = RefTrie::new();
    for root in dict { trie.insert(root); }

    sentence.split_whitespace().map(|word| {
        let mut node = &trie;
        for (i, c) in word.chars().enumerate() {
            match node.children.get(&c) {
                Some(child) => {
                    if child.is_end {
                        return word[..=i].to_string();
                    }
                    node = child;
                }
                None => break,
            }
        }
        word.to_string()
    }).collect::<Vec<_>>().join(" ")
}

// ── Medium 3: Map Sum Pairs ─────────────────────────────────────────────

struct MapSum;
struct MapSumTest {
    ops: Vec<(String, String, i32)>, // (op, key, value) -- "insert" or "sum"
}

impl Problem for MapSum {
    fn id(&self) -> &str { "tries_map_sum" }
    fn name(&self) -> &str { "Map Sum Pairs" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Implement a MapSum class:\n\
         - \"insert\" key value: insert or overwrite the key with value\n\
         - \"sum\" prefix: return sum of all values whose keys start with prefix\n\n\
         Input: Vec of (op, key, value). For \"sum\", key is the prefix and value is 0.\n\
         Return: Vec<i32> with one entry per \"sum\" operation.\n\n\
         Constraints:\n\
         - 1 <= ops.len() <= 100\n\
         - keys and prefixes are lowercase English letters"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n_inserts = rng.random_range(1..=8);
            let n_queries = rng.random_range(1..=5);
            let mut ops = Vec::new();
            let keys = random_word_list(&mut rng, n_inserts, 5);
            for k in &keys {
                ops.push(("insert".to_string(), k.clone(), rng.random_range(1..=100)));
            }
            for _ in 0..n_queries {
                let prefix = if rng.random_range(0..2) == 0 {
                    let k = &keys[rng.random_range(0..keys.len())];
                    let plen = rng.random_range(1..=k.len());
                    k[..plen].to_string()
                } else {
                    let plen = rng.random_range(1..=2);
                    helpers::random_string(&mut rng, plen)
                };
                ops.push(("sum".to_string(), prefix, 0));
            }
            TestCase { data: Box::new(MapSumTest { ops }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MapSumTest>().unwrap();
        let expected = ref_map_sum(&t.ops);
        let actual = solutions::map_sum(&t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("ops={:?}", t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_map_sum(ops: &[(String, String, i32)]) -> Vec<i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    let mut result = Vec::new();
    for (op, key, value) in ops {
        match op.as_str() {
            "insert" => { map.insert(key.clone(), *value); }
            "sum" => {
                let s: i32 = map.iter()
                    .filter(|(k, _)| k.starts_with(key))
                    .map(|(_, v)| v)
                    .sum();
                result.push(s);
            }
            _ => {}
        }
    }
    result
}

// ── Medium 4: Search Suggestions System ─────────────────────────────────

struct SearchSuggestions;
struct SearchSuggestionsTest { products: Vec<String>, search: String }

impl Problem for SearchSuggestions {
    fn id(&self) -> &str { "tries_search_suggestions" }
    fn name(&self) -> &str { "Search Suggestions System" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an array of product names and a search word, for each character typed in the \
         search word, return the top 3 product suggestions (lexicographically smallest) that \
         match the current prefix.\n\n\
         Return Vec<Vec<String>> where result[i] contains up to 3 suggestions after typing \
         the first i+1 characters.\n\n\
         Constraints:\n\
         - 1 <= products.len() <= 1000\n\
         - 1 <= search.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=15);
            let products = random_word_list(&mut rng, n, 8);
            let search = random_word(&mut rng, 5);
            TestCase { data: Box::new(SearchSuggestionsTest { products, search }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SearchSuggestionsTest>().unwrap();
        let expected = ref_search_suggestions(&t.products, &t.search);
        let actual = solutions::search_suggestions(&t.products, &t.search);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("products={:?}, search=\"{}\"", t.products, t.search),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_search_suggestions(products: &[String], search: &str) -> Vec<Vec<String>> {
    let mut sorted = products.to_vec();
    sorted.sort();
    sorted.dedup();
    let mut result = Vec::new();
    for i in 1..=search.len() {
        let prefix = &search[..i];
        let suggestions: Vec<String> = sorted.iter()
            .filter(|p| p.starts_with(prefix))
            .take(3)
            .cloned()
            .collect();
        result.push(suggestions);
    }
    result
}

// ── Medium 5: Add and Search Word ───────────────────────────────────────

struct AddSearchWord;
struct AddSearchWordTest {
    ops: Vec<(String, String)>, // ("add", word) or ("search", pattern)
}

impl Problem for AddSearchWord {
    fn id(&self) -> &str { "tries_add_search_word" }
    fn name(&self) -> &str { "Add and Search Word" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Design a data structure that supports:\n\
         - \"add\" word: adds a word to the data structure\n\
         - \"search\" pattern: returns true if any word matches the pattern\n\n\
         A pattern can contain '.' which matches any single character.\n\n\
         Return Vec<Option<bool>> -- None for \"add\" ops, Some(result) for \"search\" ops.\n\n\
         Constraints:\n\
         - 1 <= ops.len() <= 100\n\
         - Words and patterns consist of lowercase letters and '.'"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n_adds = rng.random_range(1..=8);
            let n_searches = rng.random_range(1..=8);
            let mut ops = Vec::new();
            let words = random_word_list(&mut rng, n_adds, 5);
            for w in &words {
                ops.push(("add".to_string(), w.clone()));
            }
            for _ in 0..n_searches {
                let pattern = if rng.random_range(0..3) == 0 {
                    // Add wildcards to existing word
                    let w = &words[rng.random_range(0..words.len())];
                    w.chars().map(|c| {
                        if rng.random_range(0..3) == 0 { '.' } else { c }
                    }).collect()
                } else {
                    let len = rng.random_range(1..=5);
                    (0..len).map(|_| {
                        if rng.random_range(0..4) == 0 {
                            '.'
                        } else {
                            (b'a' + rng.random_range(0..26u8)) as char
                        }
                    }).collect()
                };
                ops.push(("search".to_string(), pattern));
            }
            TestCase { data: Box::new(AddSearchWordTest { ops }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AddSearchWordTest>().unwrap();
        let expected = ref_add_search_word(&t.ops);
        let actual = solutions::add_search_word(&t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("ops={:?}", t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_add_search_word(ops: &[(String, String)]) -> Vec<Option<bool>> {
    let mut trie = RefTrie::new();
    let mut result = Vec::new();
    for (op, arg) in ops {
        match op.as_str() {
            "add" => {
                trie.insert(arg);
                result.push(None);
            }
            "search" => {
                result.push(Some(trie.search_with_wildcard(arg)));
            }
            _ => {}
        }
    }
    result
}

// ── Hard 1: Palindrome Pairs ────────────────────────────────────────────

struct PalindromePairs;
struct PalindromePairsTest { words: Vec<String> }

impl Problem for PalindromePairs {
    fn id(&self) -> &str { "tries_palindrome_pairs" }
    fn name(&self) -> &str { "Palindrome Pairs" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a list of unique words, find all pairs (i, j) where i != j and \
         words[i] + words[j] forms a palindrome.\n\n\
         Return pairs sorted by (i, j).\n\n\
         Constraints:\n\
         - 1 <= words.len() <= 200\n\
         - 0 <= words[i].len() <= 50"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=8);
            let mut words: Vec<String> = Vec::new();
            let mut used = HashSet::new();
            while words.len() < n {
                let wlen = rng.random_range(0..=3);
                let w = helpers::random_string_from(&mut rng, wlen, b"abc");
                if !used.contains(&w) {
                    used.insert(w.clone());
                    words.push(w);
                }
            }
            // Add some reversed words to increase palindrome pair probability
            if rng.random_range(0..2) == 0 && words.len() >= 2 {
                let rev: String = words[0].chars().rev().collect();
                if !used.contains(&rev) {
                    let idx = rng.random_range(1..words.len());
                    used.remove(&words[idx]);
                    words[idx] = rev.clone();
                    used.insert(rev);
                }
            }
            TestCase { data: Box::new(PalindromePairsTest { words }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PalindromePairsTest>().unwrap();
        let expected = ref_palindrome_pairs(&t.words);
        let actual = solutions::palindrome_pairs(&t.words);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("words={:?}", t.words),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let n = bytes.len();
    for i in 0..n / 2 {
        if bytes[i] != bytes[n - 1 - i] { return false; }
    }
    true
}

fn ref_palindrome_pairs(words: &[String]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for i in 0..words.len() {
        for j in 0..words.len() {
            if i != j {
                let combined = format!("{}{}", words[i], words[j]);
                if is_palindrome(&combined) {
                    result.push((i, j));
                }
            }
        }
    }
    result.sort();
    result
}

// ── Hard 2: Word Break II ──────────────────────────────────────────────

struct WordBreakII;
struct WordBreakIITest { s: String, word_dict: Vec<String> }

impl Problem for WordBreakII {
    fn id(&self) -> &str { "tries_word_break_ii" }
    fn name(&self) -> &str { "Word Break II" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a string `s` and a dictionary of words, add spaces to `s` to construct every \
         possible sentence where each word is a valid dictionary word.\n\n\
         Return all such sentences sorted lexicographically.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 20\n\
         - 1 <= word_dict.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n_words = rng.random_range(2..=6);
            let word_dict: Vec<String> = (0..n_words)
                .map(|_| {
                    let wlen = rng.random_range(1..=4);
                    helpers::random_string_from(&mut rng, wlen, b"abcd")
                })
                .collect();
            // Build s from concatenation of dictionary words
            let n_concat = rng.random_range(1..=3);
            let s: String = (0..n_concat)
                .map(|_| word_dict[rng.random_range(0..word_dict.len())].as_str())
                .collect::<Vec<_>>()
                .join("");
            TestCase { data: Box::new(WordBreakIITest { s, word_dict }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WordBreakIITest>().unwrap();
        let expected = ref_word_break_ii(&t.s, &t.word_dict);
        let actual = solutions::word_break_ii(&t.s, &t.word_dict);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\", word_dict={:?}", t.s, t.word_dict),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_word_break_ii(s: &str, word_dict: &[String]) -> Vec<String> {
    let dict: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let mut result = Vec::new();
    let mut path = Vec::new();
    wb_backtrack(s, 0, &dict, &mut path, &mut result);
    result.sort();
    result.dedup();
    result
}

fn wb_backtrack(s: &str, start: usize, dict: &HashSet<&str>, path: &mut Vec<String>, result: &mut Vec<String>) {
    if start == s.len() {
        result.push(path.join(" "));
        return;
    }
    for end in start + 1..=s.len() {
        let word = &s[start..end];
        if dict.contains(word) {
            path.push(word.to_string());
            wb_backtrack(s, end, dict, path, result);
            path.pop();
        }
    }
}

// ── Hard 3: Concatenated Words ──────────────────────────────────────────

struct ConcatenatedWords;
struct ConcatenatedWordsTest { words: Vec<String> }

impl Problem for ConcatenatedWords {
    fn id(&self) -> &str { "tries_concatenated_words" }
    fn name(&self) -> &str { "Concatenated Words" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a list of words, find all words that can be formed by concatenating at least \
         two other words from the list.\n\n\
         Return the result sorted lexicographically.\n\n\
         Constraints:\n\
         - 1 <= words.len() <= 1000\n\
         - 0 <= words[i].len() <= 30"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n_base = rng.random_range(2..=5);
            let mut words: Vec<String> = (0..n_base)
                .map(|_| {
                    let wlen = rng.random_range(1..=3);
                    helpers::random_string_from(&mut rng, wlen, b"abc")
                })
                .collect();
            // Add some concatenated words
            let n_concat = rng.random_range(0..=3);
            for _ in 0..n_concat {
                let ai = rng.random_range(0..n_base);
                let bi = rng.random_range(0..n_base);
                let concat = format!("{}{}", words[ai], words[bi]);
                words.push(concat);
            }
            words.sort();
            words.dedup();
            TestCase { data: Box::new(ConcatenatedWordsTest { words }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ConcatenatedWordsTest>().unwrap();
        let expected = ref_concatenated_words(&t.words);
        let actual = solutions::concatenated_words(&t.words);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("words={:?}", t.words),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_concatenated_words(words: &[String]) -> Vec<String> {
    let word_set: HashSet<&str> = words.iter().map(|w| w.as_str()).collect();
    let mut result = Vec::new();
    for word in words {
        if word.is_empty() { continue; }
        if can_form(word, &word_set) {
            result.push(word.clone());
        }
    }
    result.sort();
    result
}

fn can_form(word: &str, word_set: &HashSet<&str>) -> bool {
    let n = word.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if j == 0 && i == n { continue; } // Skip the word itself
            if dp[j] && word_set.contains(&word[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}

// ── Hard 4: Maximum XOR of Two Numbers ──────────────────────────────────

struct MaximumXor;
struct MaximumXorTest { nums: Vec<i32> }

impl Problem for MaximumXor {
    fn id(&self) -> &str { "tries_maximum_xor" }
    fn name(&self) -> &str { "Maximum XOR of Two Numbers" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an integer array `nums`, return the maximum XOR of any two elements.\n\n\
         Use a bitwise trie for O(n * 32) solution.\n\n\
         Constraints:\n\
         - 2 <= nums.len() <= 1000\n\
         - 0 <= nums[i] <= 2^30"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=20);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=1000)).collect();
            TestCase { data: Box::new(MaximumXorTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaximumXorTest>().unwrap();
        let expected = ref_maximum_xor(&t.nums);
        let actual = solutions::maximum_xor(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_maximum_xor(nums: &[i32]) -> i32 {
    let mut max_xor = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            max_xor = max_xor.max(nums[i] ^ nums[j]);
        }
    }
    max_xor
}

// ── Hard 5: Stream of Characters ────────────────────────────────────────

struct StreamOfCharacters;
struct StreamTest { words: Vec<String>, stream: Vec<char> }

impl Problem for StreamOfCharacters {
    fn id(&self) -> &str { "tries_stream_of_characters" }
    fn name(&self) -> &str { "Stream of Characters" }
    fn topic(&self) -> &str { "tries" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "You are given a list of words and a stream of characters. For each character in \
         the stream, determine if the suffix of all characters received so far matches \
         any word in the list.\n\n\
         Return Vec<bool> where result[i] is true if any word is a suffix of the stream \
         up to and including stream[i].\n\n\
         Constraints:\n\
         - 1 <= words.len() <= 200\n\
         - 1 <= stream.len() <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n_words = rng.random_range(1..=5);
            let words: Vec<String> = (0..n_words)
                .map(|_| {
                    let wlen = rng.random_range(1..=4);
                    helpers::random_string_from(&mut rng, wlen, b"abcd")
                })
                .collect();
            let stream_len = rng.random_range(1..=15);
            let mut stream: Vec<char> = Vec::new();
            for _ in 0..stream_len {
                if rng.random_range(0..3) == 0 && !words.is_empty() {
                    // Feed characters from a word to increase matches
                    let w = &words[rng.random_range(0..words.len())];
                    let chars: Vec<char> = w.chars().collect();
                    if !chars.is_empty() {
                        let idx = rng.random_range(0..chars.len());
                        stream.push(chars[idx]);
                        continue;
                    }
                }
                stream.push((b'a' + rng.random_range(0..4u8)) as char);
            }
            TestCase { data: Box::new(StreamTest { words, stream }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<StreamTest>().unwrap();
        let expected = ref_stream_of_characters(&t.words, &t.stream);
        let actual = solutions::stream_of_characters(&t.words, &t.stream);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("words={:?}, stream={:?}", t.words, t.stream),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_stream_of_characters(words: &[String], stream: &[char]) -> Vec<bool> {
    let mut result = Vec::new();
    let mut received = String::new();
    for &c in stream {
        received.push(c);
        let mut found = false;
        for word in words {
            if received.ends_with(word.as_str()) {
                found = true;
                break;
            }
        }
        result.push(found);
    }
    result
}
