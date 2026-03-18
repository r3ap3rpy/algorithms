use algorithmz::queue::ZigZagIterator;

#[test]
fn test_zigzagiterator() {
    let mut it = ZigZagIterator::new(vec![1,2],vec![3,4,5]);
    let mut result = Vec::new();    
    while it.has_next() {
        result.push(it.next().unwrap());
    }
    assert_eq!(result,vec![1,3,2,4,5]);
}

