/// Find the longest common substring between s1 and s2.
///
/// Assumes s2 has all unique characters, enabling an index-based matching approach.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::map::longest_common_subsequence("abcdef","adcbef");
/// assert_eq!(result, "ef");
/// ```
pub fn longest_common_subsequence(s1: &str, s2: &str) -> String {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    let char_index: std::collections::HashMap<u8, usize> = s2_bytes.iter().enumerate().map(|(i,&c)| (c,i)).collect();
    let mut max_length = 0;
    let mut best_start = 0;
    let mut i = 0;
    while i < s1_bytes.len() {
        if let Some(&j_start) = char_index.get(&s1_bytes[i]) {
            let mut j = j_start;
            let mut k = i;

            while j < s2_bytes.len()
                && k < s1_bytes.len()
                && s1_bytes[k] == s2_bytes[j]
            {
                k += 1;
                j += 1;
            }

            if k - i > max_length {
                max_length = k - i;
                best_start = i;
            }

            i = k;
        } else {
            i += 1;
        }
    }
    s1[best_start..best_start + max_length].to_string()
}
