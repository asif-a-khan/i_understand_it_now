use super::{HighlightKind, VizFrame};
use crate::tracker::Operation;

/// Generate visualization frames by replaying an operation log.
///
/// For sorting algorithms: reconstructs the initial array by reversing all swaps
/// from a sorted [1..n] sequence, then replays operations forward. Only Compare
/// and Swap operations produce frames.
pub fn replay_from_ops(ops: &[Operation]) -> Vec<VizFrame> {
    // Find array size from max index in any operation
    let max_idx = ops
        .iter()
        .filter_map(|op| match op {
            Operation::Compare {
                left_idx,
                right_idx,
            }
            | Operation::Swap {
                left_idx,
                right_idx,
            } => Some((*left_idx).max(*right_idx)),
            Operation::Read { idx } | Operation::Write { idx } => Some(*idx),
            _ => None,
        })
        .max();

    let Some(max_idx) = max_idx else {
        return vec![VizFrame {
            array: vec![],
            highlights: vec![],
            annotation: "No array operations recorded".to_string(),
            ..Default::default()
        }];
    };

    let n = max_idx + 1;
    if n > 50 {
        return vec![VizFrame {
            array: (1..=20i32).collect(),
            highlights: vec![],
            annotation: format!("Array too large to replay ({} elements)", n),
            ..Default::default()
        }];
    }

    // Start with sorted [1..n], reverse all swaps to reconstruct initial state
    let mut initial: Vec<i32> = (1..=n as i32).collect();
    for op in ops.iter().rev() {
        if let Operation::Swap {
            left_idx,
            right_idx,
        } = op
        {
            if *left_idx < n && *right_idx < n {
                initial.swap(*left_idx, *right_idx);
            }
        }
    }

    // Replay forward, generating frames for Compare and Swap only
    let mut array = initial;
    let mut frames = Vec::new();
    let mut cmp_count = 0usize;
    let mut swap_count = 0usize;

    frames.push(VizFrame {
        array: array.clone(),
        highlights: vec![],
        annotation: format!("Your solution — {} elements", n),
        ..Default::default()
    });

    for op in ops {
        match op {
            Operation::Compare {
                left_idx,
                right_idx,
            } => {
                if *left_idx < n && *right_idx < n {
                    cmp_count += 1;
                    frames.push(VizFrame {
                        array: array.clone(),
                        highlights: vec![
                            (*left_idx, HighlightKind::Comparing),
                            (*right_idx, HighlightKind::Comparing),
                        ],
                        annotation: format!(
                            "Compare [{}]={} vs [{}]={} — cmp: {}, swaps: {}",
                            left_idx,
                            array[*left_idx],
                            right_idx,
                            array[*right_idx],
                            cmp_count,
                            swap_count,
                        ),
                        ..Default::default()
                    });
                }
            }
            Operation::Swap {
                left_idx,
                right_idx,
            } => {
                if *left_idx < n && *right_idx < n {
                    swap_count += 1;
                    array.swap(*left_idx, *right_idx);
                    frames.push(VizFrame {
                        array: array.clone(),
                        highlights: vec![
                            (*left_idx, HighlightKind::Swapping),
                            (*right_idx, HighlightKind::Swapping),
                        ],
                        annotation: format!(
                            "Swap [{}] <-> [{}] — cmp: {}, swap #{}",
                            left_idx, right_idx, cmp_count, swap_count,
                        ),
                        ..Default::default()
                    });
                }
            }
            _ => {}
        }
    }

    frames.push(VizFrame {
        array: array.clone(),
        highlights: (0..n).map(|i| (i, HighlightKind::Sorted)).collect(),
        annotation: format!(
            "Done — {} comparisons, {} swaps, {} total ops",
            cmp_count,
            swap_count,
            ops.len()
        ),
        ..Default::default()
    });

    frames
}
