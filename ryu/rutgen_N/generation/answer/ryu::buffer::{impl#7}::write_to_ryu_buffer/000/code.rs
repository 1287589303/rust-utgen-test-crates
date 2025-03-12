// Answer 0

#[test]
fn test_write_to_ryu_buffer() {
    struct MockSelf {
        value: f64,
    }

    let mock = MockSelf { value: 123.456 };

    let mut buffer = vec![0u8; 32]; 
    let result_ptr = buffer.as_mut_ptr();

    let bytes_written = unsafe { mock.write_to_ryu_buffer(result_ptr) };

    assert!(bytes_written > 0);
    assert_eq!(&buffer[..bytes_written as usize], b"123.456");
}

