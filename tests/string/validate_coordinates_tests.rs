use algorithmz::string::validate_coordinates;

#[test]
fn test_validate_coordinates_empty() {
    let result = validate_coordinates("");
    assert_eq!(result,Err("Empty string cannot be validated!".to_string()));
}

#[test]
fn test_validate_coordinates_invalid() {
    let result = validate_coordinates("-91,-180");
    assert_eq!(result, Err("Invalid coordinates specified!".to_string()));
}

#[test]
fn test_validate_coordinates() {
    let result = validate_coordinates("-44, -110");
    assert_eq!(result, Ok(true));
}
