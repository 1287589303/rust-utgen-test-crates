// Answer 0

#[test]
fn test_try_unsplit_with_empty_other_and_same_ptr() {
    struct SharedForTest {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let mut self_bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 5,
        cap: 10,
        data: &SharedForTest {
            vec: vec![1, 2, 3, 4, 5],
            original_capacity_repr: KIND_ARC,
            ref_count: AtomicUsize::new(1),
        } as *const _ as *mut _,
    };

    let other_bytes_mut = BytesMut {
        ptr: self_bytes_mut.ptr,
        len: 0,
        cap: 0,
        data: &SharedForTest {
            vec: vec![],
            original_capacity_repr: KIND_VEC,
            ref_count: AtomicUsize::new(0),
        } as *const _ as *mut _,
    };

    let _result = self_bytes_mut.try_unsplit(other_bytes_mut);
}

#[test]
fn test_try_unsplit_with_non_arc_other_and_same_ptr() {
    struct SharedForTest {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let mut self_bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 5,
        cap: 10,
        data: &SharedForTest {
            vec: vec![1, 2, 3, 4, 5],
            original_capacity_repr: KIND_ARC,
            ref_count: AtomicUsize::new(1),
        } as *const _ as *mut _,
    };

    let other_bytes_mut = BytesMut {
        ptr: self_bytes_mut.ptr,
        len: 0,
        cap: 0,
        data: &SharedForTest {
            vec: vec![],
            original_capacity_repr: KIND_VEC,
            ref_count: AtomicUsize::new(0),
        } as *const _ as *mut _,
    };

    let _result = self_bytes_mut.try_unsplit(other_bytes_mut);
}

