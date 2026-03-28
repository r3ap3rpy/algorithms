/// Add binary
///
/// Given two binary strings, return their sum as a binary string.
///
/// # Examples
///
/// Basic example:
/// ```
/// let result = algorithmz::string::add_binary("11","1");
/// assert_eq!(result, "100".to_string());
/// ```
pub fn add_binary(first: &str, second: &str) -> String {
    let first_bytes = first.as_bytes();
    let second_bytes = second.as_bytes();
    let mut i = first_bytes.len() as i32 - 1;
    let mut j = second_bytes.len() as i32 - 1;
    let mut carry = 0;
    let mut result = String::new();

    while i >= 0 || j >= 0 || carry == 1 {
        if i >= 0 {
            carry += (first_bytes[i as usize] - b'0') as i32;
            i -= 1;
        }
        if j >= 0 {
            carry += (second_bytes[j as usize] - b'0') as i32;
            j -= 1;
        }

        result.push(((carry % 2) as u8 + b'0') as char);
        carry /= 2;
    }

    // reverse because we built it backwards
    result.chars().rev().collect()
}
