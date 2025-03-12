// Answer 0

#[test]
fn test_promote_to_shared_with_ref_count_one() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.ptr = NonNull::new_unchecked(Box::into_raw(Box::new(0u8))); 
        bytes_mut.len = 0;
        bytes_mut.cap = 10;
        bytes_mut.data = (KIND_VEC as usize) as *mut Shared;
        bytes_mut.promote_to_shared(1);
    }
}

#[test]
fn test_promote_to_shared_with_capacity_edge() {
    let mut bytes_mut = BytesMut::with_capacity(17);
    unsafe {
        bytes_mut.ptr = NonNull::new_unchecked(Box::into_raw(Box::new(0u8)));
        bytes_mut.len = 0;
        bytes_mut.cap = 17;
        bytes_mut.data = (KIND_VEC as usize) as *mut Shared;
        bytes_mut.promote_to_shared(1);
    }
}

#[test]
#[should_panic]
fn test_promote_to_shared_with_ref_count_two() {
    let mut bytes_mut = BytesMut::with_capacity(12);
    unsafe {
        bytes_mut.ptr = NonNull::new_unchecked(Box::into_raw(Box::new(0u8)));
        bytes_mut.len = 0;
        bytes_mut.cap = 12;
        bytes_mut.data = (KIND_VEC as usize) as *mut Shared;
        bytes_mut.promote_to_shared(2);
    }
}

