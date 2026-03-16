use algorithmz::queue::max_sliding_window;

#[test]
fn test_max_sliding_window() {
    let result = max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3).unwrap();
    assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
}
#[test]
fn test_max_sliding_window_empty() {
    let result = max_sliding_window(&[], 3);
    assert!(matches!(result, Err(ref e) if e == "Cannot use max_sliding_window on an empty list!"));
}
