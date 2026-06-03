use algorithmz::string::reverse_words;

#[test]
fn test_reverse_words_empty() {
    let result = reverse_words("");
    assert_eq!(result, Err("Cannot reverse words of an empty string!".to_string()));
}

#[test]
fn test_reverse_words_single_word() {
    let result = reverse_words("dani");
    assert_eq!(result, Err("Cannot reverse a single word!".to_string()));
}

#[test]
fn test_reverse_words() {
    let result = reverse_words("dani szabo");
    assert_eq!(result, Ok("szabo dani".to_string()));
}
