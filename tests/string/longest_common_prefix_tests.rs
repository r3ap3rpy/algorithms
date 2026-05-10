use algorithmz::string::longest_common_prefix;

#[test]
fn test_longest_common_prefix_empty() {
    let result = longest_common_prefix("","");
    assert_eq!(result, String::from(""));
}
#[test] 
fn test_longest_common_prefix() {
    let result = longest_common_prefix("daniel","dan");
    assert_eq!(result, String::from("dan"));
}
