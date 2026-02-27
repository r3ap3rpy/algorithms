use algorithmz::sorting::bogo_sort;

#[test]
fn test_bogo_sort_empty() {
    let result = bogo_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot bogo sort an empty list!"));
}
#[test]
fn test_bogo_sort() {
    let result = bogo_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_bogo_sort_already_sorted() {
    let result = bogo_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
