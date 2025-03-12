// Answer 0

#[test]
fn test_get_octal_false() {
    let config = Config::new()
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .crlf(false)
        .line_terminator(0)
        .swap_greed(false)
        .ignore_whitespace(false)
        .unicode(false)
        .utf8(false)
        .nest_limit(0)
        .octal(false);
    config.get_octal();
}

#[test]
fn test_get_octal_true() {
    let config = Config::new()
        .case_insensitive(true)
        .multi_line(true)
        .dot_matches_new_line(true)
        .crlf(true)
        .line_terminator(255)
        .swap_greed(true)
        .ignore_whitespace(true)
        .unicode(true)
        .utf8(true)
        .nest_limit(4294967295)
        .octal(true);
    config.get_octal();
}

