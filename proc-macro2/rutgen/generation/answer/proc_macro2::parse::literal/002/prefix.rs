// Answer 0

#[test]
fn test_literal_string() {
    let input = Cursor { rest: "hello" };
    let result = literal(input);
}

#[test]
fn test_literal_negative_integer() {
    let input = Cursor { rest: "-123" };
    let result = literal(input);
}

#[test]
fn test_literal_positive_integer() {
    let input = Cursor { rest: "456" };
    let result = literal(input);
}

#[test]
fn test_literal_float() {
    let input = Cursor { rest: "3.14" };
    let result = literal(input);
}

#[test]
fn test_literal_character() {
    let input = Cursor { rest: "'a'" };
    let result = literal(input);
}

#[test]
fn test_literal_byte_string() {
    let input = Cursor { rest: r#""byte_string""# };
    let result = literal(input);
}

#[test]
fn test_literal_empty_string() {
    let input = Cursor { rest: r#""""# };
    let result = literal(input);
}

#[test]
fn test_literal_negative_float() {
    let input = Cursor { rest: "-1.23" };
    let result = literal(input);
}

