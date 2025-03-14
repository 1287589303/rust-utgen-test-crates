// Answer 0

#[test]
fn test_fmt_success() {
    struct TestFormatter;
    
    impl std::fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    struct MockChunkedEncoder;
    impl MockChunkedEncoder {
        fn encode(&self, _: &[u8], _: &mut FormatterSink) -> Result<(), fmt::Error> {
            Ok(())
        }
    }

    struct TestStruct {
        chunked_encoder: MockChunkedEncoder,
        bytes: Vec<u8>,
    }

    let test_struct = TestStruct {
        chunked_encoder: MockChunkedEncoder,
        bytes: vec![1, 2, 3],
    };

    let mut formatter = TestFormatter;
    let result = test_struct.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_fmt_panic() {
    struct TestFormatter;
    
    impl std::fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Err(std::fmt::Error)
        }
    }

    struct MockChunkedEncoder;
    impl MockChunkedEncoder {
        fn encode(&self, _: &[u8], _: &mut FormatterSink) -> Result<(), fmt::Error> {
            Ok(())
        }
    }

    struct TestStruct {
        chunked_encoder: MockChunkedEncoder,
        bytes: Vec<u8>,
    }

    let test_struct = TestStruct {
        chunked_encoder: MockChunkedEncoder,
        bytes: vec![1, 2, 3],
    };

    let mut formatter = TestFormatter;
    test_struct.fmt(&mut formatter).unwrap(); // This should panic.
}

