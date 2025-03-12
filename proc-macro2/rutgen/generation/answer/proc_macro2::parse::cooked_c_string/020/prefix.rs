// Answer 0

#[test]
fn test_cooked_c_string_invalid_escape_sequence() {
    let input = Cursor {
        rest: "\\z",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_unescaped_newline() {
    let input = Cursor {
        rest: "\\\n",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_trailing_backslash() {
    let input = Cursor {
        rest: "\\",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_multiple_invalid_escape_sequences() {
    let input = Cursor {
        rest: "\\xg\\y",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

