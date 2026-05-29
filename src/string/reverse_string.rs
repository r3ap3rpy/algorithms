/// Reverse string
///
/// Reverse a string using four different approaches: recursive, iterative, pythonic (using reversed), and ultra-pythonic (using slicing).
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::reverse_string("abc");
/// assert_eq!(result, String::from("cba"));
/// ```
pub fn reverse_string(original: &str) -> String {
    let reversed: String = original.chars().rev().collect();

    return reversed;
}
