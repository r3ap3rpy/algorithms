/// Returns the quick sorted version of the input.
///
/// The function takes a list reference and returns either a `Vec<i32>` or an error message.
///
/// # Examples
/// 
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::quick_sort(&[2,1,3,5,4]).unwrap();
/// assert_eq!(result,[1,2,3,4,5]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::quick_sort;
/// let my_list = [1,3,2,5,4];
/// match quick_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn quick_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot quick sort an empty array.".to_string());
    }
    let mut result = list.to_vec();
    let len = result.len();
    _quick_sort_recursive(&mut result, 0, len - 1);
    Ok(result)
}
fn _quick_sort_recursive(array: &mut [i32], first: usize, last: usize) {
    if first < last {
        let pivot = _partition(array, first, last);
        if pivot > 0 {
            _quick_sort_recursive(array, first, pivot - 1);
        }
        _quick_sort_recursive(array, pivot + 1, last);
    }
}
fn _partition(array: &mut [i32], first: usize, last: usize) -> usize {
    let pivot_value = array[last];
    let mut wall = first;
    for pos in first..last {
        if array[pos] < pivot_value {
            array.swap(pos,wall);
            wall += 1;
        }
    }
    array.swap(wall, last);
    wall
}

