// Answer 0

#[test]
fn test_get_dot_matches_new_line_true() {
    let config = Config::new()
        .dot_matches_new_line(true);
    let result = config.get_dot_matches_new_line();
}

#[test]
fn test_get_dot_matches_new_line_false() {
    let config = Config::new()
        .dot_matches_new_line(false);
    let result = config.get_dot_matches_new_line();
}

