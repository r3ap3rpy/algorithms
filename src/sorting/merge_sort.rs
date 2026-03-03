/// Returns the merge sorted version of the input list
///
/// Takes a list as a reference and returns `Vec<i32>` or an error explaining the problem.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::merge_sort(&[2,1,4,3,6,5]).unwrap();
/// assert_eq!(result, [1,2,3,4,5,6]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::merge_sort;
/// let my_list = vec![2,1,4,3,6,5];
/// match merge_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn merge_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use merge sort on an empty list!".to_string());
    }
    let n = list.len();
    if n == 1 {
        return Ok(list.to_vec());
    }
    let mid = n / 2;
    let left = merge_sort(&list[..mid])?;
    let right = merge_sort(&list[mid..])?;
    Ok(_merge(left,right))
}
fn _merge(left: Vec<i32>,right: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0,0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    merged
}

