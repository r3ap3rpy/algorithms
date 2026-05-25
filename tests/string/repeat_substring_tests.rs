use algorithmz::string::repeat_substring;

#[test]
fn test_repeat_substring_false() {
    let result = repeat_substring("abcd");
    assert_eq!(result, false);
}

#[test]
fn test_repeat_substring_true() {
    let result = repeat_substring("abcabc");
    assert_eq!(result, true);
}
