use std::collections::{HashMap, HashSet};
/// The powerset helper function.
fn powerset(items: &[String]) -> Vec<Vec<String>> {
    let n = items.len();
    let mut result = Vec::new();
    for mask in 0..(1<<n){
        let mut subset = Vec::new();
        for i in 0..n {
            if mask & (1<<i) != 0 {
                subset.push(items[i].clone());
            }
        }
        result.push(subset);
    }
    result
}
/// The set_covering function.
///
/// Find the minimum-cost exact set cover via brute force.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use std::collections::{HashMap, HashSet};
/// let universe: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
/// let mut subsets: HashMap<String, HashSet<i32>> = HashMap::new();
/// subsets.insert("S1".to_string(), [4, 1, 3].into_iter().collect());
/// subsets.insert("S2".to_string(), [2, 5].into_iter().collect());
/// subsets.insert("S3".to_string(), [1, 4, 3, 2].into_iter().collect());
/// let mut costs = HashMap::new();
/// costs.insert("S1".to_string(), 5);
/// costs.insert("S2".to_string(), 10);
/// costs.insert("S3".to_string(), 3);
/// let result = algorithmz::set::set_cover(&universe, &subsets, &costs);
/// println!("{:?}", result);
/// ```
pub fn set_cover(
    universe: &HashSet<i32>,
    subsets: &HashMap<String, HashSet<i32>>,
    costs: &HashMap<String, i32>,
) -> Option<Vec<String>> {
    let keys: Vec<String> = subsets.keys().cloned().collect();
    let pset = powerset(&keys);

    let mut best_set: Option<Vec<String>> = None;
    let mut best_cost = i32::MAX;

    for subset in pset {
        let mut covered: HashSet<i32> = HashSet::new();
        let mut cost = 0;

        for name in &subset {
            if let Some(elements) = subsets.get(name) {
                covered.extend(elements);
            }

            if let Some(c) = costs.get(name) {
                cost += c;
            }
        }

        if covered.len() == universe.len() && cost < best_cost {
            best_set = Some(subset.clone());
            best_cost = cost;
        }
    }

    best_set
}
