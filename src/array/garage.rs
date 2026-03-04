/// Find the minimum swaps to rearrange a parking lot from initial to final state.
///
/// A tuple of (number_of_steps, sequence_of_states) showing each intermediate arrangement.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::garage(vec![1,2,3,0,4],vec![0,3,2,1,4]).unwrap();
/// assert_eq!(result,(4,vec![vec![0, 2, 3, 1, 4], vec![2, 0, 3, 1, 4], vec![2, 3, 0, 1, 4], vec![0, 3, 2, 1, 4]]));
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::garage;
/// let start = vec![1, 2, 3, 0, 4];
/// let end = vec![0, 3, 2, 1, 4];
/// match garage(start,end) {
///     Ok((moves,steps)) => println!("The moves: {} and steps: {:?}",moves, steps),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn garage(initial: Vec<i32>, final_state: Vec<i32>) -> Result<(i32,Vec<Vec<i32>>),String> {
    if initial.is_empty() {
        return Err("Cannot use garage on an empty layout!".to_string());
    }
    if final_state.is_empty() {
        return Err("Cannot useg garage on an empty final state!".to_string());
    }
    let mut steps = Vec::new();
    let mut current = initial.clone();
    let mut moves = 0;
    while current != final_state {
        let zero_pos = current.iter().position(|&x| x == 0).unwrap();
        if zero_pos == final_state.iter().position(|&x| x == 0).unwrap() {
            for i in 0..current.len() {
                if current[i] != final_state[i] {
                    current.swap(zero_pos, i);
                    steps.push(current.clone());
                    moves += 1;
                    break;
                }
            }
        } else {
            let target_val = final_state[zero_pos];
            let target_val_pos = current.iter().position(|&x| x == target_val).unwrap();
            current.swap(zero_pos, target_val_pos);
            steps.push(current.clone());
            moves += 1;
        }
    }
    Ok((moves,steps))
}

