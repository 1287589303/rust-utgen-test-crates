// Answer 0

#[test]
fn test_ident_any_creates_err_for_raw_crate() {
    let input = Cursor {
        rest: "r#crate",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = ident_any(input);
}

#[test]
fn test_ident_any_creates_err_for_raw_creates() {
    let input = Cursor {
        rest: "r#super",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = ident_any(input);
}

#[test]
fn test_ident_any_creates_err_for_raw_self() {
    let input = Cursor {
        rest: "r#self",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = ident_any(input);
}

#[test]
fn test_ident_any_creates_err_for_raw_Self() {
    let input = Cursor {
        rest: "r#Self",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = ident_any(input);
}

#[test]
fn test_ident_any_creates_err_for_raw_() {
    let input = Cursor {
        rest: "r#_",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = ident_any(input);
}

