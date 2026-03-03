use algorithmz::sorting::wiggle_sort;

#[test]
fn test_wiggle_sort_empty() {
    let result = wiggle_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use wiggle sort on an empty input!"));
}
#[test]
fn test_wiggle_sort() {
    let result = wiggle_sort(&[3, 5, 2, 1, 6, 4]).unwrap();
    assert_eq!(result,[3, 5, 1, 6, 2, 4]);
}
