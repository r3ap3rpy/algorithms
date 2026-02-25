/// Returns the shell sorted version of the input.
///
/// The function takes a list reference and returns the shell sorted version as `Vec<i32>` or an
/// error string.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::shell_sort(&[2,1,4,5,3]).unwrap();
/// assert_eq!(result,[1,2,3,4,5]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::shell_sort;
/// let my_list = [1,3,2,5,4];
/// match shell_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn shell_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot sort an empty list!".to_string());
    }
    let mut my_list = list.to_vec();
    let length = my_list.len();
    let mut gap = length / 2;
    while gap > 0 {
        let mut y_index = gap;
        while y_index < length {
            let y = my_list[y_index];
            let mut x_index = y_index;
            while x_index >= gap && y < my_list[x_index - gap] {
                my_list[x_index] = my_list[x_index - gap];
                x_index -= gap;
            }
            my_list[x_index] = y;
            y_index += 1;
        }
        gap /= 2;
    } 
    Ok(my_list)
}

