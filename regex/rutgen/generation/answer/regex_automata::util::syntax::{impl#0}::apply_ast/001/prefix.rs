// Answer 0

#[test]
fn test_apply_ast_case_insensitive_true() {
    let config = Config::new()
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .crlf(false)
        .line_terminator(10)
        .swap_greed(false)
        .ignore_whitespace(true)
        .unicode(false)
        .utf8(true)
        .nest_limit(1000)
        .octal(false);

    let mut builder = ast::parse::ParserBuilder::new();
    config.apply_ast(&mut builder);
}

#[test]
fn test_apply_ast_case_insensitive_false() {
    let config = Config::new()
        .case_insensitive(false)
        .multi_line(true)
        .dot_matches_new_line(true)
        .crlf(true)
        .line_terminator(255)
        .swap_greed(true)
        .ignore_whitespace(false)
        .unicode(true)
        .utf8(false)
        .nest_limit(500)
        .octal(true);

    let mut builder = ast::parse::ParserBuilder::new();
    config.apply_ast(&mut builder);
}

#[test]
fn test_apply_ast_boundary_nest_limit_zero() {
    let config = Config::new()
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .crlf(false)
        .line_terminator(0)
        .swap_greed(false)
        .ignore_whitespace(true)
        .unicode(false)
        .utf8(true)
        .nest_limit(0)
        .octal(false);

    let mut builder = ast::parse::ParserBuilder::new();
    config.apply_ast(&mut builder);
}

#[test]
fn test_apply_ast_boundary_nest_limit_max() {
    let config = Config::new()
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .crlf(false)
        .line_terminator(1)
        .swap_greed(false)
        .ignore_whitespace(true)
        .unicode(false)
        .utf8(true)
        .nest_limit(1_000_000)
        .octal(false);

    let mut builder = ast::parse::ParserBuilder::new();
    config.apply_ast(&mut builder);
} 

#[test]
fn test_apply_ast_all_combination() {
    for case_insensitive in [true, false].iter() {
        for multi_line in [true, false].iter() {
            for dot_matches_new_line in [true, false].iter() {
                for crlf in [true, false].iter() {
                    for line_terminator in 0..=255 {
                        for swap_greed in [true, false].iter() {
                            for ignore_whitespace in [true, false].iter() {
                                for unicode in [true, false].iter() {
                                    for utf8 in [true, false].iter() {
                                        for nest_limit in [0, 1_000_000].iter() {
                                            for octal in [true, false].iter() {
                                                let config = Config::new()
                                                    .case_insensitive(*case_insensitive)
                                                    .multi_line(*multi_line)
                                                    .dot_matches_new_line(*dot_matches_new_line)
                                                    .crlf(*crlf)
                                                    .line_terminator(line_terminator)
                                                    .swap_greed(*swap_greed)
                                                    .ignore_whitespace(*ignore_whitespace)
                                                    .unicode(*unicode)
                                                    .utf8(*utf8)
                                                    .nest_limit(*nest_limit)
                                                    .octal(*octal);
    
                                                let mut builder = ast::parse::ParserBuilder::new();
                                                config.apply_ast(&mut builder);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

