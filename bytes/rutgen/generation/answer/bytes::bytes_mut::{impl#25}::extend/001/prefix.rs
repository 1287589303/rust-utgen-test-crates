// Answer 0

#[test]
fn test_extend_with_non_empty_iter() {
    let mut bytes_mut = BytesMut::new();
    let bytes_vec = vec![Bytes::from(vec![1, 2, 3])];
    bytes_mut.extend(bytes_vec);
}

#[test]
fn test_extend_with_empty_iter() {
    let mut bytes_mut = BytesMut::new();
    let empty_bytes_vec: Vec<Bytes> = Vec::new();
    bytes_mut.extend(empty_bytes_vec);
}

