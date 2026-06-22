use algorithmz::stack::is_sorted;

#[test]
fn test_is_sorted() {
    let result = is_sorted(vec![5,6,7]);
    assert_eq!(result, true);
}

#[test]
fn test_is_sorted_false() {
    let result = is_sorted(vec![6,5,7]);
    assert_eq!(result, false);
}
