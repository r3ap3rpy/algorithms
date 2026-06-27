use algorithmz::stack::valid_parenthesis;

#[test]
fn test_valid_parenthesis() {
    let result = valid_parenthesis("[]{}()");
    assert_eq!(result, true);
}

#[test]
fn test_valid_parenthesis_false() {
    let result = valid_parenthesis("{[(");
    assert_eq!(result, false);
}
