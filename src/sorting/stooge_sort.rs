/// Returns the stooge sorted version of the input
///
/// Takes a list reference as input and returns a `Vec<i32>` or an error explaining the situation.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::stooge_sort(&[2,1,4,3,6,5]).unwrap();
/// assert_eq!(result,[1,2,3,4,5,6]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::stooge_sort;
///
/// match stooge_sort(&[2,1,4,3,6,5]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn stooge_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use stooge sort on an empty input!".to_string());
    }
    let mut result = list.to_vec();
    let end = result.len() - 1;
    _stooge(&mut result, 0, end);
    Ok(result)
}
fn _stooge(list: &mut [i32], low: usize, high: usize) {
    if list[low] > list[high] {
        list.swap(low,high);
    }
    if high - low + 1 > 2 {
        let third = (high - low + 1) / 3;
        _stooge(list, low, high - third);
        _stooge(list, low + third, high);
        _stooge(list, low, high - third);
    }
}

