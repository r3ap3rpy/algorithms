use algorithmz::sorting::gnome_sort;

#[test]
fn test_gnome_sort_empty() {
    let result = gnome_sort(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use gnome sort on an empty array!"));
}
#[test]
fn test_gnome_sort() {
    let result = gnome_sort(&[2,1,4,3,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
#[test]
fn test_gnome_sort_already_sorted() {
    let result = gnome_sort(&[1,2,3,4,5]).unwrap();
    assert_eq!(result,[1,2,3,4,5]);
}
