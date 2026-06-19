use algorithmz::string::z_algorithm;

#[test]
fn test_z_algorithm() {
    let text = "aaaaa";
    let pattern = "aa";
    let result = z_algorithm(text, pattern);
    assert_eq!(result, vec![0,1,2,3]);
}

#[test]
fn test_z_algorithm_zero_match() {
    let text = "abcdefg";
    let pattern = "xyz";
    let result = z_algorithm(text, pattern);
    assert_eq!(result, vec![]);
}
