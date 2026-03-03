use algorithmz::sorting::pigeonhole_sort;

#[test]
fn test_pigeonhole_sort_empty() {
    let result = pigeonhole_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use pigeonhole sort on an empty list!"));
}
#[test]
fn test_pigeonhole_sort() {
    let result = pigeonhole_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_pigeonhole_sort_already_sorted() {
    let result = pigeonhole_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
