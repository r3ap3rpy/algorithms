use algorithmz::sorting::merge_sort;

#[test]
fn test_merge_sort_empty() {
    let result = merge_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use merge sort on an empty list!"));
}
#[test]
fn test_merge_sort() {
    let result = merge_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_merge_sort_already_sorted() {
    let result = merge_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
