pub mod reference;

/// A reference visualization for a specific algorithm.
pub trait ReferenceViz {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
}

/// Look up a reference visualization by algorithm name.
pub fn get_reference(name: &str) -> Option<Box<dyn ReferenceViz>> {
    match name {
        "bubble_sort" => Some(Box::new(reference::BubbleSortViz)),
        _ => None,
    }
}

/// List all available reference visualization identifiers.
pub fn list_references() -> Vec<&'static str> {
    vec!["bubble_sort"]
}
