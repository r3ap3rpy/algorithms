use algorithmz::sorting::heap_sort;

#[test]
fn test_heap_sort_empty() {
    let result = heap_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use heap sort on an empty list!"));
}
#[test]
fn test_heap_sort() {
    let result = heap_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_heap_sort_already_sorted() {
    let result = heap_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
