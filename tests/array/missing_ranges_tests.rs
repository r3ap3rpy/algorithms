use algorithmz::array::missing_ranges;

#[test]
fn test_missing_ranges_empty() {
    let result = missing_ranges(&[],1,10);
    assert!(matches!(result, Err(ref e) if e == "Cannot use missing ranges on an empty list!"));
}

#[test]
fn test_missing_ranges() {
    let result = missing_ranges(&[3,5],1,10).unwrap();
    assert_eq!(result, vec![(1,2),(4,4),(6,10)]);
}
