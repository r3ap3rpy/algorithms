/// Check if two strings are isomorphic.
///
/// Takes two string slices an returns true or false based on whether they are isomorphic or not.
///
/// # Examples
///
/// Basic usage:
/// ```
/// assert!(algorithmz::map::is_isomorphic("egg", "add"));
/// ```
///
/// Another example:
/// ```
/// assert!(!algorithmz::map::is_isomorphic("foo", "bar"));
/// ```
pub fn is_isomorphic(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return  false;
    } 
    let mut mapping: std::collections::HashMap<char, char> = std::collections::HashMap::new();
    let mut mapped_values: std::collections::HashSet<char> = std::collections::HashSet::new();
    for (c1, c2) in s.chars().zip(t.chars()) {
        if !mapping.contains_key(&c1) {
            if mapped_values.contains(&c2) {
                return false;
            }
            mapping.insert(c1,c2);
            mapped_values.insert(c2);
        } else {
            if mapping[&c1] != c2 {
                return false;
            }
        }
    }
    true
}

