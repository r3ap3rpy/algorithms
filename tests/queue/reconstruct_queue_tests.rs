use algorithmz::queue::reconstruct_queue;

#[test]
fn test_reconstruct_queue_empty() {
    let result = reconstruct_queue(vec![]);
    assert!(matches!(result, Err (ref e) if e == "Cannot reconstruct queue if it is empty!"));
}

#[test]
fn test_reconstruct_queue() {
    let result = reconstruct_queue(vec![vec![7, 0], vec![4, 4], vec![7, 1], vec![5, 0], vec![6, 1],vec![5, 2]]).unwrap();
    assert_eq!(result, vec![vec![5, 0], vec![7, 0], vec![5, 2], vec![6, 1], vec![4, 4], vec![7,1]]);
}
