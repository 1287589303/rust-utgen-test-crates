// Answer 0

#[test]
fn test_owned_to_vec_with_zero_length() {
    let data = AtomicPtr::new(ptr::null_mut());
    let ptr: *const u8 = ptr::null();
    let len: usize = 0;
    let _ = unsafe { owned_to_vec(&data, ptr, len) };
}

#[test]
fn test_owned_to_vec_with_small_length() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(1)) as *mut ());
    let slice = [1u8, 2, 3];
    let ptr: *const u8 = slice.as_ptr();
    let len: usize = slice.len();
    let _ = unsafe { owned_to_vec(&data, ptr, len) };
}

#[test]
fn test_owned_to_vec_with_large_length() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(1)) as *mut ());
    let slice: Vec<u8> = (0..usize::MAX).map(|x| x as u8).collect();
    let ptr: *const u8 = slice.as_ptr();
    let len: usize = slice.len();
    let _ = unsafe { owned_to_vec(&data, ptr, len) };
}

