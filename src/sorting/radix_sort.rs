/// Returns the radix sorted version of the input.
///
/// Takes a list reference and returns a `Vec<i32>` or an error explaining the situation.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::radix_sort(&[2,1,4,3,6,5]).unwrap();
/// assert_eq!(result, [1,2,3,4,5,6]);
/// ```
///
/// Match example:
///
/// ```
/// use algorithmz::sorting::radix_sort;
///
/// match radix_sort(&[2,1,3,5,4,6]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn radix_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot use radix sort on an empty list!".to_string());
    }
    if list.iter().any(|&x| x < 0) {
        return Err("All items must be non-negative!".to_string());
    }
    let mut results = list.to_vec();
    let mut position = 1;
    let max = *list.iter().max().unwrap_or(&0);
    while position <= max {
        let mut buckets: Vec<Vec<i32>> = (0..10).map(|_| Vec::new()).collect();
        for &num in &results {
            let digit = (num / position % 10) as usize;
            buckets[digit].push(num);
        }
        let mut index = 0;
        for bucket in buckets {
            for num in bucket {
                results[index] = num;
                index += 1;
            }
        }
        position *= 10;
    }
    Ok(results)
}

