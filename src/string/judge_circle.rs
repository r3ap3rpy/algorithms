/// Judge Route Circle
///
/// Given a sequence of robot moves (R, L, U, D), determine whether the robot returns to its starting position after completing all moves.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::string::judge_circle("UD");
/// assert_eq!(result, true);
/// ```
pub fn judge_circle(moves: &str) -> bool {
    if moves.len() == 0 {
        return true;
    }
    let mut move_counts: std::collections::HashMap<char,i32> = std::collections::HashMap::new();
    move_counts.insert('U',0);
    move_counts.insert('D',0);
    move_counts.insert('R',0);
    move_counts.insert('L',0);
    for character in moves.chars() {
        *move_counts.entry(character).or_insert(0) += 1;
    }
    let Some(first) = move_counts.get(&'U') else { todo!() };
    let Some(second) = move_counts.get(&'D') else { todo!() };
    let Some(third) = move_counts.get(&'R') else { todo!() };
    let Some(fourth) = move_counts.get(&'L') else { todo!() };
    return fourth == third && first == second;
}
