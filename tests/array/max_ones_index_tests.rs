use algorithmz::array::max_ones_index;

#[test]
fn test_max_ones_index_empty() {
    let result = max_ones_index(&[]);
    assert!(matches!(result, Err(ref e)  if e == "Cannot use max ones on an empty list!"));
} 

#[test]
fn test_max_ones_index_not_valid() {
    let result = max_ones_index(&[1,2,3,4,5,6,7]);
    assert!(matches!(result, Err(ref e)  if e == "The list may only contain 1-s and 0-s!"));
}

#[test]
fn test_max_ones_index() {
    let result = max_ones_index(&[1,1,1,0,1,1,1,1,1,0,1,1,1]).unwrap();
    assert_eq!(result, 3);
}
