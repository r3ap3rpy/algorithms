use algorithmz::linked_list::{copy_random_pointer, RandomListNode};
use std::rc::Rc;

#[test]
fn test_copy_random_pointer() {
    let node = RandomListNode::new(1);
    node.borrow_mut().random = Some(node.clone());
    let copied = copy_random_pointer(Some(node.clone())).unwrap();
    assert_eq!(copied.borrow().label, 1);
    assert!(!Rc::ptr_eq(&node, &copied));
    let copied_random = copied.borrow().random.clone().unwrap();
    assert!(Rc::ptr_eq(&copied, &copied_random));
}
