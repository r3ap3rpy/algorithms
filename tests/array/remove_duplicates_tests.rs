use algorithmz::array::remove_duplicates;

#[test]
fn test_remove_duplicates() {
    let result = remove_duplicates(&[1,1,2,2,3,4,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}

#[test]
fn test_remove_duplicates_empty() {
    let result = remove_duplicates(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot remove duplicates from an empty list."));
}
