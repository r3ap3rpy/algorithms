use algorithmz::sorting::meeting_rooms_sort;

#[test]
fn test_meeting_rooms_sort_sort_empty() {
    let result = meeting_rooms_sort(&vec![]);
    assert!(matches!(result, Err(ref e) if e == "Cannot use meeting room sort on an empty list!"));
}
#[test]
fn test_meeting_rooms_sort_sort_true() {
    let result = meeting_rooms_sort(&vec![[7,10],[4,2]]).unwrap();
    assert_eq!(result,true);
}
#[test]
fn test_meeting_rooms_sort_sort_false() {
    let result = meeting_rooms_sort(&vec![[0,30],[5,10],[15,20]]).unwrap();
    assert_eq!(result,false);
}
