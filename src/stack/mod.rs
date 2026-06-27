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
/// The `remove_min` algorithm
pub mod remove_min;
pub use remove_min::remove_min;
/// The `simplify_path` algorithm
pub mod simplify_path;
pub use simplify_path::simplify_path;
/// The `slutter` algorithm
pub mod slutter;
pub use slutter::slutter;
/// The `switch_pairs` algorithm
pub mod switch_pairs;
pub use switch_pairs::switch_pairs;
/// The `valid_parenthesis` algorithm
pub mod valid_parenthesis;
pub use valid_parenthesis::valid_parenthesis;
