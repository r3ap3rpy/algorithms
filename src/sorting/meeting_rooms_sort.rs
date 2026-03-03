/// Returns whether someone can attend a meeting based on a list of lists.
///
/// Takes a list of list reference and returns a `bool` or an error string.
///
/// # Examples
/// 
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::meeting_rooms_sort(&vec![[7,10],[4,2]]).unwrap();
/// assert_eq!(result,true);
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::meeting_rooms_sort;
/// let my_list = vec![[7,10],[4,2]];
/// match meeting_rooms_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn meeting_rooms_sort(list: &Vec<[i32; 2]>) -> Result<bool,String> {
    if list.is_empty() {
        return Err("Cannot use meeting room sort on an empty list!".to_string());
    }
    let mut can_attend = true;

    let mut my_list = list.to_vec();
    my_list.sort_by_key(|k| k[0]);
    println!("{:?}",my_list);
    for i in 1..my_list.len(){
        if my_list[i][0] < my_list[i-1][1] {
            can_attend = false;
            return Ok(can_attend);
        }
    }
    Ok(can_attend)
}
