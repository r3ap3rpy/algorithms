use algorithmz::string::rabin_karp;

#[test]
fn test_rabin_karp() {
    let result = rabin_karp("abc", "zsnabckfkd");
    assert_eq!(result, Some(3));
}
