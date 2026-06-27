use algorithmz::linked_list::{Node, add_two_numbers, to_vec};

#[test]
fn test_add_two_numbers() {
    let mut l1 = Box::new(Node::new(2));
    l1.next = Some(Box::new(Node::new(4)));
    l1.next.as_mut().unwrap().next = Some(Box::new(Node::new(3)));
    let mut l2 = Box::new(Node::new(5));
    l2.next = Some(Box::new(Node::new(6)));
    l2.next.as_mut().unwrap().next = Some(Box::new(Node::new(4)));
    let result = add_two_numbers(Some(l1), Some(l2));
    assert_eq!(to_vec(result),[7,0,8]);
}

