/// Breaking Bad Symbol Matching
/// Given an array of words and an array of symbols, display each word with its matched symbol surrounded by square brackets. If a word matches more than one symbol, choose the one with the longest length
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::breaking_bad(&vec!["Google"],&vec!["le"]);
/// assert_eq!(result, vec!["Goog[le]".to_string()]);
/// ```
pub fn breaking_bad(words: &[&str], symbols: &[&str]) -> Vec<String> {
    let mut combined = Vec::new();
    for symbol in symbols {
        for word in words {
            if word.contains(symbol) {
                let replaced = word.replace(symbol, &format!("[{}]",symbol));
                combined.push(replaced);
            }
        }
    }
    combined
}
