use algorithmz::array::rotate;

#[test]
fn test_rotate_empty() {
    let result = rotate(&[],3);
    assert!(matches!(result, Err(ref e) if e == "Cannot rotate an empty list!"));
}

#[test]
fn test_rotate() {
    let result = rotate(&[1,2,3,4],4).unwrap();
    assert_eq!(result,vec![1,2,3,4]);
}
