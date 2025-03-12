// Answer 0

#[test]
fn test_delimiter_of_raw_string_at_edge() {
    let cursor = Cursor {
        rest: "#\"valid string\"",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = delimiter_of_raw_string(cursor);
}

#[test]
fn test_delimiter_of_raw_string_at_start() {
    let cursor = Cursor {
        rest: "\"another valid string\"",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = delimiter_of_raw_string(cursor);
}

#[test]
fn test_delimiter_of_raw_string_single_character() {
    let cursor = Cursor {
        rest: "#\"a\"",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = delimiter_of_raw_string(cursor);
}

