//! # Sorting
//! `sorting` is a submodule that contains sorting based algorithms.

/// The `bead_sort` algorithm
pub mod bead_sort;
pub use bead_sort::bead_sort;
/// The `bubble_sort` algorithm
pub mod bubble_sort;
pub use bubble_sort::bubble_sort;
/// The `insertion_sort` algorithm
pub mod insertion_sort;
pub use insertion_sort::insertion_sort;
/// The `counting_sort` algorithm
pub mod counting_sort;
pub use counting_sort::counting_sort;
