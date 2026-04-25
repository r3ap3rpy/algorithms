use algorithmz::string::contain_string;

#[test]
fn test_contain_string() {
    let result = contain_string("hello world", "ll");
    assert_eq!(result, true);
}

#[test]
fn test_contain_string_fail() {
    let result = contain_string("hello world", "kk");
    assert_eq!(result, false);
}
