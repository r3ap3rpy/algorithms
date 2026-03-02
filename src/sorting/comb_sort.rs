/// Returns the comb sorted version of the input.
///
/// Takes a list as reference and returns `Vec<i32>` or an error string.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::comb_sort(&[2,1,3,5,4,7,6,8]).unwrap();
/// assert_eq!(result,[1,2,3,4,5,6,7,8]);
/// ```
///
/// Match example:
/// ``` 
/// use algorithmz::sorting::comb_sort;
/// let my_list = [1,3,2,5,4,7,6,8];
/// match comb_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn comb_sort(list: &[i32]) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Cannot use comb sort on an empty list!".to_string());
    }
    let mut result = list.to_vec();
    let n = result.len();
    let mut gap = n;
    let shrink_factor = 1.3;
    let mut is_sorted = false;
    while !is_sorted {
        gap = ((gap as f64) / shrink_factor) as usize;
        if gap <= 1 {
            gap = 1;
            is_sorted = true;
        }
        let mut i = 0;
        while i + gap < n {
            if result[i] > result[i + gap] {
                result.swap(i, i+gap);
                is_sorted = false;
            }
            i += 1;
        }
    }
    Ok(result)
}

