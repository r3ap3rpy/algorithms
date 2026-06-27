use algorithmz::stack::slutter;

#[test]
fn test_slutter() {
    let result = slutter(vec![1,2,3]);
    assert_eq!(result, vec![1,1,2,2,3,3]);
}
