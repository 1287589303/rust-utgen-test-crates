// Answer 0

#[test]
fn test_token_stream_with_doc_comment_and_open_parenthesis() {
    let input = Cursor {
        rest: "/// This is a doc comment\n( )",
        off: 0,
    };
    let mut builder = TokenStreamBuilder::new();
    let _ = token_stream(input);
}

#[test]
fn test_token_stream_with_doc_comment_and_open_bracket() {
    let input = Cursor {
        rest: "/// This is a doc comment\n[ ]",
        off: 0,
    };
    let mut builder = TokenStreamBuilder::new();
    let _ = token_stream(input);
}

#[test]
fn test_token_stream_with_doc_comment_and_open_brace() {
    let input = Cursor {
        rest: "/// This is a doc comment\n{ }",
        off: 0,
    };
    let mut builder = TokenStreamBuilder::new();
    let _ = token_stream(input);
}

#[test]
fn test_token_stream_with_error_after_whitespace() {
    let input = Cursor {
        rest: "   /*ERROR*/  )",
        off: 0,
    };
    let mut builder = TokenStreamBuilder::new();
    let result = token_stream(input);
}

#[test]
fn test_token_stream_whitespace_and_error_before_valid_delimiter() {
    let input = Cursor {
        rest: "    /*ERROR*/ [ ]",
        off: 0,
    };
    let mut builder = TokenStreamBuilder::new();
    let _ = token_stream(input);
}

#[test]
fn test_token_stream_with_only_closing_delimiter() {
    let input = Cursor {
        rest: "   )",
        off: 0,
    };
    let mut builder = TokenStreamBuilder::new();
    let result = token_stream(input);
}

#[test]
fn test_token_stream_with_only_deep_closing_delimiter() {
    let input = Cursor {
        rest: "   )    )   ",
        off: 0,
    };
    let mut builder = TokenStreamBuilder::new();
    let result = token_stream(input);
}

