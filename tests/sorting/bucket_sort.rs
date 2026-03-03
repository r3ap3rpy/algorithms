use algorithmz::sorting::bucket_sort;

#[test]
fn test_bucket_sort_empty() {
    let result = bucket_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot bucket sort an empty list!"));
}
#[test]
fn test_bucket_sort() {
    let result = bucket_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_bucket_sort_already_sorted() {
    let result = bucket_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
