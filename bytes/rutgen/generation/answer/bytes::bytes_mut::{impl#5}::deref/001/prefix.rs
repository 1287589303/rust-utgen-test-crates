// Answer 0

#[test]
fn test_deref_empty_bytesmut() {
    let vec: Vec<u8> = Vec::new();
    let shared = Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let ptr = NonNull::from(&shared.buf as *const _ as *mut u8);
    let bytes_mut = BytesMut {
        ptr,
        len: 0,
        cap: 0,
        data: &shared as *const _ as *mut Shared,
    };
    let _ref = &*bytes_mut;
}

#[test]
fn test_deref_non_empty_bytesmut() {
    let vec = vec![1, 2, 3];
    let shared = Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let ptr = NonNull::from(&shared.buf as *const _ as *mut u8);
    let bytes_mut = BytesMut {
        ptr,
        len: 3,
        cap: 3,
        data: &shared as *const _ as *mut Shared,
    };
    let _ref = &*bytes_mut;
}

#[test]
fn test_deref_boundary_case() {
    let vec = vec![9; 10];
    let shared = Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let ptr = NonNull::from(&shared.buf as *const _ as *mut u8);
    let bytes_mut = BytesMut {
        ptr,
        len: 10,
        cap: 10,
        data: &shared as *const _ as *mut Shared,
    };
    let _ref = &*bytes_mut;
}

