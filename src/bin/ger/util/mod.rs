pub mod validate;

/// Function to check if boolean is true.
/// Used for serde 'path' attributes.
pub fn is_true(b: &bool) -> bool {
    *b
}

/// Yield boolean true
/// Used for serde 'path' attributes.
pub fn new_true() -> bool {
    true
}
