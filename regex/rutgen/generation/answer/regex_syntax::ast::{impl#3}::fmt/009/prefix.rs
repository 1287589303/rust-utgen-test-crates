// Answer 0

#[test]
fn test_error_kind_repetition_count_decimal_empty() {
    use crate::ast::ErrorKind;
    use core::fmt::Formatter;

    struct TestFormatter;
    
    impl core::fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let error = ErrorKind::RepetitionCountDecimalEmpty;
    let mut formatter = TestFormatter;

    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_error_kind_repetition_count_decimal_empty_alternative() {
    use crate::ast::ErrorKind;
    use core::fmt::Formatter;

    struct TestFormatter;
    
    impl core::fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let error = ErrorKind::RepetitionCountDecimalEmpty;
    let mut formatter = TestFormatter;

    let _ = error.fmt(&mut formatter);
}

