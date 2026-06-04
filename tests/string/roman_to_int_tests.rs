use algorithmz::string::roman_to_int;

#[test]
fn test_roman_to_int_bad_character() {
    let result = roman_to_int("MMCK");
    assert_eq!(result, Err("The specified character is invalid: K".to_string()))
}

#[test]
fn test_roman_to_int() {
    let result = roman_to_int("MCMXCIV");
    assert_eq!(result, Ok(1994));
}

#[test]
fn test_roman_to_int_empty() {
    let result = roman_to_int("");
    assert_eq!(result, Ok(0));
}
