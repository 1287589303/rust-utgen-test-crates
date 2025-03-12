// Answer 0

#[test]
fn test_take_until_newline_or_eof_no_newline_or_carriage_return() {
    let cursor = Cursor { rest: "Hello, World!", off: 0 };
    take_until_newline_or_eof(cursor);
}

#[test]
fn test_take_until_newline_or_eof_no_newline_or_carriage_return_at_end() {
    let cursor = Cursor { rest: "This line ends with a character.", off: 0 };
    take_until_newline_or_eof(cursor);
}

