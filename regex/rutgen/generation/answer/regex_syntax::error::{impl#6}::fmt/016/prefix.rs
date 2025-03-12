// Answer 0

#[test]
fn test_formatter_display_with_single_line_pattern() {
    let pattern = "a*b+c?";
    let error_message = "expected a token";
    let span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 6 }};
    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span,
        aux_span: None,
    };
    
    let mut output = String::new();
    let result = formatter.fmt(&mut core::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
}

#[test]
fn test_formatter_display_with_empty_pattern() {
    let pattern = "";
    let error_message = "empty regex";
    let span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 0 }};
    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span,
        aux_span: None,
    };
    
    let mut output = String::new();
    let result = formatter.fmt(&mut core::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
}

#[test]
fn test_formatter_display_with_single_character_pattern() {
    let pattern = "a";
    let error_message = "expected more";
    let span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 1 }};
    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span,
        aux_span: None,
    };
    
    let mut output = String::new();
    let result = formatter.fmt(&mut core::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
}

