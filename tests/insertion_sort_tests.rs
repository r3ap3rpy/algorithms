use algorithmz::sorting::insertion_sort;

#[test]
fn test_insertion_sort_empty() {
    let result = insertion_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot sort an empty list!"));
}
#[test]
fn test_insertion_sort() {
    let result = insertion_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_insertion_sort_already_sorted() {
    let result = insertion_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
