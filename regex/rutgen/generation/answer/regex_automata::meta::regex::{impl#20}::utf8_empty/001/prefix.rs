// Answer 0

#[test]
fn test_utf8_empty_true() {
    let config = Config::new();
    let new_config = config.utf8_empty(true);
}

#[test]
fn test_utf8_empty_false() {
    let config = Config::new();
    let new_config = config.utf8_empty(false);
}

