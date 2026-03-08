/// Find the index of 0 to replace with 1 for the longest run of 1s.
///
/// Takes a list reference and returns an `i32` or an error detailing the problem.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::max_ones_index(&[1,1,1,0,1,1,1,1,1,0,1,1,1]).unwrap();
/// assert_eq!(result,3);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::max_ones_index;
/// match max_ones_index(&[1,1,1,0,1,1,1,1,1,0,1,1,1]) {
///     Ok(n) => println!("The result was: {}",n),
///     Err(e) => eprintln!("The problems was: {}",e),
/// }
/// ```
pub fn max_ones_index(list: &[i32]) -> Result<i32, String> {
    if list.is_empty() {
        return Err("Cannot use max ones on an empty list!".to_string());
    }
    if !list.iter().all(|&x| x == 0 || x == 1) {
        return Err("The list may only contain 1-s and 0-s!".to_string());
    }
    let length = list.len();
    let mut max_count: i32 = 0;
    let mut max_index: i32 = 0;
    let mut prev_zero: i32 = -1;
    let mut prev_prev_zero: i32 = -1;
    for current in 0..length {
        if list[current] == 0 {
           if current as i32 - prev_prev_zero > max_count {
                max_count = current as i32 - prev_prev_zero;
                max_index = prev_zero;
           } 
           prev_prev_zero = prev_zero;
           prev_zero = current as i32;
        }
    }
    if length as i32 - prev_prev_zero > max_count {
        max_index = prev_zero;
    }
    Ok(max_index)
}


