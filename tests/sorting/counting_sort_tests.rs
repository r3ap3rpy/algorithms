use algorithmz::sorting::counting_sort;

#[test]
fn test_counting_sort_empty() {
    let result = counting_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot sort and empty input!"));
}
#[test]
fn test_counting_sort() {
    let result = counting_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_counting_sort_already_sorted() {
    let result = counting_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
