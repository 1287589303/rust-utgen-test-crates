// Answer 0

#[test]
fn test_expecting_with_valid_formatter() {
    use std::fmt::Formatter;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }
    }

    let mut formatter = Formatter::default();
    let visitor = TestVisitor;

    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_with_string_formatter() {
    use std::fmt::Formatter;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }
    }

    let mut formatter = Formatter::new();
    let visitor = TestVisitor;

    let _ = visitor.expecting(&mut formatter);
}

