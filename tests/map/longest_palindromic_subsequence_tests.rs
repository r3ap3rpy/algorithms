use algorithmz::map::longest_palindromic_subsequence;

#[test]
fn test_longest_palindromic_subsequence_non_zero() {
    let result = longest_palindromic_subsequence("babad");
    assert_eq!(result, 3);
}

#[test]
fn test_longest_palindromic_subsequence_zero() {
    let result = longest_palindromic_subsequence("abcdefg");
    assert_eq!(result, 1);
}
