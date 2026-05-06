/// Is palindrome?
///
/// Determine if a string is a palindrome, considering only alphanumeric characters and ignoring cases. Multiple approaches are provided.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::is_palindrome("otto");
/// assert_eq!(result, true);
/// ```
pub fn is_palindrome(text: &str) -> bool {
    let text_lower = text.to_lowercase();
    let bytes = text_lower.as_bytes();
    for index in 0..(bytes.len() / 2) {
        if bytes[index] != bytes[bytes.len() - index - 1] {
            return false;
        }
    }
    return true;
}
