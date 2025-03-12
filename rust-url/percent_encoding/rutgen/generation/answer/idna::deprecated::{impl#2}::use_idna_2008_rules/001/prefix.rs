// Answer 0

#[test]
#[should_panic]
fn test_use_idna_2008_rules_panics_with_true() {
    let config = Config {
        use_std3_ascii_rules: false,
        transitional_processing: false,
        verify_dns_length: false,
        check_hyphens: false,
    };
    let _ = config.use_idna_2008_rules(true);
}

