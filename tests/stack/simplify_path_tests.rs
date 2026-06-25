use algorithmz::stack::simplify_path;

#[test]
fn test_simplify_path() {
    let result = simplify_path("/a/./b/../../c/");
    assert_eq!(result, String::from("/c"));
}
