// Answer 0

#[test]
fn test_drop_with_non_vec_non_arc() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    let bytes_mut = TestBytesMut {
        ptr: NonNull::new_unchecked(core::ptr::null_mut()),
        len: 0,
        cap: 0,
        data: core::ptr::null_mut(),
    };

    unsafe {
        // This will call the drop function, as it's designed to do so
        let _ = bytes_mut;
    }
}

#[test]
fn test_drop_with_kind_other_than_vec_and_arc() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    let bytes_mut = TestBytesMut {
        ptr: NonNull::new_unchecked(core::ptr::null_mut()),
        len: 0,
        cap: 0,
        data: core::ptr::null_mut(),
    };

    unsafe {
        // This will call the drop function, as it's designed to do so
        let _ = bytes_mut;
    }
}

