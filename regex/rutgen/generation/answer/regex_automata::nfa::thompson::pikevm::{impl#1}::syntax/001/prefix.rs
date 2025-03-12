// Answer 0

#[test]
fn test_syntax_case_insensitive() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: true,
        multi_line: false,
        line_terminator: 10, // New line
        swap_greed: false,
        unicode: true,
        utf8: true,
        nest_limit: 256,
        crlf: false,
        dot_matches_new_line: false,
        ignore_whitespace: false,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_unicode_and_multi_line() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: false,
        multi_line: true,
        line_terminator: 13, // Carriage return
        swap_greed: true,
        unicode: true,
        utf8: true,
        nest_limit: 256,
        crlf: true,
        dot_matches_new_line: false,
        ignore_whitespace: false,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_all_options_set() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: true,
        multi_line: true,
        line_terminator: 255, // Maximum value for u8
        swap_greed: true,
        unicode: true,
        utf8: true,
        nest_limit: 256,
        crlf: true,
        dot_matches_new_line: true,
        ignore_whitespace: true,
        octal: true,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_no_options() {
    let mut builder = Builder::new();
    let config = crate::util::syntax::Config {
        case_insensitive: false,
        multi_line: false,
        line_terminator: 0, // Minimum value for u8
        swap_greed: false,
        unicode: false,
        utf8: false,
        nest_limit: 256,
        crlf: false,
        dot_matches_new_line: false,
        ignore_whitespace: false,
        octal: false,
    };
    builder.syntax(config);
}

