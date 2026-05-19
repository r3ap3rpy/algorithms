/// Multiply strings
///
/// Given two non-negative integers represented as strings, return their product as a string without using built-in BigInteger or direct integer conversion.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::multiply_strings("23","23");
/// assert_eq!(result, String::from("529"));
/// ```
pub fn multiply_strings(num1: &str, num2: &str) -> String {
    let zero = '0' as u32;
    let mut intermediate: Vec<u64> = Vec::new();

    let mut position_i: u64 = 1;

    for digit_i in num1.chars().rev() {
        let mut position_j: u64 = 1;
        let mut accumulator: u64 = 0;

        for digit_j in num2.chars().rev() {
            let product =
                ((digit_i as u32 - zero) as u64)
                * ((digit_j as u32 - zero) as u64)
                * position_j
                * position_i;

            position_j *= 10;
            accumulator += product;
        }

        position_i *= 10;
        intermediate.push(accumulator);
    }

    intermediate.iter().sum::<u64>().to_string()
} 

