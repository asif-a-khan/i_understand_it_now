use super::{HighlightKind, VizData, VizFrame};
use crate::tracker::Operation;

/// Generate visualization frames by replaying an operation log.
///
/// Uses the problem topic to determine the right visualization type:
/// - Sorting/array topics → array box visualization
/// - Tree topics → tree visualization
/// - Grid/graph topics → grid visualization
/// - String topics → array with char labels
/// - Linked list topics → array with arrow labels
///
/// Handles all operation types: Compare, Swap, Read, Write, HashLookup, HashInsert.
pub fn replay_from_ops(ops: &[Operation], topic: &str) -> Vec<VizFrame> {
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
            Operation::Read { idx }
            | Operation::Write { idx }
            | Operation::HashLookup { key_idx: idx }
            | Operation::HashInsert { key_idx: idx } => Some(*idx),
            _ => None,
        })
        .max();

    let Some(max_idx) = max_idx else {
        return vec![VizFrame {
            annotation: "No operations recorded — implement your solution to see a replay"
                .to_string(),
            data: Some(VizData::None {
                message: "No operations recorded".to_string(),
            }),
            ..Default::default()
        }];
    };

    let n = max_idx + 1;
    if n > 200 {
        return vec![VizFrame {
            annotation: format!("Data too large to replay ({} elements)", n),
            data: Some(VizData::None {
                message: format!("Too large: {} elements", n),
            }),
            ..Default::default()
        }];
    }

    // Determine visualization type from topic
    let viz_type = match topic {
        "binary_trees" | "bst" | "balanced_bst" | "heaps_priority_queues" | "tries" => "tree",
        "matrix_grid"
        | "graph_representations"
        | "graph_bfs_dfs"
        | "topological_sort"
        | "shortest_path"
        | "mst"
        | "union_find" => "grid",
        "strings" | "string_algorithms" => "string",
        "linked_lists" => "linked_list",
        _ => "array", // sorting, arrays, stacks_queues, hash_maps, etc.
    };

    // For sorting: reconstruct initial array from swaps
    // For other types: use placeholder values [0, 1, 2, ..., n-1]
    let is_sorting = matches!(
        topic,
        "basic_sorts" | "merge_sort" | "quick_sort" | "heap_sort" | "counting_radix"
    );

    let mut array: Vec<i32> = if is_sorting {
        // Reconstruct initial state by reversing swaps from sorted [1..n]
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
        initial
    } else {
        (0..n as i32).collect()
    };

    // Build frames from ALL operation types
    let mut frames = Vec::new();
    let mut cmp_count = 0usize;
    let mut swap_count = 0usize;
    let mut read_count = 0usize;
    let mut write_count = 0usize;

    // Build initial frame with appropriate VizData
    let initial_frame = build_frame(
        &array,
        &[],
        format!("Your solution — {} elements", n),
        viz_type,
        n,
    );
    frames.push(initial_frame);

    // Accumulate visited indices for grid/graph visualizations
    let mut visited: Vec<(usize, HighlightKind)> = Vec::new();

    for op in ops {
        match op {
            Operation::Compare {
                left_idx,
                right_idx,
            } => {
                if *left_idx < n && *right_idx < n {
                    cmp_count += 1;
                    let mut highlights = visited.clone();
                    highlights.push((*left_idx, HighlightKind::Comparing));
                    highlights.push((*right_idx, HighlightKind::Comparing));
                    let annotation = format!(
                        "Compare [{}] vs [{}] — cmp #{}, swaps: {}",
                        left_idx, right_idx, cmp_count, swap_count,
                    );
                    frames.push(build_frame(&array, &highlights, annotation, viz_type, n));
                }
            }
            Operation::Swap {
                left_idx,
                right_idx,
            } => {
                if *left_idx < n && *right_idx < n {
                    swap_count += 1;
                    array.swap(*left_idx, *right_idx);
                    let mut highlights = visited.clone();
                    highlights.push((*left_idx, HighlightKind::Swapping));
                    highlights.push((*right_idx, HighlightKind::Swapping));
                    let annotation = format!(
                        "Swap [{}] <-> [{}] — cmp: {}, swap #{}",
                        left_idx, right_idx, cmp_count, swap_count,
                    );
                    frames.push(build_frame(&array, &highlights, annotation, viz_type, n));
                }
            }
            Operation::Read { idx } => {
                if *idx < n {
                    read_count += 1;
                    let mut highlights = visited.clone();
                    highlights.push((*idx, HighlightKind::Reading));
                    let annotation =
                        format!("Read [{}] = {} — reads: {}", idx, array[*idx], read_count,);
                    frames.push(build_frame(&array, &highlights, annotation, viz_type, n));

                    // For grid/graph: mark as visited
                    if viz_type == "grid" && !visited.iter().any(|(i, _)| *i == *idx) {
                        visited.push((*idx, HighlightKind::Sorted));
                    }
                }
            }
            Operation::Write { idx } => {
                if *idx < n {
                    write_count += 1;
                    let mut highlights = visited.clone();
                    highlights.push((*idx, HighlightKind::Writing));
                    let annotation = format!("Write [{}] — writes: {}", idx, write_count);
                    frames.push(build_frame(&array, &highlights, annotation, viz_type, n));
                }
            }
            Operation::HashLookup { key_idx } => {
                if *key_idx < n {
                    let mut highlights = visited.clone();
                    highlights.push((*key_idx, HighlightKind::Active));
                    let annotation = format!("Hash lookup at [{}]", key_idx);
                    frames.push(build_frame(&array, &highlights, annotation, viz_type, n));
                }
            }
            Operation::HashInsert { key_idx } => {
                if *key_idx < n {
                    let mut highlights = visited.clone();
                    highlights.push((*key_idx, HighlightKind::Found));
                    let annotation = format!("Hash insert at [{}]", key_idx);
                    frames.push(build_frame(&array, &highlights, annotation, viz_type, n));
                }
            }
            Operation::FunctionCall { name } => {
                let annotation = format!("Call: {}", name);
                frames.push(build_frame(&array, &visited, annotation, viz_type, n));
            }
        }
    }

    // Final frame
    let final_highlights: Vec<(usize, HighlightKind)> =
        (0..n).map(|i| (i, HighlightKind::Sorted)).collect();
    let annotation = format!(
        "Done — {} comparisons, {} swaps, {} reads, {} writes, {} total ops",
        cmp_count,
        swap_count,
        read_count,
        write_count,
        ops.len()
    );
    frames.push(build_frame(
        &array,
        &final_highlights,
        annotation,
        viz_type,
        n,
    ));

    // Limit frame count to avoid UI lag
    if frames.len() > 500 {
        let step = frames.len() / 500;
        let mut sampled: Vec<VizFrame> = frames.iter().step_by(step).cloned().collect();
        // Always include the last frame
        if let Some(last) = frames.last() {
            sampled.push(last.clone());
        }
        return sampled;
    }

    frames
}

/// Build a VizFrame with the appropriate VizData type based on the viz_type.
fn build_frame(
    array: &[i32],
    highlights: &[(usize, HighlightKind)],
    annotation: String,
    viz_type: &str,
    n: usize,
) -> VizFrame {
    let data = match viz_type {
        "tree" => Some(VizData::Tree {
            nodes: array.iter().map(|v| Some(v.to_string())).collect(),
        }),
        "grid" => {
            let cols = (n as f64).sqrt().ceil() as usize;
            let rows = if cols > 0 { n.div_ceil(cols) } else { 1 };
            let mut cells: Vec<Vec<String>> = Vec::new();
            for r in 0..rows {
                let mut row = Vec::new();
                for c in 0..cols {
                    let idx = r * cols + c;
                    if idx < n {
                        row.push(array[idx].to_string());
                    } else {
                        row.push(" ".to_string());
                    }
                }
                cells.push(row);
            }
            Some(VizData::Grid { cells })
        }
        "string" => Some(VizData::Array {
            values: array
                .iter()
                .map(|v| {
                    let c = (*v as u8 + b'a' - 1) as char;
                    if c.is_ascii_lowercase() {
                        c.to_string()
                    } else {
                        v.to_string()
                    }
                })
                .collect(),
        }),
        "linked_list" => Some(VizData::Array {
            values: array
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    if i + 1 < array.len() {
                        format!("{} →", v)
                    } else {
                        v.to_string()
                    }
                })
                .collect(),
        }),
        _ => None, // "array" — use legacy array field via viz_data() fallback
    };

    VizFrame {
        array: array.to_vec(),
        data,
        highlights: highlights.to_vec(),
        annotation,
        pointers: vec![],
    }
}
