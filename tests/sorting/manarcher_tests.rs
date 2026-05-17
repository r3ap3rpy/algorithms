use algorithmz::string::manarcher;

#[test]
fn test_manarcher() {
    let result = manarcher("babad");
    assert_eq!(result, String::from("aba"));
}
