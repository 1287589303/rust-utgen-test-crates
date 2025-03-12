// Answer 0

#[test]
fn test_from_bytes_mut_kind_vec_zero_length() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(0);
        let _ = Vec::from(bytes_mut);
    }
}

#[test]
fn test_from_bytes_mut_kind_vec_max_length() {
    let mut bytes_mut = BytesMut::with_capacity(MAX_VEC_POS);
    unsafe {
        bytes_mut.set_len(MAX_VEC_POS);
        let _ = Vec::from(bytes_mut);
    }
}

#[test]
fn test_from_bytes_mut_kind_vec_non_zero_length() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    let data: &[u8] = &[1, 2, 3, 4, 5];
    bytes_mut.extend_from_slice(data);
    unsafe {
        bytes_mut.set_len(data.len());
        let _ = Vec::from(bytes_mut);
    }
}

#[test]
fn test_from_bytes_mut_kind_vec_full_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(MAX_ORIGINAL_CAPACITY_WIDTH);
    let data = vec![0u8; MAX_ORIGINAL_CAPACITY_WIDTH];
    bytes_mut.extend_from_slice(&data);
    unsafe {
        bytes_mut.set_len(MAX_ORIGINAL_CAPACITY_WIDTH);
        let _ = Vec::from(bytes_mut);
    }
}

#[test]
fn test_from_bytes_mut_kind_vec_boundary_case() {
    let mut bytes_mut = BytesMut::with_capacity(1);
    let data: &[u8] = &[42];
    bytes_mut.extend_from_slice(data);
    unsafe {
        bytes_mut.set_len(1);
        let _ = Vec::from(bytes_mut);
    }
}

