// Answer 0

#[test]
fn test_drop_with_kind_vec() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(5);
        bytes_mut.ptr = NonNull::new_unchecked(Box::into_raw(Vec::with_capacity(10).into_boxed_slice()) as *mut u8);
        bytes_mut.cap = 10;
        bytes_mut.data = Box::into_raw(Box::new(Shared {
            vec: Vec::with_capacity(10),
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        })) as *mut Shared;
        bytes_mut.set_vec_pos(0);
    }
    drop(bytes_mut);
}

#[test]
fn test_drop_with_kind_vec_boundary() {
    let mut bytes_mut = BytesMut::with_capacity(1);
    unsafe {
        bytes_mut.set_len(1);
        bytes_mut.ptr = NonNull::new_unchecked(Box::into_raw(Vec::with_capacity(1).into_boxed_slice()) as *mut u8);
        bytes_mut.cap = 1;
        bytes_mut.data = Box::into_raw(Box::new(Shared {
            vec: Vec::with_capacity(1),
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        })) as *mut Shared;
        bytes_mut.set_vec_pos(0);
    }
    drop(bytes_mut);
}

#[test]
fn test_drop_with_kind_vec_multiple() {
    let mut bytes_mut = BytesMut::with_capacity(15);
    unsafe {
        bytes_mut.set_len(10);
        bytes_mut.ptr = NonNull::new_unchecked(Box::into_raw(Vec::with_capacity(15).into_boxed_slice()) as *mut u8);
        bytes_mut.cap = 15;
        bytes_mut.data = Box::into_raw(Box::new(Shared {
            vec: Vec::with_capacity(15),
            original_capacity_repr: 0,
            ref_count: AtomicUsize::new(1),
        })) as *mut Shared;
        bytes_mut.set_vec_pos(5);
    }
    drop(bytes_mut);
}

