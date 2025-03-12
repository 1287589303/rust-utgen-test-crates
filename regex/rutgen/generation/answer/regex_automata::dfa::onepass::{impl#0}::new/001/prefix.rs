// Answer 0

#[test]
fn test_config_new_default_fields() {
    let config = Config::new();
}

#[test]
fn test_config_new_match_kind_default() {
    let config = Config::new();
    let match_kind = config.get_match_kind();
}

#[test]
fn test_config_new_starts_for_each_pattern_default() {
    let config = Config::new();
    let starts_for_each_pattern = config.get_starts_for_each_pattern();
}

#[test]
fn test_config_new_byte_classes_default() {
    let config = Config::new();
    let byte_classes = config.get_byte_classes();
}

#[test]
fn test_config_new_size_limit_default() {
    let config = Config::new();
    let size_limit = config.get_size_limit();
}

