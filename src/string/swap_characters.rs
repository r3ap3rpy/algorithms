/// Swap characters
///
/// Given two strings of equal length, determine whether exactly one swap of two characters in the first string can make it equal to the second.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::swap_characters("bank","kanb");
/// assert_eq!(result, Ok(true));
/// ```
pub fn swap_characters(first: &str, second: &str) -> Result<bool,String> {
    if first.len() != second.len() {
        return Err("Length must be equal!".to_string());
    }
    let diffs: Vec<(char, char)> = first
    .chars()
    .zip(second.chars())
    .filter(|(a, b)| a != b)
    .collect();
    return Ok( matches!(
        diffs.as_slice(),
        [(a, b), (c, d)] if *a == *d && *b == *c
    ))
}

