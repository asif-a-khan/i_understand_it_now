use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Neg, Rem, Sub};
use std::rc::Rc;

/// Records all operations performed by a solution for visualization and metrics.
#[derive(Debug, Clone, Default)]
pub struct OperationLog {
    operations: Vec<Operation>,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Compare { left_idx: usize, right_idx: usize },
    Swap { left_idx: usize, right_idx: usize },
    Read { idx: usize },
    Write { idx: usize },
    HashLookup { key_idx: usize },
    HashInsert { key_idx: usize },
    FunctionCall { name: String },
}

impl OperationLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, op: Operation) {
        self.operations.push(op);
    }

    pub fn comparisons(&self) -> usize {
        self.operations
            .iter()
            .filter(|op| matches!(op, Operation::Compare { .. }))
            .count()
    }

    pub fn swaps(&self) -> usize {
        self.operations
            .iter()
            .filter(|op| matches!(op, Operation::Swap { .. }))
            .count()
    }

    pub fn reads(&self) -> usize {
        self.operations
            .iter()
            .filter(|op| matches!(op, Operation::Read { .. }))
            .count()
    }

    pub fn writes(&self) -> usize {
        self.operations
            .iter()
            .filter(|op| matches!(op, Operation::Write { .. }))
            .count()
    }

    pub fn hash_lookups(&self) -> usize {
        self.operations
            .iter()
            .filter(|op| matches!(op, Operation::HashLookup { .. }))
            .count()
    }

    pub fn hash_inserts(&self) -> usize {
        self.operations
            .iter()
            .filter(|op| matches!(op, Operation::HashInsert { .. }))
            .count()
    }

    pub fn total_operations(&self) -> usize {
        self.operations.len()
    }

    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    pub fn clear(&mut self) {
        self.operations.clear();
    }
}

/// A wrapper around a value that records operations into an OperationLog.
///
/// Users write normal Rust code with operators like `<`, `>`, `==` —
/// the Tracked wrapper records every comparison, swap, etc. automatically.
pub struct Tracked<T> {
    pub value: T,
    idx: usize,
    log: Rc<RefCell<OperationLog>>,
}

impl<T> Tracked<T> {
    pub fn new(value: T, idx: usize, log: Rc<RefCell<OperationLog>>) -> Self {
        Self { value, idx, log }
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    /// Get the inner value by reference (records a Read).
    pub fn get(&self) -> &T {
        self.log
            .borrow_mut()
            .record(Operation::Read { idx: self.idx });
        &self.value
    }

    /// Set the inner value (records a Write).
    pub fn set(&mut self, value: T) {
        self.log
            .borrow_mut()
            .record(Operation::Write { idx: self.idx });
        self.value = value;
    }

    /// Access value without recording (for display/debug only).
    pub fn peek(&self) -> &T {
        &self.value
    }
}

impl<T: Clone> Clone for Tracked<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            idx: self.idx,
            log: self.log.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Tracked<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<T: fmt::Display> fmt::Display for Tracked<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: PartialEq> PartialEq for Tracked<T> {
    fn eq(&self, other: &Self) -> bool {
        self.log.borrow_mut().record(Operation::Compare {
            left_idx: self.idx,
            right_idx: other.idx,
        });
        self.value == other.value
    }
}

impl<T: Eq + PartialEq> Eq for Tracked<T> {}

impl<T: PartialOrd> PartialOrd for Tracked<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.log.borrow_mut().record(Operation::Compare {
            left_idx: self.idx,
            right_idx: other.idx,
        });
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord + Eq> Ord for Tracked<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.log.borrow_mut().record(Operation::Compare {
            left_idx: self.idx,
            right_idx: other.idx,
        });
        self.value.cmp(&other.value)
    }
}

impl<T: Hash> Hash for Tracked<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: Add<Output = T> + Clone> Add for &Tracked<T> {
    type Output = T;
    fn add(self, rhs: Self) -> T {
        self.value.clone() + rhs.value.clone()
    }
}

impl<T: Sub<Output = T> + Clone> Sub for &Tracked<T> {
    type Output = T;
    fn sub(self, rhs: Self) -> T {
        self.value.clone() - rhs.value.clone()
    }
}

impl<T: Mul<Output = T> + Clone> Mul for &Tracked<T> {
    type Output = T;
    fn mul(self, rhs: Self) -> T {
        self.value.clone() * rhs.value.clone()
    }
}

impl<T: Rem<Output = T> + Clone> Rem for &Tracked<T> {
    type Output = T;
    fn rem(self, rhs: Self) -> T {
        self.value.clone() % rhs.value.clone()
    }
}

impl<T: Neg<Output = T> + Clone> Neg for Tracked<T> {
    type Output = T;
    fn neg(self) -> T {
        -self.value
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Type-specific convenience methods
// ═══════════════════════════════════════════════════════════════════════

// ─── Tracked<i32> ────────────────────────────────────────────────────

#[allow(dead_code)]
impl Tracked<i32> {
    pub fn abs(&self) -> i32 {
        self.value.abs()
    }
    pub fn pow(&self, exp: u32) -> i32 {
        self.value.pow(exp)
    }
    pub fn wrapping_add(&self, rhs: i32) -> i32 {
        self.value.wrapping_add(rhs)
    }
    pub fn wrapping_sub(&self, rhs: i32) -> i32 {
        self.value.wrapping_sub(rhs)
    }
    pub fn checked_add(&self, rhs: i32) -> Option<i32> {
        self.value.checked_add(rhs)
    }
    pub fn checked_sub(&self, rhs: i32) -> Option<i32> {
        self.value.checked_sub(rhs)
    }
    pub fn signum(&self) -> i32 {
        self.value.signum()
    }
}

impl PartialEq<i32> for Tracked<i32> {
    fn eq(&self, other: &i32) -> bool {
        self.value == *other
    }
}

impl PartialOrd<i32> for Tracked<i32> {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

// ─── Tracked<i64> ────────────────────────────────────────────────────

#[allow(dead_code)]
impl Tracked<i64> {
    pub fn abs(&self) -> i64 {
        self.value.abs()
    }
    pub fn pow(&self, exp: u32) -> i64 {
        self.value.pow(exp)
    }
    pub fn signum(&self) -> i64 {
        self.value.signum()
    }
}

impl PartialEq<i64> for Tracked<i64> {
    fn eq(&self, other: &i64) -> bool {
        self.value == *other
    }
}

impl PartialOrd<i64> for Tracked<i64> {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

// ─── Tracked<f64> ────────────────────────────────────────────────────

#[allow(dead_code)]
impl Tracked<f64> {
    pub fn abs(&self) -> f64 {
        self.value.abs()
    }
    pub fn sqrt(&self) -> f64 {
        self.value.sqrt()
    }
    pub fn floor(&self) -> f64 {
        self.value.floor()
    }
    pub fn ceil(&self) -> f64 {
        self.value.ceil()
    }
    pub fn round(&self) -> f64 {
        self.value.round()
    }
    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }
    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }
    pub fn min_f(&self, other: f64) -> f64 {
        self.value.min(other)
    }
    pub fn max_f(&self, other: f64) -> f64 {
        self.value.max(other)
    }
}

impl PartialEq<f64> for Tracked<f64> {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

impl PartialOrd<f64> for Tracked<f64> {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

// ─── Tracked<char> ───────────────────────────────────────────────────

#[allow(dead_code)]
impl Tracked<char> {
    pub fn is_alphabetic(&self) -> bool {
        self.value.is_alphabetic()
    }
    pub fn is_alphanumeric(&self) -> bool {
        self.value.is_alphanumeric()
    }
    pub fn is_ascii(&self) -> bool {
        self.value.is_ascii()
    }
    pub fn is_ascii_lowercase(&self) -> bool {
        self.value.is_ascii_lowercase()
    }
    pub fn is_ascii_uppercase(&self) -> bool {
        self.value.is_ascii_uppercase()
    }
    pub fn is_ascii_digit(&self) -> bool {
        self.value.is_ascii_digit()
    }
    pub fn is_ascii_alphabetic(&self) -> bool {
        self.value.is_ascii_alphabetic()
    }
    pub fn is_ascii_whitespace(&self) -> bool {
        self.value.is_ascii_whitespace()
    }
    pub fn is_whitespace(&self) -> bool {
        self.value.is_whitespace()
    }
    pub fn to_ascii_lowercase(&self) -> char {
        self.value.to_ascii_lowercase()
    }
    pub fn to_ascii_uppercase(&self) -> char {
        self.value.to_ascii_uppercase()
    }
    pub fn is_numeric(&self) -> bool {
        self.value.is_ascii_digit()
    }
    pub fn to_digit(&self, radix: u32) -> Option<u32> {
        self.value.to_digit(radix)
    }
}

impl PartialEq<char> for Tracked<char> {
    fn eq(&self, other: &char) -> bool {
        self.value == *other
    }
}

impl PartialOrd<char> for Tracked<char> {
    fn partial_cmp(&self, other: &char) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

// ─── Tracked<u8> ─────────────────────────────────────────────────────

#[allow(dead_code)]
impl Tracked<u8> {
    pub fn is_ascii_digit(&self) -> bool {
        self.value.is_ascii_digit()
    }
    pub fn is_ascii_alphabetic(&self) -> bool {
        self.value.is_ascii_alphabetic()
    }
    pub fn wrapping_add(&self, rhs: u8) -> u8 {
        self.value.wrapping_add(rhs)
    }
    pub fn wrapping_sub(&self, rhs: u8) -> u8 {
        self.value.wrapping_sub(rhs)
    }
}

impl PartialEq<u8> for Tracked<u8> {
    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }
}

impl PartialOrd<u8> for Tracked<u8> {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

// ─── Tracked<Option<i32>> ────────────────────────────────────────────

#[allow(dead_code)]
impl Tracked<Option<i32>> {
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }
    pub fn unwrap(&self) -> i32 {
        self.value.unwrap()
    }
    pub fn unwrap_or(&self, default: i32) -> i32 {
        self.value.unwrap_or(default)
    }
    pub fn map<F: FnOnce(i32) -> i32>(&self, f: F) -> Option<i32> {
        self.value.map(f)
    }
}

// ─── Tracked<Option<usize>> ─────────────────────────────────────────

#[allow(dead_code)]
impl Tracked<Option<usize>> {
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }
    pub fn unwrap(&self) -> usize {
        self.value.unwrap()
    }
    pub fn unwrap_or(&self, default: usize) -> usize {
        self.value.unwrap_or(default)
    }
}

// ─── TrackedString trait for [Tracked<char>] ─────────────────────────

/// String-like operations on tracked character slices.
#[allow(dead_code)]
pub trait TrackedString {
    /// Collect all characters into a String.
    fn to_string_value(&self) -> String;
    /// Check if the slice contains a specific character.
    fn contains_char(&self, c: char) -> bool;
    /// Extract raw characters as a Vec.
    fn chars_vec(&self) -> Vec<char>;
}

impl TrackedString for [Tracked<char>] {
    fn to_string_value(&self) -> String {
        self.iter().map(|t| t.value).collect()
    }
    fn contains_char(&self, c: char) -> bool {
        self.iter().any(|t| t.value == c)
    }
    fn chars_vec(&self) -> Vec<char> {
        self.iter().map(|t| t.value).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════

/// Create a tracked slice from a regular slice, sharing one OperationLog.
pub fn track_slice<T: Clone>(values: &[T], log: Rc<RefCell<OperationLog>>) -> Vec<Tracked<T>> {
    values
        .iter()
        .enumerate()
        .map(|(i, v)| Tracked::new(v.clone(), i, log.clone()))
        .collect()
}

/// Create a tracked char slice from a string.
pub fn track_string(s: &str, log: Rc<RefCell<OperationLog>>) -> Vec<Tracked<char>> {
    let chars: Vec<char> = s.chars().collect();
    track_slice(&chars, log)
}

/// Record a swap between two positions in a tracked slice.
#[allow(dead_code)]
pub fn tracked_swap<T>(slice: &mut [Tracked<T>], a: usize, b: usize) {
    if a < slice.len() && b < slice.len() {
        slice[a].log.borrow_mut().record(Operation::Swap {
            left_idx: a,
            right_idx: b,
        });
        slice.swap(a, b);
    }
}

// ═══════════════════════════════════════════════════════════════════════
// TrackedGraph — records vertex visits for visualization
// ═══════════════════════════════════════════════════════════════════════

/// A tracked unweighted graph that records vertex accesses for visualization.
///
/// Users interact through natural graph methods:
/// ```ignore
/// let neighbors = graph.neighbors(v);  // records a Read at vertex v
/// let n = graph.n();                    // number of nodes
/// let has = graph.has_edge(u, v);       // records a Compare
/// ```
#[allow(dead_code)]
pub struct TrackedGraph {
    adj: Vec<Vec<usize>>,
    edge_list: Vec<(usize, usize)>,
    directed: bool,
    log: Rc<RefCell<OperationLog>>,
}

#[allow(dead_code)]
impl TrackedGraph {
    /// Build a tracked graph from node count and edge list.
    pub fn new(
        n: usize,
        edges: &[(usize, usize)],
        directed: bool,
        log: Rc<RefCell<OperationLog>>,
    ) -> Self {
        let mut adj = vec![vec![]; n];
        for &(u, v) in edges {
            if u < n && v < n {
                adj[u].push(v);
                if !directed {
                    adj[v].push(u);
                }
            }
        }
        for neighbors in &mut adj {
            neighbors.sort_unstable();
            neighbors.dedup();
        }
        Self {
            adj,
            edge_list: edges.to_vec(),
            directed,
            log,
        }
    }

    /// Number of nodes.
    pub fn n(&self) -> usize {
        self.adj.len()
    }

    /// Get neighbors of vertex v (records a Read operation).
    pub fn neighbors(&self, v: usize) -> &[usize] {
        self.log.borrow_mut().record(Operation::Read { idx: v });
        &self.adj[v]
    }

    /// Check if edge (u, v) exists (records a Compare operation).
    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.log.borrow_mut().record(Operation::Compare {
            left_idx: u,
            right_idx: v,
        });
        self.adj[u].contains(&v)
    }

    /// Get the full adjacency list (no operation recorded — for setup/display).
    pub fn adj(&self) -> &[Vec<usize>] {
        &self.adj
    }

    /// Get the original edge list.
    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edge_list
    }

    /// Whether the graph is directed.
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Degree of vertex v.
    pub fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    /// Number of edges.
    pub fn num_edges(&self) -> usize {
        if self.directed {
            self.adj.iter().map(|ns| ns.len()).sum()
        } else {
            self.adj.iter().map(|ns| ns.len()).sum::<usize>() / 2
        }
    }
}

/// A tracked weighted graph that records vertex accesses for visualization.
///
/// Edge weights are stored inline: adjacency list contains `(neighbor, weight)` pairs.
/// ```ignore
/// let neighbors = graph.neighbors(v);  // returns &[(usize, i32)], records Read
/// let n = graph.n();
/// ```
#[allow(dead_code)]
pub struct TrackedWeightedGraph {
    adj: Vec<Vec<(usize, i32)>>,
    edge_list: Vec<(usize, usize, i32)>,
    directed: bool,
    log: Rc<RefCell<OperationLog>>,
}

#[allow(dead_code)]
impl TrackedWeightedGraph {
    /// Build a tracked weighted graph from node count and weighted edge list.
    pub fn new(
        n: usize,
        edges: &[(usize, usize, i32)],
        directed: bool,
        log: Rc<RefCell<OperationLog>>,
    ) -> Self {
        let mut adj = vec![vec![]; n];
        for &(u, v, w) in edges {
            if u < n && v < n {
                adj[u].push((v, w));
                if !directed {
                    adj[v].push((u, w));
                }
            }
        }
        for neighbors in &mut adj {
            neighbors.sort_unstable();
        }
        Self {
            adj,
            edge_list: edges.to_vec(),
            directed,
            log,
        }
    }

    /// Number of nodes.
    pub fn n(&self) -> usize {
        self.adj.len()
    }

    /// Get weighted neighbors of vertex v (records a Read operation).
    /// Returns `&[(neighbor_id, weight)]`.
    pub fn neighbors(&self, v: usize) -> &[(usize, i32)] {
        self.log.borrow_mut().record(Operation::Read { idx: v });
        &self.adj[v]
    }

    /// Get the full adjacency list (no operation recorded).
    pub fn adj(&self) -> &[Vec<(usize, i32)>] {
        &self.adj
    }

    /// Get the original weighted edge list.
    pub fn edges(&self) -> &[(usize, usize, i32)] {
        &self.edge_list
    }

    /// Whether the graph is directed.
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Degree of vertex v.
    pub fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    /// Get edge weight between u and v (records a Compare, returns None if no edge).
    pub fn weight(&self, u: usize, v: usize) -> Option<i32> {
        self.log.borrow_mut().record(Operation::Compare {
            left_idx: u,
            right_idx: v,
        });
        self.adj[u].iter().find(|(nb, _)| *nb == v).map(|(_, w)| *w)
    }

    /// Number of edges.
    pub fn num_edges(&self) -> usize {
        self.edge_list.len()
    }
}

// ─── Tracked<usize> convenience methods ──────────────────────────────

#[allow(dead_code)]
impl Tracked<usize> {
    pub fn checked_add(&self, rhs: usize) -> Option<usize> {
        self.value.checked_add(rhs)
    }
    pub fn checked_sub(&self, rhs: usize) -> Option<usize> {
        self.value.checked_sub(rhs)
    }
    pub fn saturating_sub(&self, rhs: usize) -> usize {
        self.value.saturating_sub(rhs)
    }
    pub fn min_val(&self, other: usize) -> usize {
        self.value.min(other)
    }
    pub fn max_val(&self, other: usize) -> usize {
        self.value.max(other)
    }
}

impl PartialEq<usize> for Tracked<usize> {
    fn eq(&self, other: &usize) -> bool {
        self.value == *other
    }
}

impl PartialOrd<usize> for Tracked<usize> {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}
