//! `linked_list`
//!
//! The `linked_list` module containing queue related algorithms

/// The `add_two_numbers` algorithm
pub mod add_two_numbers;
pub use add_two_numbers::{Node, from_vec, to_vec, add_two_numbers};
/// The `copy_random_pointer` algorithm
pub mod copy_random_pointer;
pub use copy_random_pointer::{copy_random_pointer, RandomListNode};
