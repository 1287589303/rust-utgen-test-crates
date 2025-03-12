// Answer 0

#[test]
fn test_write_mantissa_long() {
    use std::ptr;

    const DIGIT_TABLE: [u8; 200] = [
        // Filling in a small part of the digit table for testing purposes.
        b'0', b'1', b'2', b'3', b'4',
        b'5', b'6', b'7', b'8', b'9',
        // ... etc. Fill in appropriately
    ];

    let mut result: [u8; 16] = [0; 16];
    let result_ptr = result.as_mut_ptr();

    unsafe {
        write_mantissa_long(123456789012, result_ptr.add(15)); // Output will be written from end
    }

    assert_eq!(&result[8..], b"1234"); // Adjust this based on the expected output in the result
}

#[test]
fn test_write_mantissa_long_boundary() {
    use std::ptr;

    const DIGIT_TABLE: [u8; 200] = [
        // Filling in a small part of the digit table for testing purposes.
        b'0', b'1', b'2', b'3', b'4',
        b'5', b'6', b'7', b'8', b'9',
        // ... etc. Fill in appropriately
    ];

    let mut result: [u8; 16] = [0; 16];
    let result_ptr = result.as_mut_ptr();

    unsafe {
        write_mantissa_long(999999999999, result_ptr.add(15)); // Test maximum value
    }

    assert_eq!(&result[8..], b"9999"); // Adjust this based on the expected output in the result
}

