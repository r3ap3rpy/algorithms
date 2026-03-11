/// Summarize consecutive runs in a sorted array as (start, end) tuples.
///
/// Takes a list reference and returns a `Vec<(i32,i32)>` or the corresponding error.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::summarize_ranges(&[0, 1, 2, 4, 5, 7]).unwrap();
/// assert_eq!(result, vec![(0, 2), (4, 5), (7, 7)]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::summarize_ranges;
/// match summarize_ranges(&[0,1,2,4,5,7]) {
///     Ok(n) => println!("Result was: {:?}",n),
///     Err(e) => eprintln!("Error was: {}",e),
/// }
/// ```
pub fn summarize_ranges(list: &[i32]) -> Result<Vec<(i32,i32)>,String> {
    if list.is_empty() {
        return Err("Cannot summarize ranges of an empty list!".to_string());
    }
    let mut result: Vec<(i32,i32)> = Vec::new();
    let mut iterator = list.iter();
    let value = iterator.next().unwrap();
    let mut start = value;
    let mut end = value;
    for num in iterator {
        if num - end == 1 {
            end = num;
        } else {
            result.push((*start, *end));
            start = num;
            end = num;
        }
    }
    result.push((*start,*end));
    Ok(result)
}
