// Answer 0

#[test]
fn test_advance_unchecked_count_zero() {
    unsafe {
        let mut buffer = BytesMut::new();
        buffer.reserve(10); // Ensure we have some capacity
        buffer.advance_unchecked(0);
    }
}

#[test]
#[should_panic]
fn test_advance_unchecked_count_greater_than_cap() {
    unsafe {
        let mut buffer = BytesMut::with_capacity(10);
        buffer.advance_unchecked(11); // count is greater than capacity
    }
}

#[test]
#[should_panic]
fn test_advance_unchecked_count_equal_to_cap_plus_one() {
    unsafe {
        let mut buffer = BytesMut::with_capacity(10);
        buffer.advance_unchecked(11); // count is cap + 1
    }
}

#[test]
#[should_panic]
fn test_advance_unchecked_count_max_usize() {
    unsafe {
        let mut buffer = BytesMut::with_capacity(10);
        buffer.advance_unchecked(usize::MAX); // count is maximum representable usize
    }
}

