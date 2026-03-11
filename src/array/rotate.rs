/// Rotate array to the right by k steps using repeated single shifts.
///
/// Takes a list reference as input and returns a `Vec<i32>` or a corresponding error.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::rotate(&[1,1,2,3,3,4,5,5,9],3).unwrap();
/// assert_eq!(result,vec![5, 5, 9, 1, 1, 2, 3, 3, 4]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::rotate;
/// match rotate(&[1,1,2,3,3,4,5,5,9],3) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn rotate(list: &[i32], steps: i32) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot rotate an empty list!".to_string());
    }
    let mut result: Vec<i32> = list.to_vec();
    let length = result.len();
    for _ in 0..steps {
        let temp = result[length - 1];
        for position in (1..length).rev() {
            result[position] = result[position - 1];
        }
        result[0] = temp;
    }
    Ok(result)
}

