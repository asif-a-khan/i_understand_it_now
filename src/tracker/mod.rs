use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub, Mul, Rem, Neg};
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
        self.log.borrow_mut().record(Operation::Read { idx: self.idx });
        &self.value
    }

    /// Set the inner value (records a Write).
    pub fn set(&mut self, value: T) {
        self.log.borrow_mut().record(Operation::Write { idx: self.idx });
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
        self.log
            .borrow_mut()
            .record(Operation::Compare {
                left_idx: self.idx,
                right_idx: other.idx,
            });
        self.value == other.value
    }
}

impl<T: Eq + PartialEq> Eq for Tracked<T> {}

impl<T: PartialOrd> PartialOrd for Tracked<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.log
            .borrow_mut()
            .record(Operation::Compare {
                left_idx: self.idx,
                right_idx: other.idx,
            });
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord + Eq> Ord for Tracked<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.log
            .borrow_mut()
            .record(Operation::Compare {
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

/// Create a tracked slice from a regular slice, sharing one OperationLog.
pub fn track_slice<T: Clone>(values: &[T], log: Rc<RefCell<OperationLog>>) -> Vec<Tracked<T>> {
    values
        .iter()
        .enumerate()
        .map(|(i, v)| Tracked::new(v.clone(), i, log.clone()))
        .collect()
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
