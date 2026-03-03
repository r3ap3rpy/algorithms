use algorithmz::sorting::quick_sort;

#[test]
fn test_quick_sort_empty() {
    let result = quick_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot quick sort an empty array."));
}
#[test]
fn test_quick_sort() {
    let result = quick_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_quick_sort_already_sorted() {
    let result = quick_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
