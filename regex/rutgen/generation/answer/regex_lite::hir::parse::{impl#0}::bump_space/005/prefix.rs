// Answer 0

#[test]
fn test_bump_space_with_ignore_whitespace_enabled() {
    let flags = Flags { ignore_whitespace: true, ..Flags::default() };
    let config = Config { flags, ..Config::default() };
    let pattern = "non-whitespace\n";
    let parser = Parser::new(config, pattern);

    parser.bump_space();
}

#[test]
fn test_bump_space_with_ignore_whitespace_enabled_and_multiple_lines() {
    let flags = Flags { ignore_whitespace: true, ..Flags::default() };
    let config = Config { flags, ..Config::default() };
    let pattern = "line 1\n# comment\nline 2\n";
    let parser = Parser::new(config, pattern);

    parser.bump_space();
}

#[test]
fn test_bump_space_with_ignore_whitespace_enabled_followed_by_other_characters() {
    let flags = Flags { ignore_whitespace: true, ..Flags::default() };
    let config = Config { flags, ..Config::default() };
    let pattern = "xyz\nother\n";
    let parser = Parser::new(config, pattern);

    parser.bump_space();
}

#[test]
fn test_bump_space_with_ignore_whitespace_enabled_and_newline() {
    let flags = Flags { ignore_whitespace: true, ..Flags::default() };
    let config = Config { flags, ..Config::default() };
    let pattern = "just a line\nwith newline\n";
    let parser = Parser::new(config, pattern);

    parser.bump_space();
}

#[test]
fn test_bump_space_with_ignore_whitespace_enabled_and_no_comments() {
    let flags = Flags { ignore_whitespace: true, ..Flags::default() };
    let config = Config { flags, ..Config::default() };
    let pattern = "abc\n123\n";
    let parser = Parser::new(config, pattern);

    parser.bump_space();
}

