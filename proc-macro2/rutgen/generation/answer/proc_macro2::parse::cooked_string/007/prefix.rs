// Answer 0

#[test]
fn test_cooked_string_with_valid_backslash_u_sequence() {
    let cursor = Cursor {
        rest: "\\u1234 valid text".into(),
        #[cfg(span_locations)]
        off: 0,
    };
    cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_valid_backslash_x_sequence() {
    let cursor = Cursor {
        rest: "\\xFF valid text".into(),
        #[cfg(span_locations)]
        off: 0,
    };
    cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_newline_after_backslash() {
    let cursor = Cursor {
        rest: "\\\n valid text".into(),
        #[cfg(span_locations)]
        off: 0,
    };
    cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_backslash_followed_by_space() {
    let cursor = Cursor {
        rest: "\\ \nvalid text".into(),
        #[cfg(span_locations)]
        off: 0,
    };
    cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_multiple_escape_sequences() {
    let cursor = Cursor {
        rest: "\\n\\t\\r valid text".into(),
        #[cfg(span_locations)]
        off: 0,
    };
    cooked_string(cursor);
}

