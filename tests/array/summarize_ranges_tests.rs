use algorithmz::array::summarize_ranges;

#[test]
fn test_summarize_ranges_empty() {
    let result = summarize_ranges(&[]);
    assert!(matches!(result, Err(ref e) if e == "Cannot summarize ranges of an empty list!"));
}

#[test]
fn test_summarize_ranges() {
    let result = summarize_ranges(&[0, 1, 2, 4, 5, 7]).unwrap();
    assert_eq!(result, vec![(0, 2), (4, 5), (7, 7)]);
}
