// Answer 0

#[test]
fn test_apply_all_options_enabled() {
    let mut builder = ParserBuilder::new();
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
        .nest_limit(10)
        .octal(true);
    config.apply(&mut builder);
}

#[test]
fn test_apply_all_options_disabled() {
    let mut builder = ParserBuilder::new();
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
    config.apply(&mut builder);
}

#[test]
fn test_apply_boundary_conditions() {
    let mut builder = ParserBuilder::new();
    
    let config_zero_nest_limit = Config::new()
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(true)
        .crlf(false)
        .line_terminator(0)
        .swap_greed(false)
        .ignore_whitespace(true)
        .unicode(false)
        .utf8(true)
        .nest_limit(0)
        .octal(true);
    config_zero_nest_limit.apply(&mut builder);

    let config_max_nest_limit = Config::new()
        .case_insensitive(false)
        .multi_line(true)
        .dot_matches_new_line(false)
        .crlf(true)
        .line_terminator(255)
        .swap_greed(true)
        .ignore_whitespace(false)
        .unicode(true)
        .utf8(false)
        .nest_limit(10)
        .octal(false);
    config_max_nest_limit.apply(&mut builder);
}

