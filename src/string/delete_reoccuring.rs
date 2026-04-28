/// Delete Reoccurring Characters
///
/// Given a string, delete any reoccurring characters and return the new string containing only the first occurrence of each character.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let results = algorithmz::string::delete_reoccuring("aaabcccc");
/// assert_eq!(results, String::from("abc"));
/// ```
pub fn delete_reoccuring(input_text: &str) -> String {
    let mut seen_characters = std::collections::HashSet::new();
    let mut result_string = String::new();
    for character in input_text.chars() {
        if !seen_characters.contains(&character) {
            seen_characters.insert(character);
            result_string.push(character);
        }
    }
    result_string
}
