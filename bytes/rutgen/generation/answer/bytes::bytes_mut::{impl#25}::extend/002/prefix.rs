// Answer 0

#[test]
fn test_extend_with_empty_iterator() {
    let mut bytes_mut = BytesMut::new();
    let empty_iter = vec![].into_iter();
    bytes_mut.extend(empty_iter);
}

#[test]
fn test_extend_with_iterator_of_zero_elements() {
    let mut bytes_mut = BytesMut::new();
    let zero_element_iter = vec![Bytes::from(vec![])].into_iter().filter(|_| false);
    bytes_mut.extend(zero_element_iter);
}

