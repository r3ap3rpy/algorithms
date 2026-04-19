/// Caesar Cipher
///
/// Caesar's cipher shifts each letter by a fixed number of positions in the
/// alphabet. Letters wrap around when they pass the end of the alphabet.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::caesar_cipher("Hello_World!",10);
/// assert_eq!(result, String::from("Rovvy_Gybvn!"));
/// ```
pub fn caesar_cipher(text: &str, shift: u32) -> String {
    let mut result: String = String::new();
    for character in text.chars() {
        let code = character as u32;
        let mut charcode: u32 = code;
        if (65..91).contains(&code) {
            charcode = ((code - 65 + shift) % 26) + 65;
        }
        if (97..123).contains(&code) {
            charcode = ((code - 97 + shift) % 26) + 97;
        }
        result.push(char::from_u32(charcode).unwrap());
    }
    result
}
