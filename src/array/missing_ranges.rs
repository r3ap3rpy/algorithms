/// Find gaps between low and high not covered by elements in array.
///
/// Takes a list reference and a low and a high as argument and returns a `Vec(i32,i32)` or an
/// error explaining the situation.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::missing_ranges(&[3,5],1,10).unwrap();
/// assert_eq!(result,vec![(1,2),(4,4),(6,10)]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::missing_ranges;
/// match missing_ranges(&[3,5],1,10) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The problems was: {}",e),
/// }
/// ```
pub fn missing_ranges(list: &[i32], low: i32, high: i32) -> Result<Vec<(i32,i32)>,String> {
    if list.is_empty() {
        return Err("Cannot use missing ranges on an empty list!".to_string());
    }
    let mut result: Vec<(i32,i32)> = Vec::new();
    let mut start = low;
    for num in list {
        if *num == start {
            start += 1;
        } else if *num > start {
            result.push((start, num - 1));
            start = num + 1;
        }
    }
    if start <= high {
        result.push((start, high));
    }
    Ok(result)
}

