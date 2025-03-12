// Answer 0

#[test]
fn test_promote_to_shared_with_ref_cnt_zero() {
    let mut bytes_mut = {
        let mut instance = BytesMut::with_capacity(10);
        unsafe { instance.set_len(5) }; // Ensure len is set
        instance
    };
    bytes_mut.data = (KIND_VEC as usize | (0 << VEC_POS_OFFSET)) as *mut _;
    unsafe {
        bytes_mut.promote_to_shared(0);
    }
}

#[test]
fn test_promote_to_shared_with_ref_cnt_three() {
    let mut bytes_mut = {
        let mut instance = BytesMut::with_capacity(10);
        unsafe { instance.set_len(5) }; // Ensure len is set
        instance
    };
    bytes_mut.data = (KIND_VEC as usize | (0 << VEC_POS_OFFSET)) as *mut _;
    unsafe {
        bytes_mut.promote_to_shared(3);
    }
}

#[test]
fn test_promote_to_shared_with_large_original_capacity_repr() {
    let mut bytes_mut = {
        let mut instance = BytesMut::with_capacity(10);
        unsafe { instance.set_len(5) }; // Ensure len is set
        instance
    };
    let original_capacity_repr = (u32::MAX >> ORIGINAL_CAPACITY_OFFSET) & ORIGINAL_CAPACITY_MASK;
    bytes_mut.data = (KIND_VEC as usize | (original_capacity_repr << ORIGINAL_CAPACITY_OFFSET) | (0 << VEC_POS_OFFSET)) as *mut _;
    unsafe {
        bytes_mut.promote_to_shared(1);
    }
}

