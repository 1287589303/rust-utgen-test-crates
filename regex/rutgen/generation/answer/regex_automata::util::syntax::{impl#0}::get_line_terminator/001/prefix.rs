// Answer 0

#[test]
fn test_get_line_terminator_zero() {
    let config = Config::new().line_terminator(0);
    config.get_line_terminator();
}

#[test]
fn test_get_line_terminator_mid() {
    let config = Config::new().line_terminator(128);
    config.get_line_terminator();
}

#[test]
fn test_get_line_terminator_max() {
    let config = Config::new().line_terminator(255);
    config.get_line_terminator();
}

