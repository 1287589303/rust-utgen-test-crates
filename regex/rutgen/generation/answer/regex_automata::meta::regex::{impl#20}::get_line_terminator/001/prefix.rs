// Answer 0

#[test]
fn test_get_line_terminator_default() {
    let config = Config::new();
    let _ = config.get_line_terminator();
}

#[test]
fn test_get_line_terminator_set_to_zero() {
    let config = Config::new().line_terminator(0);
    let _ = config.get_line_terminator();
}

#[test]
fn test_get_line_terminator_set_to_ten() {
    let config = Config::new().line_terminator(10);
    let _ = config.get_line_terminator();
}

#[test]
fn test_get_line_terminator_set_to_one_hundred() {
    let config = Config::new().line_terminator(100);
    let _ = config.get_line_terminator();
}

#[test]
fn test_get_line_terminator_set_to_two_hundred_fifty_five() {
    let config = Config::new().line_terminator(255);
    let _ = config.get_line_terminator();
}

