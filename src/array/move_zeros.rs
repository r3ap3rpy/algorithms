/// Move all integer zeros to the end, preserving order of other elements.
///
/// Takes a list reference and returns the `Vec<i32>` or an error explaining the situation.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::move_zeros(&[1,0,2,3,0,4,0,5,6]).unwrap();
/// assert_eq!(result,vec![1,2,3,4,5,6,0,0,0]);
/// ```
///
/// Match usage:
/// ```
/// use algorithmz::array::move_zeros;
/// match move_zeros(&[1,0,1,0,1,0,1,1,0]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The problems was: {}",e),
/// }
pub fn move_zeros(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot move zeros in an empty list!".to_string());
    }
    let mut results: Vec<i32> = Vec::new();
    let mut zeros: i32 = 0;
    for element in list{
        if *element == 0 {
            zeros += 1;
        } else {
            results.push(*element);
        }
    }
    for _number in 0..zeros {
        results.push(0);
    }
    Ok(results)
}

