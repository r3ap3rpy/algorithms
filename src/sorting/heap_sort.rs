/// Returns the heap sorted version of the input.
///
/// Takes a list reference and returns a `Vec<i32>` or an error string.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::heap_sort(&[2,3,1,5,4]).unwrap();
/// assert_eq!(result,[1,2,3,4,5]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::heap_sort;
/// let my_list = [1,3,2,5,4];
/// match heap_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn heap_sort(list: &[i32]) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Cannot use heap sort on an empty list!".to_string());
    }
    let mut result = list.to_vec();
    let n = result.len();
    for i in (0..n/2).rev() {
        _max_heapify(&mut result, i, n);
    }
    for i in (1..n).rev() {
        result.swap(0,i);
        _max_heapify(&mut result, 0, i);
    }
    Ok(result)
}
fn _max_heapify(array: &mut [i32], mut parent: usize, end: usize) {
    while 2 * parent + 1 < end {
        let mut child = 2 * parent + 1;
        if child + 1 < end && array[child] < array[child + 1] {
            child += 1;
        }
        if array[child] > array[parent] {
            array.swap(parent, child);
            parent = child;
        } else {
            break;
        }
    }
}

