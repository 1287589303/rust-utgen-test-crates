// Answer 0

#[test]
fn test_get_utf8_empty_some_true() {
    let config = Config::new().utf8_empty(true);
    config.get_utf8_empty();
}

#[test]
fn test_get_utf8_empty_some_false() {
    let config = Config::new().utf8_empty(false);
    config.get_utf8_empty();
}

#[test]
fn test_get_utf8_empty_none() {
    let config = Config::new().utf8_empty(None);
    config.get_utf8_empty();
}

