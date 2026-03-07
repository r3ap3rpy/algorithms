use algorithmz::array::limit;


#[test]
fn test_limit_empty() {
    let result = limit(&[],2,4);
    assert!(matches!(result, Err(ref e) if e == "Cannot use limit on an empty list!"));
}

#[test] 
fn test_limit() {
    let result = limit(&[1,2,3,4,5],2,4).unwrap();
    assert_eq!(result,vec![2,3,4]);
}
