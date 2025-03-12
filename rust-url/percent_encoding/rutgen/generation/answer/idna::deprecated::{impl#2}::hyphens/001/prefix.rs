// Answer 0

#[test]
fn test_hyphens_check_first_last() {
    let config = Config {
        use_std3_ascii_rules: false,
        transitional_processing: false,
        verify_dns_length: false,
        check_hyphens: true,
    };
    let result = config.hyphens();
}

#[test]
fn test_hyphens_check_first_last_with_other_flags() {
    let config = Config {
        use_std3_ascii_rules: true,
        transitional_processing: true,
        verify_dns_length: true,
        check_hyphens: true,
    };
    let result = config.hyphens();
}

