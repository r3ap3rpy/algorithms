use algorithmz::string::convert_morse;

#[test]
fn test_convert_morse_empty() {
    let result = convert_morse("");
    assert_eq!(result, Err("Cannot convert an empty string!".to_string()));
}

#[test]
fn test_convert_morse() {
    let result = convert_morse("tonic");
    assert_eq!(result, Ok(String::from("-----...-.-.")));
}
