// Answer 0

#[test]
fn test_syntax_case_insensitive() {
    let mut builder = Builder::new();
    let config = Config::new().case_insensitive(true);
    builder.syntax(config);
}

#[test]
fn test_syntax_case_sensitive() {
    let mut builder = Builder::new();
    let config = Config::new().case_insensitive(false);
    builder.syntax(config);
}

#[test]
fn test_syntax_multi_line() {
    let mut builder = Builder::new();
    let config = Config::new().multi_line(true);
    builder.syntax(config);
}

#[test]
fn test_syntax_single_line() {
    let mut builder = Builder::new();
    let config = Config::new().multi_line(false);
    builder.syntax(config);
}

#[test]
fn test_syntax_dot_matches_new_line() {
    let mut builder = Builder::new();
    let config = Config::new().dot_matches_new_line(true);
    builder.syntax(config);
}

#[test]
fn test_syntax_dot_matches_not_new_line() {
    let mut builder = Builder::new();
    let config = Config::new().dot_matches_new_line(false);
    builder.syntax(config);
}

#[test]
fn test_syntax_crlf() {
    let mut builder = Builder::new();
    let config = Config::new().crlf(true);
    builder.syntax(config);
}

#[test]
fn test_syntax_no_crlf() {
    let mut builder = Builder::new();
    let config = Config::new().crlf(false);
    builder.syntax(config);
}

#[test]
fn test_syntax_line_terminator_zero() {
    let mut builder = Builder::new();
    let config = Config::new().line_terminator(0);
    builder.syntax(config);
}

#[test]
fn test_syntax_line_terminator_one() {
    let mut builder = Builder::new();
    let config = Config::new().line_terminator(1);
    builder.syntax(config);
}

#[test]
fn test_syntax_line_terminator_high() {
    let mut builder = Builder::new();
    let config = Config::new().line_terminator(255);
    builder.syntax(config);
}

#[test]
fn test_syntax_swap_greed() {
    let mut builder = Builder::new();
    let config = Config::new().swap_greed(true);
    builder.syntax(config);
}

#[test]
fn test_syntax_no_swap_greed() {
    let mut builder = Builder::new();
    let config = Config::new().swap_greed(false);
    builder.syntax(config);
}

#[test]
fn test_syntax_ignore_whitespace() {
    let mut builder = Builder::new();
    let config = Config::new().ignore_whitespace(true);
    builder.syntax(config);
}

#[test]
fn test_syntax_no_ignore_whitespace() {
    let mut builder = Builder::new();
    let config = Config::new().ignore_whitespace(false);
    builder.syntax(config);
}

#[test]
fn test_syntax_unicode() {
    let mut builder = Builder::new();
    let config = Config::new().unicode(true);
    builder.syntax(config);
}

#[test]
fn test_syntax_no_unicode() {
    let mut builder = Builder::new();
    let config = Config::new().unicode(false);
    builder.syntax(config);
}

#[test]
fn test_syntax_utf8() {
    let mut builder = Builder::new();
    let config = Config::new().utf8(true);
    builder.syntax(config);
}

#[test]
fn test_syntax_no_utf8() {
    let mut builder = Builder::new();
    let config = Config::new().utf8(false);
    builder.syntax(config);
}

#[test]
fn test_syntax_nest_limit_zero() {
    let mut builder = Builder::new();
    let config = Config::new().nest_limit(0);
    builder.syntax(config);
}

#[test]
fn test_syntax_nest_limit_one() {
    let mut builder = Builder::new();
    let config = Config::new().nest_limit(1);
    builder.syntax(config);
}

#[test]
fn test_syntax_nest_limit_high() {
    let mut builder = Builder::new();
    let config = Config::new().nest_limit(100);
    builder.syntax(config);
}

#[test]
fn test_syntax_octal() {
    let mut builder = Builder::new();
    let config = Config::new().octal(true);
    builder.syntax(config);
}

#[test]
fn test_syntax_no_octal() {
    let mut builder = Builder::new();
    let config = Config::new().octal(false);
    builder.syntax(config);
}

