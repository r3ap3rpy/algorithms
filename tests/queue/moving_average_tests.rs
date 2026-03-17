use algorithmz::queue::MovingAverage;

#[test]
fn test_moving_average() {
    let mut ma = MovingAverage::new(3);
    assert_eq!(ma.next(1),1.0);
    assert_eq!(ma.next(10),5.5);
}
