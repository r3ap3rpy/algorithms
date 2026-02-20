use algorithmz::sorting::bubble_sort;

#[test]
fn test_bubble_sort_empty() {
    let result = bubble_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "The list cannot be empty!"));
}
#[test]
fn test_bubble_sort() {
    let result = bubble_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_bubble_sort_already_sorted() {
    let result = bubble_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
