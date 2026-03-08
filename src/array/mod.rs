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
/// The `limit` algorithm
pub mod limit;
pub use limit::limit;
/// The `longest_non_repeat` algorithm
pub mod longest_non_repeat;
pub use longest_non_repeat::longest_non_repeat;
/// The `max_ones_index` algorithm
pub mod max_ones_index;
pub use max_ones_index::max_ones_index;
/// The `missing_ranges` algorithm
pub mod missing_ranges;
pub use missing_ranges::missing_ranges;
