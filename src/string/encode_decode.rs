/// Encode Strings
///
/// Design an algorithm to encode a list of strings to a single string.
///
/// # Examples
///
/// Basic usage encode:
/// ```
/// let result = algorithmz::string::encode("keon is awesome");
/// assert_eq!(result, String::from("4:keon2:is7:awesome"));
/// ```
pub fn encode(text: &str) -> String {
    let mut result: String = String::new();
    for word in text.split_whitespace() {
        result.push_str(&word.len().to_string());
        result.push(':');
        result.push_str(word);
    }
    result
}
/// Decode Strings
///
/// Design an algorithm to decode a list of strings to a single string.
///
/// # Examples
///
/// Basic usage encode:
/// ```
/// let result = algorithmz::string::decode("4:keon2:is7:awesome");
/// assert_eq!(result, String::from("keon is awesome"));
/// ```
pub fn decode(text: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut index: usize = 0;
    while index < text.len() {
        let colon_index = text[index..].find(':').map(|i| i + index).unwrap();
        let size: usize = text[index..colon_index].parse().unwrap();
        result.push(text[colon_index + 1..colon_index + 1 + size].to_string());
        index = colon_index + 1 + size;
    }
    result.join(" ")
}
