use std::fmt::Debug;
/// Returns the flattened version of the input.
///
/// Takes a list of nested items as a reference and returns the flattened version of it.
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
pub fn flatten(input: &[Nested]) -> Result<Vec<usize>, String> {
    if input.is_empty() {
        return Err("Input slice cannot be empty".to_string());
    }

    let mut output = Vec::new();
    flatten_inner(input, &mut output);
    Ok(output)
}


/// The enum representing a multi dimensional array
///
/// # Examples
///
/// Simple nested list of values
/// ```
/// use algorithmz::array::Nested;
/// let input = vec![Nested::Item(10),Nested::Item(20)];
/// ```
///
/// Nested lists:
/// ```
/// use algorithmz::array::Nested;
/// let input = Nested::List(vec![Nested::Item(10),Nested::List(vec![Nested::Item(20),Nested::Item(30)])]);
/// ```
#[derive(Debug, Clone)]
pub enum Nested {
    /// Single item
    Item(usize),
    /// Nested items
    List(Vec<Nested>),
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
