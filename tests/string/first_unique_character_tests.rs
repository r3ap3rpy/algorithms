use algorithmz::string::first_unique_character;

#[test] 
fn test_first_unique_character_empty() {
    let result = first_unique_character("");
    assert_eq!(result,0);
}
#[test]
fn test_first_unique_character() {
    let result = first_unique_character("teetlcode");
    assert_eq!(result,4);
}
#[test]
fn test_first_unique_character_nounique() {
    let result = first_unique_character("teet");
    assert_eq!(result, -1);
}
