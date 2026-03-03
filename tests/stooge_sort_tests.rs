use algorithmz::sorting::stooge_sort;

#[test]
fn test_stooge_sort_empty() {
    let result = stooge_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use stooge sort on an empty input!"));
}
#[test]
fn test_stooge_sort() {
    let result = stooge_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_stooge_sort_already_sorted() {
    let result = stooge_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
