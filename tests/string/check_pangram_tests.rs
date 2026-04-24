use algorithmz::string::check_pangram;

#[test]
fn test_check_pangram_empty() {
    let result = check_pangram("");
    assert!(matches!(result,Err(ref e) if e == "The input cannot be empty!"));
}

#[test]
fn test_check_pangram_ok() {
    match check_pangram("The quick brown fox jumps over the lazy dog") {
        Ok(result) => assert!(result),
        Err(error) => panic!("The sentence faced an error: {}",error),
    } 
}

#[test]
fn test_check_pangram_false() {
    let result = check_pangram("The quick brown fox jumps over the laxy do").unwrap();
    assert_eq!(result,false);
}
