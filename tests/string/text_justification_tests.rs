use algorithmz::string::text_justification;

#[test]
fn test_text_justification() {
    let result = text_justification(&vec!["This".to_string(),"is".to_string(),"to".to_string(),"be".to_string()],30);
    assert_eq!(result, Ok(vec!["This is to be                 ".to_string()]));
}
