//! `queue`
//!
//! The `queue` module containing queue related algorithms

/// The `max_sliding_window` algorithm
pub mod max_sliding_window;
pub use max_sliding_window::max_sliding_window;
/// The `moving_average` algorithm
pub mod moving_average;
pub use moving_average::MovingAverage;
/// The `reconstruct_queue` algorithm
pub mod reconstruct_queue;
pub use reconstruct_queue::reconstruct_queue;
/// The `zigzagiterator` algorithm
pub mod zigzagiterator;
pub use zigzagiterator::ZigZagIterator;
