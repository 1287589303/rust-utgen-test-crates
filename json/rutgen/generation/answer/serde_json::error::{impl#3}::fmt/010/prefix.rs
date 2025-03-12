// Answer 0

#[test]
fn test_fmt_invalid_unicode_code_point() {
    struct DummyFormatter;

    impl fmt::Formatter for DummyFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            fmt::Result::Ok(())
        }
    }

    let error_code = ErrorCode::InvalidUnicodeCodePoint;
    let mut formatter = DummyFormatter;

    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_message() {
    struct DummyFormatter;

    impl fmt::Formatter for DummyFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            fmt::Result::Ok(())
        }
    }

    let error_code = ErrorCode::Message(Box::from("Test message"));
    let mut formatter = DummyFormatter;

    let _ = error_code.fmt(&mut formatter);
}

