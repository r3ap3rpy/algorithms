/// Ordered stack
///
/// A stack that maintains elements in sorted order, with the highest value at the top and the lowest at the bottom. Push operations preserve the ordering invariant.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let mut ordstack = algorithmz::stack::OrderedStack::new();
/// ordstack.push(3);
/// ordstack.push(1);
/// ordstack.push(2);
/// assert_eq!(ordstack.items, vec![1,2,3]);
/// ```
#[derive(Debug)]
pub struct OrderedStack {
    /// items of the stack
    pub items: Vec<i32>,
}

impl OrderedStack {
    /// create a new stack
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// push item
    pub fn push_direct(&mut self, item: i32) {
        self.items.push(item);
    }
    /// check item without removing
    pub fn peek(&self) -> Option<i32> {
        self.items.last().copied()
    }
    /// push item
    pub fn push(&mut self, item: i32) {
        let mut temp_stack = OrderedStack::new();

        match self.peek() {
            None => self.push_direct(item),
            Some(top) if item > top => self.push_direct(item),
            Some(_) => {
                while !self.is_empty()
                    && item < self.peek().unwrap()
                {
                    temp_stack.push_direct(self.pop().unwrap());
                }

                self.push_direct(item);

                while let Some(value) = temp_stack.pop() {
                    self.push_direct(value);
                }
            }
        }
    }
    /// remove item
    pub fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }
    /// check size
    pub fn size(&self) -> usize {
        self.items.len()
    }
}

