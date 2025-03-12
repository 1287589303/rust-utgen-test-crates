// Answer 0

#[test]
fn test_slice_unbounded_range() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(..);
}

#[test]
fn test_slice_empty_range() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(5..5);
}

#[test]
#[should_panic]
fn test_slice_out_of_bounds() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(0..12);
}

