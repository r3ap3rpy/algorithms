use algorithmz::array::garage;

#[test]
fn test_garage_initial_empty() {
    let result = garage(vec![],vec![1,2,0,3,4]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use garage on an empty layout!"));
}
#[test]
fn test_garage_final_state_empty() {
    let result = garage(vec![0,2,1,3],vec![]);
    assert!(matches!(result, Err(ref e) if e == "Cannot useg garage on an empty final state!"));
}
#[test]
fn test_garage() {
    let result = algorithmz::array::garage(vec![1,2,3,0,4],vec![0,3,2,1,4]).unwrap();
    assert_eq!(result,(4,vec![vec![0, 2, 3, 1, 4], vec![2, 0, 3, 1, 4], vec![2, 3, 0, 1, 4], vec![0, 3, 2, 1, 4]]));
}

