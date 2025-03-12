// Answer 0

#[test]
fn test_next_b_at_upper_bound_no_bytes() {
    let mut byte_set = ByteSet::default();
    let mut iter = ByteSetIter { set: &byte_set, b: 255 };
    let result = iter.next();
}

#[test]
fn test_next_b_at_upper_bound_partial_no_bytes() {
    let mut byte_set = ByteSet::default();
    let mut iter = ByteSetIter { set: &byte_set, b: 255 };
    let _ = iter.next(); // Consume the iterator to guarantee state can be reused
    let result = iter.next();
}

