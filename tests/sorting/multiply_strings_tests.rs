use algorithmz::string::multiply_strings;

#[test]
fn test_multiply_strings() {
    let result = multiply_strings("11","11");
    assert_eq!(result,String::from("121"));
}
