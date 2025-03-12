fn default() -> Self {
        Config {
            use_std3_ascii_rules: false,
            transitional_processing: false,
            check_hyphens: false,
            // Only use for to_ascii, not to_unicode
            verify_dns_length: false,
        }
    }