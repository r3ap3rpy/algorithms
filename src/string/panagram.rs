/// Panagram
///
/// Check whether a given string is a panagram (a sentence using every letter of the English alphabet at least once).
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::panagram("the quick brown fox jumps over the lazy dog");
/// assert_eq!(result, true);
/// ```
pub fn panagram(text: &str) -> bool {
    let mut letters: std::collections::HashSet<char> = ('a'..='z').collect();

    for letter in text.to_lowercase().chars() {
        letters.remove(&letter);
    }

    if letters.is_empty() {
        return true;
    } else {
        return false;
    }
}

