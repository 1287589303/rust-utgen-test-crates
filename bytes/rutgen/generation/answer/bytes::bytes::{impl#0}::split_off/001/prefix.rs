// Answer 0

#[test]
fn test_split_off_at_zero() {
    let mut a = Bytes::copy_from_slice(b"hello");
    let b = a.split_off(0);
    // Note: Not asserting results, just calling the function.
}

#[test]
fn test_split_off_at_length() {
    let mut a = Bytes::copy_from_slice(b"hello");
    let b = a.split_off(a.len());
    // Note: Not asserting results, just calling the function.
}

#[test]
#[should_panic]
fn test_split_off_at_length_plus_one() {
    let mut a = Bytes::copy_from_slice(b"hello");
    let _b = a.split_off(a.len() + 1);
    // Expected to panic as at > self.len().
}

#[test]
fn test_split_off_mid_point() {
    let mut a = Bytes::copy_from_slice(b"hello world");
    let b = a.split_off(5);
    // Note: Not asserting results, just calling the function.
}

