use algorithmz::string::swap_characters;

#[test]
fn test_swap_characters_not_equal_length() {
    let result = swap_characters("dani","abc");
    assert_eq!(result,Err("Length must be equal!".to_string()));
}

#[test]
fn test_swap_characters() {
    let result = swap_characters("abcd","bacd");
    assert_eq!(result, Ok(true));
}
