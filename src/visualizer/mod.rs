pub mod engine;
pub mod reference;

/// What kind of highlighting to apply to an element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

/// A single frame of a visualization.
#[derive(Debug, Clone)]
pub struct VizFrame {
    /// The array state at this point
    pub array: Vec<i32>,
    /// Which indices are highlighted and how
    pub highlights: Vec<(usize, HighlightKind)>,
    /// Text annotation for this step
    pub annotation: String,
}

/// A reference visualization for a specific algorithm.
pub trait ReferenceViz {
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
        "binary_search",
    ]
}
