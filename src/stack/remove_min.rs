/// Remove min
///
/// Remove the smallest value from a stack, preserving the relative order of the remaining elements.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::stack::remove_min(vec![]);
/// assert_eq!(result, vec![]);
/// ```
pub fn remove_min(mut stack: Vec<i32>) -> Vec<i32> {
    let mut storage_stack: Vec<i32> = Vec::new();
    if stack.len() == 0 {
        return stack;
    }
    let mut minimum = stack.pop().unwrap();
    stack.push(minimum);
    for _ in 0..stack.len() {
        let val = stack.pop().unwrap();
        if val <= minimum {
            minimum = val;
        }
        storage_stack.push(minimum);
    }
    for _ in 0..storage_stack.len() {
        let val = storage_stack.pop().unwrap();
        if val != minimum {
            stack.push(val);
        }
    }
    return stack;
}
