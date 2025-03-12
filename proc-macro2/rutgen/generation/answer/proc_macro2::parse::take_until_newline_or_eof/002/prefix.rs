// Answer 0

#[test]
fn test_take_until_newline_or_eof_with_crlf() {
    let cursor = Cursor {
        rest: "Hello, World!\r\nThis is a test.",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = take_until_newline_or_eof(cursor);
}

#[test]
fn test_take_until_newline_or_eof_with_crlf_at_start() {
    let cursor = Cursor {
        rest: "\r\nThis is a test.",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = take_until_newline_or_eof(cursor);
}

#[test]
fn test_take_until_newline_or_eof_with_crlf_long_input() {
    let cursor = Cursor {
        rest: "Line 1\nLine 2\r\nLine 3",
        #[cfg(span_locations)]
        off: 0,
    };
    let result = take_until_newline_or_eof(cursor);
}

