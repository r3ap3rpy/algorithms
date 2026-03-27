/// Check if a string follows the given pattern.
///
/// True if the string follows the pattern, False otherwise.
///
/// # Examples
///
/// Basic example:
/// ```
/// let result = algorithmz::map::word_pattern("abba", "dog cat cat dog").unwrap();
/// assert_eq!(result,true);
/// ```
///
/// Match example:
/// ```
/// match algorithmz::map::word_pattern("abba","cat dog dog cat") {
///     Ok(n) => println!("The return value was: {}",n),
///     Err(e) => println!("The error was: {}",e),
/// }
///
pub fn word_pattern(pattern: &str, string: &str) -> Result<bool,String> {
    if pattern.is_empty() {
        return Err("Pattern cannot be empty!".to_string());
    }
    if string.is_empty() {
        return Err("String cannot be empty!".to_string());
    }
    let mut mapping: std::collections::HashMap<char,&str> = std::collections::HashMap::new();
    let mut mapped_values: std::collections::HashSet<&str> = std::collections::HashSet::new();
    let words: Vec<&str> = string.split_whitespace().collect();
    if words.len() != pattern.len() {
        return Err("Number of words must match number of characters in the pattern!".to_string());
    }
    for (i, ch) in pattern.chars().enumerate() {
        let word = words[i];

        if !mapping.contains_key(&ch) {
            if mapped_values.contains(word) {
                return Ok(false);
            }
            mapping.insert(ch, word);
            mapped_values.insert(word);
        } else {
            if mapping.get(&ch) != Some(&word) {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
