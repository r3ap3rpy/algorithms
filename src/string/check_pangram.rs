/// Check pangram
///
/// Checks whether a given string is a pangram, meaning it contains every letter of the English alphabet at least once.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::check_pangram("The quick brown fox jumps over the lazy dog").unwrap();
/// assert_eq!(result, true);
/// ```
pub fn check_pangram(input_string: &str) -> Result<bool,String> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    if input_string.is_empty() {
        return Err("The input cannot be empty!".to_string());
    }
    let input_lowercase = input_string.to_lowercase();
    Ok(alphabet.chars().all(|c| input_lowercase.contains(c)))
}
