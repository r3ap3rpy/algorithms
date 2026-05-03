/// First Unique Character in a String
///
/// Given a string, find the first non-repeating character and return its index. If no unique character exists, return -1.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::first_unique_character("leetcode");
/// assert_eq!(result, 0);
/// ```
pub fn first_unique_character(text: &str) -> i32 {
    if text.len() == 0 {
        return 0;
    }
    let mut banned: Vec<char> = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    for index in 0..chars.len() {
        let is_unique_letter = (index + 1..chars.len()).all(|other| chars[index] != chars[other]);
        if is_unique_letter && !banned.contains(&chars[index]) {
            return index as i32;
        } else {
            banned.push(chars[index]);
        }
    }
    return -1;
}
