use algorithmz::string::breaking_bad;

#[test]
fn test_breaking_bad() {
    let result = breaking_bad(&vec!["Google"],&vec!["le"]);
    assert_eq!(result, vec!["Goog[le]".to_string()]);
}
