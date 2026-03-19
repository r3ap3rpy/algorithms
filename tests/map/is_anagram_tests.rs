use algorithmz::map::is_anagram;

#[test]
fn test_is_anagram() {
    assert!(is_anagram("anagram", "nagaram"));
}
