use algorithmz::sorting::pancake_sort;

#[test]
fn test_pancake_sort_empty() {
    let result = pancake_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use pancake sort on an empty list!"));
}
#[test]
fn test_pancake_sort() {
    let result = pancake_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_pancake_sort_already_sorted() {
    let result = pancake_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
