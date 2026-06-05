use algorithmz::string::rotate;

#[test]
fn test_rotate_empty() {
    let result = rotate("",10);
    assert_eq!(result, Err("Cannot rotate an empty string!".to_string()));
}

#[test]
fn test_rotate() {
    let result = rotate("dani",2);
    assert_eq!(result, Ok(String::from("nida")));
}
