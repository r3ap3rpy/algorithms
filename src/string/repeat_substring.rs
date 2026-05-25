/// Repeat substring
///
/// Given a non-empty string, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.
///
/// # Examples
///
/// Basic usage:
/// ``` 
/// let result = algorithmz::string::repeat_substring("abab");
/// assert_eq!(result, true);
/// ```
pub fn repeat_substring(text: &str) -> bool {
    let doubled = format!("{}{}", text, text);

    let trimmed = &doubled[1..doubled.len() - 1];

    trimmed.contains(text)
}
