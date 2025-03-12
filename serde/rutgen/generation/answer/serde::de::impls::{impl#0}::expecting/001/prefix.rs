// Answer 0

#[test]
fn test_expecting_valid_formatter() {
    struct TestFormatter;
    
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut formatter = TestFormatter;
    let visitor = UnitVisitor;
    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_formatter_empty() {
    struct EmptyFormatter;
    
    impl fmt::Write for EmptyFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut formatter = EmptyFormatter;
    let visitor = UnitVisitor;
    let _ = visitor.expecting(&mut formatter);
}

