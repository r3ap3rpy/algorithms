use algorithmz::array::top_1;

#[test]
fn test_top_1_empty() {
    let result = top_1(&[]);
    assert!(matches!(result,Err(ref e) if e == "Cannot identify top elements in an empty array!"));
}

#[test]
fn test_top_1() {
    let result = top_1(&[1,1,2,3,4,5,5,-2,-2]).unwrap();
    assert!(result.contains(&1));
    assert!(result.contains(&5));
    assert!(result.contains(&-2));
}
