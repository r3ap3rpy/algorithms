/// Returns the counting sorted version of the input.
///
/// This function takes a list reference as an input and returns either with an error or the
/// counting sorted version of the list.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::counting_sort(&[1,3,2,5,4]).unwrap();
/// assert_eq!(result,[1,2,3,4,5]);
/// ```
///
/// Match example:
/// ``` 
/// use algorithmz::sorting::counting_sort;
/// let my_list = [1,3,2,5,4];
/// match counting_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn counting_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot sort and empty input!".to_string());
    } 
    let min = list.iter().min().unwrap();
    let offset = if *min < 0 { -min } else { 0 };
    let shifted: Vec<i32> = list.iter().map(|&v| v + offset).collect();
    let max = shifted.iter().max().unwrap();
    let mut counts = vec![0;(max + 1) as usize];
    for &value in &shifted {
        counts[value as usize] += 1;
    }
    for i in 1..max+1 {
        counts[i as usize] += counts[(i-1) as usize];
    }
    let mut result = vec![0;*max as usize]; 
    for i in (0..(*max as usize)).rev() {
        let value = shifted[i];
        counts[value as usize] -= 1;
        let pos = counts[value as usize];
        result[pos] = value - offset;
    }
    Ok(result)
}

