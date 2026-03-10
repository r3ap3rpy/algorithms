/// Add one to a big-endian digit array by scanning from the right.
///
/// Takes a list reference and returns a `Vec<i32>` or an error explaining the situation.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::plus_one(&[1,2,9]).unwrap();
/// assert_eq!(result,vec![1,3,0]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::plus_one;
/// match plus_one(&[1,2,9]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
///}
/// ```

pub fn plus_one(list:&[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use plus one on an empty list!".to_string());
    }
    let length = list.len();
    let mut result = list.to_vec();
    for index in (0..length).rev() {
       if result[index] < 9 {
            result[index] += 1;
            return Ok(result);
       } 
       result[index] = 0;
    }
    result.insert(0,1);
    Ok(result)
}

