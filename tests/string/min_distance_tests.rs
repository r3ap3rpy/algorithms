use algorithmz::string::min_distance;

#[test]
fn test_min_distance() {
    let result = min_distance("lion","ion");
    assert_eq!(result, 1);
}
