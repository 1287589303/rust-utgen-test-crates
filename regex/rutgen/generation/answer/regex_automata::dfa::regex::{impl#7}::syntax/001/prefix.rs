// Answer 0

#[test]
fn test_syntax_case_insensitive() {
    let mut builder = Builder::new();
    let config = Config {
        case_insensitive: true,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        utf8: false,
        nest_limit: 100,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_multi_line() {
    let mut builder = Builder::new();
    let config = Config {
        case_insensitive: false,
        multi_line: true,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 13,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        utf8: false,
        nest_limit: 100,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_dot_matches_new_line() {
    let mut builder = Builder::new();
    let config = Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: true,
        crlf: false,
        line_terminator: 10,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        utf8: false,
        nest_limit: 100,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_line_terminator() {
    let mut builder = Builder::new();
    let config = Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 255,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        utf8: false,
        nest_limit: 100,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_swap_greed() {
    let mut builder = Builder::new();
    let config = Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        swap_greed: true,
        ignore_whitespace: false,
        unicode: false,
        utf8: false,
        nest_limit: 100,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_ignore_whitespace() {
    let mut builder = Builder::new();
    let config = Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        swap_greed: false,
        ignore_whitespace: true,
        unicode: false,
        utf8: false,
        nest_limit: 100,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_unicode() {
    let mut builder = Builder::new();
    let config = Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: true,
        utf8: false,
        nest_limit: 100,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_utf8() {
    let mut builder = Builder::new();
    let config = Config {
        case_insensitive: false,
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
fn test_syntax_nest_limit() {
    let mut builder = Builder::new();
    let config = Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        utf8: false,
        nest_limit: 0,
        octal: false,
    };
    builder.syntax(config);
}

#[test]
fn test_syntax_octal() {
    let mut builder = Builder::new();
    let config = Config {
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        crlf: false,
        line_terminator: 10,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        utf8: false,
        nest_limit: 100,
        octal: true,
    };
    builder.syntax(config);
}

