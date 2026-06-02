use algorithmz::string::reverse_vowels;

#[test]
fn test_reverse_vowels_empty() {
    let result = reverse_vowels("");
    assert_eq!(result, Err(String::from("Cannot reverse vowels of an empty string!")));
}

#[test]
fn test_reverse_vowels() {
    let result = reverse_vowels("hello");
    assert_eq!(result, Ok(String::from("holle")));
}
