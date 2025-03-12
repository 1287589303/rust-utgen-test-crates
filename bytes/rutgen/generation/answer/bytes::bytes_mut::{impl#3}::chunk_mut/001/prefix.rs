// Answer 0

#[test]
fn test_chunk_mut_empty() {
    let mut bytes_mut = BytesMut::new();
    assert_eq!(bytes_mut.capacity(), bytes_mut.len());
    let _ = bytes_mut.chunk_mut();
}

#[test]
fn test_chunk_mut_with_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(128);
    assert_eq!(bytes_mut.capacity(), bytes_mut.len());
    let _ = bytes_mut.chunk_mut();
}

#[test]
fn test_chunk_mut_after_reserve() {
    let mut bytes_mut = BytesMut::new();
    assert_eq!(bytes_mut.capacity(), bytes_mut.len());
    bytes_mut.reserve(64);
    let _ = bytes_mut.chunk_mut();
}

#[test]
fn test_chunk_mut_exact_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    let _ = bytes_mut.chunk_mut();
}

