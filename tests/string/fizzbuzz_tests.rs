use algorithmz::string::{fizzbuzz,Buzz};

#[test]
fn test_fizzbuzz() {
    let result = fizzbuzz(5);
    assert_eq!(result, vec![Buzz::Number(1),Buzz::Number(2),Buzz::Text(String::from("Fizz")),Buzz::Number(4),Buzz::Text(String::from("Buzz"))]);
}

#[test]
fn test_fizzbuzz_empty() {
    let result = fizzbuzz(0);
    assert_eq!(result,Vec::new());
}
