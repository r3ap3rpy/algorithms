/// Alphabet board path — find moves on a 5x5+1 letter board.
///
/// Given a board where 'a'-'z' are laid out in rows of 5:
///     a b c d e
///     f g h i j
///     k l m n o
///     p q r s t
///     u v w x y
///     z
///
/// Return the sequence of moves (U/D/L/R/!) to spell a target word starting from 'a'.
///
/// # Examples:
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::alphabet_board_path("dani");
/// assert_eq!(result,"RRR!!DDRRR!DRRR!".to_string());
/// ```
pub fn alphabet_board_path(target: &str) -> String {
    let mut moves: Vec<String> = Vec::new();
    let row: usize = 0;
    let column: usize = 0;
    for ch in target.chars() {
        let idx = ch as u8 - b'a';
        let target_row = idx / 5;
        let target_column = idx % 5;
        if usize::from(target_row) < row {
            moves.push("U".repeat(row - target_row as usize));
        }
        if usize::from(target_column) < column {
            moves.push("L".repeat(column - target_column as usize));
        }
        if usize::from(target_row) > row {
            moves.push("D".repeat(target_row as usize - row));
        }
        if usize::from(target_column) > column {
            moves.push("R".repeat(target_column as usize - column));
        }
        moves.push("!".to_string());

    }
    return moves.join("");
}
