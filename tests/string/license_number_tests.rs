use algorithmz::string::license_number;

#[test]
fn test_license_number() {
    let result = license_number("a-b-c-d-e-f",3);
    assert_eq!(result, "abc-def".to_string());
}
