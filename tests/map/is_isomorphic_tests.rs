use algorithmz::map::is_isomorphic;

#[test]
fn test_is_isomorphic_true() {
    assert!(is_isomorphic("egg","add"));
}

#[test]
fn test_is_isomorphic_false() {
    assert!(!is_isomorphic("foo","bar"));
}
