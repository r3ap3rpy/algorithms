use algorithmz::string::decode_string;

#[test]
fn test_decode_string() {
    let result = decode_string("3[a]2[bc]");
    assert_eq!(result,String::from("aaabcbc"))
}

#[test]
fn test_decode_string_plain() {
    let result = decode_string("abcd");
    assert_eq!(result, String::from("abcd"));
}

#[test]
fn test_decode_string_empty() {
    let result = decode_string("");
    assert_eq!(result, String::from(""));
}
