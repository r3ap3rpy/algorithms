/// Returns the bubble sorted version of the input
///
/// This function takes a list as a reference and returns either the sorted list as a `Vec<usize>` or an error.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::bubble_sort(&[2,1,4,3,5]).unwrap();
/// assert_eq!(result, [1,2,3,4,5]);
/// ```
///
/// With match statement:
///
/// ```
/// use algorithmz::sorting::bubble_sort;
/// let my_list = [1,3,2,5,4];
/// match bubble_sort(&my_list) {
///     Err(e) => eprintln!("The error was: {}", e),
///     Ok(nums) => println!("The result was: {:?}", nums),
/// }
/// ```
pub fn bubble_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("The list cannot be empty!".to_string());
    }
    let mut result =  list.to_vec();
    let length = result.len();
    for i in 0..length {
        let mut swapped = false;
        for j in 0..(length - 1 - i) {
            if result[j] > result[j+1] {
                result.swap(j,j+1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }  
    Ok(result)
}

