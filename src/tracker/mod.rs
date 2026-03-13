use std::cell::RefCell;
use std::fmt;
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

    pub fn total_operations(&self) -> usize {
        self.operations.len()
    }

    pub fn operations(&self) -> &[Operation] {
        &self.operations
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

/// Create a tracked slice from a regular slice, sharing one OperationLog.
pub fn track_slice<T: Clone>(values: &[T], log: Rc<RefCell<OperationLog>>) -> Vec<Tracked<T>> {
    values
        .iter()
        .enumerate()
        .map(|(i, v)| Tracked::new(v.clone(), i, log.clone()))
        .collect()
}

/// Record a swap between two positions in a tracked slice.
pub fn tracked_swap<T>(slice: &mut [Tracked<T>], a: usize, b: usize) {
    if a < slice.len() && b < slice.len() {
        slice[a].log.borrow_mut().record(Operation::Swap {
            left_idx: a,
            right_idx: b,
        });
        slice.swap(a, b);
    }
}
