/// Find the longest substring without repeating characters using sliding window.
///
/// Takes a string slice and returns a tuple with the length and the longest non repeating
/// substring or an error if applicable
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::longest_non_repeat("abcabcbb").unwrap();
/// assert_eq!(result,(3,"abc".to_string()));
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::longest_non_repeat;
/// match longest_non_repeat("abcabcbb") {
///     Ok((n,s)) => println!("Works: {}, {}",n,s),
///     Err(e) => eprintln!("The problem was: {}",e),
/// }
/// ```
pub fn longest_non_repeat(string: &str) -> Result<(usize,String),String> {
    if string.is_empty() {
        return Err("Cannot identify longest non repeat on an empty string!".to_string());
    }
    let chars: Vec<char> = string.chars().collect();
    let mut longest_substring = String::new();
    let mut seen: std::collections::HashSet<char> = std::collections::HashSet::new();
    let mut start_index = 0;
    for i in 0..chars.len() {
        while seen.contains(&chars[i]) {
            seen.remove(&chars[start_index]);
            start_index += 1;
        }
        seen.insert(chars[i]);
        let candidate: String = chars[start_index..=i].iter().collect();
        if candidate.len() > longest_substring.len() {
            longest_substring = candidate;
        }
    }

    Ok((longest_substring.len(),longest_substring))

}

