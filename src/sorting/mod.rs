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
/// The `cycle_sort` algorithm
pub mod cycle_sort;
pub use cycle_sort::cycle_sort;
/// The `shell_sort` algorithm
pub mod shell_sort;
pub use shell_sort::shell_sort;
/// The `quick_sort` algorithm
pub mod quick_sort;
pub use quick_sort::quick_sort;
/// The `bitonic_sort` algorithm
pub mod bitonic_sort;
pub use bitonic_sort::bitonic_sort;
/// The `bogo_sort` algorithm
pub mod bogo_sort;
pub use bogo_sort::bogo_sort;
/// The `bucket_sort` algorithm
pub mod bucket_sort;
pub use bucket_sort::bucket_sort;
/// The `cocktail_shaker_sort` algorithm
pub mod cocktail_shaker_sort;
pub use cocktail_shaker_sort::cocktail_shaker_sort;
/// The `comb_sort` algorithm
pub mod comb_sort;
pub use comb_sort::comb_sort;
/// The `exchange_sort` algorithm
pub mod exchange_sort;
pub use exchange_sort::exchange_sort;
/// The `gnome_sort` algorithm
pub mod gnome_sort;
pub use gnome_sort::gnome_sort;
