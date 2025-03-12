// Answer 0

#[test]
fn test_format_integer_exceeds_max_str_len() {
    use std::mem::MaybeUninit;

    struct TestInteger;

    trait Integer {
        const MAX_STR_LEN: usize;
        fn write(&self, buffer: &mut [MaybeUninit<u8>]) -> &str;
    }

    impl Integer for TestInteger {
        const MAX_STR_LEN: usize = 5; // example maximum length

        fn write(&self, buffer: &mut [MaybeUninit<u8>]) -> &str {
            let string = "123456"; // string length exceeds the MAX_STR_LEN
            for (i, byte) in string.bytes().enumerate() {
                buffer[i].write(byte);
            }
            unsafe { std::str::from_utf8_unchecked(&buffer[..string.len()]) }
        }
    }

    struct Buffer {
        bytes: [MaybeUninit<u8>; 6], // size greater than TestInteger::MAX_STR_LEN
    }

    let mut buffer = Buffer {
        bytes: Default::default(),
    };

    let result = buffer.format(TestInteger);

    assert_eq!(result, "123456");
}

