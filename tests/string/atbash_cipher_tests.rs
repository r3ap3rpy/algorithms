use algorithmz::string::atbash_cipher;

#[test]
fn test_atbash_cipher() {
    let result = atbash_cipher("abcdefghijklmno");
    assert_eq!(result,"zyxwvutsrqponml".to_string());
}

#[test]
fn test_atbash_cipher_backwards() {
    let result = atbash_cipher("zyxwvutsrqponml");
    assert_eq!(result,"abcdefghijklmno".to_string());
}
