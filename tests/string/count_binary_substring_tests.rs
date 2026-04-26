use algorithmz::string::count_binary_substring;

#[test]
fn tests_count_binary_substring() {
    let result = count_binary_substring("00110011");
    assert_eq!(result, 6);
}

#[test]
fn test_count_binary_substring_empty() {
    let result = count_binary_substring("");
    assert_eq!(result, 0);
}
