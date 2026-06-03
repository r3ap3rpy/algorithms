/// Reverse words
///
/// Given a string, reverse the order of words. Leading and trailing spaces are trimmed and words are separated by single spaces.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::reverse_words("I am keon kim and I like pizza");
/// assert_eq!(result, Ok(String::from("pizza like I and kim keon am I")));
/// ```
pub fn reverse_words(text: &str) -> Result<String,String> {
    if text.is_empty() {
        return Err("Cannot reverse words of an empty string!".to_string());
    }
    let mut splitted: Vec<&str> = text.split_whitespace().collect();
    let mut left = 0;
    let mut right = splitted.len() - 1;
    if splitted.len() == 1 {
        return Err("Cannot reverse a single word!".to_string());
    }
    while left < right {
        splitted.swap(left,right);
        left += 1;
        right -= 1;
    }
    return Ok(splitted.join(" "));
}
