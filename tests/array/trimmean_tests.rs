use algorithmz::array::trimmean;

#[test]
fn test_trimmean_empty() {
    let result = trimmean(&[],20.0);
    assert!(matches!(result, Err(ref e) if e == "Cannot calculate the trimmed mean of an empty list!"));
}

#[test]
fn test_trimmean() {
    let result = trimmean(&[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0],20.0).unwrap();
    assert_eq!(result, 5.5);
}
