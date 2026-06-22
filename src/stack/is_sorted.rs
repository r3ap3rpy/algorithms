/// Is Sorted
///
/// Check whether a stack is sorted in ascending order from bottom to top using a single auxiliary stack.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::stack::is_sorted(vec![3,4,5]);
/// assert_eq!(result, true);
/// ```
pub fn is_sorted(mut stack: Vec<i32>) -> bool {
    let mut storage_stack: Vec<i32> = Vec::new();
    if stack.len() == 0 {
        return true;
    }
    for _ in 0..stack.len() {
        let first = stack.pop().unwrap();
        storage_stack.push(first);
        if stack.len() == 0 {
            return true;
        }
        let second = stack.pop().unwrap();
        if first < second {
            return false;
        }
        stack.push(second);
    }
    return true;
}
