use algorithmz::map::is_valid_sudoku;

#[test]
fn test_is_valid_sudoku_empty() {
    let result = is_valid_sudoku(vec![]);
    assert!(matches!(result, Err (ref e) if e == "Cannot be an empty board!"));
}

#[test]
fn test_is_valid_sudoku() {
    let board = vec![vec!['.';9];9];
    let result = is_valid_sudoku(board).unwrap();
    assert_eq!(result,true)
}
