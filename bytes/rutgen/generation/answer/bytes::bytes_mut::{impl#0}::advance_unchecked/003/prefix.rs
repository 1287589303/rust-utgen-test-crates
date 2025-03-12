// Answer 0

#[test]
fn test_advance_unchecked_count_zero() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    unsafe { bytes_mut.advance_unchecked(0) };
}

#[test]
fn test_advance_unchecked_count_equal_cap() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    unsafe { bytes_mut.resize(10, 0) };
    let cap = bytes_mut.capacity();
    unsafe { bytes_mut.advance_unchecked(cap) };
}

#[test]
fn test_advance_unchecked_pos_greater_than_max_vec_pos() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    let cap = bytes_mut.capacity();
    unsafe { bytes_mut.resize(cap, 0) };

    let kind = bytes_mut.kind(); // ensure it's KIND_VEC
    assert_eq!(kind, KIND_VEC);

    let max_vec_pos = MAX_VEC_POS + 1;
    unsafe {
        bytes_mut.set_vec_pos(max_vec_pos);
        bytes_mut.advance_unchecked(1);
    }
}

