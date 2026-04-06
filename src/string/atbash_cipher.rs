/// Atbash Cipher
///
/// Atbash cipher maps each letter of the alphabet to its reverse. The first letter 'a' maps to 'z', 'b' maps to 'y', and so on.
///
/// # Examples
///
/// Basic example:
/// ```
/// let result = algorithmz::string::atbash_cipher("abcdefghijklmno");
/// assert_eq!(result, "zyxwvutsrqponml".to_string());
/// ```
pub fn atbash_cipher(text: &str) -> String {
    text.chars().map(|c|{
        if c.is_ascii_uppercase() {
            let offset = c as u8 - b'A';
            (b'Z' - offset) as char
        } else if c.is_ascii_lowercase() {
            let offset = c as u8 - b'a';
            (b'z' - offset) as char
        } else {
            c
        }
    }).collect()
}

