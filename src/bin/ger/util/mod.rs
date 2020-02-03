pub mod validate;

/// Function to check if boolean is false.
/// Used for serde 'path' attributes.
pub fn is_false(b: &bool) -> bool {
    !*b
}
