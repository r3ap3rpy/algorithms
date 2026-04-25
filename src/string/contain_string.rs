/// Contain string
///
/// Find the first occurrence of needle in haystack.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::contain_string("hello", "ll");
/// assert_eq!(result, true);
/// ```
pub fn contain_string(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    if needle.len() > haystack.len() {
        return false;
    }

    for i in 0..haystack.len() {
        if haystack.len() - i < needle.len() {
            return false;
        }
        if &haystack[i..i + needle.len()] == needle {
            return true;
        }
    }

    false
}
