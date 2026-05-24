use algorithmz::string::repeat_string;

#[test]
fn test_repeat_string_negative() {
    let result = repeat_string("abc", "xyz");
    assert_eq!(result, -1);
}

#[test]
fn test_repeat_string() {
    let result = repeat_string("abc", "cabcabca");
    assert_eq!(result, 4);
}
