// Answer 0

#[test]
fn test_fmt_with_valid_regex() {
    struct TestFormatter;
    impl core::fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let valid_pattern = Arc::from("^[a-zA-Z0-9]+$");
    let regex = Regex {
        meta: meta::Regex::new(valid_pattern.as_ref()).unwrap(),
        pattern: valid_pattern,
    };
    let mut formatter = TestFormatter;
    let _ = regex.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_with_empty_regex() {
    struct TestFormatter;
    impl core::fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let valid_pattern = Arc::from("");
    let regex = Regex {
        meta: meta::Regex::new(valid_pattern.as_ref()).unwrap(),
        pattern: valid_pattern,
    };
    let mut formatter = TestFormatter;
    let _ = regex.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_with_invalid_regex() {
    struct TestFormatter;
    impl core::fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let invalid_pattern = Arc::from("[[a-zA-Z0-9+]");
    let regex = Regex {
        meta: meta::Regex::new(invalid_pattern.as_ref()).unwrap(),
        pattern: invalid_pattern,
    };
    let mut formatter = TestFormatter;
    let _ = regex.fmt(&mut formatter);
}

