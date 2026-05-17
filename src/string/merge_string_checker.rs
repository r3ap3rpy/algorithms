/// Merge string checker
///
/// Determine if a given string can be formed by interleaving two other strings, preserving the character order from each part.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::merge_string_checker(b"codewars", b"cdw", b"oears");
/// assert_eq!(result, true);
/// ```
pub fn merge_string_checker(text: &[u8], part1: &[u8], part2: &[u8]) -> bool {
    if part1.is_empty() {
        return text == part2;
    }

    if part2.is_empty() {
        return text == part1;
    }

    if text.is_empty() {
        return part1.is_empty() && part2.is_empty();
    }

    if text[0] == part1[0]
        && merge_string_checker(&text[1..], &part1[1..], part2)
    {
        return true;
    }

    text[0] == part2[0]
        && merge_string_checker(&text[1..], part1, &part2[1..])
}

