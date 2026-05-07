use algorithmz::string::knuth_morris_pratt;

#[test]
fn test_knuth_morris_pratt() {
    let text: Vec<char> = "hello there hero!".chars().collect();
    let pattern: Vec<char> = "he".chars().collect();

    let result = knuth_morris_pratt(&text,&pattern);
    assert_eq!(result,vec![0, 7, 12]);
}
