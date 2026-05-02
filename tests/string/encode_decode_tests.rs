use algorithmz::string::{encode,decode};

#[test]
fn test_encode() {
    let result  = encode("daniel is also awesome");
    assert_eq!(result,String::from("6:daniel2:is4:also7:awesome"));
}

#[test]
fn test_decode() {
    let result = decode("6:daniel2:is4:also7:awesome");
    assert_eq!(result, String::from("daniel is also awesome"));
}
