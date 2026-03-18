/// Zigzag Iterator
///
/// Interleave elements from two lists in a zigzag fashion. Elements are yielded alternately from each list until both are exhausted.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let mut it = algorithmz::queue::ZigZagIterator::new(vec![1,2],vec![3,4,5]);
/// while it.has_next() {
///     println!("{}",it.next().unwrap());
/// }
/// ```
pub struct ZigZagIterator {
    queue: std::collections::VecDeque<std::vec::IntoIter<i32>>,
}

impl ZigZagIterator {
    /// new iterator constructor
    pub fn new(v1: Vec<i32>,v2: Vec<i32>) -> Self {
        let mut queue = std::collections::VecDeque::new();
        if !v1.is_empty() {
            queue.push_back(v1.into_iter());
        }
        if !v2.is_empty() {
            queue.push_back(v2.into_iter());
        }
        Self { queue }
    }
    /// grab the next item
    pub fn next(&mut self) -> Option<i32> {
        if let Some(mut iter) = self.queue.pop_front() {
            if let Some(val) = iter.next() {
                if !iter.as_slice().is_empty() {
                    self.queue.push_back(iter);
                }
                return Some(val);
            }
        }
        None
    }
    /// is there a next element?
    pub fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }
}
