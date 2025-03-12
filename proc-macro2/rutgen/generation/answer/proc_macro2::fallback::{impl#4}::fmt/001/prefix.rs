// Answer 0

#[test]
fn test_lex_error_fmt_ok() {
    struct MockFormatter {
        output: String,
        should_fail: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                self.output.push_str(s);
                Ok(())
            }
        }
    }

    let mut formatter = MockFormatter { output: String::new(), should_fail: false };
    let error = LexError { span: Span::Fallback(Span { lo: 0, hi: 0 }) };
    error.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_lex_error_fmt_fail() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mut formatter = MockFormatter { should_fail: true };
    let error = LexError { span: Span::Fallback(Span { lo: 0, hi: 0 }) };
    error.fmt(&mut formatter);
}

