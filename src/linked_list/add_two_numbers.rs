/// Add Two Numbers
///
/// Given two non-empty linked lists representing two non-negative integers with digits stored in reverse order, add the two numbers and return the sum as a linked list.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use algorithmz::linked_list::{Node, add_two_numbers, from_vec, to_vec};
/// let mut l1 = Box::new(Node::new(2));
/// l1.next = Some(Box::new(Node::new(4)));
/// l1.next.as_mut().unwrap().next = Some(Box::new(Node::new(3)));
/// let mut l2 = Box::new(Node::new(5));
/// l2.next = Some(Box::new(Node::new(6)));
/// l2.next.as_mut().unwrap().next = Some(Box::new(Node::new(4)));
/// let result = add_two_numbers(Some(l1), Some(l2));
/// assert_eq!(to_vec(result),[7,0,8]);
/// ```
pub fn add_two_numbers(mut left: Option<Box<Node>>,mut right: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut dummy = Box::new(Node::new(0));
    let mut current = &mut dummy;
    let mut carry = 0;

    while left.is_some() || right.is_some() {
        carry /= 10;

        if let Some(mut node) = left {
            carry += node.val;
            left = node.next.take();
        } else {
            left = None;
        }

        if let Some(mut node) = right {
            carry += node.val;
            right = node.next.take();
        } else {
            right = None;
        }

        current.next = Some(Box::new(Node::new(carry % 10)));
        current = current.next.as_mut().unwrap();
    }

    if carry / 10 == 1 {
        current.next = Some(Box::new(Node::new(1)));
    }

    dummy.next
}
/// The struct for the Node
pub struct Node {
    /// The value
    pub val: i32,
    /// The pointer for the next value
    pub next: Option<Box<Node>>,
}

impl Node {
    /// Create a new node
    pub fn new(x: i32) -> Self {
        Self {
            val: x,
            next: None,
        }
    }
}
/// Convert from vector
pub fn from_vec(v: Vec<i32>) -> Option<Box<Node>> {
    let mut head = None;

    for x in v.into_iter().rev() {
        let mut node = Box::new(Node::new(x));
        node.next = head;
        head = Some(node);
    }

    head
}
/// Convert to vector
pub fn to_vec(mut head: Option<Box<Node>>) -> Vec<i32> {
    let mut result = Vec::new();

    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }

    result
}
