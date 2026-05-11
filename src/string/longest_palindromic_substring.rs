/// Longest palindromic substring
///
/// Given a string, find the longest palindromic substring using Manacher's algorithm, which runs in linear time.
///
/// # Examples
/// 
/// Basic usage:
/// ``` 
/// let result = algorithmz::string::longest_palindromic_substring("cbbd");
/// assert_eq!(result, String::from("bb"));
/// ```
pub fn longest_palindromic_substring(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();

    if n == 0 {
        return String::new();
    }

    let mut start = 0;
    let mut max_len = 1;

    // Expand around center
    fn expand(chars: &[char], mut left: isize, mut right: isize) -> (usize, usize) {
        while left >= 0
            && right < chars.len() as isize
            && chars[left as usize] == chars[right as usize]
        {
            left -= 1;
            right += 1;
        }

        let start = (left + 1) as usize;
        let len = (right - left - 1) as usize;
        (start, len)
    }

    for i in 0..n {
        // Odd-length palindrome
        let (s1, l1) = expand(&chars, i as isize, i as isize);

        if l1 > max_len {
            start = s1;
            max_len = l1;
        }

        // Even-length palindrome
        let (s2, l2) = expand(&chars, i as isize, i as isize + 1);

        if l2 > max_len {
            start = s2;
            max_len = l2;
        }
    }

    chars[start..start + max_len].iter().collect()
}

