// Answer 0

#[test]
fn test_promote_to_shared_with_ref_count_2_and_capacity_10() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(10);
        bytes_mut.promote_to_shared(2);
    }
}

#[test]
fn test_promote_to_shared_with_ref_count_2_and_capacity_15() {
    let mut bytes_mut = BytesMut::with_capacity(15);
    unsafe {
        bytes_mut.set_len(15);
        bytes_mut.promote_to_shared(2);
    }
}

#[test]
fn test_promote_to_shared_with_ref_count_2_and_capacity_20() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    unsafe {
        bytes_mut.set_len(20);
        bytes_mut.promote_to_shared(2);
    }
}

#[test]
fn test_promote_to_shared_with_max_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(MAX_VEC_POS);
    unsafe {
        bytes_mut.set_len(MAX_VEC_POS);
        bytes_mut.promote_to_shared(2);
    }
}

