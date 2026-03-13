use algorithmz::set::RandomizedSet;

#[test]
fn test_randomized_set() {
    let mut rs = RandomizedSet::new();
    rs.insert(10);
    rs.insert(20);
    rs.insert(30);
    assert_eq!(rs.elements.len(), 3);
    assert!(rs.index_map.contains_key(&10));
    assert!(rs.index_map.contains_key(&20));
    assert!(rs.index_map.contains_key(&30));
}

