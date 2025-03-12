// Answer 0

#[test]
fn test_eq_bytes_mut_equals_non_empty_vec() {
    let shared = Shared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.vec.as_ptr() as *mut u8).unwrap(),
        len: shared.vec.len(),
        cap: shared.vec.capacity(),
        data: &shared as *const Shared as *mut Shared,
    };
    let other = vec![1, 2, 3];
    let _ = bytes_mut.eq(&other);
}

#[test]
fn test_eq_bytes_mut_not_equals_non_empty_vec() {
    let shared = Shared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.vec.as_ptr() as *mut u8).unwrap(),
        len: shared.vec.len(),
        cap: shared.vec.capacity(),
        data: &shared as *const Shared as *mut Shared,
    };
    let other = vec![4, 5, 6];
    let _ = bytes_mut.eq(&other);
}

#[test]
fn test_eq_bytes_mut_empty_self() {
    let shared = Shared {
        vec: vec![],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.vec.as_ptr() as *mut u8).unwrap(),
        len: shared.vec.len(),
        cap: shared.vec.capacity(),
        data: &shared as *const Shared as *mut Shared,
    };
    let other = vec![1, 2, 3];
    let _ = bytes_mut.eq(&other);
}

#[test]
fn test_eq_bytes_mut_non_empty_self_empty_other() {
    let shared = Shared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.vec.as_ptr() as *mut u8).unwrap(),
        len: shared.vec.len(),
        cap: shared.vec.capacity(),
        data: &shared as *const Shared as *mut Shared,
    };
    let other: Vec<u8> = vec![];
    let _ = bytes_mut.eq(&other);
}

#[test]
fn test_eq_bytes_mut_edge_case_max_length() {
    let shared = Shared {
        vec: (0..MAX_VEC_POS).map(|i| i as u8).collect(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(shared.vec.as_ptr() as *mut u8).unwrap(),
        len: shared.vec.len(),
        cap: shared.vec.capacity(),
        data: &shared as *const Shared as *mut Shared,
    };
    let other = (0..MAX_VEC_POS).map(|i| i as u8).collect::<Vec<u8>>();
    let _ = bytes_mut.eq(&other);
}

