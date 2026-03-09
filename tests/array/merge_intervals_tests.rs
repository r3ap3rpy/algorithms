use algorithmz::array::{Interval,merge_intervals};

#[test]
fn test_merge_intervals() {
    let intervals = vec![   
        Interval::new(1, 3),
        Interval::new(2, 6),
        Interval::new(8, 10),
    ];
    let merged = Interval::merge(intervals);
    assert_eq!(merged,vec![Interval::new(1,6),Interval::new(8,10)]);
}

#[test]
fn test_merge_intervals_function() {
    let intervals = vec![
        vec![1, 3],
        vec![2, 6],
        vec![8, 10],
    ];
    let merged = merge_intervals(intervals).unwrap();
    assert_eq!(merged, vec![vec![1, 6], vec![8, 10]])
}
