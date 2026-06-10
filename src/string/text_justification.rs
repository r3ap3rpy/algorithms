/// Text Justification
///
/// Given an array of words and a max width, format the text such that each line has exactly max_width characters and is fully justified. Extra spaces are distributed as evenly as possible with left slots getting more.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::text_justification(&vec!["What".to_string(), "must".to_string(), "be".to_string()],16);
/// assert_eq!(result, Ok(vec!["What must be    ".to_string()]));
/// ```
pub fn text_justification(words: &[String], max_width: usize) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    let mut row_length = 0usize;
    let mut row_words: Vec<&String> = Vec::new();
    let mut index = 0usize;
    let mut is_first_word = true;

    while index < words.len() {
        while row_length <= max_width && index < words.len() {
            if words[index].len() > max_width {
                return Err(
                    "there exists word whose length is larger than max_width".to_string(),
                );
            }

            let mut tentative_length = row_length;

            row_words.push(&words[index]);
            tentative_length += words[index].len();

            if !is_first_word {
                tentative_length += 1;
            }

            if tentative_length > max_width {
                row_words.pop();
                break;
            }

            row_length = tentative_length;
            index += 1;
            is_first_word = false;
        }

        let mut row = String::new();

        if index == words.len() {
            row = row_words
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(" ");

            row.push_str(&" ".repeat(max_width - row.len()));
        } else if row_words.len() != 1 {
            let extra_spaces = max_width - row_length;
            let gaps = row_words.len() - 1;

            let spaces_per_gap = extra_spaces / gaps;
            let mut remaining_spaces = extra_spaces % gaps;

            for (i, word) in row_words.iter().enumerate() {
                row.push_str(word);

                if i != row_words.len() - 1 {
                    row.push_str(&" ".repeat(1 + spaces_per_gap));

                    if remaining_spaces > 0 {
                        row.push(' ');
                        remaining_spaces -= 1;
                    }
                }
            }
        } else {
            row.push_str(row_words[0]);
            row.push_str(&" ".repeat(max_width - row.len()));
        }

        result.push(row);

        row_length = 0;
        row_words.clear();
        is_first_word = true;
    }

    Ok(result)
}
