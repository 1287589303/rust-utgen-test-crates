// Answer 0

#[test]
fn test_from_static_empty() {
    let b = Bytes::from_static(b"");
}

#[test]
fn test_from_static_single_byte() {
    let b = Bytes::from_static(b"a");
}

#[test]
fn test_from_static_multiple_bytes() {
    let b = Bytes::from_static(b"hello");
}

#[test]
fn test_from_static_large_slice() {
    let large_slice: &[u8] = &[0; usize::MAX]; // Maximum length in practice can't exceed usize limit.
    let b = Bytes::from_static(large_slice);
}

#[test]
fn test_from_static_non_empty_slice() {
    let non_empty_slice: &[u8] = b"rust is fun";
    let b = Bytes::from_static(non_empty_slice);
}

