/// Returns the bitonic sorted version of the input
///
/// Takes a list as reference which has to have the length of the power of 2, then returns a `Vec<i32>` or an error.
///
/// # Examples:
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::bitonic_sort(&[2,1,3,5,4,7,6,8],false).unwrap();
/// assert_eq!(result,[1,2,3,4,5,6,7,8]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::bitonic_sort;
/// let my_list = [1,3,2,5,4,7,6,8];
/// match bitonic_sort(&my_list,false) {
///    Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn bitonic_sort(list: &[i32], reverse: bool) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Bitonic sort does not work on empty lists!".to_string());
    }
    let result = list.to_vec();
    let n = result.len();
    if n == 1 {
        return Ok(result);
    }
    if !n.is_power_of_two() {
        return Err("The size of input should be a power of two!".to_string());
    }
    let mid = n / 2;
    let left = bitonic_sort(&result[..mid],true)?;
    let right = bitonic_sort(&result[mid..],false)?;
    let mut merged = [left,right].concat();
    Ok(bitonic_merge(&mut merged, reverse))
}
fn compare(array: &mut [i32], reverse: bool) {
    let half = array.len() / 2;
    for i in 0..half {
        if reverse != (array[i] > array[i + half]) {
            array.swap(i,i+half);
        }
    }
}
fn bitonic_merge(array: &mut [i32], reverse:bool) -> Vec<i32> {
    let n = array.len();
    if n <= 1 {
        return array.to_vec();
    }
    compare(array, reverse);
    let mid = n / 2;
    let left = bitonic_merge(&mut array[..mid],reverse);
    let right = bitonic_merge(&mut array[mid..],reverse);
    [left,right].concat()
}

