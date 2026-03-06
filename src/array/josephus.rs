/// The struct for implementing Josephus
///
/// Yield elements eliminated in Josephus-problem order.
///
/// # Examples
///
/// Basic example:
/// ``` 
/// let result: Vec<i32> = algorithmz::array::Josephus::new(vec![1,2,3,4,5,6],3).collect();
/// assert_eq!(result,[3, 6, 4, 2, 5, 1]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::Josephus;
/// let mut game = Josephus::new(vec!["Alice", "Bob", "Charlie"], 2);
/// assert_eq!(game.next().unwrap(), "Bob");
/// assert_eq!(game.next().unwrap(), "Alice");
/// assert_eq!(game.next().unwrap(), "Charlie");
/// assert!(game.next().is_none());
/// ```
pub struct Josephus<T> {
    items: Vec<T>,
    skip: usize,
    current_index: usize,
}

/// Constructor for Josephus
impl<T> Josephus<T> {
    /// The function to initialize the instance
    pub fn new(items: Vec<T>, skip: usize) -> Self {
        Josephus {
            items,
            skip,
            current_index: 0,
        }
    }
}

/// Iterator for Josephus
impl<T> Iterator for Josephus<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return None;
        }
        self.current_index = (self.current_index + self.skip - 1) % self.items.len();
        Some(self.items.remove(self.current_index))
    }
}

