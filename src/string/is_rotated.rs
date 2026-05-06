/// Is Rotated String
///
/// Given two strings, determine if the second is a rotated version of the first. Two approaches are provided: concatenation check and brute force.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::is_rotated("hello", "llohe");
/// assert_eq!(result, true);
/// ```
pub fn is_rotated(string_a: &str, string_b: &str) -> bool {
    if string_a.len() == string_b.len() {
        let concatenated = format!("{}{}",string_a,string_a);
        if concatenated.contains(string_b) {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

