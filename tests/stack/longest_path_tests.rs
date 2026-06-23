use algorithmz::stack::longest_path;

#[test]
fn test_longest_path() {
    let result = longest_path("dir\\n\\tfile.txt");
    assert_eq!(result, 15);
}
