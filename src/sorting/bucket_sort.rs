/// Returns the bucket sorted version of the input
///
/// The function takes a list reference and returns a `Vec<i32>` or an error.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::sorting::bucket_sort(&[2,1,3,5,4,7,6,8]).unwrap();
/// assert_eq!(result,[1,2,3,4,5,6,7,8]); 
/// ```
///
/// Match example:
/// ```
/// use algorithmz::sorting::bucket_sort;
/// let my_list = [1,3,2,5,4,7,6,8];
/// match bucket_sort(&my_list) {
///     Ok(n) => println!("The result was: {:?}",n),
///     Err(e) => eprintln!("The error was: {}",e),
/// }
/// ```
pub fn bucket_sort(list: &[i32]) -> Result<Vec<i32>,String> {
    if list.is_empty() {
        return Err("Cannot bucket sort an empty list!".to_string());
    }
    let num_buckets = list.len();
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); num_buckets];
    let max_value = list.iter().max().unwrap() + 1;
    for &value in list {
        let index = (value * (num_buckets as i32) / max_value)  as usize;
        buckets[index].push(value);
    }
    let mut sorted_list: Vec<i32> = Vec::new();
    for mut bucket in buckets {
        insertion_sort(&mut bucket);
        sorted_list.extend(bucket);
    }
    Ok(sorted_list)
}
fn insertion_sort(list: &mut [i32]){
    for i in 1..list.len() {
        let key = list[i];
        let mut j = i - 1;
        while list[j] > key {
            list[j + 1] = list[j];
            j -= 1;
        }
        list[j + 1] = key;
    }
}

