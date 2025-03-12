// Answer 0

#[test]
fn test_format_with_i8() {
    struct IntegerI8;

    impl private::Sealed for IntegerI8 {}
    impl Integer for IntegerI8 {
        const MAX_STR_LEN: usize = 4; // For example, the maximum length for i8 could be 4 (-128 to 127)
        
        fn write(&self, buffer: &mut [MaybeUninit<u8>]) -> &str {
            let value = 127i8; // A value that fits in i8 and at maximum string length
            let str_value = value.to_string();
            let bytes = str_value.as_bytes();

            for (i, &byte) in bytes.iter().enumerate() {
                buffer[i].write(byte);
            }

            std::str::from_utf8(&buffer[..bytes.len()]).unwrap()
        }
    }

    let mut buffer = [MaybeUninit::uninit(); 4]; // Initializing the buffer
    let result = buffer.format(IntegerI8);
    assert_eq!(result.len(), IntegerI8::MAX_STR_LEN);
    assert_eq!(result, "127");
}

#[test]
fn test_format_with_i16() {
    struct IntegerI16;

    impl private::Sealed for IntegerI16 {}
    impl Integer for IntegerI16 {
        const MAX_STR_LEN: usize = 6; // For example, the maximum length for i16 could be 6 (-32768 to 32767)
        
        fn write(&self, buffer: &mut [MaybeUninit<u8>]) -> &str {
            let value = 32767i16; // A value that fits in i16 and at maximum string length
            let str_value = value.to_string();
            let bytes = str_value.as_bytes();

            for (i, &byte) in bytes.iter().enumerate() {
                buffer[i].write(byte);
            }

            std::str::from_utf8(&buffer[..bytes.len()]).unwrap()
        }
    }

    let mut buffer = [MaybeUninit::uninit(); 6]; // Initializing the buffer
    let result = buffer.format(IntegerI16);
    assert_eq!(result.len(), IntegerI16::MAX_STR_LEN);
    assert_eq!(result, "32767");
}

#[test]
fn test_format_with_i32() {
    struct IntegerI32;

    impl private::Sealed for IntegerI32 {}
    impl Integer for IntegerI32 {
        const MAX_STR_LEN: usize = 11; // For example, the maximum length for i32 could be 11 (-2147483648 to 2147483647)
        
        fn write(&self, buffer: &mut [MaybeUninit<u8>]) -> &str {
            let value = 2147483647i32; // A value that fits in i32 and at maximum string length
            let str_value = value.to_string();
            let bytes = str_value.as_bytes();

            for (i, &byte) in bytes.iter().enumerate() {
                buffer[i].write(byte);
            }

            std::str::from_utf8(&buffer[..bytes.len()]).unwrap()
        }
    }

    let mut buffer = [MaybeUninit::uninit(); 11]; // Initializing the buffer
    let result = buffer.format(IntegerI32);
    assert_eq!(result.len(), IntegerI32::MAX_STR_LEN);
    assert_eq!(result, "2147483647");
}

