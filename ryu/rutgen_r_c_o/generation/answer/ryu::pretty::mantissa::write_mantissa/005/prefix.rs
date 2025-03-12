// Answer 0

#[test]
fn test_write_mantissa_bound_output_100() {
    let mut buffer = [0u8; 10]; // Prepare a buffer for the result
    let result_ptr = buffer.as_mut_ptr(); // Obtain a mutable pointer to the buffer
    unsafe {
        write_mantissa(100, result_ptr); // Call the function with output = 100
    }
}

#[test]
fn test_write_mantissa_bound_output_10() {
    let mut buffer = [0u8; 10]; // Prepare a buffer for the result
    let result_ptr = buffer.as_mut_ptr(); // Obtain a mutable pointer to the buffer
    unsafe {
        write_mantissa(10, result_ptr); // Call the function with output = 10
    }
}

