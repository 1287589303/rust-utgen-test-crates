// Answer 0

#[test]
fn test_case_fold_error_display() {
    use core::fmt::Formatter;

    struct MyFormatter;

    impl Formatter {
        fn new() -> Self {
            MyFormatter {}
        }

        fn write(&mut self, _: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let error = CaseFoldError(());
    let mut formatter = MyFormatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_case_fold_error_display_empty_formatter() {
    use core::fmt::Formatter;

    struct EmptyFormatter;

    impl Formatter {
        fn new() -> Self {
            EmptyFormatter {}
        }

        fn write(&mut self, _: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let error = CaseFoldError(());
    let mut formatter = EmptyFormatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_case_fold_error_formatter_fail() {
    use core::fmt::Formatter;

    struct FailingFormatter;

    impl Formatter {
        fn new() -> Self {
            FailingFormatter {}
        }

        fn write(&mut self, _: &str) -> core::fmt::Result {
            panic!("Simulated panic in formatter write");
        }
    }

    let error = CaseFoldError(());
    let mut formatter = FailingFormatter::new();
    let _ = error.fmt(&mut formatter);
}

