// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    let config = Config::new().ignore_whitespace(true);
}

#[test]
fn test_ignore_whitespace_false() {
    let config = Config::new().ignore_whitespace(false);
}

