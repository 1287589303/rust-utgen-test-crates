// Answer 0

#[test]
#[should_panic]
fn test_split_to_out_of_bounds() {
    let mut byte_vec = BytesMut::with_capacity(5);
    byte_vec.resize(5, 0u8);
    let at = byte_vec.len() + 1;
    let _ = byte_vec.split_to(at);
}

