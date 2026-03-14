use super::{HighlightKind, ReferenceViz, VizFrame};

// ── Bubble Sort ────────────────────────────────────────────────────────

pub struct BubbleSortViz;

impl ReferenceViz for BubbleSortViz {
    fn id(&self) -> &str { "bubble_sort" }
    fn name(&self) -> &str { "Bubble Sort" }
    fn description(&self) -> &str {
        "Repeatedly steps through the list, compares adjacent elements, and swaps them \
         if they are in the wrong order. O(n^2) time, O(1) space."
    }
    fn default_input(&self) -> Vec<i32> { vec![5, 3, 8, 1, 9, 2, 7] }

    fn generate_frames(&self, input: &[i32]) -> Vec<VizFrame> {
        let mut arr = input.to_vec();
        let n = arr.len();
        let mut frames = vec![VizFrame {
            array: arr.clone(),
            highlights: vec![],
            annotation: "Initial array".to_string(),
        }];

        for i in 0..n {
            for j in 0..n - 1 - i {
                frames.push(VizFrame {
                    array: arr.clone(),
                    highlights: vec![(j, HighlightKind::Comparing), (j + 1, HighlightKind::Comparing)],
                    annotation: format!("Compare arr[{}]={} and arr[{}]={}", j, arr[j], j + 1, arr[j + 1]),
                });

                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                    frames.push(VizFrame {
                        array: arr.clone(),
                        highlights: vec![(j, HighlightKind::Swapping), (j + 1, HighlightKind::Swapping)],
                        annotation: format!("Swap! arr[{}] and arr[{}]", j, j + 1),
                    });
                }
            }
            frames.push(VizFrame {
                array: arr.clone(),
                highlights: vec![(n - 1 - i, HighlightKind::Sorted)],
                annotation: format!("Element {} is now in its final position", arr[n - 1 - i]),
            });
        }

        frames.push(VizFrame {
            array: arr.clone(),
            highlights: (0..n).map(|i| (i, HighlightKind::Sorted)).collect(),
            annotation: "Array is sorted!".to_string(),
        });
        frames
    }
}

// ── Selection Sort ─────────────────────────────────────────────────────

pub struct SelectionSortViz;

impl ReferenceViz for SelectionSortViz {
    fn id(&self) -> &str { "selection_sort" }
    fn name(&self) -> &str { "Selection Sort" }
    fn description(&self) -> &str {
        "Finds the minimum element from the unsorted part and puts it at the beginning. \
         O(n^2) time, O(1) space."
    }
    fn default_input(&self) -> Vec<i32> { vec![64, 25, 12, 22, 11] }

    fn generate_frames(&self, input: &[i32]) -> Vec<VizFrame> {
        let mut arr = input.to_vec();
        let n = arr.len();
        let mut frames = vec![VizFrame {
            array: arr.clone(),
            highlights: vec![],
            annotation: "Initial array".to_string(),
        }];

        for i in 0..n - 1 {
            let mut min_idx = i;
            frames.push(VizFrame {
                array: arr.clone(),
                highlights: vec![(i, HighlightKind::Active)],
                annotation: format!("Finding minimum in unsorted portion starting at index {}", i),
            });

            for j in i + 1..n {
                frames.push(VizFrame {
                    array: arr.clone(),
                    highlights: vec![(min_idx, HighlightKind::Active), (j, HighlightKind::Comparing)],
                    annotation: format!("Compare current min arr[{}]={} with arr[{}]={}", min_idx, arr[min_idx], j, arr[j]),
                });
                if arr[j] < arr[min_idx] {
                    min_idx = j;
                }
            }

            if min_idx != i {
                arr.swap(i, min_idx);
                frames.push(VizFrame {
                    array: arr.clone(),
                    highlights: vec![(i, HighlightKind::Swapping), (min_idx, HighlightKind::Swapping)],
                    annotation: format!("Swap arr[{}] and arr[{}]", i, min_idx),
                });
            }

            frames.push(VizFrame {
                array: arr.clone(),
                highlights: vec![(i, HighlightKind::Sorted)],
                annotation: format!("Position {} is finalized with value {}", i, arr[i]),
            });
        }

        frames.push(VizFrame {
            array: arr.clone(),
            highlights: (0..n).map(|i| (i, HighlightKind::Sorted)).collect(),
            annotation: "Array is sorted!".to_string(),
        });
        frames
    }
}

// ── Insertion Sort ─────────────────────────────────────────────────────

pub struct InsertionSortViz;

impl ReferenceViz for InsertionSortViz {
    fn id(&self) -> &str { "insertion_sort" }
    fn name(&self) -> &str { "Insertion Sort" }
    fn description(&self) -> &str {
        "Builds the sorted array one element at a time by inserting each element \
         into its correct position. O(n^2) worst case, O(n) best case."
    }
    fn default_input(&self) -> Vec<i32> { vec![5, 2, 4, 6, 1, 3] }

    fn generate_frames(&self, input: &[i32]) -> Vec<VizFrame> {
        let mut arr = input.to_vec();
        let n = arr.len();
        let mut frames = vec![VizFrame {
            array: arr.clone(),
            highlights: vec![(0, HighlightKind::Sorted)],
            annotation: "First element is trivially sorted".to_string(),
        }];

        for i in 1..n {
            let key = arr[i];
            frames.push(VizFrame {
                array: arr.clone(),
                highlights: vec![(i, HighlightKind::Active)],
                annotation: format!("Insert arr[{}]={} into sorted portion", i, key),
            });

            let mut j = i;
            while j > 0 && arr[j - 1] > key {
                frames.push(VizFrame {
                    array: arr.clone(),
                    highlights: vec![(j, HighlightKind::Comparing), (j - 1, HighlightKind::Comparing)],
                    annotation: format!("arr[{}]={} > {} — shift right", j - 1, arr[j - 1], key),
                });
                arr[j] = arr[j - 1];
                j -= 1;
            }
            arr[j] = key;

            let sorted_highlights: Vec<_> = (0..=i).map(|k| (k, HighlightKind::Sorted)).collect();
            frames.push(VizFrame {
                array: arr.clone(),
                highlights: sorted_highlights,
                annotation: format!("Inserted {} at position {}", key, j),
            });
        }

        frames.push(VizFrame {
            array: arr.clone(),
            highlights: (0..n).map(|i| (i, HighlightKind::Sorted)).collect(),
            annotation: "Array is sorted!".to_string(),
        });
        frames
    }
}

// ── Merge Sort ─────────────────────────────────────────────────────────

pub struct MergeSortViz;

impl ReferenceViz for MergeSortViz {
    fn id(&self) -> &str { "merge_sort" }
    fn name(&self) -> &str { "Merge Sort" }
    fn description(&self) -> &str {
        "Divides the array in half, recursively sorts each half, then merges. \
         O(n log n) time, O(n) space."
    }
    fn default_input(&self) -> Vec<i32> { vec![38, 27, 43, 3, 9, 82, 10] }

    fn generate_frames(&self, input: &[i32]) -> Vec<VizFrame> {
        let mut arr = input.to_vec();
        let mut frames = Vec::new();
        frames.push(VizFrame {
            array: arr.clone(),
            highlights: vec![],
            annotation: "Initial array".to_string(),
        });
        let len = arr.len();
        merge_sort_frames(&mut arr, 0, len, &mut frames);
        let n = arr.len();
        frames.push(VizFrame {
            array: arr,
            highlights: (0..n).map(|i| (i, HighlightKind::Sorted)).collect(),
            annotation: "Array is sorted!".to_string(),
        });
        frames
    }
}

fn merge_sort_frames(arr: &mut Vec<i32>, lo: usize, hi: usize, frames: &mut Vec<VizFrame>) {
    if hi - lo <= 1 {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    let highlights: Vec<_> = (lo..mid).map(|i| (i, HighlightKind::Active))
        .chain((mid..hi).map(|i| (i, HighlightKind::Comparing)))
        .collect();
    frames.push(VizFrame {
        array: arr.clone(),
        highlights,
        annotation: format!("Split [{}, {}) into [{}, {}) and [{}, {})", lo, hi, lo, mid, mid, hi),
    });

    merge_sort_frames(arr, lo, mid, frames);
    merge_sort_frames(arr, mid, hi, frames);

    // Merge
    let left = arr[lo..mid].to_vec();
    let right = arr[mid..hi].to_vec();
    let (mut i, mut j, mut k) = (0, 0, lo);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }

    let highlights: Vec<_> = (lo..hi).map(|i| (i, HighlightKind::Sorted)).collect();
    frames.push(VizFrame {
        array: arr.clone(),
        highlights,
        annotation: format!("Merged [{}, {}): {:?}", lo, hi, &arr[lo..hi]),
    });
}

// ── Quick Sort ─────────────────────────────────────────────────────────

pub struct QuickSortViz;

impl ReferenceViz for QuickSortViz {
    fn id(&self) -> &str { "quick_sort" }
    fn name(&self) -> &str { "Quick Sort" }
    fn description(&self) -> &str {
        "Picks a pivot, partitions around it, then recursively sorts partitions. \
         O(n log n) average, O(n^2) worst case."
    }
    fn default_input(&self) -> Vec<i32> { vec![10, 7, 8, 9, 1, 5] }

    fn generate_frames(&self, input: &[i32]) -> Vec<VizFrame> {
        let mut arr = input.to_vec();
        let n = arr.len();
        let mut frames = Vec::new();
        frames.push(VizFrame {
            array: arr.clone(),
            highlights: vec![],
            annotation: "Initial array".to_string(),
        });
        if n > 0 {
            quick_sort_frames(&mut arr, 0, n - 1, &mut frames);
        }
        frames.push(VizFrame {
            array: arr,
            highlights: (0..n).map(|i| (i, HighlightKind::Sorted)).collect(),
            annotation: "Array is sorted!".to_string(),
        });
        frames
    }
}

fn quick_sort_frames(arr: &mut Vec<i32>, lo: usize, hi: usize, frames: &mut Vec<VizFrame>) {
    if lo >= hi {
        return;
    }
    let pivot_val = arr[hi];
    frames.push(VizFrame {
        array: arr.clone(),
        highlights: vec![(hi, HighlightKind::Pivot)],
        annotation: format!("Pivot = arr[{}] = {}", hi, pivot_val),
    });

    let mut i = lo;
    for j in lo..hi {
        frames.push(VizFrame {
            array: arr.clone(),
            highlights: vec![(j, HighlightKind::Comparing), (hi, HighlightKind::Pivot), (i, HighlightKind::Active)],
            annotation: format!("Compare arr[{}]={} with pivot {}", j, arr[j], pivot_val),
        });
        if arr[j] < pivot_val {
            if i != j {
                arr.swap(i, j);
                frames.push(VizFrame {
                    array: arr.clone(),
                    highlights: vec![(i, HighlightKind::Swapping), (j, HighlightKind::Swapping), (hi, HighlightKind::Pivot)],
                    annotation: format!("Swap arr[{}] and arr[{}]", i, j),
                });
            }
            i += 1;
        }
    }
    arr.swap(i, hi);
    frames.push(VizFrame {
        array: arr.clone(),
        highlights: vec![(i, HighlightKind::Sorted)],
        annotation: format!("Pivot {} placed at index {}", pivot_val, i),
    });

    if i > lo {
        quick_sort_frames(arr, lo, i - 1, frames);
    }
    if i < hi {
        quick_sort_frames(arr, i + 1, hi, frames);
    }
}

// ── Binary Search ──────────────────────────────────────────────────────

pub struct BinarySearchViz;

impl ReferenceViz for BinarySearchViz {
    fn id(&self) -> &str { "binary_search" }
    fn name(&self) -> &str { "Binary Search" }
    fn description(&self) -> &str {
        "Searches a sorted array by repeatedly dividing the search interval in half. \
         O(log n) time."
    }
    fn default_input(&self) -> Vec<i32> { vec![2, 5, 8, 12, 16, 23, 38, 56, 72, 91] }

    fn generate_frames(&self, input: &[i32]) -> Vec<VizFrame> {
        let arr = input.to_vec();
        // Search for a value that exists (pick the middle-ish one)
        let target = arr[arr.len() * 2 / 3];
        let mut frames = Vec::new();

        frames.push(VizFrame {
            array: arr.clone(),
            highlights: vec![],
            annotation: format!("Search for target = {}", target),
        });

        let mut lo = 0i32;
        let mut hi = arr.len() as i32 - 1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let mid_u = mid as usize;

            let mut highlights: Vec<_> = vec![
                (lo as usize, HighlightKind::Active),
                (mid_u, HighlightKind::Comparing),
            ];
            if hi as usize != lo as usize {
                highlights.push((hi as usize, HighlightKind::Active));
            }

            frames.push(VizFrame {
                array: arr.clone(),
                highlights,
                annotation: format!(
                    "lo={}, hi={}, mid={} | arr[mid]={} vs target={}",
                    lo, hi, mid, arr[mid_u], target
                ),
            });

            if arr[mid_u] == target {
                frames.push(VizFrame {
                    array: arr.clone(),
                    highlights: vec![(mid_u, HighlightKind::Found)],
                    annotation: format!("Found target {} at index {}!", target, mid_u),
                });
                break;
            } else if arr[mid_u] < target {
                frames.push(VizFrame {
                    array: arr.clone(),
                    highlights: vec![(mid_u, HighlightKind::Comparing)],
                    annotation: format!("arr[{}]={} < {} — search right half", mid, arr[mid_u], target),
                });
                lo = mid + 1;
            } else {
                frames.push(VizFrame {
                    array: arr.clone(),
                    highlights: vec![(mid_u, HighlightKind::Comparing)],
                    annotation: format!("arr[{}]={} > {} — search left half", mid, arr[mid_u], target),
                });
                hi = mid - 1;
            }
        }

        frames
    }
}
