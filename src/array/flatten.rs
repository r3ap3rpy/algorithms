/// Returns the flattened version of the input array.
///
/// This function takes an array and returns the flattened version of it
///
/// Basic usage:
/// ```
/// use algorithmz::array::{Nested, flatten};
/// let input = vec![Nested::Item(2),Nested::List(vec![Nested::Item(3),Nested::Item(5)])];
/// let result = flatten(&input).unwrap();
/// assert_eq!(result,vec![2,3,5]);
/// ```
///
/// Usage with match statement:
/// ```
/// use algorithmz::array::{flatten, Nested};
/// let input = vec![Nested::Item(2),Nested::List(vec![Nested::Item(3),Nested::Item(5)])];
/// match flatten(&input) {
///     Err(e) => eprintln!("The error was: {}",e),
///     Ok(n) => println!("The flattened list: {:?}",n),
/// }
/// ```
use std::fmt::Debug;

/// The enum representing a multi dimensional array
#[derive(Debug, Clone)]
pub enum Nested {
    /// Single item
    Item(usize),
    /// Nested items
    List(Vec<Nested>),
}

/// The main flatten function which will be exposed
pub fn flatten(input: &[Nested]) -> Result<Vec<usize>, String> {
    if input.is_empty() {
        return Err("Input slice cannot be empty".to_string());
    }

    let mut output = Vec::new();
    flatten_inner(input, &mut output);
    Ok(output)
}
/// The private helper function 
fn flatten_inner(input: &[Nested], output: &mut Vec<usize>) {
    for element in input {
        match element {
            Nested::Item(v) => output.push(*v),
            Nested::List(list) => flatten_inner(list, output),
        }
    }
}
