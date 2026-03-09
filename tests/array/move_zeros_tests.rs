use algorithmz::array::move_zeros;

#[test]
fn test_move_zeros_empty() {
    let result = move_zeros(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot move zeros in an empty list!"));
}

#[test]
fn test_move_zeros() {
    let result = move_zeros(&[0,1,0,2,0,3,0,4,0,5]).unwrap();
    assert_eq!(result, vec![1,2,3,4,5,0,0,0,0,0]);
}
