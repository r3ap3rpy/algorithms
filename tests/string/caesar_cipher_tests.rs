use algorithmz::string::caesar_cipher;

#[test]
fn test_caesar_cipher_full_circle() {
    let result = caesar_cipher("Daniel",26);
    assert_eq!(result,String::from("Daniel"));
}

#[test]
fn test_caesar_cipher_no_alphabet() {
    let result = caesar_cipher("!>+#&",10);
    assert_eq!(result,String::from("!>+#&"));
}

#[test]
fn test_caesar_cipher() {
    let result = caesar_cipher("Hello_World!",10);
    assert_eq!(result, String::from("Rovvy_Gybvn!"));
}
