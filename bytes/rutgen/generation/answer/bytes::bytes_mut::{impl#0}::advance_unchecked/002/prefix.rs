// Answer 0

#[test]
fn test_advance_unchecked_count_zero() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(134217727) };
    unsafe { bytes_mut.advance_unchecked(0) };
}

#[test]
fn test_advance_unchecked_count_cap() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(134217727) };
    unsafe { bytes_mut.advance_unchecked(bytes_mut.capacity()) };
}

#[test]
fn test_advance_unchecked_count_max_vec_pos() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(134217727) };
    unsafe { bytes_mut.set_vec_pos(134217727) };
    let count = 134217727 - unsafe { bytes_mut.get_vec_pos() };
    unsafe { bytes_mut.advance_unchecked(count) };
} 

#[test]
fn test_advance_unchecked_count_half_max_vec_pos() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(134217727) };
    unsafe { bytes_mut.set_vec_pos(67108863) }; // Half of MAX_VEC_POS
    let count = 134217727 - unsafe { bytes_mut.get_vec_pos() };
    unsafe { bytes_mut.advance_unchecked(count) };
} 

#[test]
fn test_advance_unchecked_count_near_max_vec_pos() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(134217727) };
    unsafe { bytes_mut.set_vec_pos(134217726) }; // One less than MAX_VEC_POS
    let count = 1;
    unsafe { bytes_mut.advance_unchecked(count) };
}

