/// Returns the pigeonhole sorted version of the input list reference.
///
/// Takes a list reference and returns a `Vec<i32>` or an error with the reason.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::pigeonhole_sort(&[2,1,4,3,6,5]).unwrap();
/// assert_eq!(result,[1,2,3,4,5,6]);
/// ```
///
/// Match example:
/// ``` 
/// use algorithmz::sorting::pigeonhole_sort;
/// match pigeonhole_sort(&[2,1,4,3,6,5]) {
///     Ok(n) => println!("The result is {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn pigeonhole_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use pigeonhole sort on an empty list!".to_string());
    }
    let max = list.iter().max().unwrap();
    let min = list.iter().min().unwrap();
    let mut result = list.to_vec();
    let size = max - min + 1;
    let mut holes = vec![0;size as usize];
    for value in &result {
        let index = value - min;
        holes[index as usize] += 1;
    }
    let mut i = 0;
    for count in 0..size {
        while holes[count as usize] > 0 {
            holes[count as usize] -= 1;
            result[i] = count + min;
            i += 1;
        }
    }
    Ok(result)
}

