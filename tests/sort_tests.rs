use algorithms::sorting::bead_sort;

#[test]
fn test_bead_sort_negative_error(){
    let result = bead_sort(&[-1,2,3,4,5]);
    assert!(matches!(result, Err(ref e) if e == "The items cannot be smaller than ZERO!"));
}
#[test]
fn test_bead_sort_empty_error() {
    let result = bead_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "The list cannot be empty!"));
}
#[test]
fn test_bead_sort() {
    let result = bead_sort(&[1,3,2,5,4]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
