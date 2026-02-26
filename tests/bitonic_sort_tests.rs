use algorithmz::sorting::bitonic_sort;

#[test]
fn test_bitonic_sort_empty() {
    let result = bitonic_sort(&[],false);
    assert!(matches!(result, Err(ref e) if e == "Bitonic sort does not work on empty lists!"));
}
#[test]
fn test_bitonic_sort_not_power_of_two() {
    let result = bitonic_sort(&[2,1,4,3,5],false);
    assert!(matches!(result, Err(ref e) if e == "The size of input should be a power of two!"));
}
#[test]
fn test_bitonic_sort() {
    let result = bitonic_sort(&[3,1,2,4,5,6,8,7],false).unwrap();
    assert_eq!(result,[1,2,3,4,5,6,7,8]);
}
#[test]
fn test_bitonic_sort_already_sorted() {
    let result = bitonic_sort(&[1,2,3,4,5,6,7,8],false).unwrap();
    assert_eq!(result,[1,2,3,4,5,6,7,8]);
}
