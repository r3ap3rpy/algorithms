/// Copy Random Pointer
///
/// Given a linked list where each node contains an additional random pointer that could point to any node in the list or null, return a deep copy of the list.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use algorithmz::linked_list::{RandomListNode, copy_random_pointer};
/// use std::rc::Rc;
/// let node = RandomListNode::new(1);
/// node.borrow_mut().random = Some(node.clone());
/// let copied = copy_random_pointer(Some(node.clone())).unwrap();
/// assert_eq!(copied.borrow().label, 1);
/// assert!(!Rc::ptr_eq(&node, &copied));
/// let copied_random = copied.borrow().random.clone().unwrap();
/// assert!(Rc::ptr_eq(&copied, &copied_random));
/// ```
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

type Node = Rc<RefCell<RandomListNode>>;

/// RandomListNode struct definition
#[derive(Debug)]
pub struct RandomListNode {
    /// label of the node
    pub label: i32,
    /// next node 
    pub next: Option<Node>,
    /// random node
    pub random: Option<Node>,
}

impl RandomListNode {
    /// function for creation of node
    pub fn new(label: i32) -> Node {
        Rc::new(RefCell::new(RandomListNode {
            label,
            next: None,
            random: None,
        }))
    }
}
/// Function for the implementation of the algorithm
pub fn copy_random_pointer(head: Option<Node>) -> Option<Node> {
    let mut node_map: HashMap<*const RefCell<RandomListNode>, Node> = HashMap::new();

    let mut current = head.clone();
    while let Some(node) = current {
        let ptr = Rc::as_ptr(&node);
        let label = node.borrow().label;
        node_map.insert(ptr, RandomListNode::new(label));
        current = node.borrow().next.clone();
    }

    current = head.clone();
    while let Some(node) = current {
        let ptr = Rc::as_ptr(&node);
        let copy = node_map.get(&ptr).unwrap().clone();

        let next = node
            .borrow()
            .next
            .as_ref()
            .and_then(|n| node_map.get(&Rc::as_ptr(n)).cloned());

        let random = node
            .borrow()
            .random
            .as_ref()
            .and_then(|n| node_map.get(&Rc::as_ptr(n)).cloned());

        {
            let mut copy_mut = copy.borrow_mut();
            copy_mut.next = next;
            copy_mut.random = random;
        }

        current = node.borrow().next.clone();
    }

    head.as_ref()
        .and_then(|node| node_map.get(&Rc::as_ptr(node)).cloned())
}
