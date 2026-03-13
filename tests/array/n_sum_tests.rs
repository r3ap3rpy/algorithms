use algorithmz::array::n_sum;

#[test]
fn test_n_sum() {
    let result = n_sum(3, vec![-1, 0, 1, 2, -1, -4], 0);
    assert_eq!(result,vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

