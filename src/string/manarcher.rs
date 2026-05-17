/// Manarcher algorithm
///
/// Manacher's algorithm uses the symmetry of palindromes to avoid redundant comparisons, achieving linear time. It transforms the input to handle both odd- and even-length palindromes uniformly.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::manarcher("babad");
/// assert_eq!(result, String::from("aba"));
/// ```
pub fn manarcher(s: &str) -> String {
    let t = format!(
        "^#{}#$",
        s.chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("#")
    );
    let t_chars: Vec<char> = t.chars().collect();
    let n = t_chars.len();

    let mut p = vec![0usize; n];
    let mut center = 0usize;
    let mut right = 0usize;

    for i in 1..n - 1 {
        let mirror = 2 * center as isize - i as isize;

        if i < right {
            p[i] = p[mirror as usize].min(right - i);
        }

        while t_chars[i + p[i] + 1] == t_chars[i - p[i] - 1] {
            p[i] += 1;
        }

        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
    }

    let (center_index, &max_len) = p
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .unwrap();

    let start = (center_index - max_len) / 2;

    s.chars()
        .skip(start)
        .take(max_len)
        .collect()
}
