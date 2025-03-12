// Answer 0

#[test]
fn test_slice_inclusive_bounds() {
    let bytes = Bytes::from_static(b"example");
    let result = bytes.slice(0..7);
}

#[test]
fn test_slice_exclusive_end() {
    let bytes = Bytes::from_static(b"example");
    let result = bytes.slice(0..6);
}

#[test]
fn test_slice_exclusive_start() {
    let bytes = Bytes::from_static(b"example");
    let result = bytes.slice(1..7);
}

#[test]
fn test_slice_successive_inclusive() {
    let bytes = Bytes::from_static(b"example");
    let result = bytes.slice(2..5);
}

#[test]
fn test_slice_single_element() {
    let bytes = Bytes::from_static(b"example");
    let result = bytes.slice(6..7);
}

