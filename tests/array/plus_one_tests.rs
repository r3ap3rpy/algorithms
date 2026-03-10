use algorithmz::array::plus_one;

#[test]
fn test_plus_one_empty() {
    let result = algorithmz::array::plus_one(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use plus one on an empty list!"));
}

#[test]
fn test_plus_one() {
    match plus_one(&[1,2,9]) {
        Ok(n) => println!("The result was: {:?}",n),
        Err(e) => eprintln!("The error was: {}",e),
    }
}
