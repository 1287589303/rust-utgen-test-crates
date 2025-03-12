// Answer 0

#[test]
fn test_ident_any_with_raw_self() {
    let span = Span::call_site();
    let cursor = Cursor {
        rest: "r#Self",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = ident_any(cursor);
}

#[test]
fn test_ident_any_with_raw_crate() {
    let span = Span::call_site();
    let cursor = Cursor {
        rest: "r#crate",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = ident_any(cursor);
}

#[test]
fn test_ident_any_with_raw_super() {
    let span = Span::call_site();
    let cursor = Cursor {
        rest: "r#super",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = ident_any(cursor);
}

#[test]
fn test_ident_any_with_raw_underscore() {
    let span = Span::call_site();
    let cursor = Cursor {
        rest: "r#_",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = ident_any(cursor);
}

