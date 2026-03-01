use algorithmz::sorting::cocktail_shaker_sort;

#[test]
fn test_cocktail_shaker_sort_empty() {
    let result = cocktail_shaker_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use cocktail shaker sort on an empty list!"));
}
#[test]
fn test_cocktail_shaker_sort() {
    let result = cocktail_shaker_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_cocktail_shaker_sort_already_sorted() {
    let result = cocktail_shaker_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
