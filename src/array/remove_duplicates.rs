/// Remove duplicate elements from an array, preserving order.
///
/// Takes a list reference and returns a `Vec<i32>` or an error explaining the situation.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::remove_duplicates(&[1,1,2,3,3,4,4,5]).unwrap();
/// assert_eq!(result, [1,2,3,4,5]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::remove_duplicates;
/// match remove_duplicates(&[1,1,2,3,3,4,5,5,9]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn remove_duplicates(list: &[i32]) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Cannot remove duplicates from an empty list.".to_string());
    }
    let mut result: Vec<i32> = Vec::new();
    let mut seen: Vec<i32> = Vec::new();
    for item in list {
        if !seen.contains(item) {
            seen.push(*item);
            result.push(*item);
        }
    }
    Ok(result)
}

