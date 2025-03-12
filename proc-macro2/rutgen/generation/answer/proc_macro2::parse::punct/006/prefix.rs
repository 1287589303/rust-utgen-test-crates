// Answer 0

#[test]
fn test_punct_with_valid_single_quote() {
    let cursor = Cursor { rest: "'+" };
    let result = punct(cursor);
}

#[test]
fn test_punct_with_valid_single_quote_and_alone_punct() {
    let cursor = Cursor { rest: "'-" };
    let result = punct(cursor);
}

#[test]
fn test_punct_with_valid_single_quote_and_joint_punct() {
    let cursor = Cursor { rest: "'=" };
    let result = punct(cursor);
}

#[test]
fn test_punct_with_valid_single_quote_and_multiple_punct() {
    let cursor = Cursor { rest: "';:" };
    let result = punct(cursor);
}

#[test]
fn test_punct_with_single_quote_and_valid_inner_punct() {
    let cursor = Cursor { rest: "'*+" };
    let result = punct(cursor);
}

