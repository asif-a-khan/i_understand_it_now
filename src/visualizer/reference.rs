use super::ReferenceViz;

pub struct BubbleSortViz;

impl ReferenceViz for BubbleSortViz {
    fn id(&self) -> &str {
        "bubble_sort"
    }

    fn name(&self) -> &str {
        "Bubble Sort"
    }
}
