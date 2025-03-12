// Answer 0

#[test]
fn test_promote_to_shared_ref_cnt_1() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.ptr = NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 10])) as *mut u8);
        bytes_mut.len = 5;
        bytes_mut.cap = 10;
        bytes_mut.data = (KIND_VEC | (4 << VEC_POS_OFFSET)) as *mut Shared;

        bytes_mut.promote_to_shared(1);
    }
}

#[test]
fn test_promote_to_shared_ref_cnt_2() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.ptr = NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 10])) as *mut u8);
        bytes_mut.len = 5;
        bytes_mut.cap = 10;
        bytes_mut.data = (KIND_VEC | (4 << VEC_POS_OFFSET)) as *mut Shared;

        bytes_mut.promote_to_shared(2);
    }
}

#[test]
fn test_promote_to_shared_original_capacity_repr() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.ptr = NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 10])) as *mut u8);
        bytes_mut.len = 5;
        bytes_mut.cap = 10;
        bytes_mut.data = (KIND_VEC | (0 << VEC_POS_OFFSET)) as *mut Shared;

        bytes_mut.promote_to_shared(1);

        bytes_mut.data = (KIND_VEC | (1 << VEC_POS_OFFSET)) as *mut Shared;
        bytes_mut.promote_to_shared(2);
    }
}

#[test]
fn test_promote_to_shared_cap_zero() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        bytes_mut.ptr = NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 0])) as *mut u8);
        bytes_mut.len = 0;
        bytes_mut.cap = 0;
        bytes_mut.data = (KIND_VEC | (0 << VEC_POS_OFFSET)) as *mut Shared;

        bytes_mut.promote_to_shared(1);
    }
}

