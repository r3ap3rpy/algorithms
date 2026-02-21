/// Returns the insertion sorted version of the input
///
/// It takes a list as reference and returns a `Vec<i32>` as a result
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::insertion_sort(&[2,1,3,5,4]).unwrap();
/// assert_eq!(result,[1,2,3,4,5]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::insertion_sort;
/// let my_list = [1,3,2,5,4];
/// match insertion_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn insertion_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot sort an empty list!".to_string());
    }
    let mut result = list.to_vec();
    let length = result.len();
    let mut cursor: i32;
    let mut pos: usize;
    for i in 0..length {
        cursor = result[i];
        pos = i;
        while pos > 0 && result[pos - 1] > cursor{
            result[pos] = result[pos - 1];
            pos -= 1;
        }
        result[pos] = cursor;
    }
    Ok(result)
}

