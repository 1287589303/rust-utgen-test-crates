// Answer 0

#[test]
fn test_cooked_c_string_with_backslash_x_error() {
    let input = Cursor {
        rest: "\\x".to_string().as_str(),
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash_u_error() {
    let input = Cursor {
        rest: "\\u".to_string().as_str(),
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_trailing_backslash() {
    let input = Cursor {
        rest: "\\n".to_string().as_str(),
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash_valid_char() {
    let input = Cursor {
        rest: "\\t".to_string().as_str(),
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

