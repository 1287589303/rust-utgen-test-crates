// Answer 0

#[test]
fn test_to_ascii_valid_ascii_pасsthrough() {
    let mut out = String::new();
    let config = Config::default()
        .use_std3_ascii_rules(true)
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(false);
    let mut idna = Idna::new(config);
    
    let result = idna.to_ascii("example.com", &mut out);
}

#[test]
fn test_to_ascii_valid_ascii_wrote_to_sink() {
    let mut out = String::new();
    let config = Config::default()
        .use_std3_ascii_rules(true)
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(false);
    let mut idna = Idna::new(config);
    
    let result = idna.to_ascii("my-domain.com", &mut out);
}

#[test]
fn test_to_ascii_single_character() {
    let mut out = String::new();
    let config = Config::default()
        .use_std3_ascii_rules(true)
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(false);
    let mut idna = Idna::new(config);
    
    let result = idna.to_ascii("a", &mut out);
}

#[test]
fn test_to_ascii_multiple_labels() {
    let mut out = String::new();
    let config = Config::default()
        .use_std3_ascii_rules(true)
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(false);
    let mut idna = Idna::new(config);
    
    let result = idna.to_ascii("example.label.com", &mut out);
}

#[test]
fn test_to_ascii_long_valid_input() {
    let mut out = String::new();
    let config = Config::default()
        .use_std3_ascii_rules(true)
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(false);
    let mut idna = Idna::new(config);
    
    let result = idna.to_ascii("a-very-long-valid-domain-name.com", &mut out);
}

#[test]
fn test_to_ascii_short_label_exceeding_hyphens() {
    let mut out = String::new();
    let config = Config::default()
        .use_std3_ascii_rules(true)
        .transitional_processing(true)
        .verify_dns_length(true)
        .check_hyphens(true);
    let mut idna = Idna::new(config);
    
    let result = idna.to_ascii("example-.com", &mut out);
}

