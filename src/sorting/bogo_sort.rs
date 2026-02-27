use std::time::{SystemTime, UNIX_EPOCH};
/// Returns the bogo sorted version of the input
///
/// Takes a list reference as input and returns a `Vec<i32>` or an error string.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::bogo_sort(&[2,1,3,5,4,7,6,8]).unwrap();
/// assert_eq!(result,[1,2,3,4,5,6,7,8]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::bogo_sort;
/// let my_list = [1,3,2,5,4,7,6,8];
/// match bogo_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn bogo_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot bogo sort an empty list!".to_string());
    }
    let mut result = list.to_vec();
    let mut seed = current_time_seed();
    while !result.is_sorted(){
        fisher_yates_shuffle(&mut result, &mut seed)
    }
    Ok(result)
}
fn fisher_yates_shuffle(array: &mut [i32], seed: &mut u64) {
    let len = array.len();
    for i in (1..len).rev() {
        let j = (next_random(seed) % (i as u64 + 1)) as usize;
        array.swap(i,j);
    }
}
fn next_random(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    *seed
}
fn current_time_seed() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
}

