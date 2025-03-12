// Answer 0

#[test]
fn test_len_empty() {
    let bytes_mut = BytesMut::new();
    let _ = bytes_mut.len();
}

#[test]
fn test_len_non_empty() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3]);
    let _ = bytes_mut.len();
}

#[test]
fn test_len_large_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(usize::MAX);
    bytes_mut.resize(usize::MAX, 0);
    let _ = bytes_mut.len();
}

#[test]
fn test_len_after_truncate() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    bytes_mut.truncate(3);
    let _ = bytes_mut.len();
}

#[test]
fn test_len_after_clear() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3]);
    bytes_mut.clear();
    let _ = bytes_mut.len();
}

#[test]
fn test_len_zeroed() {
    let bytes_mut = BytesMut::zeroed(0);
    let _ = bytes_mut.len();
}

