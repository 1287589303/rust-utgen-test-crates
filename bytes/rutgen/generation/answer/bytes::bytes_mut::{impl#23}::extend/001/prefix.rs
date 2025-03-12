// Answer 0

#[test]
fn test_extend_with_non_empty_iter() {
    let mut buf = BytesMut::with_capacity(10);
    let input: Vec<u8> = vec![1, 2, 3, 4, 5];
    buf.extend(input);
}

#[test]
fn test_extend_with_empty_iter() {
    let mut buf = BytesMut::with_capacity(10);
    let input: Vec<u8> = vec![];
    buf.extend(input);
}

