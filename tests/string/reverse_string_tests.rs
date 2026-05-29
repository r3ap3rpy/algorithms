use algorithmz::string::reverse_string;

#[test]
fn test_reverse_string_empty() {
    let result = reverse_string("");
    assert_eq!(result, String::from(""));
}

#[test]
fn test_reverse_string() {
    let result = reverse_string("abcdef");
    assert_eq!(result, String::from("fedcba"));
}
