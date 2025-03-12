// Answer 0

#[test]
fn test_cooked_string_with_hex_escape_and_newline() {
    let input = Cursor {
        rest: "\\x41\\notherText",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_double_backslash_and_newline() {
    let input = Cursor {
        rest: "\\\\\notherText",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_multiple_escapes() {
    let input = Cursor {
        rest: "\\x41\\n\\t\\\"\\'otherText",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = cooked_string(input);
}

