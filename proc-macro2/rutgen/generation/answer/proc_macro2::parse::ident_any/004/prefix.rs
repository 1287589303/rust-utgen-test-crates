// Answer 0

#[test]
fn test_ident_any_raw_self() {
    let cursor = Cursor {
        rest: "r#self_extra",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = ident_any(cursor);
}

#[test]
fn test_ident_any_raw_crate() {
    let cursor = Cursor {
        rest: "r#crate_more",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = ident_any(cursor);
}

#[test]
fn test_ident_any_raw_super() {
    let cursor = Cursor {
        rest: "r#super_additional",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = ident_any(cursor);
}

