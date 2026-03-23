/// Return the length of the longest palindromic substring in s.
///
/// Takes a string and returns a number which is the length of the common substring which is
/// palindromic.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::map::longest_palindromic_subsequence("babad");
/// assert_eq!(result,3);
/// ```
pub fn longest_palindromic_subsequence(s: &str) -> usize {
    let bytes = s.as_bytes();
    let length = bytes.len();
    let mut previous_row: Vec<bool> = vec![false;length];
    let mut current_row: Vec<bool> = vec![false;length];
    let mut longest_length: usize = 0;
    for end in 0..length {
        for start in 0..=end {
            if end - start <= 1 {
                current_row[start] = bytes[start] == bytes[end];
            } else {
                current_row[start] = bytes[start] == bytes[end] && previous_row[start + 1];
            }
            if current_row[start] {
                longest_length = longest_length.max(end - start + 1);
            }
        }
        previous_row.clone_from(&current_row);
        current_row.fill(false);
    }
    return longest_length;
}

