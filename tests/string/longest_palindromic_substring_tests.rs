use algorithmz::string::longest_palindromic_substring;

#[test]
fn test_longest_palindromic_substring() {
    let result = longest_palindromic_substring("babad");
    assert_eq!(result, String::from("bab"));
}
