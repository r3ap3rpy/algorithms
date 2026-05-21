use algorithmz::string::panagram;

#[test]
fn test_panagram_first() {
    let result = panagram("Jackdaws love my big sphinx of quartz.");
    assert_eq!(result, true);
}

#[test]
fn test_panagram_second() {
    let result = panagram("Crazy Fredrick bought many very exquisite opal jewels.");
    assert_eq!(result, true);
}

#[test]
fn test_panagram_fail() {
    let result = panagram("daniel");
    assert_eq!(result, false);
}
