// Answer 0

#[test]
fn test_eq_identical_empty() {
    let a = BytesMut::new();
    let b = BytesMut::new();
    let _ = a.eq(&b);
}

#[test]
fn test_eq_identical_single_byte() {
    let mut a = BytesMut::with_capacity(1);
    let mut b = BytesMut::with_capacity(1);
    a.extend_from_slice(&[1]);
    b.extend_from_slice(&[1]);
    let _ = a.eq(&b);
}

#[test]
fn test_eq_different_single_byte() {
    let mut a = BytesMut::with_capacity(1);
    let mut b = BytesMut::with_capacity(1);
    a.extend_from_slice(&[1]);
    b.extend_from_slice(&[2]);
    let _ = a.eq(&b);
}

#[test]
fn test_eq_identical_multiple_bytes() {
    let mut a = BytesMut::with_capacity(3);
    let mut b = BytesMut::with_capacity(3);
    a.extend_from_slice(&[1, 2, 3]);
    b.extend_from_slice(&[1, 2, 3]);
    let _ = a.eq(&b);
}

#[test]
fn test_eq_different_multiple_bytes() {
    let mut a = BytesMut::with_capacity(3);
    let mut b = BytesMut::with_capacity(3);
    a.extend_from_slice(&[1, 2, 3]);
    b.extend_from_slice(&[4, 5, 6]);
    let _ = a.eq(&b);
}

#[test]
fn test_eq_identical_with_different_length() {
    let mut a = BytesMut::with_capacity(3);
    let mut b = BytesMut::with_capacity(2);
    a.extend_from_slice(&[1, 2, 3]);
    b.extend_from_slice(&[1, 2]);
    let _ = a.eq(&b);
}

