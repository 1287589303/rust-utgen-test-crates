// Answer 0

#[test]
fn test_borrow_empty_bytes_mut() {
    let vec = vec![1u8, 2u8, 3u8];
    let ptr = NonNull::from(vec.as_ptr() as *mut u8);
    let bytes_mut = BytesMut {
        ptr,
        len: 0,
        cap: vec.len(),
        data: ptr::null_mut(),
    };
    let _ = bytes_mut.borrow();
}

#[test]
fn test_borrow_full_bytes_mut() {
    let vec = vec![1u8, 2u8, 3u8];
    let ptr = NonNull::from(vec.as_ptr() as *mut u8);
    let bytes_mut = BytesMut {
        ptr,
        len: vec.len(),
        cap: vec.len(),
        data: ptr::null_mut(),
    };
    let _ = bytes_mut.borrow();
}

#[test]
fn test_borrow_non_empty_bytes_mut() {
    let vec = vec![1u8, 2u8, 3u8];
    let ptr = NonNull::from(vec.as_ptr() as *mut u8);
    let bytes_mut = BytesMut {
        ptr,
        len: 2,
        cap: vec.len(),
        data: ptr::null_mut(),
    };
    let _ = bytes_mut.borrow();
}

#[test]
fn test_borrow_boundary_case_min_len() {
    let vec = vec![1u8];
    let ptr = NonNull::from(vec.as_ptr() as *mut u8);
    let bytes_mut = BytesMut {
        ptr,
        len: 0,
        cap: vec.len(),
        data: ptr::null_mut(),
    };
    let _ = bytes_mut.borrow();
}

#[test]
fn test_borrow_boundary_case_max_len() {
    let vec = vec![1u8, 2u8, 3u8];
    let ptr = NonNull::from(vec.as_ptr() as *mut u8);
    let bytes_mut = BytesMut {
        ptr,
        len: vec.len(),
        cap: vec.len(),
        data: ptr::null_mut(),
    };
    let _ = bytes_mut.borrow();
}

