/// One Edit Distance
///
/// Given two strings, determine if they are exactly one edit distance apart. An edit is an insertion, deletion, or replacement of a single character.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::is_one_edit("abc","abd");
/// assert_eq!(result, true);
/// ```
pub fn is_one_edit(source: &str, target: &str) -> bool {
    if source.len() > target.len() {
        return is_one_edit(target, source);
    }

    if target.len() - source.len() > 1 || source == target {
        return false;
    }

    let s: Vec<char> = source.chars().collect();
    let t: Vec<char> = target.chars().collect();

    for i in 0..s.len() {
        if s[i] != t[i] {
            let replace = s[i + 1..] == t[i + 1..];

            let insert_delete = s[i..] == t[i + 1..];

            return replace || insert_delete;
        }
    }

    true
}

