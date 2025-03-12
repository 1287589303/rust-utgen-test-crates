// Answer 0

#[test]
fn test_expecting_valid_formatter() {
    struct ValidVisitor;
    impl<'de> Visitor<'de> for ValidVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str("any value")
        }
    }

    let visitor = ValidVisitor;
    let mut buffer = vec![];
    let mut formatter = fmt::Formatter::new(&mut buffer);
    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_empty_formatter() {
    struct EmptyVisitor;
    impl<'de> Visitor<'de> for EmptyVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str("any value")
        }
    }

    let visitor = EmptyVisitor;
    let mut buffer = vec![];
    let formatter = fmt::Formatter::new(&mut buffer);
    let _ = visitor.expecting(&formatter);
}

#[test]
fn test_expecting_formatter_many_writes() {
    struct MultipleWritesVisitor;
    impl<'de> Visitor<'de> for MultipleWritesVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str("any value")?;
            fmt.write_str(" additional value")?;
            fmt.write_str(" another value")
        }
    }

    let visitor = MultipleWritesVisitor;
    let mut buffer = vec![];
    let mut formatter = fmt::Formatter::new(&mut buffer);
    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_formatter_invalid_state() {
    struct InvalidStateVisitor;
    impl<'de> Visitor<'de> for InvalidStateVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str("any value")
        }
    }

    let visitor = InvalidStateVisitor;
    let mut buffer = vec![];
    let formatter = fmt::Formatter::new(&mut buffer);
    let _ = visitor.expecting(&formatter);
}

