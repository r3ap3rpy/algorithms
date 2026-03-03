/// Returns the wiggle sorted version of the input.
///
/// Takes a list reference and returns a `Vec<i32>` or an error explaining the situation.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::wiggle_sort(&[3, 5, 2, 1, 6, 4]).unwrap();
/// assert_eq!(result,[3, 5, 1, 6, 2, 4]);
/// ```
///
/// Match example:
/// ``` 
/// use algorithmz::sorting::wiggle_sort;
/// match wiggle_sort(&[3, 5, 2, 1, 6, 4]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn wiggle_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use wiggle sort on an empty input!".to_string());
    }
    let mut result = list.to_vec();
    for i in 1..result.len() {
        if (i % 2 == 1) == (result[i-1] > result[i]) {
            result.swap(i-1,i);
        }
    }
    Ok(result)
}

