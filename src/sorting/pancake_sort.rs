/// Returns the pancake sorted version of the input.
///
/// Takes a list reference and returns `Vec<i32>` or an error explaining the problem.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::pancake_sort(&[2,1,4,3,6,5]).unwrap();
/// assert_eq!(result,[1,2,3,4,5,6]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::pancake_sort;
/// let my_list = vec![2,1,4,3,6,5];
/// match pancake_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn pancake_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use pancake sort on an empty list!".to_string());
    }
    if list.len() == 1 {
        return Ok(list.to_vec());
    }
    let mut result = list.to_vec();
    for i in (2..=result.len()).rev() {
        let index_max = result[0..i].iter().enumerate().max_by_key(|&(_idx, &val)| val).map(|(idx, _val)| idx).unwrap_or(0);
        if index_max != i - 1 {
            if index_max != 0 {
                result[0..=index_max].reverse();
            }
            result[0..i].reverse();
        }
    }
    Ok(result)
}

