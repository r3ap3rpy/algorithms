use algorithmz::stack::OrderedStack;

#[test]
fn test_empty_stack() {
    let empty_stack = OrderedStack::new();
    assert_eq!(empty_stack.size(), 0);
}

#[test]
fn test_stack_order() {
    let mut ordstack = OrderedStack::new();
    ordstack.push(4);
    ordstack.push(2);
    ordstack.push(3);
    assert_eq!(ordstack.items, vec![2,3,4]);
}
