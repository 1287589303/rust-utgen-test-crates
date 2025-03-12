// Answer 0

#[test]
fn test_get_starts_for_each_pattern_some_true() {
    let config = Config::new().starts_for_each_pattern(true);
    let _ = config.get_starts_for_each_pattern();
}

#[test]
fn test_get_starts_for_each_pattern_some_false() {
    let config = Config::new().starts_for_each_pattern(false);
    let _ = config.get_starts_for_each_pattern();
}

#[test]
fn test_get_starts_for_each_pattern_none() {
    let config = Config::new();
    let _ = config.get_starts_for_each_pattern();
}

