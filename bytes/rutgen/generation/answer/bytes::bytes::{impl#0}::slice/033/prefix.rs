// Answer 0

#[test]
fn test_slice_empty_range() {
    let bytes = Bytes::from_static(b"");
    let result = bytes.slice(0..0);
}

#[test]
fn test_slice_full_range() {
    let bytes = Bytes::from_static(b"hello");
    let result = bytes.slice(0..5);
}

#[test]
fn test_slice_single_element_range() {
    let bytes = Bytes::from_static(b"hello");
    let result = bytes.slice(2..3);
}

#[test]
fn test_slice_zero_length_range() {
    let bytes = Bytes::from_static(b"hello");
    let result = bytes.slice(3..3);
}

