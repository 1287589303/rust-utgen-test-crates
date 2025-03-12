// Answer 0

#[test]
fn test_get_multi_line_true() {
    let config = Config::new().multi_line(true);
    let result = config.get_multi_line();
}

#[test]
fn test_get_multi_line_false() {
    let config = Config::new().multi_line(false);
    let result = config.get_multi_line();
}

