// Answer 0

#[test]
fn test_unsupported_optional() {
    use std::fmt::Formatter;

    struct MockFormatter;

    impl Formatter {
        fn new() -> Self {
            MockFormatter
        }

        fn write_str(&mut self, _s: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    let mut formatter = MockFormatter::new();
    let unsupported = Unsupported::Optional;

    let _ = unsupported.fmt(&mut formatter);
}

