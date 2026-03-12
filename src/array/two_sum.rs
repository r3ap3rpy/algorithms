/// The struct for different return values and types
#[derive(Debug, PartialEq)]
pub enum MyResult {
    /// The good result
    Tuple(i32,i32),
    /// The bad result
    Message(String),
    /// Ran but not found
    None
}
/// Find two indices whose corresponding values sum to target.
///
/// Takes a list reference and a target and returns `MyResult` based on the decisions.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::two_sum(&[2,7,11,15],9);
/// assert_eq!(result, algorithmz::array::MyResult::Tuple(0,1));
/// ```
/// 
/// Match example:
/// ```
/// use algorithmz::array::{two_sum, MyResult};
/// match two_sum(&[2, 7, 11, 15],9) {
///     MyResult::Tuple(a, b) => println!("Tuple: {}, {}", a, b),
///     MyResult::Message(msg) => eprintln!("Message: {}", msg),
///     MyResult::None => println!("Nothing"),
/// }
/// ```
pub fn two_sum(list: &[i32], target: i32) -> MyResult {
    if list.is_empty() {
        return MyResult::Message("Cannot use two_sum on an empty list!".to_string());
    }
    let mut seen: std::collections::HashMap<i32,i32> = std::collections::HashMap::new();
    for (index, &number) in list.iter().enumerate() {
        if let Some(&prev_index) = seen.get(&number) {
            return MyResult::Tuple(prev_index, index as i32);
        }

        seen.insert(target - number, index as i32);
    }

    MyResult::None
}

