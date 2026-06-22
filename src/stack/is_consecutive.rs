/// The is_consecutive function
///
/// heck whether a stack contains a sequence of consecutive integers starting from the bottom. Two approaches are provided: one using an auxiliary stack, and one using an auxiliary queue.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::stack::is_consecutive(vec![3,4,5,6,7]);
/// assert_eq!(result, true);
/// ```
pub fn is_consecutive(mut stack: Vec<i32>) -> bool {
    let mut storage_stack: Vec<i32> = Vec::new();
    for _ in 0..stack.len() {
        let first: i32 = stack.pop().unwrap();
        if stack.len() == 0 {
            return true;
        }
        let second: i32 = stack.pop().unwrap();
        if first - second != 1 {
            return false;
        }
        stack.push(second);
        storage_stack.push(first);
    }
    for _ in 0..storage_stack.len() {
        stack.push(storage_stack.pop().unwrap());
    }
    return true;
}

