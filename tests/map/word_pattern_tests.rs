use algorithmz::map::word_pattern;

#[test]
fn test_word_pattern_empty_pattern() {
    let result = word_pattern("","dog cat cat dog");
    assert!(matches!(result, Err (ref e) if e == "Pattern cannot be empty!"))
}

#[test]
fn test_word_pattern_empty_string() {
    let result = word_pattern("abba","");
    assert!(matches!(result, Err (ref e) if e == "String cannot be empty!"))
}

#[test]
fn test_word_pattern_mismatch() {
    let result = word_pattern("abba","dog cat dog cat dog");
    assert!(matches!(result, Err (ref e) if e == "Number of words must match number of characters in the pattern!"))
}

#[test]
fn test_word_pattern() {
    let result = word_pattern("abba","dog cat cat dog").unwrap();
    assert_eq!(result,true);
}
