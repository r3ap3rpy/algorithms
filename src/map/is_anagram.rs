/// Is Anagram
///
/// Determine whether two strings are anagrams of each other by comparing character frequency maps.
///
/// # Examples
///
/// Basic usage:
/// ```
/// assert!(algorithmz::map::is_anagram("anagram", "nagaram"));
/// ```
pub fn is_anagram(s: &str, t: &str) -> bool {
    let mut freq_s: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
    let mut freq_t: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
    for c in s.chars() {
        *freq_s.entry(c).or_insert(0) += 1;
    }
    for c in t.chars() {
        *freq_t.entry(c).or_insert(0) += 1;
    }
    freq_s == freq_t
}
