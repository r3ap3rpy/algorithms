use algorithmz::map::longest_common_subsequence;

#[test]
fn test_longest_common_subsequence_success() {
    let result = longest_common_subsequence("abcdef","adcbef");
    assert_eq!(result, "ef");
}

#[test]
fn test_longest_common_subsequence_empty() {
    let result = longest_common_subsequence("abc","def");
    assert_eq!(result,"");
}
