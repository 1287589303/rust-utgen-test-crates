// Answer 0

#[test]
fn test_cooked_byte_string_with_non_escape_after_backslash() {
    let input = Cursor {
        rest: "Hello \\Z",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_byte_string(input);
}

#[test]
fn test_cooked_byte_string_with_backslash_followed_by_non_escape() {
    let input = Cursor {
        rest: "Line 1\\A Line 2",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_byte_string(input);
}

#[test]
fn test_cooked_byte_string_with_backslash_and_non_newline_after() {
    let input = Cursor {
        rest: "A\\B C",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_byte_string(input);
}

#[test]
fn test_cooked_byte_string_with_backslash_and_invalid_character() {
    let input = Cursor {
        rest: "Test\\C",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_byte_string(input);
}

