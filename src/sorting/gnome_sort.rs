/// Returns the gnome sorted version of the input.
///
/// Takes a list reference and returns a `Vec<i32>` or an error.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::gnome_sort(&[2,3,1,5,4]).unwrap();
/// assert_eq!(result,[1,2,3,4,5]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::gnome_sort;
/// let my_list = [1,3,2,5,4,7,6,8];
/// match gnome_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn gnome_sort(list: &[i32]) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Cannot use gnome sort on an empty array!".to_string()); 
    }
    let mut result = list.to_vec();
    let n = result.len();
    let mut index = 0;
    while index < n  {
        if index == 0 || result[index] >= result[index - 1] {
            index += 1;
        } else {
            result.swap(index,index-1);
            index -= 1;
        }
    }
    Ok(result)
    
}

