/// Slutter
///
/// Replace every value in a stack with two occurrences of that value.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::stack::slutter(vec![3, 7, 1, 14, 9]);
/// assert_eq!(result, vec![3, 3, 7, 7, 1, 1, 14, 14, 9, 9]);
/// ```
pub fn slutter(mut input: Vec<i32>) -> Vec<i32> {
    let mut storage_stack: Vec<i32> = Vec::new();

    for _ in 0..input.len() {
        storage_stack.push(input.pop().unwrap());
    }
    for _ in 0..storage_stack.len() {
        let val = storage_stack.pop().unwrap();
        input.push(val);
        input.push(val);
    }
    return input;
}
