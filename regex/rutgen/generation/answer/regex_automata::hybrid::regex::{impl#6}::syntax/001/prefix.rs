// Answer 0

#[test]
fn test_syntax_case_insensitive_true() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: true,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        utf8: true,
        nest_limit: 100,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_case_insensitive_false() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: false,
        multi_line: true,
        dot_matches_new_line: false,
        crlf: true,
        line_terminator: 13,
        swap_greed: true,
        ignore_whitespace: true,
        unicode: true,
        utf8: false,
        nest_limit: 500,
        octal: true,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_multiple_conditions() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: true,
        multi_line: true,
        dot_matches_new_line: true,
        crlf: false,
        line_terminator: 255,
        swap_greed: true,
        ignore_whitespace: false,
        unicode: true,
        utf8: true,
        nest_limit: 1,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_boundary_values() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 0,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        utf8: false,
        nest_limit: 1000,
        octal: false,
    };
    builder.syntax(config);
}

