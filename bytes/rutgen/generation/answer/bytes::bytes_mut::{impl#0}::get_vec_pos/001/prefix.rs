// Answer 0

#[test]
fn test_get_vec_pos_valid_pointer() {
    let data: *mut Shared = &mut Shared {
        vec: vec![1u8, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let data_ptr = data as usize | KIND_VEC; // Set KIND_VEC
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(data as *mut u8).unwrap(),
        len: 3,
        cap: 10,
        data: data_ptr as *mut Shared,
    };
    let _ = unsafe { bytes_mut.get_vec_pos() };
}

#[test]
fn test_get_vec_pos_boundary_case_min_offset() {
    let data: *mut Shared = &mut Shared {
        vec: vec![1u8, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let data_ptr = data as usize | KIND_VEC; // Set KIND_VEC
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(data as *mut u8).unwrap(),
        len: 3,
        cap: 10,
        data: data_ptr as *mut Shared,
    };
    unsafe {
        bytes_mut.data = (data_ptr >> 1) as *mut Shared; // Simulate valid vec position
        let _ = bytes_mut.get_vec_pos();
    }
}

#[test]
fn test_get_vec_pos_boundary_case_max_offset() {
    let data: *mut Shared = &mut Shared {
        vec: vec![1u8, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    };
    let data_ptr = data as usize | KIND_VEC; // Set KIND_VEC
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(data as *mut u8).unwrap(),
        len: 3,
        cap: 10,
        data: data_ptr as *mut Shared,
    };
    unsafe {
        bytes_mut.data = (data_ptr >> 31) as *mut Shared; // Simulate valid vec position
        let _ = bytes_mut.get_vec_pos();
    }
}

