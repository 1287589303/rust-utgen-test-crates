// Answer 0

#[test]
fn test_fmt_unexpected_end_of_hex_escape() {
    struct MockFormatter;
    
    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let error_code = ErrorCode::UnexpectedEndOfHexEscape;
    let mut formatter = MockFormatter;

    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_double_quote() {
    struct MockFormatter;
    
    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let error_code = ErrorCode::UnexpectedEndOfHexEscape;
    let mut formatter = MockFormatter;

    let _ = error_code.fmt(&mut formatter);
}

