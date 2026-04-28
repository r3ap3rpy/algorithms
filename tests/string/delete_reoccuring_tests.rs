use algorithmz::string::delete_reoccuring;

#[test]
fn test_delete_reoccuring() {
    let result = delete_reoccuring("aaabbbccc");
    assert_eq!(result,String::from("abc"));
}

#[test]
fn test_delete_reoccuring_empty() {
    let result = delete_reoccuring("");
    assert_eq!(result, String::from(""));
}
