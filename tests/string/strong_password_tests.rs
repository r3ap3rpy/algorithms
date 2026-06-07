use algorithmz::string::strong_password;

#[test]
fn test_strong_password() {
    let result = strong_password(11,"Start!12345");
    assert_eq!(result, 0);
}
