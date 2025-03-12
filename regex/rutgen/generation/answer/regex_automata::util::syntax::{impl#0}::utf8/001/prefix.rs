// Answer 0

#[test]
fn test_utf8_true() {
    let config = Config::new().utf8(true);
}

#[test]
fn test_utf8_false() {
    let config = Config::new().utf8(false);
}

#[test]
fn test_utf8_chain() {
    let config = Config::new()
        .utf8(true)
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(true)
        .crlf(false)
        .line_terminator(10)
        .swap_greed(false)
        .ignore_whitespace(true)
        .unicode(true)
        .nest_limit(10)
        .octal(false);
}

#[test]
fn test_utf8_chain_with_false() {
    let config = Config::new()
        .utf8(false)
        .case_insensitive(false)
        .multi_line(true)
        .dot_matches_new_line(false)
        .crlf(true)
        .line_terminator(255)
        .swap_greed(true)
        .ignore_whitespace(false)
        .unicode(false)
        .nest_limit(0)
        .octal(true);
}

