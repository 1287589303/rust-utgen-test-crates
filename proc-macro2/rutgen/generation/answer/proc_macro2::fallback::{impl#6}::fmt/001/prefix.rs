// Answer 0

#[test]
#[should_panic]
fn test_fmt_invalid_formatter() {
    struct InvalidFormatter;

    impl fmt::Write for InvalidFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut invalid_formatter = InvalidFormatter;
    let token_stream = TokenStream {
        inner: RcVec::new(),
    };
    let _ = token_stream.fmt(&mut invalid_formatter);
}

#[test]
fn test_fmt_empty_token_stream() {
    struct InvalidFormatter;

    impl fmt::Write for InvalidFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut invalid_formatter = InvalidFormatter;
    let token_stream = TokenStream {
        inner: RcVec::new(),
    };
    let _ = token_stream.fmt(&mut invalid_formatter);
}

#[test]
fn test_fmt_invalid_write_error() {
    struct InvalidFormatter;

    impl fmt::Write for InvalidFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut invalid_formatter = InvalidFormatter;
    let token_tree = TokenTree::Ident(Ident::new("invalid", proc_macro::Span::call_site()));
    let token_stream = TokenStream {
        inner: RcVec::from(vec![token_tree]),
    };
    let _ = token_stream.fmt(&mut invalid_formatter);
}

