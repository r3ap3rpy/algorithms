use algorithmz::array::{delete_nth, delete_nth_naive};

#[test]
fn delete_nth_empty() {
    let result = delete_nth(&[],2);
    assert!(matches!(result, Err(ref e) if e == "Cannot delete nth element of an empty list!"));
}

#[test]
fn delete_nth_naive_empty() {
    let result = delete_nth_naive(&[],2);
    assert!(matches!(result, Err(ref e) if e == "Cannot delete nth element of an empty list!"));
}

#[test]
fn delete_nth_no_delete() {
    let result = delete_nth(&[3, 5, 2, 3, 1, 4, 3, 6, 4, 4],4).unwrap();
    assert_eq!(result, [3, 5, 2, 3, 1, 4, 3, 6, 4, 4]);
}

#[test]
fn delete_nth_naive_no_delete() {
    let result = delete_nth_naive(&[3, 5, 2, 3, 1, 4, 3, 6, 4, 4],4).unwrap();
    assert_eq!(result, [3, 5, 2, 3, 1, 4, 3, 6, 4, 4]);
}

#[test]
fn delete_nth_naive_result() {
    let result = delete_nth_naive(&[3, 5, 2, 3, 1, 4, 3, 6, 4, 4],2).unwrap();
    assert_eq!(result, [3, 5, 2, 3, 1, 4, 6, 4]);
}

#[test]
fn delete_nth_result() {
    let result = delete_nth(&[3, 5, 2, 3, 1, 4, 3, 6, 4, 4],2).unwrap();
    assert_eq!(result, [3, 5, 2, 3, 1, 4, 6, 4]);
}

