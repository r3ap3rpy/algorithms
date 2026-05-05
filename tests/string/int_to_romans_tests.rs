use algorithmz::string::int_to_roman;

#[test]
fn test_int_to_romans() {
    let result = int_to_roman(100);
    assert_eq!(result,"C".to_string());
}

#[test]
fn test_int_to_romans_too_low(){
    let result = int_to_roman(-1);
    assert_eq!(result,"Too Low!".to_string());
}

#[test]
fn test_int_to_romans_too_high() {
    let result = int_to_roman(4000);
    assert_eq!(result, "Too High!".to_string());
}
