/// Delete Node
///
/// Given only access to a node (not the tail) in a singly linked list, delete that node by copying the next node's value and skipping over it.
///
/// # Examples
/// ``` 
/// use algorithmz::linked_list::{NNode as Node,delete_node};
///
/// let mut head = Box::new(Node::new(1));
/// head.next = Some(Box::new(Node::new(2)));
/// head.next.as_mut().unwrap().next = Some(Box::new(Node::new(3)));
/// delete_node(head.next.as_mut().unwrap()).unwrap();
/// assert_eq!(head.val, 1);
/// assert_eq!(head.next.as_ref().unwrap().val, 3);
///assert!(head.next.as_ref().unwrap().next.is_none());
/// ```
/// The Node implementation
#[derive(Debug)]
pub struct Node {
    /// Value of node
    pub val: i32,
    /// Pointer to next node
    pub next: Option<Box<Node>>,
}

impl Node {
    /// Creation of the new node
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}
/// Function which can delete a node
pub fn delete_node(node: &mut Node) -> Result<(), &'static str> {
    let mut next = node.next.take().ok_or("node is the tail")?;

    node.val = next.val;
    node.next = next.next.take();

    Ok(())
}


