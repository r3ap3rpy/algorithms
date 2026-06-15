/// Covnert morse
///
/// Convert a word to its Morse code representation.
///
/// # Examples
///
/// Basic usage:
/// ``` 
/// let result = algorithmz::string::convert_morse("gin");
/// assert_eq!(result, Ok(String::from("--...-.")));
/// ```
pub fn convert_morse(text: &str) -> Result<String, String> {
    let morse: std::collections::HashMap<char,&str> = std::collections::HashMap::from([
        ('a', ".-"),
        ('b', "-..."),
        ('c', "-.-."),
        ('d', "-.."),
        ('e', "."),
        ('f', "..-."),
        ('g', "--."),
        ('h', "...."),
        ('i', ".."),
        ('j', ".---"),
        ('k', "-.-"),
        ('l', ".-.."),
        ('m', "--"),
        ('n', "-."),
        ('o', "---"),
        ('p', ".--."),
        ('q', "--.-"),
        ('r', ".-."),
        ('s', "..."),
        ('t', "-"),
        ('u', "..-"),
        ('v', "...-"),
        ('w', ".--"),
        ('x', "-..-"),
        ('y', "-.--"),
        ('z', "--.."),
    ]);
    if text.is_empty() {
        return Err("Cannot convert an empty string!".to_string());
    }
    let mut converted: String = String::new();
    for character in text.chars() {
        let lowercase = character.to_ascii_lowercase();
        if let Some(lc) = morse.get(&lowercase) {
            converted.push_str(lc);
        }

    }

    return Ok(converted);
}
