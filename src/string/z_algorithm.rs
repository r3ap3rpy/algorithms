/// Z-algorithm
///
/// The Z-array for a string S stores at Z[i] the length of the longest substring starting at S[i] that is also a prefix of S. By concatenating pattern + '$' + text, occurrences of the pattern correspond to positions where Z[i] == len(pattern).
///
/// # Examples
///
/// Basic usage:
/// ```
/// let text = "ababcababcabc";
/// let pattern = "ababc";
/// let result = algorithmz::string::z_algorithm(text, pattern);
/// assert_eq!(result,vec![0,5]);
/// ```
fn compute_z_array(s: &str) -> Vec<usize> {
    let bytes = s.as_bytes();
    let n = bytes.len();

    if n == 0 {
        return Vec::new();
    }

    let mut z = vec![0; n];
    z[0] = n;

    let mut left = 0;
    let mut right = 0;

    for i in 1..n {
        if i < right {
            z[i] = (right - i).min(z[i - left]);
        }

        while i + z[i] < n && bytes[z[i]] == bytes[i + z[i]] {
            z[i] += 1;
        }

        if i + z[i] > right {
            left = i;
            right = i + z[i];
        }
    }
    z
}
/// The z_algorithm function
pub fn z_algorithm(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() || text.is_empty() {
        return Vec::new();
    }

    let concat = format!("{}${}", pattern, text);
    let z = compute_z_array(&concat);
    let m = pattern.len();

    (m + 1..concat.len())
        .filter(|&i| z[i] == m)
        .map(|i| i - m - 1)
        .collect()
}

