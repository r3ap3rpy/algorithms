/// Given an array of integers, find all unique triplets that sum to zero using the two-pointer technique.
///
/// Takes a list reference as an input and returns a `HashSet<(i32,i32,i32)>` or an error
/// explaining the problem
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::three_sum(&[-1, 0, 1, 2, -1, -4]).unwrap();
/// assert_eq!(result,std::collections::HashSet::from([(-1, 0, 1), (-1, -1, 2)]));
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::three_sum;
/// match three_sum(&[-1, 0, 1, 2, -1, -4]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn three_sum(list: &[i32]) -> Result<std::collections::HashSet<(i32,i32,i32)>,String> {
    if list.is_empty() {
        return Err("Cannot use three sum on an empty list!".to_string());
    }
    let mut result: std::collections::HashSet<(i32,i32,i32)> = std::collections::HashSet::new();
    let mut original = list.to_vec();
    original.sort();
    for i in 0..(original.len() - 2) {
        if i > 0 && original[i] == original[i-1] {
            continue;
        }
        let mut left = i + 1;
        let mut right = original.len() - 1;
        while left < right {
            let current_sum = original[i] + original[left] + original[right];
            if current_sum > 0 {
                right -= 1;
            } else if current_sum < 0 {
                left += 1;
            } else {
                result.insert((original[i],original[left],original[right]));
                while left < right && original[left] == original[left + 1] {
                    left += 1;
                }
                while left < right && original[right] == original[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
            }
        }

    }
    Ok(result)
}

