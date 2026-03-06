use algorithmz::array::Josephus;

#[test]
fn test_josephus_strings() {
    let mut result = Josephus::new(vec!["Rust","Python","Go"],2);
    assert_eq!(result.next().unwrap(),"Python");
    assert_eq!(result.next().unwrap(),"Rust");
    assert_eq!(result.next().unwrap(),"Go");
    assert!(result.next().is_none());
}
#[test]
fn test_josephus_numbers() {    
    let mut game = Josephus::new(vec![10,20,30], 2);
    assert!(matches!(game.next(),Some(20)));
    assert!(matches!(game.next(),Some(10)));
    assert!(matches!(game.next(),Some(30)));
    assert!(game.next().is_none());
}
