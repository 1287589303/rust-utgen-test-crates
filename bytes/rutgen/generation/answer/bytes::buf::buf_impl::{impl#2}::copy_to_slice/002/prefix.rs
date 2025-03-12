// Answer 0

#[test]
fn test_copy_to_slice_equal_length_non_empty() {
    let mut buf: &[u8] = &[1, 2, 3, 4, 5];
    let mut dst = vec![0; 5];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_equal_length_empty() {
    let mut buf: &[u8] = &[];
    let mut dst: Vec<u8> = vec![];
    buf.copy_to_slice(&mut dst);
}

