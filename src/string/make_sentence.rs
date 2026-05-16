/// Make sentence
///
/// For a given string and dictionary, count how many sentences can be formedfrom the string such that all words are contained in the dictionary.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::make_sentence("applet", &["", "app", "let", "t", "apple", "applet"]);
/// assert_eq!(result, true);
/// ```
pub fn make_sentence(text: &str, dictionaries: &[&str] ) -> bool {
    if text.is_empty() {
        return true;
    }
    for index in 1..=text.len() {
        let prefix = &text[..index];
        let suffix = &text[index..];
        if dictionaries.contains(&prefix) { 
            if suffix.is_empty() || make_sentence(suffix, dictionaries) 
                {
                    COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    return true;
            }
        }
    }
    return false;
}
static COUNT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
