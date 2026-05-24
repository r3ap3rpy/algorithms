/// Repeat string
///
/// Given two strings A and B, find the minimum number of times A has to be repeated such that B is a substring of the result. Return -1 if impossible.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::repeat_string("abcd", "cdabcdab");
/// assert_eq!(result, 3);
/// ```
pub fn repeat_string(base: &str, target: &str) -> i32 {
    let mut repetition_count = 1;
    let mut repeated = base.to_string();

    let max_count = (target.len() / base.len()) + 2;

    while !repeated.contains(target) {
        repeated.push_str(base);

        if repetition_count > max_count as i32 {
            return -1;
        }

        repetition_count += 1;
    }

    repetition_count
}
