use algorithmz::sorting::comb_sort;

#[test]
fn test_comb_sort_empty() {
    let result = comb_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use comb sort on an empty list!"));
}
#[test]
fn test_comb_sort() {
    let result = comb_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_comb_sort_already_sorted() {
    let result = comb_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
