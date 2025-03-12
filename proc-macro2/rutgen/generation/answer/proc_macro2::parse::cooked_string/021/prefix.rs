// Answer 0

#[test]
fn test_cooked_string_with_carriage_return_followed_by_non_newline() {
    let cursor = Cursor {
        rest: "hello\rabc",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = cooked_string(cursor);
}

#[test]
fn test_cooked_string_with_trailing_carriage_return() {
    let cursor = Cursor {
        rest: "data_with_cr\r",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = cooked_string(cursor);
}

