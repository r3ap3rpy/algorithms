use algorithmz::string::strip_url_params;

#[test]
fn test_strip_url_params() {
    let result = strip_url_params("www.saadbenn.com?a=1&b=2&a=2",Some(&["b"]));
    assert_eq!(result, String::from("www.saadbenn.com?a=1"));
}
