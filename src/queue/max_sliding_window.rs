/// Max Sliding Window (Deque-based)
///
/// Given an array and a window size k, find the maximum element in each sliding window using a monotonic deque that stores indices of elements in decreasing order of their values.
///
/// # Examples
///
/// Basic example:
/// ```
/// let result = algorithmz::queue::max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3).unwrap();
/// assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::queue::max_sliding_window;
/// match max_sliding_window(&[1,3,-1-3,5,3,6,7],3) {
///     Ok(n) => println!("{:?}",n),
///     Err(e) => eprintln!("{}",e),
/// }
pub fn max_sliding_window(list: &[i32], k: usize) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use max_sliding_window on an empty list!".to_string());
    }
    let mut index_dequeue: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    let mut result: Vec<i32> = Vec::new();
    for (i, &value) in list.iter().enumerate() {
        while let Some(&idx) = index_dequeue.back() {
            if list[idx] < value {
                index_dequeue.pop_back();
            } else {
                break;
            }
        }
        index_dequeue.push_back(i);
        if let Some(&front) = index_dequeue.front() {
            if front + k <= i {
                index_dequeue.pop_front();
            }
        }
        if i + 1 >= k {
            if let Some(&front) = index_dequeue.front() {
                result.push(list[front]);
            }
        }
    }
    Ok(result)
}
