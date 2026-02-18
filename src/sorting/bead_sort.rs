/// Returns the bead sorted version of the input
///
/// This function takes a list as reference and returns a `Vec<usize>` with the results!
///
/// # Examples
///
/// ```
/// let result = algorithmz::sorting::bead_sort(&[1,3,2,5,4]).unwrap();
/// assert_eq!(result, [1,2,3,4,5]);
/// ```
pub fn bead_sort(list: &[i32]) -> Result<Vec<usize>, String> {
    if list.is_empty() {
        return Err("The list cannot be empty!".to_string());
    }
    for item in list {
        if *item < 0 {
            return Err("The items cannot be smaller than ZERO!".to_string());
        }
    }
    let mut sorted:Vec<usize>;
    let max = *list.iter().max().unwrap();
    println!("The biggest number: {}",max);
    let mut grid = vec![vec![0;list.len()];max.try_into().unwrap()];
    for (col, &num) in list.iter().enumerate() {
        let num_rows = num as usize;
        for row in 0..num_rows {
            grid[row][col] = 1;
        }
    }
    for row in &mut grid {
        let bead_count: usize = row.iter().sum();
        for col in 0..list.len() {
            row[col] = if col < bead_count { 1 } else { 0 }; 
        }
    }
    let n = list.len();
    sorted = vec![0;n];
    for col in 0..n {
        let mut col_sum = 0;
        for row in 0..max.try_into().unwrap() {
            col_sum += grid[row][n - 1 - col];
        }
        sorted[col] = col_sum;
    }
    Ok(sorted)
}

