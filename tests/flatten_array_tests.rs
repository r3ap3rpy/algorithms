use algorithmz::array::{flatten,Nested};

#[test]
fn test_flatten_array_empty() {
    let result = flatten(&[]);
    assert!(matches!(result, Err(ref e) if e == "Input slice cannot be empty"));
}

#[test]
fn test_flatten_array(){
    let input = vec![Nested::Item(2),Nested::List(vec![Nested::Item(3),Nested::Item(5)])];
    let result = flatten(&input).unwrap();
    assert_eq!(result, [2,3,5]);
}
#[test]
fn test_flatten_array_empty_lists() {
    let input = vec![Nested::List(vec![]), Nested::List(vec![])];
    let result = flatten(&input).unwrap();
    assert_eq!(result, []);
}
