// Answer 0

#[test]
fn test_idna_new_all_flags_true() {
    let config = Config {
        use_std3_ascii_rules: true,
        transitional_processing: true,
        verify_dns_length: true,
        check_hyphens: true,
    };
    let idna = Idna::new(config);
}

#[test]
fn test_idna_new_all_flags_false() {
    let config = Config {
        use_std3_ascii_rules: false,
        transitional_processing: false,
        verify_dns_length: false,
        check_hyphens: false,
    };
    let idna = Idna::new(config);
}

#[test]
fn test_idna_new_mixed_flags_1() {
    let config = Config {
        use_std3_ascii_rules: true,
        transitional_processing: false,
        verify_dns_length: true,
        check_hyphens: false,
    };
    let idna = Idna::new(config);
}

#[test]
fn test_idna_new_mixed_flags_2() {
    let config = Config {
        use_std3_ascii_rules: false,
        transitional_processing: true,
        verify_dns_length: false,
        check_hyphens: true,
    };
    let idna = Idna::new(config);
}

#[test]
fn test_idna_new_transitional_processing_only() {
    let config = Config {
        use_std3_ascii_rules: false,
        transitional_processing: true,
        verify_dns_length: false,
        check_hyphens: false,
    };
    let idna = Idna::new(config);
}

