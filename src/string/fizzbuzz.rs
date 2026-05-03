/// FizzBuzz
///
/// Return an array of numbers from 1 to N, replacing multiples of 3 with 'Fizz', multiples of 5 with 'Buzz', and multiples of both with 'FizzBuzz'.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use algorithmz::string::{fizzbuzz,Buzz};
/// let result = fizzbuzz(5);
/// assert_eq!(result, vec![Buzz::Number(1),Buzz::Number(2),Buzz::Text(String::from("Fizz")),Buzz::Number(4),Buzz::Text(String::from("Buzz"))]);
/// ```
pub fn fizzbuzz(n: usize) -> Vec<Buzz> {
    let mut result: Vec<Buzz> = Vec::new();
    if n == 0 {
        return result;
    }
    for i in 1..=n {
        if i % 15 == 0 {
            result.push(Buzz::Text(String::from("FizzBuzz")));
        } else if i % 3 == 0 {
            result.push(Buzz::Text(String::from("Fizz")));
        } else if i % 5 == 0 {
            result.push(Buzz::Text(String::from("Buzz")));
        } else {
            result.push(Buzz::Number(i as i32))
        }
    }
    return result;
}
/// Enum representing the result
///
/// Used as a helper because the vec is only able to hold homogeneous items.
#[derive(Debug, PartialEq)]
pub enum Buzz {
    /// Text portion
    Text(String),
    /// Number portion
    Number(i32),
}
