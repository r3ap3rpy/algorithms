use algorithmz::stack::switch_pairs;

#[test]
fn test_switch_pairs() {
    let result = switch_pairs(vec![1,2,3,4,5,6]);
    assert_eq!(result, vec![2,1,4,3,6,5]);
}
