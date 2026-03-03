use algorithmz::sorting::cycle_sort;

#[test]
fn test_cycle_sort_empty_error() {
    let result = cycle_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot sort an empty array!"));
}
#[test]
fn test_cycle_sort() {
    let result = cycle_sort(&[1,3,2,5,4]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_cycle_sort_already_sorted() {
    let result = cycle_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
