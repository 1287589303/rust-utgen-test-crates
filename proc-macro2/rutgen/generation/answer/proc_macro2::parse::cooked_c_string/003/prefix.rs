// Answer 0

#[test]
fn test_cooked_c_string_with_null_character() {
    let input = Cursor {
        rest: "Hello\0World",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_multiple_null_characters() {
    let input = Cursor {
        rest: "Line1\0Line2\0Line3",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_no_escape_sequences() {
    let input = Cursor {
        rest: "Sample text with a null character\0",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = cooked_c_string(input);
}

