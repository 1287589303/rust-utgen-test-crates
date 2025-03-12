// Answer 0

#[test]
fn test_write_exponent2_k_100_sign_true() {
    let mut buffer: [u8; 4] = [0; 4];
    let result_ptr = buffer.as_mut_ptr();
    unsafe {
        let k: isize = -100;
        let _ = write_exponent2(k, result_ptr);
    }
}

#[test]
fn test_write_exponent2_k_100_sign_false() {
    let mut buffer: [u8; 4] = [0; 4];
    let result_ptr = buffer.as_mut_ptr();
    unsafe {
        let k: isize = 100;
        let _ = write_exponent2(k, result_ptr);
    }
}

