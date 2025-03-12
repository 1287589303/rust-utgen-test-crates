// Answer 0

#[test]
fn test_write_exponent2_negative_k() {
    let mut buffer = [0u8; 3]; // Enough to hold a negative two-digit exponent
    let result_ptr = buffer.as_mut_ptr();
    let k: isize = -10;

    let result_size = unsafe { write_exponent2(k, result_ptr) };

    assert_eq!(result_size, 3);
    assert_eq!(&buffer[..result_size as usize], b"-10");
}

#[test]
#[should_panic]
fn test_write_exponent2_k_equal_100() {
    let mut buffer = [0u8; 2]; // Not enough space for a two-digit exponent
    let result_ptr = buffer.as_mut_ptr();
    let k: isize = 100;

    // This will panic due to the debug assertion failing
    let _ = unsafe { write_exponent2(k, result_ptr) };
}

