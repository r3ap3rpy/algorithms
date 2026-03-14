//! # set
//!
//! `set` is a submodule that contains set related algorithms

/// The `find_keyboard_row` algorithm
pub mod find_keyboard_row;
pub use find_keyboard_row::find_keyboard_row;
/// The `randomized_set` algorithm
pub mod randomized_set;
pub use randomized_set::RandomizedSet;
/// The `set_covering` algorithm
pub mod set_covering;
pub use set_covering::set_cover;
