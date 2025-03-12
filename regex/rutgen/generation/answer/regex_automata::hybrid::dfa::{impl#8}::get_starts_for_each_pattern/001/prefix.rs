// Answer 0

#[test]
fn test_get_starts_for_each_pattern_none() {
    let config = Config::default();
    let result = config.get_starts_for_each_pattern();
}

#[test]
fn test_get_starts_for_each_pattern_true() {
    let config = Config::default().starts_for_each_pattern(true);
    let result = config.get_starts_for_each_pattern();
}

#[test]
fn test_get_starts_for_each_pattern_false() {
    let config = Config::default().starts_for_each_pattern(false);
    let result = config.get_starts_for_each_pattern();
}

