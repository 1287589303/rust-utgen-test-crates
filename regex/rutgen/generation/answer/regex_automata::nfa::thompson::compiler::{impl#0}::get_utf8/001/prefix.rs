// Answer 0

#[test]
fn test_get_utf8_enabled() {
    let config = Config::new().utf8(true);
    let result = config.get_utf8();
}

#[test]
fn test_get_utf8_disabled() {
    let config = Config::new().utf8(false);
    let result = config.get_utf8();
}

#[test]
fn test_get_utf8_default() {
    let config = Config::new();
    let result = config.get_utf8();
}

