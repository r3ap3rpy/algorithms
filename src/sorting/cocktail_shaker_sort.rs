/// Returns the cocktail shaker sorted version of the input.
///
/// Takes a list reference as an input an returns a `Vec<i32>` or an error string.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::cocktail_shaker_sort(&[2,1,3,5,4,7,6,8]).unwrap();
/// assert_eq!(result,[1,2,3,4,5,6,7,8]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::cocktail_shaker_sort;
/// let my_list = [1,3,2,5,4,7,6,8];
/// match cocktail_shaker_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn cocktail_shaker_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use cocktail shaker sort on an empty list!".to_string());
    }
    let mut result = list.to_vec();
    let n = result.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..n {
            if result[i-1] > result[i] {
                result.swap(i-1,i);
                swapped = true;
            }
        }
        if !swapped {
            return Ok(result);
        }
        swapped = false;
        for i in (1..n).rev() {
            if result[i-1] > result[i] {
                result.swap(i-1,i);
                swapped = true
            }
        }
    }
    Ok(result)
}
