// Answer 0

#[test]
fn test_check_hyphens_true() {
    let config = Config { 
        use_std3_ascii_rules: false, 
        transitional_processing: false, 
        verify_dns_length: false, 
        check_hyphens: false 
    };
    let result = config.check_hyphens(true);
}

#[test]
fn test_check_hyphens_false() {
    let config = Config { 
        use_std3_ascii_rules: false, 
        transitional_processing: false, 
        verify_dns_length: false, 
        check_hyphens: false 
    };
    let result = config.check_hyphens(false);
}

