/// Word Squares
///
///
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::word_squares(vec!["area".to_string(), "lead".to_string(), "wall".to_string(), "lady".to_string(), "ball".to_string()]);
/// assert_eq!(result, [vec!["wall".to_string(), "area".to_string(), "lead".to_string(), "lady".to_string()], vec!["ball".to_string(), "area".to_string(), "lead".to_string(), "lady".to_string()]])
/// ```
pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
    if words.is_empty() {
        return Vec::new();
    }

    let word_length = words[0].len();

    let mut prefix_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for word in &words {
        for index in 0..word_length {
            let prefix = word[..index].to_string();
            prefix_map
                .entry(prefix)
                .or_default()
                .push(word.clone());
        }
    }

    fn build(
        square: Vec<String>,
        word_length: usize,
        prefix_map: &std::collections::HashMap<String, Vec<String>>,
        squares: &mut Vec<Vec<String>>) {
        if square.len() == word_length {
            squares.push(square);
            return;
        }

        let col = square.len();

        let prefix: String = square
            .iter()
            .map(|word| word.as_bytes()[col] as char)
            .collect();

        if let Some(candidates) = prefix_map.get(&prefix) {
            for word in candidates {
                let mut next_square = square.clone();
                next_square.push(word.clone());
                build(next_square, word_length, prefix_map, squares);
            }
        }
    }

    let mut squares = Vec::new();

    for word in &words {
        build(
            vec![word.clone()],
            word_length,
            &prefix_map,
            &mut squares,
        );
    }

    squares
}
