// Answer 0

#[test]
fn test_expecting_valid_formatter() {
    struct TestFormatter;
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let visitor = TagOrContentFieldVisitor {
        tag: "tagField",
        content: "contentField",
    };
    let mut formatter = TestFormatter;

    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_different_valid_formatter() {
    struct TestFormatter;
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let visitor = TagOrContentFieldVisitor {
        tag: "anotherTag",
        content: "anotherContent",
    };
    let mut formatter = TestFormatter;

    let _ = visitor.expecting(&mut formatter);
}

