// Answer 0

#[test]
fn test_unicode_enable() {
    let config = Config::new();
    let result = config.unicode(true);
}

#[test]
fn test_unicode_disable() {
    let config = Config::new();
    let result = config.unicode(false);
}

