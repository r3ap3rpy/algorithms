use algorithmz::string::merge_string_checker;

#[test]
fn test_merge_string_checker_text_empty() {
    let result = merge_string_checker(b"",b"a",b"b");
    assert_eq!(result, false);
}

#[test]
fn test_merge_string_checker_first_empty() {
    let result = merge_string_checker(b"codewars",b"",b"wars");
    assert_eq!(result,false);
}

#[test]
fn test_merge_string_checker_second_empty() {
    let result = merge_string_checker(b"codewars",b"code",b"");
    assert_eq!(result,false);
}

#[test]
fn test_merge_string_checker() {
    let result = merge_string_checker(b"codewars",b"code",b"wars");
    assert_eq!(result, true);
}
