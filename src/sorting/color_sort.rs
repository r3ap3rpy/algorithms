/// Returns the color sort output of the input
///
/// Takes a list reference that contains 0,1,2 values and returns `Vec<i32>` or an error explainig
/// the situation.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::color_sort(&[0,1,2,1,0,1,1,2,0,1]).unwrap();
/// assert_eq!(result,[0,0,0,1,1,1,1,1,2,2]);
/// ```
///
/// Match example:
/// ``` 
/// use algorithmz::sorting::color_sort;
///
/// match color_sort(&[1,0,1,2,1,0,1,2,0,0]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn color_sort(list: &[i32]) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Cannot use color sort on an empty input!".to_string());
    }
    if !&list.iter().all(|&x| x == 0 || x == 1 || x == 2) {
        return Err("Only 1-s, 2-s and 0-s are allowed as list items!".to_string());
    }
    let mut result = list.to_vec();
    let (mut red, mut white) = (0, 0);
    for k in 0..result.len() {
        let value = result[k];
        result[k] = 2;
        if value < 2 {
            result[white] = 1;
            white += 1;
        }
        if value == 0 {
            result[red] = 0;
            red += 1;
        }
    }
    Ok(result)
}

