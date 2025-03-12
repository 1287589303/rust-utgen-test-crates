// Answer 0

#[test]
fn test_format_with_positive_integer() {
    struct TestInteger;

    impl Copy for TestInteger {}

    impl Sealed for TestInteger {
        type Buffer = [MaybeUninit<u8>; 20];

        fn write(self, buf: &mut Self::Buffer) -> &str {
            let str_representation = "42";
            buf[..str_representation.len()].clone_from_slice(&str_representation.as_bytes());
            unsafe { str::from_utf8_unchecked(&buf[..str_representation.len()]) }
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(TestInteger);
    assert_eq!(result, "42");
}

#[test]
fn test_format_with_negative_integer() {
    struct TestNegativeInteger;

    impl Copy for TestNegativeInteger {}

    impl Sealed for TestNegativeInteger {
        type Buffer = [MaybeUninit<u8>; 20];

        fn write(self, buf: &mut Self::Buffer) -> &str {
            let str_representation = "-42";
            buf[..str_representation.len()].clone_from_slice(&str_representation.as_bytes());
            unsafe { str::from_utf8_unchecked(&buf[..str_representation.len()]) }
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(TestNegativeInteger);
    assert_eq!(result, "-42");
}

#[test]
fn test_format_with_zero_integer() {
    struct TestZeroInteger;

    impl Copy for TestZeroInteger {}

    impl Sealed for TestZeroInteger {
        type Buffer = [MaybeUninit<u8>; 20];

        fn write(self, buf: &mut Self::Buffer) -> &str {
            let str_representation = "0";
            buf[..str_representation.len()].clone_from_slice(&str_representation.as_bytes());
            unsafe { str::from_utf8_unchecked(&buf[..str_representation.len()]) }
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(TestZeroInteger);
    assert_eq!(result, "0");
}

