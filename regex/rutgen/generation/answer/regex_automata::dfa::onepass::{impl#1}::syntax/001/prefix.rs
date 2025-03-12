// Answer 0

#[test]
fn test_syntax_case_insensitive() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: true,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        unicode: false,
        utf8: false,
        ..Default::default()
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_multi_line() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: false,
        multi_line: true,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        unicode: false,
        utf8: false,
        ..Default::default()
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_dot_matches_new_line() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: true,
        crlf: false,
        line_terminator: 10,
        unicode: false,
        utf8: false,
        ..Default::default()
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_crlf() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: true,
        line_terminator: 13,
        unicode: false,
        utf8: false,
        ..Default::default()
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_unicode() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        unicode: true,
        utf8: false,
        ..Default::default()
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_utf8() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        unicode: false,
        utf8: true,
        ..Default::default()
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_full_configuration() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: true,
        multi_line: true,
        dot_matches_new_line: true,
        crlf: true,
        line_terminator: 255,
        unicode: true,
        utf8: true,
        ..Default::default()
    };
    builder.syntax(config);
}

