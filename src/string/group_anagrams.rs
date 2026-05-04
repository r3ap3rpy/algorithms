/// Group Anagrams
///
/// Given an array of strings, group anagrams together. Anagrams are words that contain the same letters in a different order.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::group_anagrams(vec!["eat".to_string(), "tea".to_string()]);
/// assert_eq!(result, vec![vec!["eat".to_string(), "tea".to_string()]]);
/// ```
pub fn group_anagrams(strings: Vec<String>) -> Vec<Vec<String>> {
    let mut map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for word in strings {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let key: String = chars.into_iter().collect();

        map.entry(key).or_default().push(word);
    }

    map.into_values().collect()
}
