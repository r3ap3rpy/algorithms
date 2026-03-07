use algorithmz::array::longest_non_repeat;

#[test]
fn test_longest_non_repeat() {
    let result = longest_non_repeat("abcabcbb").unwrap();
    assert_eq!(result,(3,"abc".to_string()));
}

#[test]
fn test_longest_non_repeat_empty() {
    let result = longest_non_repeat("");
    assert!(matches!(result, Err(ref e) if e == "Cannot identify longest non repeat on an empty string!"));
}
