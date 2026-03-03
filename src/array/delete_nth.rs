use std::collections::HashMap;
/// Returns a list where no more than n occurence can be in the output, naive version.
///
/// Takes a list reference as input and a maximum occurence then returns a `Vec<i32>` or an error
/// explaining the problem.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::delete_nth_naive(&[3, 5, 2, 3, 1, 4, 3, 6, 4, 4],2).unwrap();
/// assert_eq!(result,[3, 5, 2, 3, 1, 4, 6, 4]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::delete_nth_naive;
/// match delete_nth_naive(&[3, 5, 2, 3, 1, 4, 3, 6, 4, 4],2) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn delete_nth_naive(list: &[i32], max_occurence: usize) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot delete nth element of an empty list!".to_string());
    }
    let mut result: Vec<i32> = vec![];
    for &element in list {
        let occurence = result.iter().filter(|&&x| x == element).count();
        if occurence < max_occurence {
            result.push(element);
        }
    }
    Ok(result)
}

/// Returns a list where no more than n occurence can be in the output, HashSet version.
///
/// Takes a list reference as input and a maximum occurence then returns a `Vec<i32>` or an error
/// explaining the problem.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::delete_nth(&[3, 5, 2, 3, 1, 4, 3, 6, 4, 4],2).unwrap();
/// assert_eq!(result,[3, 5, 2, 3, 1, 4, 6, 4]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::delete_nth;
/// match delete_nth(&[3, 5, 2, 3, 1, 4, 3, 6, 4, 4],2) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn delete_nth(list: &[i32], max_occurence: usize) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot delete nth element of an empty list!".to_string());
    }
    let mut result = Vec::new();
    let mut counts = HashMap::new();

    for &element in list {
        let occurence = counts.entry(element).or_insert(0);

        if *occurence < max_occurence {
            result.push(element);
            *occurence += 1;
        }
    }

    Ok(result)
}

