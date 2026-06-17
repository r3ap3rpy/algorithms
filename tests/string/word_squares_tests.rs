use algorithmz::string::word_squares;


#[test]
fn test_word_squares() {
    let words = vec![
        "area".to_string(),
        "lead".to_string(),
        "wall".to_string(),
        "lady".to_string(),
        "ball".to_string(),
    ];
    let result = word_squares(words);
    assert_eq!(result, vec![vec!["wall".to_string(), "area".to_string(), "lead".to_string(), "lady".to_string()],vec!["ball".to_string(), "area".to_string(), "lead".to_string(), "lady".to_string()]]);    
}
