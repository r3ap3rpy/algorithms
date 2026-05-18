/// Minimum distance
/// 
/// Given two words, find the minimum number of steps required to make them the same, where each step deletes one character from either string.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::min_distance("sea","eat");
/// assert_eq!(result, 2);
/// ```
pub fn min_distance(word1: &str, word2: &str) -> usize {
    let lcs_len = lcs(
        word1.as_bytes(),
        word2.as_bytes(),
        word1.len(),
        word2.len(),
    );

    word1.len() + word2.len() - 2 * lcs_len
}
/// Helper function for the `min_distance` function.
fn lcs(word1: &[u8], word2: &[u8], length1: usize, length2: usize) -> usize {
    if length1 == 0 || length2 == 0 {
        return 0;
    }

    if word1[length1 - 1] == word2[length2 - 1] {
        return 1 + lcs(word1, word2, length1 - 1, length2 - 1);
    }

    std::cmp::max(
        lcs(word1, word2, length1 - 1, length2),
        lcs(word1, word2, length1, length2 - 1),
    )
}
