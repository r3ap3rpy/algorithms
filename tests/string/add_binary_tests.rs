use algorithmz::string::add_binary;

#[test]
fn test_add_binary() {
    let result = add_binary("11","1");
    assert_eq!(result, "100".to_string());
}

