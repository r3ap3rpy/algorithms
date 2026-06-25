//! `stack`
//!
//! `stack` is a submodule containing algorithms for stack operations

/// The `is_consecutive` algorithm
pub mod is_consecutive;
pub use is_consecutive::is_consecutive;
/// The `is_sorted` algorithm
pub mod is_sorted;
pub use is_sorted::is_sorted;
/// The `longest_path` algorithm
pub mod longest_path;
pub use longest_path::longest_path;
/// The `OrderedStack` implementation
pub mod ordered_stack;
pub use ordered_stack::OrderedStack;
