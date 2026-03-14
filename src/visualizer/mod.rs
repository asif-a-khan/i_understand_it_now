pub mod engine;
pub mod instrumented;
#[allow(
    clippy::needless_range_loop,
    clippy::int_plus_one,
    clippy::collapsible_else_if,
    clippy::unnecessary_cast,
    clippy::manual_saturating_arithmetic,
    unused_variables,
    unused_mut
)]
pub mod problem_viz;
pub mod reference;

/// What kind of highlighting to apply to an element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum HighlightKind {
    /// Currently being compared
    Comparing,
    /// Currently being swapped
    Swapping,
    /// Already sorted / in final position
    Sorted,
    /// Active pointer or cursor
    Active,
    /// Pivot element
    Pivot,
    /// Found / target element
    Found,
    /// Element being read
    Reading,
    /// Element being written
    Writing,
    /// Search target — what the algorithm is looking for
    Target,
}

/// Typed visualization data — determines which renderer is used.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum VizData {
    /// Flat array/string — rendered as horizontal boxes.
    Array { values: Vec<String> },
    /// Binary tree — rendered with spatial node layout (level-order).
    Tree { nodes: Vec<Option<String>> },
    /// Graph — 2D spatial layout with edges.
    Graph {
        n: usize,
        labels: Vec<String>,
        edges: Vec<(usize, usize)>,
        weighted_edges: Vec<(usize, usize, String)>,
        directed: bool,
    },
    /// 2D grid — colored cell matrix.
    Grid { cells: Vec<Vec<String>> },
    /// No visual data (theory/scalar problems).
    None { message: String },
}

/// A single frame of a visualization.
#[derive(Debug, Clone, Default)]
pub struct VizFrame {
    /// Legacy array data — used when `data` is None.
    pub array: Vec<i32>,
    /// Typed visualization data. When Some, the renderer uses this instead of `array`.
    pub data: Option<VizData>,
    /// Which indices are highlighted and how
    pub highlights: Vec<(usize, HighlightKind)>,
    /// Text annotation for this step
    pub annotation: String,
    /// Named pointers to show below the array (index, label)
    pub pointers: Vec<(usize, String)>,
}

impl VizFrame {
    /// Get the effective VizData for rendering. Falls back to converting `array` to strings.
    pub fn viz_data(&self) -> VizData {
        if let Some(ref data) = self.data {
            data.clone()
        } else if self.array.is_empty() && self.annotation.is_empty() {
            VizData::None {
                message: "No visualization data".to_string(),
            }
        } else {
            VizData::Array {
                values: self.array.iter().map(|v| v.to_string()).collect(),
            }
        }
    }
}

/// A reference visualization for a specific algorithm.
pub trait ReferenceViz {
    #[allow(dead_code)]
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    /// Generate a default input for the visualization.
    fn default_input(&self) -> Vec<i32>;

    /// Generate all frames for the visualization.
    fn generate_frames(&self, input: &[i32]) -> Vec<VizFrame>;
}

/// Look up a reference visualization by algorithm name.
pub fn get_reference(name: &str) -> Option<Box<dyn ReferenceViz>> {
    match name {
        "bubble_sort" => Some(Box::new(reference::BubbleSortViz)),
        "selection_sort" => Some(Box::new(reference::SelectionSortViz)),
        "insertion_sort" => Some(Box::new(reference::InsertionSortViz)),
        "merge_sort" => Some(Box::new(reference::MergeSortViz)),
        "quick_sort" => Some(Box::new(reference::QuickSortViz)),
        "binary_search" => Some(Box::new(reference::BinarySearchViz)),
        "heap_sort" => Some(Box::new(reference::HeapSortViz)),
        _ => None,
    }
}

/// List all available reference visualization identifiers.
pub fn list_references() -> Vec<&'static str> {
    vec![
        "bubble_sort",
        "selection_sort",
        "insertion_sort",
        "merge_sort",
        "quick_sort",
        "heap_sort",
        "binary_search",
    ]
}
