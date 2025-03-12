// Answer 0

#[test]
fn test_borrow_mut_valid() {
    let vec_data = vec![1, 2, 3, 4];
    let ptr = NonNull::new(vec_data.as_ptr() as *mut u8).unwrap();
    let cap = vec_data.len();
    let len = cap;

    let mut bytes_mut = BytesMut {
        ptr,
        len,
        cap,
        data: ptr::null_mut(),
    };

    let result: &mut [u8] = bytes_mut.borrow_mut();
    // Here we would consume `result` to avoid unused variable warnings,
    // but since we are focusing solely on function calls, we leave it.
}

#[test]
fn test_borrow_mut_boundary_zero_len() {
    let vec_data = vec![];
    let ptr = NonNull::new(vec_data.as_ptr() as *mut u8).unwrap();
    let cap = vec_data.len();
    let len = 0;

    let mut bytes_mut = BytesMut {
        ptr,
        len,
        cap,
        data: ptr::null_mut(),
    };

    // This should not be permissible as len is 0
    let result: &mut [u8] = bytes_mut.borrow_mut();
}

#[test]
fn test_borrow_mut_boundary_equals_cap() {
    let vec_data = vec![5, 6, 7];
    let ptr = NonNull::new(vec_data.as_ptr() as *mut u8).unwrap();
    let cap = vec_data.len();
    let len = cap;

    let mut bytes_mut = BytesMut {
        ptr,
        len,
        cap,
        data: ptr::null_mut(),
    };

    let result: &mut [u8] = bytes_mut.borrow_mut();
}

#[test]
fn test_borrow_mut_greater_than_zero() {
    let vec_data = vec![8, 9];
    let ptr = NonNull::new(vec_data.as_ptr() as *mut u8).unwrap();
    let cap = vec_data.len();
    let len = cap;

    let mut bytes_mut = BytesMut {
        ptr,
        len,
        cap,
        data: ptr::null_mut(),
    };

    let result: &mut [u8] = bytes_mut.borrow_mut();
}

