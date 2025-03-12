// Answer 0

#[test]
fn test_skip_whitespace_comments() {
    let input = Cursor { rest: "   // comment\n   /* block comment */" };
    let result = skip_whitespace(input);
}

#[test]
fn test_skip_whitespace_multiple_slashes() {
    let input = Cursor { rest: "/// this is a comment\n   // another comment" };
    let result = skip_whitespace(input);
}

#[test]
fn test_skip_whitespace_block_comment() {
    let input = Cursor { rest: "   /* start of block comment\n   still in block comment */   code" };
    let result = skip_whitespace(input);
}

#[test]
fn test_skip_whitespace_empty_after_comments() {
    let input = Cursor { rest: "// comment only\n" };
    let result = skip_whitespace(input);
}

#[test]
fn test_skip_whitespace_spaces_and_tabs() {
    let input = Cursor { rest: "\t\t   // beginning with tabs and spaces\nint x = 0;" };
    let result = skip_whitespace(input);
}

#[test]
fn test_skip_whitespace_nested_comments() {
    let input = Cursor { rest: "   /* comment start /* nested comment */ end */   code" };
    let result = skip_whitespace(input);
}

#[test]
fn test_skip_whitespace_trailing_spaces() {
    let input = Cursor { rest: "   // single-line comment with trailing spaces    \n   int y = 1;" };
    let result = skip_whitespace(input);
}

#[test]
fn test_skip_whitespace_multiline_string() {
    let input = Cursor { rest: "   \"string with \n new line\" // comment after string" };
    let result = skip_whitespace(input);
}

