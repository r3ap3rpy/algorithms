/// Return elements of array that fall within [min_limit, max_limit].
///
/// Returns a new list containing only values within the specified range.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::limit(&[1,2,3,4,5],2,4).unwrap();
/// assert_eq!(result,vec![2,3,4]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::limit;
/// let my_list = vec![1,2,3,4,5];
/// match limit(&my_list,2,4) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn limit(list: &[i32], min_limit: i32, max_limit: i32) -> Result<Vec<i32>, String>{
    if list.is_empty() {
        return Err("Cannot use limit on an empty list!".to_string());
    }
    let result: Vec<i32> = list.iter().filter(|&&x| min_limit <= x && x <= max_limit).cloned().collect();
    Ok(result)
}


