/// License number
///
/// Given a license key string and a group size k, reformat the key so that each group contains exactly k characters, separated by dashes.
///
/// # Examples
///
/// Basic usage:
/// ``` 
/// let result = algorithmz::string::license_number("a-bc-dfd-df",2);
/// assert_eq!(result, String::from("ab-cd-fd-df"));
/// ```
pub fn license_number(original: &str, sections: usize) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut alphanumeric: Vec<String> = Vec::new();
    for character in original.chars() {
        if character != '-' {
            alphanumeric.push(character.to_string());
        }
    }
    for (index, character) in alphanumeric.iter().enumerate() {
        result.push(character.to_string());
        if (index + 1) % sections == 0 && index != alphanumeric.len() -1 {
            result.push('-'.to_string());
        }
    }
    result.join("")
}
