use algorithmz::string::domain_name;

#[test]
fn test_domain_name() {
    let result = domain_name("https://google.com/@r3ap3rpy");
    assert_eq!(result, "google");
}

#[test]
fn test_domain_name_empty() {
    let result = domain_name("");
    assert_eq!(result, "");
}
