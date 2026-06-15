/// Validate Coordinates
/// Validate if given parameters are valid geographical coordinates. Latitude must be between -90 and 90, longitude between -180 and 180.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::validate_coordinates("-23, 56");
/// assert_eq!(result, Ok(true));
/// ```
pub fn validate_coordinates(text: &str) -> Result<bool,String> {
    if text.is_empty() {
        return Err("Empty string cannot be validated!".to_string());
    }
    for character in text.chars() {
        if !(character.is_ascii_digit() || ['-', '.', ',', ' '].contains(&character)) {
            return Err("Invalid character!".to_string());
        }
    }
    let parts: Vec<&str> = text.split(",").collect();
    let latitude: i16 = parts[0].trim().parse().unwrap();
    let longitude: i16 = parts[1].trim().parse().unwrap();
    if (-90..=90).contains(&latitude) && (-180..=180).contains(&longitude) {
        return Ok(true);
    } else {
        return Err("Invalid coordinates specified!".to_string());
    }
}
