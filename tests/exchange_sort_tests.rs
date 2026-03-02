use algorithmz::sorting::exchange_sort;

#[test]
fn test_exchange_sort_empty() {
    let result = exchange_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use exchange sort on an empty list!"));
}
#[test]
fn test_exchange_sort() {
    let result = exchange_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_exchange_sort_already_sorted() {
    let result = exchange_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
