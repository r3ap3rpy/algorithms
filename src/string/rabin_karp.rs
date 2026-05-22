/// Helper struct
///
/// Implementation for the RollingHash 
pub struct RollingHash<'a> {
    text: &'a str,
    hash: u64,
    window_size: usize,
    window_start: usize,
    window_end: usize,
}

impl<'a> RollingHash<'a> {
    fn new(text: &'a str, window_size: usize) -> Self {
        let bytes = text.as_bytes();
        let mut hash = 0u64;

        for i in 0..window_size {
            hash += ((bytes[i] - b'a' + 1) as u64)
                * 26u64.pow((window_size - i - 1) as u32);
        }

        Self {
            text,
            hash,
            window_size,
            window_start: 0,
            window_end: window_size,
        }
    }

    fn move_window(&mut self) {
        let bytes = self.text.as_bytes();

        if self.window_end <= bytes.len() - 1 {
            self.hash -= ((bytes[self.window_start] - b'a' + 1) as u64)
                * 26u64.pow((self.window_size - 1) as u32);

            self.hash *= 26;

            self.hash += (bytes[self.window_end] - b'a' + 1) as u64;

            self.window_start += 1;
            self.window_end += 1;
        }
    }

    fn window_text(&self) -> &str {
        &self.text[self.window_start..self.window_end]
    }
}

/// Find the first occurrence of `word` in `text` using Rabin-Karp.
///
/// Returns:
///     Some(index) if found, otherwise None.
///
/// Example:
/// ```
/// assert_eq!(algorithmz::string::rabin_karp("abc", "zsnabckfkd"), Some(3));
/// ```
pub fn rabin_karp(word: &str, text: &str) -> Option<usize> {
    if word.is_empty() || text.is_empty() {
        return None;
    }

    if word.len() > text.len() {
        return None;
    }

    let mut rolling_hash = RollingHash::new(text, word.len());
    let word_hash = RollingHash::new(word, word.len());

    for _ in 0..=(text.len() - word.len()) {
        if rolling_hash.hash == word_hash.hash
            && rolling_hash.window_text() == word
        {
            return Some(rolling_hash.window_start);
        }

        rolling_hash.move_window();
    }

    None
}
