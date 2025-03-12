// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    let config = Config::new()
        .ignore_whitespace(true);
    let result = config.get_ignore_whitespace();
}

#[test]
fn test_ignore_whitespace_false() {
    let config = Config::new()
        .ignore_whitespace(false);
    let result = config.get_ignore_whitespace();
}

#[test]
fn test_ignore_whitespace_with_case_insensitive() {
    let config = Config::new()
        .case_insensitive(true)
        .ignore_whitespace(true);
    let result = config.get_ignore_whitespace();
}

#[test]
fn test_ignore_whitespace_with_multi_line() {
    let config = Config::new()
        .multi_line(true)
        .ignore_whitespace(false);
    let result = config.get_ignore_whitespace();
}

#[test]
fn test_ignore_whitespace_with_line_terminator() {
    let config = Config::new()
        .line_terminator(10) // using newline character
        .ignore_whitespace(true);
    let result = config.get_ignore_whitespace();
}

#[test]
fn test_ignore_whitespace_with_nest_limit() {
    let config = Config::new()
        .nest_limit(5)
        .ignore_whitespace(false);
    let result = config.get_ignore_whitespace();
}

#[test]
fn test_ignore_whitespace_with_unicode_and_utf8() {
    let config = Config::new()
        .unicode(true)
        .utf8(true)
        .ignore_whitespace(true);
    let result = config.get_ignore_whitespace();
}

