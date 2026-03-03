/// Returns the selection sorted version of the input.
///
/// Takes a list reference as input and returns a `Vec<i32>` or an error explainint the situation.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::selection_sort(&[2,1,4,3,6,5]).unwrap();
/// assert_eq!(result,[1,2,3,4,5,6]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::selection_sort;
///
/// match selection_sort(&[5,4,7,6,8,7,9,10]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn selection_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use selection sort on an empty input!".to_string());
    }
    let mut result = list.to_vec();
    let n = result.len();

    for i in 0..n {
        let mut min = i;
        for j in i+1..n {
            if result[j] < result[min] {
                min = j
            }
        }
        result.swap(min, i);
    }
    Ok(result)
}

