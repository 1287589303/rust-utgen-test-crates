// Answer 0

#[test]
fn test_write_exponent3_k_10() {
    let mut buffer = [0u8; 4]; // Enough space for the result
    let result_ptr = buffer.as_mut_ptr();
    let k: isize = 10;
    
    let length = unsafe { write_exponent3(k, result_ptr) };
    
    assert_eq!(length, 2); // sign (false) + 2 for two digits "10"
    assert_eq!(&buffer[0..2], b"10"); // Result should be "10"
}

