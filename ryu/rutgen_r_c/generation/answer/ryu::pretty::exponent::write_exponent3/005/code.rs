// Answer 0

#[test]
fn test_write_exponent3_case1() {
    let mut buffer: [u8; 4] = [0; 4];
    let result_ptr = buffer.as_mut_ptr();
    let k: isize = 100;

    let return_value = unsafe { write_exponent3(k, result_ptr) };

    assert_eq!(return_value, 3);
    assert_eq!(buffer[0], b'0');
    assert_eq!(buffer[1], b'1');
    assert_eq!(buffer[2], b'0');
}

