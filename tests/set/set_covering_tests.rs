use algorithmz::set::set_cover;
use std::collections::{HashMap, HashSet};

#[test]
fn test_set_cover() {
    let universe: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();

    let mut subsets: HashMap<String, HashSet<i32>> = HashMap::new();
    subsets.insert("S1".to_string(), [4, 1, 3].into_iter().collect());
    subsets.insert("S2".to_string(), [2, 5].into_iter().collect());
    subsets.insert("S3".to_string(), [1, 4, 3, 2].into_iter().collect());

    let mut costs = HashMap::new();
    costs.insert("S1".to_string(), 5);
    costs.insert("S2".to_string(), 10);
    costs.insert("S3".to_string(), 3);

    let result = set_cover(&universe, &subsets, &costs);
    assert_eq!(result, Some(vec![String::from("S2"),String::from("S3")]));
}
