// Answer 0

#[test]
fn test_kind_arc() {
    let shared = crate::bytes::Shared {
        buf: ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(0),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.buf).unwrap(),
        len: 0,
        cap: 0,
        data: &shared as *const _ as *mut _,
    };
    let result = bytes_mut.kind();
}

#[test]
fn test_kind_vec() {
    let vec = Vec::new();
    let shared = crate::bytes::Shared {
        buf: vec.as_ptr(),
        cap: vec.capacity(),
        ref_cnt: AtomicUsize::new(0),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.buf).unwrap(),
        len: 0,
        cap: 0,
        data: &shared as *const _ as *mut _,
    };
    let result = bytes_mut.kind();
}

