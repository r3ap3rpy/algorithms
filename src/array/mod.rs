//! # array
//!
//! `array` is a submodule that contains array manipulation techniques.

/// The `flatten` algorithm
pub mod flatten;
pub use flatten::flatten;
pub use flatten::Nested;
/// The `delete_nth` algorithm
pub mod delete_nth;
pub use delete_nth::delete_nth;
pub use delete_nth::delete_nth_naive;
