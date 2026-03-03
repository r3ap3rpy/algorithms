use algorithmz::sorting::selection_sort;

#[test]
fn test_selection_sort_empty() {
    let result = selection_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use selection sort on an empty input!"));
}
#[test]
fn test_selection_sort() {
    let result = selection_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_selection_sort_already_sorted() {
    let result = selection_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
