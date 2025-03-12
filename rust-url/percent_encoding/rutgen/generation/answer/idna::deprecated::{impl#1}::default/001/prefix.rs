// Answer 0

#[test]
fn test_default_config() {
    let config = Config::default();
}

#[test]
fn test_default_config_values() {
    let config = Config::default();
    let expected = Config {
        use_std3_ascii_rules: false,
        transitional_processing: false,
        check_hyphens: false,
        verify_dns_length: false,
    };
}

