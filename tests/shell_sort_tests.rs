use algorithmz::sorting::shell_sort;

#[test]
fn test_shell_sort_empty() {
    let result = shell_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot sort an empty list!"));
}
#[test]
fn test_shell_sort() {
    let result = shell_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_shell_sort_already_sorted() {
    let result = shell_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
