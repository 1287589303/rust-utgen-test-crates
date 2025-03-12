// Answer 0

#[test]
fn test_put_bytes_exact_length() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 5];
    let cnt = 5;
    let val: u8 = 42;
    unsafe {
        // Initialize buffer to zero before testing
        ptr::write_bytes(buffer.as_mut_ptr() as *mut u8, 0, buffer.len());
        let buf_mut = &mut buffer;
        buf_mut.put_bytes(val, cnt);
    }
}

#[test]
fn test_put_bytes_exact_length_with_max_value() {
    let mut buffer: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 10];
    let cnt = 10;
    let val: u8 = 255;
    unsafe {
        // Initialize buffer to zero before testing
        ptr::write_bytes(buffer.as_mut_ptr() as *mut u8, 0, buffer.len());
        let buf_mut = &mut buffer;
        buf_mut.put_bytes(val, cnt);
    }
}

