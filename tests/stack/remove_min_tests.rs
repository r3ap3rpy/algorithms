use algorithmz::stack::remove_min;

#[test]
fn test_remove_min_empty() {
    let result = remove_min(vec![]);
    assert_eq!(result, vec![]);
}

#[test]
fn test_remove_min() {
    let result = remove_min(vec![1,2,3,4,5]);
    assert_eq!(result, vec![2,3,4,5]);
}
