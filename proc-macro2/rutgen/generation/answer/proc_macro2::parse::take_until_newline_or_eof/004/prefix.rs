// Answer 0

#[test]
fn test_take_until_newline_with_newline() {
    let input = Cursor { rest: "Hello\nWorld" };
    take_until_newline_or_eof(input);
}

#[test]
fn test_take_until_newline_with_carriage_return_newline() {
    let input = Cursor { rest: "Hello\r\nWorld" };
    take_until_newline_or_eof(input);
}

#[test]
fn test_take_until_newline_with_carriage_return() {
    let input = Cursor { rest: "Hello\rWorld" };
    take_until_newline_or_eof(input);
}

#[test]
fn test_take_until_newline_without_newline_or_carriage_return() {
    let input = Cursor { rest: "HelloWorld" };
    take_until_newline_or_eof(input);
}

#[test]
fn test_take_until_newline_empty() {
    let input = Cursor { rest: "" };
    take_until_newline_or_eof(input);
}

#[test]
fn test_take_until_newline_with_newline_only() {
    let input = Cursor { rest: "\n" };
    take_until_newline_or_eof(input);
}

#[test]
fn test_take_until_newline_with_carriage_return_only() {
    let input = Cursor { rest: "\r" };
    take_until_newline_or_eof(input);
}

