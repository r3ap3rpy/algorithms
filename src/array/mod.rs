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
/// The `garage` algorithm
pub mod garage;
pub use garage::garage;
/// The `josephus` algorithm
pub mod josephus;
pub use josephus::Josephus;
