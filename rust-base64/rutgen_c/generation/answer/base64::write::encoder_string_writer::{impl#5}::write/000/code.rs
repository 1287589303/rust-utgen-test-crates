// Answer 0

#[test]
fn test_write_valid_utf8() {
    struct MockStrConsumer {
        consumed: String,
    }

    impl StrConsumer for MockStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let mut consumer = MockStrConsumer {
        consumed: String::new(),
    };
    let mut writer = Utf8SingleCodeUnitWriter {
        str_consumer: consumer,
    };

    let data = b"Hello, World!";
    let result = writer.write(data);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), data.len());
    assert_eq!(writer.str_consumer.consumed, "Hello, World!");
}

#[test]
#[should_panic(expected = "Input must be valid UTF-8")]
fn test_write_invalid_utf8() {
    struct MockStrConsumer;

    impl StrConsumer for MockStrConsumer {
        fn consume(&mut self, _buf: &str) {
            // No implementation needed for this test
        }
    }

    let consumer = MockStrConsumer;
    let mut writer = Utf8SingleCodeUnitWriter {
        str_consumer: consumer,
    };

    // Invalid UTF-8 data
    let invalid_data: &[u8] = &[0, 159, 146, 150]; // This byte sequence is invalid in UTF-8
    let _ = writer.write(invalid_data);
}

