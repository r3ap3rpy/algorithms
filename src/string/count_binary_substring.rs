/// Binary substring
///
/// Count substrings with equal consecutive 0s and 1s
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::count_binary_substring("00110011");
/// assert_eq!(result,6);
/// ```
pub fn count_binary_substring(input_string: &str) -> i32 {
    let input = input_string.as_bytes();
    let mut current = 1;
    let mut previous = 0;
    let mut count = 0;
    for index in 1..input.len() {
        if input[index] != input[index - 1] {
            count = count + std::cmp::min(previous, current);
            previous = current;
            current = 1;
        } else {
            current = current + 1;
        }
    }
    count = count + std::cmp::min(previous, current);
    count 
}
