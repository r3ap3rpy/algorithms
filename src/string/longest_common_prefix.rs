/// Longest common prefix
///
/// Find the longest common prefix string amongst an array of strings. Three approaches: horizontal scanning, vertical scanning, and divide and conquer.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::longest_common_prefix("flower", "flow");
/// assert_eq!(result, String::from("flow"));
/// ```
pub fn longest_common_prefix(first: &str, second: &str) -> String {
    if first.len() == 0 || second.len() == 0 {
        return String::from("");
    }
    let first_bytes = first.as_bytes();
    let second_bytes = second.as_bytes();
    let mut index: usize = 0;
    while first_bytes[index] == second_bytes[index] {
        index = index + 1;
        if index >= first_bytes.len() || index >= second_bytes.len() {
            return String::from_utf8(first_bytes[0..index].to_vec()).unwrap();
        }
    }
    return String::from_utf8(first_bytes[0..index].to_vec()).unwrap();
}
