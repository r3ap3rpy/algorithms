use algorithmz::stack::is_consecutive;

#[test]
fn test_is_consecutive() {
    let result = is_consecutive(vec![3,4,5]);
    assert_eq!(result, true);
}

#[test]
fn test_is_consecutive_false() {
    let result = is_consecutive(vec![3,5,6]);
    assert_eq!(result, false);
}
