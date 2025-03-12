// Answer 0

#[test]
fn test_write_exponent3_k_10() {
    let mut buffer = [0u8; 4]; // Buffer to hold the result
    let result_ptr = buffer.as_mut_ptr(); // Convert to raw pointer

    unsafe {
        let k: isize = 10; // k is 10, sign will be false
        let length = write_exponent3(k, result_ptr);
        assert_eq!(length, 2); // Expect return value to be sign (0) + 2
        assert_eq!(&buffer[..2], b"10"); // Expect the result to be "10"
    }
}

