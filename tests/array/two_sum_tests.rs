use algorithmz::array::{two_sum, MyResult};

#[test]
fn test_two_sum_empty() {
    let result = two_sum(&[],2);
    assert_eq!(result, MyResult::Message("Cannot use two_sum on an empty list!".to_string()));
}

#[test]
fn test_two_sum_tuple() {
    let result = two_sum(&[2,7,11,15],9);
    assert_eq!(result, MyResult::Tuple(0,1)); 
}

#[test]
fn test_two_sum_none() {
   let result = two_sum(&[2,7,11,15],100);
   assert_eq!(result, MyResult::None);  
}

