const KEYBOARD_ROWS: [&str; 3] = [
    "qwertyuiop",
    "asdfghjkl",
    "zxcvbnm",
];
/// Given a list of words, return the words that can be typed using letters from only one row of an American QWERTY keyboard.
///
/// Takes a list of words and checks if those can be typed on a single row of a keyboard.
///
/// # Examples
///
/// Basic usage:
/// ``` 
/// let result =
/// algorithmz::set::find_keyboard_row(vec!["were".to_string(),"quit".to_string(),"wtf".to_string()]).unwrap();
/// assert_eq!(result,["were","quit"]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::set::find_keyboard_row;
/// match find_keyboard_row(vec!["were".to_string(),"quit".to_string(),"dota".to_string()]) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn find_keyboard_row(list: Vec<String>) -> Result<Vec<String>, String> {
    if list.is_empty() {
        return Err("Cannot verify if an empty list can be typed!".to_string());
    }
    let mut results: Vec<String> = vec![];
    for word in list {
        if KEYBOARD_ROWS.iter().any(|row| {word.chars().all(|c| row.contains(c))}) {
            results.push(word);
        }
    }
    if results.is_empty() {
        return Err("None of the words specified can be typed!".to_string());
    }
    Ok(results)
}
