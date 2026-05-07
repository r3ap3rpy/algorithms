/// Knuth-Morris-Pratt algorithm
///
/// Given two sequences (text and pattern), return the list of start indexes in text that match the pattern using the KMP algorithm.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let text: Vec<char> = "hello there hero!".chars().collect();
/// let pattern: Vec<char> = "he".chars().collect();
/// let result = algorithmz::string::knuth_morris_pratt(&text, &pattern);
/// assert_eq!(result,vec![0,7,12])
/// ```
pub fn knuth_morris_pratt<T: PartialEq>(text: &[T], pattern: &[T]) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }

    let pattern_length = pattern.len();

    // Build prefix table (also called LPS table)
    let mut prefix_table = vec![0usize; pattern_length];
    let mut match_length = 0;

    for index in 1..pattern_length {
        while match_length > 0 && pattern[index] != pattern[match_length] {
            match_length = prefix_table[match_length - 1];
        }

        if pattern[index] == pattern[match_length] {
            match_length += 1;
            prefix_table[index] = match_length;
        }
    }

    let mut matches = Vec::new();
    match_length = 0;

    for (index, item) in text.iter().enumerate() {
        while match_length > 0 && *item != pattern[match_length] {
            match_length = prefix_table[match_length - 1];
        }

        if *item == pattern[match_length] {
            match_length += 1;

            if match_length == pattern_length {
                matches.push(index + 1 - pattern_length);
                match_length = prefix_table[match_length - 1];
            }
        }
    }

    matches
}
