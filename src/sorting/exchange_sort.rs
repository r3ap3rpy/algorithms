/// Returns the exchange sorted version of the input
///
/// Takes a list reference as an input and returns a `Vec<i32>` or an error!
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::exchange_sort(&[2,1,3,5,4,7,6,8]).unwrap();
/// assert_eq!(result,[1,2,3,4,5,6,7,8]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::exchange_sort;
/// let my_list = [1,3,2,5,4,7,6,8];
/// match exchange_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn exchange_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use exchange sort on an empty list!".to_string());
    }
    let mut result = list.to_vec();
    let n = result.len();
    for i in 0..n-1 {
        for j in i+1..n {
            if result[i] > result[j] {
                result.swap(i,j);
            }
        }
    }
    Ok(result)
}

