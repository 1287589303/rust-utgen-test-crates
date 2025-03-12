// Answer 0

#[test]
fn test_set_vec_pos_boundary_min() {
    let mut bytes_mut = BytesMut::new();
    unsafe { bytes_mut.set_vec_pos(0); }
}

#[test]
fn test_set_vec_pos_boundary_max() {
    let mut bytes_mut = BytesMut::new();
    unsafe { bytes_mut.set_vec_pos(MAX_VEC_POS); }
}

#[test]
fn test_set_vec_pos_out_of_bounds() {
    let mut bytes_mut = BytesMut::new();
    unsafe { bytes_mut.set_vec_pos(MAX_VEC_POS + 1); }
}

