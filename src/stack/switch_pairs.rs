/// Switch pairs
///
/// Switch successive pairs of values in a stack starting from the bottom. If there is an odd number of values, the top element is not moved. Two approaches: one using an auxiliary stack, one using an auxiliary queue.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::stack::switch_pairs(vec![3, 8, 17, 9, 1, 10]);
/// assert_eq!(result, vec![8, 3, 9, 17, 10, 1]);
/// ```
pub fn switch_pairs(mut input: Vec<i32>) -> Vec<i32> {
    let mut storage_stack: Vec<i32> = Vec::new();

    for _ in 0..input.len() {
        storage_stack.push(input.pop().unwrap());
    }
    for _ in 0..((storage_stack.len() + 1) / 2) {
        let first = storage_stack.pop().unwrap();
        if storage_stack.len() == 0 {
            input.push(first);
            break;
        }
        let second = storage_stack.pop().unwrap();
        input.push(second);
        input.push(first);
    }
    return input;
}

