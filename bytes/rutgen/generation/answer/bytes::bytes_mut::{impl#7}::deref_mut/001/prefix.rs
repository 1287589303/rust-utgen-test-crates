// Answer 0

#[test]
fn test_deref_mut_non_empty_bytes_mut() {
    let vec = vec![1, 2, 3, 4, 5];
    let shared = Shared {
        buf: vec.as_ptr() as *mut u8,
        cap: vec.capacity(),
        ref_cnt: AtomicUsize::new(1),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.buf).unwrap(),
        len: vec.len(),
        cap: vec.capacity(),
        data: &shared as *const Shared as *mut Shared,
    };
    let _result: &mut [u8] = bytes_mut.deref_mut();
}

#[test]
fn test_deref_mut_max_capacity_bytes_mut() {
    let vec = vec![0; usize::MAX >> 5];
    let shared = Shared {
        buf: vec.as_ptr() as *mut u8,
        cap: vec.capacity(),
        ref_cnt: AtomicUsize::new(1),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.buf).unwrap(),
        len: vec.len(),
        cap: vec.capacity(),
        data: &shared as *const Shared as *mut Shared,
    };
    let _result: &mut [u8] = bytes_mut.deref_mut();
}

#[test]
fn test_deref_mut_single_element_bytes_mut() {
    let vec = vec![42];
    let shared = Shared {
        buf: vec.as_ptr() as *mut u8,
        cap: vec.capacity(),
        ref_cnt: AtomicUsize::new(1),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.buf).unwrap(),
        len: vec.len(),
        cap: vec.capacity(),
        data: &shared as *const Shared as *mut Shared,
    };
    let _result: &mut [u8] = bytes_mut.deref_mut();
}

#[test]
fn test_deref_mut_capacity_one_bytes_mut() {
    let vec = vec![255; 1];
    let shared = Shared {
        buf: vec.as_ptr() as *mut u8,
        cap: vec.capacity(),
        ref_cnt: AtomicUsize::new(1),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.buf).unwrap(),
        len: vec.len(),
        cap: vec.capacity(),
        data: &shared as *const Shared as *mut Shared,
    };
    let _result: &mut [u8] = bytes_mut.deref_mut();
}

