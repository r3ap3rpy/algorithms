//! # Sorting
//! `sorting` is a submodule that contains sorting based algorithms.

/// The `bead_sort` algorithm
pub mod bead_sort;
pub use bead_sort::bead_sort;
/// The `bubble_sort` algorithm
pub mod bubble_sort;
pub use bubble_sort::bubble_sort;
/// The `heap_sort` algorithm
pub mod heap_sort;
pub use heap_sort::heap_sort;
