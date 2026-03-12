/// Calculate the trimmed mean of an array.
///
/// Takes a list reference and returns an `f64` or an error explaining the problem.
///
/// # Examples
///
/// Basic usage:
/// ``` 
/// let result =
/// algorithmz::array::trimmean(&[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0],20.0).unwrap();
/// assert_eq!(result, 5.5);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::trimmean;
/// match trimmean(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],20.0) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn trimmean(list: &[f64], percentage: f64) -> Result<f64, String> {
    if list.is_empty() {
        return Err("Cannot calculate the trimmed mean of an empty list!".to_string());
    }
    let ratio: f64 = percentage / 200.0;
    let mut temp = list.to_vec();
    temp.sort_by(|a,b| a.partial_cmp(b).unwrap());
    let trim_count: usize = (temp.len() as f64 * ratio) as usize;
    let trimmed  = &temp[trim_count..temp.len() - trim_count];
    let mut total: f64 = 0.0;
    for value in trimmed {
        total += *value;
    }
    Ok(total / trimmed.len() as f64)
} 

