// Answer 0

#[test]
fn test_string_valid_cooked_string() {
    let cursor = Cursor {
        rest: "\"valid cooked string\"",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = string(cursor);
}

#[test]
fn test_string_invalid_raw_string() {
    let cursor = Cursor {
        rest: "\"valid cooked string\" and not r",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = string(cursor);
}

