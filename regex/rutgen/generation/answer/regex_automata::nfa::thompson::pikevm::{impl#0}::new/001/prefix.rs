// Answer 0

#[test]
fn test_config_new() {
    let config = Config::new();
}

#[test]
fn test_config_default_values() {
    let config = Config::new();
    assert_eq!(config.match_kind, None);
    assert_eq!(config.pre, None);
}

