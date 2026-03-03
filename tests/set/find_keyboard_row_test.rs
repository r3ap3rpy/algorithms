use algorithmz::set::find_keyboard_row;

#[test]
fn test_find_keyboard_row_empty() {
    let result = find_keyboard_row(vec![]);
    assert!(matches!(result, Err(ref e) if e == "Cannot verify if an empty list can be typed!"));
}
#[test]
fn test_find_keyboard_row_none_found() {
    let result = find_keyboard_row(vec!["wera".to_string(),"wdr".to_string(),"drw".to_string()]);
    assert!(matches!(result, Err(ref e) if e == "None of the words specified can be typed!"));
}
#[test]
fn test_find_keyboard_row() {
    let result = find_keyboard_row(vec!["were".to_string(),"quit".to_string(),"wtf".to_string()]).unwrap();
    assert_eq!(result,["were","quit"]);
}
