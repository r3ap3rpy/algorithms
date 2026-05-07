use algorithmz::string::judge_circle;

#[test]
fn test_judge_circle_empty() {
    let result = judge_circle("");
    assert_eq!(result, true);
}

#[test]
fn test_judge_circle() {
    let result = judge_circle("UDLR");
    assert_eq!(result, true);
}

#[test]
fn test_judge_circle_fail() {
    let result = judge_circle("UUD");
    assert_eq!(result,false);
}
