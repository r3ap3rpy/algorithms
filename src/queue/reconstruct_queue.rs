/// Reconstruct Queue by Height
///
/// Given a list of people described by (height, k) pairs where k is the number of taller-or-equal people in front, reconstruct the queue by sorting and inserting.
///
/// # Examples
///
/// Basic example:
/// ```
/// let result = algorithmz::queue::reconstruct_queue(vec![vec![7, 0], vec![4, 4], vec![7, 1], vec![5, 0], vec![6, 1],vec![5, 2]]).unwrap();
/// assert_eq!(result, vec![vec![5, 0], vec![7, 0], vec![5, 2], vec![6, 1], vec![4, 4], vec![7, 1]])
/// ```
pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>,String> {
    if people.is_empty(){
        return Err("Cannot reconstruct queue if it is empty!".to_string());
    }
    let mut queue: Vec<Vec<i32>> = Vec::new();
    people.sort_by(|a,b| {b[0].cmp(&a[0]).then_with(|| a[1].cmp(&b[1]))});
    for person in people {
        let height = person[0];
        let count = person[1] as usize;
        queue.insert(count,vec![height,person[1]]);
    }
    Ok(queue)
}
