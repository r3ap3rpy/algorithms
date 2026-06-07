/// Strong password
///
/// Given a password string, determine the minimum number of characters that must be added to make it strong. A strong password has at least 6 characters, a digit, a lowercase letter, an uppercase letter, and a special character.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::strong_password(3,"Ab1");
/// assert_eq!(result, 3);
/// ```
pub fn strong_password(length: i32, password: &str) -> i32 {
    let mut result: i32 = 0;
    if !password.chars().any(|c| c.is_ascii_digit()) {
        result += 1;
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        result += 1;
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        result += 1;
    }
    if !password.chars().any(|c| "!@#$%^&*()-+".contains(c)) {
        result += 1;
    }
    return std::cmp::max(result as i32, 6 - length as i32);
}
