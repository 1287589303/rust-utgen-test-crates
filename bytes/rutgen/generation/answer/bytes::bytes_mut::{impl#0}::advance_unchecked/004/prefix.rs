// Answer 0

#[test]
fn test_advance_unchecked_zero_count() {
    let mut buffer = BytesMut::new();
    unsafe {
        buffer.advance_unchecked(0);
    }
}

#[test]
fn test_advance_unchecked_max_capacity() {
    let cap = 10;
    let mut buffer = BytesMut::with_capacity(cap);
    unsafe {
        buffer.advance_unchecked(cap);
    }
} 

#[test]
fn test_advance_unchecked_non_vec_kind() {
    let cap = 5;
    let mut buffer = BytesMut::with_capacity(cap);
    unsafe {
        buffer.advance_unchecked(3);
    }
} 

#[test]
fn test_advance_unchecked_non_vec_kind_with_zero() {
    let mut buffer = BytesMut::new();
    unsafe {
        buffer.advance_unchecked(0);
    }
}

