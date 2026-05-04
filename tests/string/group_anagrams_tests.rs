use algorithmz::string::group_anagrams;

#[test]
fn test_group_anagrams() {
    let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
    .into_iter()
    .map(String::from)
    .collect();

    let mut result = group_anagrams(input);
    for group in &mut result {
        group.sort();
    }
    result.sort();

    let mut expected = vec![
        vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
        vec!["tan".to_string(), "nat".to_string()],
        vec!["bat".to_string()],
    ];

    for group in &mut expected {
        group.sort();
    }
    expected.sort();

    assert_eq!(result, expected);
}
