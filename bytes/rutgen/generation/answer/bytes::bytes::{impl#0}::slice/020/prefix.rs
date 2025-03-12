// Answer 0

#[test]
fn test_slice_range_excluded_start_greater() {
    let bytes = Bytes::from_static(b"hello");
    let result = bytes.slice(2..2);
}

#[test]
fn test_slice_range_excluded_start_equal() {
    let bytes = Bytes::from_static(b"hello");
    let result = bytes.slice(2..3);
}

#[test]
fn test_slice_range_excluded_end_less() {
    let bytes = Bytes::from_static(b"hello");
    let result = bytes.slice(3..1);
}

#[test]
fn test_slice_range_excluded_start_end_equal() {
    let bytes = Bytes::from_static(b"hello");
    let result = bytes.slice(1..1);
}

#[test]
fn test_slice_range_excluded_end_less_start_greater() {
    let bytes = Bytes::from_static(b"hello");
    let result = bytes.slice(5..2);
}

