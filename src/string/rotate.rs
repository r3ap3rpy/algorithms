/// Rotate
///
/// Given a string and an integer k, return the string rotated by k positions to the left. Two approaches are provided.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::rotate("hello",2);
/// assert_eq!(result, Ok(String::from("llohe")));
/// ```
pub fn rotate(text: &str, rotations: usize) -> Result<String, String> {
    if text.is_empty() {
        return Err("Cannot rotate an empty string!".to_string());
    }
    let characters: Vec<char> = text.chars().collect();
    let positions = rotations % characters.len();
    let mut to_pos = characters[positions..].to_vec();
    let mut from_pos = characters[..positions].to_vec();
    to_pos.append(&mut from_pos);
    return Ok(to_pos.into_iter().collect::<String>());
}
