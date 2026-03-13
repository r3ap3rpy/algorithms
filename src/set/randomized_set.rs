/// A data structure that supports insert, remove, and get-random-element operations, all in average O(1) time.
///
/// # Examples:
///
/// Basic usage:
/// ```
/// let mut rs = algorithmz::set::RandomizedSet::new();
/// rs.insert(10);
/// rs.insert(20);
/// rs.insert(30);
/// rs.remove(20);
///  match rs.random_element() {
///    Some(v) => println!("Random element: {}", v),
///     None => println!("Set is empty"),
/// }
/// ```
pub struct RandomizedSet {
    /// List
    pub elements: Vec<i32>,
    /// HashMap
    pub index_map: std::collections::HashMap<i32, usize>,
}

impl RandomizedSet {
    /// Instantiate
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            index_map: std::collections::HashMap::new(),
        }
    }
    /// Insert value
    pub fn insert(&mut self, new_one: i32) {
        if self.index_map.contains_key(&new_one) {
            return;
        }
        let index = self.elements.len();
        self.elements.push(new_one);
        self.index_map.insert(new_one, index);
    }
    ///  Remove value
    pub fn remove(&mut self, old_one: i32) {
        if let Some(&index) = self.index_map.get(&old_one) {
            let last = *self.elements.last().unwrap();

            self.elements[index] = last;
            self.index_map.insert(last, index);

            self.elements.pop();
            self.index_map.remove(&old_one);
        }
    }
    /// Random element to pick
    pub fn random_element(&self) -> Option<i32> {
        if self.elements.is_empty() {
            return None;
        }

        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();

        let index = (nanos as usize) % self.elements.len();
        Some(self.elements[index])
    }
}


