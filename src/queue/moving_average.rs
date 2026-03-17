/// The struct representing the moving average
pub struct MovingAverage {
    /// queue for the struct
    queue: std::collections::VecDeque<i32>,
    /// capacity of the struct
    capacity: usize,
    /// sub of the struct
    sum: i32,
}
/// Computes the moving average over a sliding window.
///
/// # Examples
///
/// Basic example:
/// ```
/// let mut ma = algorithmz::queue::MovingAverage::new(3);
/// assert_eq!(ma.next(1),1.0);
/// assert_eq!(ma.next(10), 5.5);
/// ```
///
/// Another example
/// ``` 
/// use algorithmz::queue::MovingAverage;
/// let mut ma = MovingAverage::new(5);
/// ma.next(1);
/// ma.next(3);
/// ma.next(4);
/// assert_eq!(ma.queue(), vec![1,3,4]);
/// ```
impl MovingAverage {
    /// Instantiate the struct
    pub fn new(size: usize) -> Self {
        MovingAverage  {
            queue: std::collections::VecDeque::with_capacity(size),
            capacity: size,
            sum: 0,
        }
    }
    /// Grab next element of the struct
    pub fn next(&mut self, val: i32) -> f64 {
        if self.queue.len() == self.capacity {
            if let Some(old) = self.queue.pop_front() {
                self.sum -= old;
            }
        }
        self.queue.push_back(val);
        self.sum += val;
        self.sum as f64 / self.queue.len() as f64
    }
    /// Show the queue
    pub fn queue(self) -> std::collections::VecDeque<i32> {
        self.queue
    }
}


