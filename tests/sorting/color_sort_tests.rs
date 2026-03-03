use algorithmz::sorting::color_sort;

#[test]
fn test_color_sort_empty() {
    let result = color_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use color sort on an empty input!"));
}
#[test]
fn test_color_sort_invalid_input() {
    let result = color_sort(&[1,2,3,4,5,6]);
    assert!(matches!(result, Err(ref e) if e == "Only 1-s, 2-s and 0-s are allowed as list items!"));
}
#[test]
fn test_color_sort() {
    let result = color_sort(&[0,1,2,2,1]).unwrap();
    assert_eq!(result,[0,1,1,2,2]);
}
#[test]
fn test_color_sort_already_sorted() {
    let result = color_sort(&[0,0,1,1,2,2]).unwrap();
    assert_eq!(result,[0,0,1,1,2,2]);
}
