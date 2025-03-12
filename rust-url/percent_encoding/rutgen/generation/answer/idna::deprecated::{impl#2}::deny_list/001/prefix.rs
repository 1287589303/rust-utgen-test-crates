// Answer 0

#[test]
fn test_deny_list_use_std3_ascii_rules_true() {
    let config = Config {
        use_std3_ascii_rules: true,
        transitional_processing: false,
        verify_dns_length: false,
        check_hyphens: false,
    };
    let result = config.deny_list();
}

#[test]
fn test_deny_list_use_std3_ascii_rules_false() {
    let config = Config {
        use_std3_ascii_rules: false,
        transitional_processing: false,
        verify_dns_length: false,
        check_hyphens: false,
    };
    let result = config.deny_list();
}

