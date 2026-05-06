use algorithmz::string::is_palindrome;

#[test]
fn test_is_palindrome_false() {
    let result = is_palindrome("daniel");
    assert_eq!(result, false);
}
#[test]
fn test_is_palindrom_true() {
    let result = is_palindrome("Otto");
    assert_eq!(result, true);
}
