use algorithmz::string::is_rotated;

#[test]
fn test_is_rotated_empty() {
    let result = is_rotated("","");
    assert_eq!(result, true);
}

#[test]
fn test_is_rotated_false() {
    let result = is_rotated("daniel","szabo");
    assert_eq!(result, false);
}
#[test]
fn test_is_rotated() {
    let result = is_rotated("hello", "llohe");
    assert_eq!(result, true);
}
