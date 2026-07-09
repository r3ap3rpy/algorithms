use algorithmz::linked_list::{NNode as Node, delete_node};

#[test]
fn test_delete_node() {
    let mut head = Box::new(Node::new(1));
    head.next = Some(Box::new(Node::new(2)));
    head.next.as_mut().unwrap().next = Some(Box::new(Node::new(3)));
    delete_node(head.next.as_mut().unwrap()).unwrap();
    assert_eq!(head.val, 1);
    assert_eq!(head.next.as_ref().unwrap().val, 3);
    assert!(head.next.as_ref().unwrap().next.is_none());
}
