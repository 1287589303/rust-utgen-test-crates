// Answer 0

#[test]
fn test_expectation_with_valid_formatter() {
    use std::fmt;
    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    let ignored_any = IgnoredAny;
    let _ = ignored_any.expecting(&mut formatter);
}

#[test]
fn test_expectation_with_empty_formatter() {
    use std::fmt;
    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    let ignored_any = IgnoredAny;
    let _ = ignored_any.expecting(&mut formatter);
}

