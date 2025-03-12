// Answer 0

#[test]
fn test_delimiter_of_raw_string_err_on_exceeding_index() {
    let input_str = "a".repeat(256) + "\"";
    let cursor = Cursor {
        rest: &input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = delimiter_of_raw_string(cursor);
}

#[test]
fn test_delimiter_of_raw_string_err_on_exceeding_index_with_leading_chars() {
    let input_str = "abcde".repeat(50) + "\"";
    let cursor = Cursor {
        rest: &input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = delimiter_of_raw_string(cursor);
}

#[test]
fn test_delimiter_of_raw_string_err_on_exceeding_index_with_multiple_leading_chars() {
    let input_str = "1234567890".repeat(26) + "\"";
    let cursor = Cursor {
        rest: &input_str,
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = delimiter_of_raw_string(cursor);
}

