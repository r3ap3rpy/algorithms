use algorithmz::array::three_sum;

#[test]
fn test_three_sum_empty() {
    let result = three_sum(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use three sum on an empty list!"));
}
#[test]
fn test_three_sum() {
    let result = three_sum(&[-1, 0, 1, 2, -1, -4]).unwrap();
    assert_eq!(result,std::collections::HashSet::from([(-1, 0, 1), (-1, -1, 2)]));
}
