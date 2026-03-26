/// Check whether a Sudoku board configuration is valid.
///
/// Input is a board: 9x9 grid where empty cells are represented by '.', returns true or false.
///
/// # Examples
///
/// Basic example:
/// ```
/// let board = vec![vec!['.'; 9]; 9];
/// let result = algorithmz::map::is_valid_sudoku(board).unwrap();
/// assert_eq!(result,true);
/// ```
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> Result<bool,String> {
    if board.is_empty() {
        return Err("Cannot be an empty board!".to_string());
    }
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    for (i, row) in board.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != '.' {
                let row_key = format!("row{}{}", i, cell);
                let col_key = format!("col{}{}", j, cell);
                let box_key = format!("box{}{}{}", i / 3, j / 3, cell);

                if !seen.insert(row_key)
                    || !seen.insert(col_key)
                    || !seen.insert(box_key)
                {
                    return Ok(false);
                }
            } 
        }
    }
    Ok(true)
}
