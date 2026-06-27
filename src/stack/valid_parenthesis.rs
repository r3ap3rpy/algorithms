/// Valid Parenthesis
///
/// Determine if a string containing only '(', ')', '{', '}', '[' and ']' has valid (properly closed and nested) brackets.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::stack::valid_parenthesis("()[]{}");
/// assert_eq!(result, true);
/// ```
pub fn valid_parenthesis(text: &str) -> bool {
    let matching: std::collections::HashMap<char, char> =
        [(')', '('), (']', '['), ('}', '{')].into_iter().collect();

    let mut stack: Vec<char> = Vec::new();

    for ch in text.chars() {
        if ch == '(' || ch == '[' || ch == '{' {
            stack.push(ch);
        } else if let Some(&expected) = matching.get(&ch) {
            if stack.pop() != Some(expected) {
                return false;
            }
        }
    }

    stack.is_empty()
}
