/// Longest Absolute File Path
///
/// Given a string representing a file system in a special format, find the length of the longest absolute path to a file. Directories and files are separated by newlines; depth is indicated by tab characters.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::stack::longest_path("dir\\n\\tfile.txt");
/// assert_eq!(result, 15);
/// ```
pub fn longest_path(path: &str) -> i32 {
    let mut current_length: i32 = 0;
    let mut max_length: i32 = 0;
    let mut stack: Vec<i32> = Vec::new();
    for segment in path.split('\n') {
        let tab_count = segment.matches('\t').count();
        while stack.len() > tab_count {
            current_length -= stack.pop().unwrap();
        }
        let name_length = segment.trim_matches('\t').len() as i32 + 1;
        stack.push(name_length);
        current_length += *stack.last().unwrap();
        if segment.contains('.') {
            max_length = max_length.max(current_length - 1);
        }
    }
    return max_length;
}

