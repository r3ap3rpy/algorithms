/// Simplify path
///
/// Given an absolute Unix-style file path, simplify it by resolving '.' (current directory), '..' (parent directory), and multiple slashes.
///
/// # Examples
///
/// Basic usage:
/// ``` 
/// let result = algorithmz::stack::simplify_path("/home/");
/// assert_eq!(result, String::from("/home"));
/// ```
pub fn simplify_path(original: &str) -> String {
    let skip: std::collections::HashSet<&str> = ["..", ".", ""].into_iter().collect();
    let mut stack: Vec<&str> = Vec::new();
    let tokens = original.split("/");
    for token in tokens {
        if token == ".." {
            if !stack.is_empty() {
                stack.pop();
            }
        }
        else if !skip.contains(token) {
            stack.push(token);
        }
    }
    return format!("/{}", stack.join("/"));
}

