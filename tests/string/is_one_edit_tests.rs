use algorithmz::string::is_one_edit;

#[test]
fn test_is_one_edit_false() {
    let result = is_one_edit("abc","def");
    assert_eq!(result, false);
}

#[test]
fn test_is_one_edit_true() {
    let result = is_one_edit("abc","abd");
    assert_eq!(result, true);
}

#[test]
fn test_is_one_edit_corner_case() {
    let result = is_one_edit("abc","abcc");
    assert_eq!(result, true);
}
