// Answer 0

#[test]
fn test_fmt_escape_hex_empty() {
    struct FormatterMock;
    impl core::fmt::Write for FormatterMock {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let mut formatter = FormatterMock;
    let error_kind = ErrorKind::EscapeHexEmpty;
    let _ = error_kind.fmt(&mut formatter);
}

#[test]
fn test_fmt_escape_hex_invalid() {
    struct FormatterMock;
    impl core::fmt::Write for FormatterMock {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let mut formatter = FormatterMock;
    let error_kind = ErrorKind::EscapeHexInvalid;
    let _ = error_kind.fmt(&mut formatter);
}

