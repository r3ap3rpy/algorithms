/// Find the statistical mode(s) of an array.
///
/// Takes a list reference and returns a `Vec<i32>` or an error explaining the problem!
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::top_1(&[-1,2,3,-1,4,5]).unwrap();
/// assert_eq!(result, vec![-1]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::top_1;
/// match top_1(&[1,2,1,3,3,4,5]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn top_1(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot identify top elements in an empty array!".to_string());
    }
    let mut frequency: std::collections::HashMap<i32,i32> = Default::default();
    let mut results: Vec<i32> = Vec::new();
    for element in list {
        if frequency.contains_key(element) {
            *frequency.entry(*element).or_insert(0) += 1;
        } else {
            frequency.insert(*element,1);
        }
    }
    let max = frequency.values().max().unwrap();
    for (item, value) in frequency.iter() {
        if value == max {
            results.push(*item);
        }
    }
    Ok(results)
}

