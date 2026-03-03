use algorithmz::sorting::radix_sort;

#[test]
fn test_radix_sort_empty() {
    let result = radix_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use radix sort on an empty list!"));
}
#[test]
fn test_radix_sort_negative() {
    let result = radix_sort(&[-1,2,3,5,4]);
    assert!(matches!(result, Err(ref e) if e == "All items must be non-negative!"));
}
#[test]
fn test_radix_sort() {
    let result = radix_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_radix_sort_already_sorted() {
    let result = radix_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
