/// Reverse vowel
///
/// Given a string, reverse only the vowels while keeping all other characters in their original positions. 
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::reverse_vowels("hello");
/// assert_eq!(result, Ok(String::from("holle")));
/// ```
pub fn reverse_vowels(text: &str) -> Result<String,String> {
    if text.is_empty() {
        return Err("Cannot reverse vowels of an empty string!".to_string());
    }
    let vowels = "AEIOUaeiou";
    let mut left = 0;
    let mut right = text.len() - 1;
    let mut characters: Vec<char> = text.chars().collect();
    while left < right {
        while left < right && !vowels.contains(characters[left]) {
            left += 1;
        }
        while left < right && !vowels.contains(characters[right]) {
            right -= 1;
        }
        characters.swap(left,right);
        left += 1;
        right -= 1;
    }
    return Ok(characters.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(""));
}
