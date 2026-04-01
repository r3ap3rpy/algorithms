use algorithmz::string::alphabet_board_path;

#[test]
fn test_alphabet_board_path() {
    let result = alphabet_board_path("dani");
    assert_eq!(result,"RRR!!DDRRR!DRRR!".to_string());
}
