// Answer 0

#[test]
fn test_fmt_with_valid_formatter() {
    struct MockFormatter {
        output: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = MockFormatter { output: String::new() };
    let token_iter = imp::TokenTreeIter::Compiler(proc_macro::token_stream::IntoIter::new());
    let into_iter = IntoIter {
        inner: token_iter,
        _marker: ProcMacroAutoTraits(PhantomData),
    };

    let _ = into_iter.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_empty_iterator() {
    struct MockFormatter {
        output: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = MockFormatter { output: String::new() };
    let token_iter = imp::TokenTreeIter::Fallback(fallback::TokenTreeIter::new());
    let into_iter = IntoIter {
        inner: token_iter,
        _marker: ProcMacroAutoTraits(PhantomData),
    };

    let _ = into_iter.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_with_invalid_formatter() {
    struct InvalidFormatter;

    impl fmt::Write for InvalidFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut formatter = InvalidFormatter;
    let token_iter = imp::TokenTreeIter::Compiler(proc_macro::token_stream::IntoIter::new());
    let into_iter = IntoIter {
        inner: token_iter,
        _marker: ProcMacroAutoTraits(PhantomData),
    };

    let _ = into_iter.fmt(&mut formatter);
}

