// Answer 0

fn test_write_valid_utf8() -> io::Result<()> {
    struct TestConsumer {
        consumed: String,
    }

    impl StrConsumer for TestConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let mut consumer = TestConsumer { consumed: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };
    let input = b"hello";

    let result = writer.write(input)?;
    assert_eq!(result, input.len());
    Ok(())
}

fn test_write_empty_string() -> io::Result<()> {
    struct TestConsumer {
        consumed: String,
    }

    impl StrConsumer for TestConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let mut consumer = TestConsumer { consumed: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };
    let input: &[u8] = b"";

    let result = writer.write(input)?;
    assert_eq!(result, input.len());
    Ok(())
}

fn test_write_ascii_uppercase() -> io::Result<()> {
    struct TestConsumer {
        consumed: String,
    }

    impl StrConsumer for TestConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let mut consumer = TestConsumer { consumed: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };
    let input = b"HELLO";

    let result = writer.write(input)?;
    assert_eq!(result, input.len());
    Ok(())
}

fn test_write_unicode_characters() -> io::Result<()> {
    struct TestConsumer {
        consumed: String,
    }

    impl StrConsumer for TestConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let mut consumer = TestConsumer { consumed: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };
    let input = "こんにちは".as_bytes(); // Japanese for "hello"

    let result = writer.write(input)?;
    assert_eq!(result, input.len());
    Ok(())
}

fn test_write_valid_utf8_with_special_chars() -> io::Result<()> {
    struct TestConsumer {
        consumed: String,
    }

    impl StrConsumer for TestConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let mut consumer = TestConsumer { consumed: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };
    let input = b"hello, 😀";

    let result = writer.write(input)?;
    assert_eq!(result, input.len());
    Ok(())
}

fn main() {
    test_write_valid_utf8().unwrap();
    test_write_empty_string().unwrap();
    test_write_ascii_uppercase().unwrap();
    test_write_unicode_characters().unwrap();
    test_write_valid_utf8_with_special_chars().unwrap();
}

