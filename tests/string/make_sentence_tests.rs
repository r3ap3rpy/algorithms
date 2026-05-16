use algorithmz::string::make_sentence;

#[test]
fn test_make_sentence_empty_text() {
    let result = make_sentence("", &["","ap","ple"]);
    assert_eq!(result, true);
}

#[test]
fn test_make_sentence() {
    let result = make_sentence("applet", &["", "app", "let", "t", "apple", "applet"]);
    assert_eq!(result, true);
}

