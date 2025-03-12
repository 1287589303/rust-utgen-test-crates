// Answer 0

#[test]
fn test_cooked_c_string_with_quote() {
    let input = Cursor {
        rest: "\"text\" more text",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash() {
    let input = Cursor {
        rest: "abc\\def",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_carriage_return() {
    let input = Cursor {
        rest: "abc\r\nnext line",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_null_character() {
    let input = Cursor {
        rest: "abc\0def",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_invalid_character() {
    let input = Cursor {
        rest: "abc\xFFdef",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

