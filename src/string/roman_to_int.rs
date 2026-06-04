/// Roman to Int 
///
/// Given a Roman numeral string, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.
///
/// # Examples
///
/// Basic usage:
/// ``` 
/// let result = algorithmz::string::roman_to_int("DCXXI");
/// assert_eq!(result, Ok(621));
/// ```
pub fn roman_to_int(letters: &str) -> Result<i32,String> {
    if letters.is_empty() {
        return Ok(0);
    }
    let roman_values: std::collections::HashMap<char, i32> = std::collections::HashMap::from([
        ('M', 1000),
        ('D', 500),
        ('C', 100),
        ('L', 50),
        ('X', 10),
        ('V', 5),
        ('I', 1),
    ]);
    let chars: Vec<char> = letters.chars().collect();
    let mut result: i32 = 0;
    for pair in chars.windows(2) {
        let current = roman_values
            .get(&pair[0])
            .ok_or_else(|| format!("The specified character is invalid: {}", pair[0]))?;

        let next = roman_values
            .get(&pair[1])
            .ok_or_else(|| format!("The specified character is invalid: {}", pair[1]))?;

        if current < next {
            result -= current;
        } else {
            result += current;
        }
    }
    return Ok(result + roman_values.get(&chars.last().unwrap()).unwrap());
}
