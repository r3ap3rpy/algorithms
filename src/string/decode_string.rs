/// Decode string
///
/// Given an encoded string, return its decoded string. The encoding rule is k[encoded_string], where the encoded_string inside the brackets is repeated exactly k times.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::decode_string("3[a]2[bc]");
/// assert_eq!(result, String::from("aaabcbc"))
/// ```
pub fn decode_string(text: &str) -> String {
    let mut stack: Vec<(String, i32)> = Vec::new();
    let mut current_num: i32 = 0;
    let mut current_string: String = String::new();

    for character in text.chars() {
        println!("{:?}",character);
        if character == '[' {
            stack.push((current_string.clone(), current_num.clone()));
            current_string = String::from("");
            current_num = 0;
        } else if character == ']' {
            let (prev_string, num) = stack.pop().unwrap();
            current_string = prev_string + &current_string.repeat(num as usize);
        } else if character.is_digit(10) {
            current_num = current_num * 10 + character.to_digit(10).unwrap() as i32;
        } else {
            current_string.push(character);
        }
    }
    current_string
}
