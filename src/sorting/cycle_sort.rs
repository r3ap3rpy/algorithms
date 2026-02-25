/// Returns the cycle sorted version of the input
///
/// Takes a list as reference, and resturns to cycle sorted version as a `Vec<i32>` or error
/// string.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let result = algorithmz::sorting::cycle_sort(&[1,3,2,5,4]).unwrap();
/// assert_eq!(result, [1,2,3,4,5]);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::cycle_sort;
/// let my_list = [1,3,2,5,4];
/// match cycle_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn cycle_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot sort an empty array!".to_string());
    }
    let length = list.len();
    let mut my_list = list.to_vec();

    for start in 0..length-1 {
        let mut item = my_list[start];
        let mut position = start;
        for i in start+1..length {
            if my_list[i] < item {
                position += 1;
            }
        }
        if position == start {
            continue;
        }
        while item == my_list[position] {
            position += 1;
        }
        let old = my_list[position];
        my_list[position] = item;
        item = old;
        while position != start {
            position = start;
            for i in start+1..length {
                if my_list[i] < item {
                    position+=1;
                }
            }
            while item == my_list[position] {
                position+=1;
            }
            let old = my_list[position];
            my_list[position] = item;
            item = old;
        }
    }
    Ok(my_list)
}

