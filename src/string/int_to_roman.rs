/// Int to Roman
///
/// Given an integer, convert it to a Roman numeral string. Input is guaranteed to be within the range from 1 to 3999.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::int_to_roman(10);
/// assert_eq!(result,"X".to_string());
/// ```
pub fn int_to_roman(number: i32) -> String {
    if number < 1 {
        return String::from("Too Low!");
    }
    if number > 3999 {
        return String::from("Too High!");
    }
    let th: Vec<&str> = vec!["","M","MM","MMM"];
    let h: Vec<&str> = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let t: Vec<&str> = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let o: Vec<&str> = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let thousands: usize = (number / 1000) as usize;
    let hundreds: usize = ((number % 1000) / 100) as usize;
    let tens: usize = ((number % 100) / 10) as usize;
    let ones: usize = (number % 10) as usize;
    let result = vec![th[thousands],h[hundreds],t[tens],o[ones]];
    result.join("")
} 

