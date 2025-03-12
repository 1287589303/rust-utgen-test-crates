// Answer 0

#[test]
fn test_domain_to_ascii_strict_valid() {
    let domain = "example.com";
    let _ = domain_to_ascii_strict(domain);
}

#[test]
fn test_domain_to_ascii_strict_valid_with_hyphens() {
    let domain = "sub-domain.example.com";
    let _ = domain_to_ascii_strict(domain);
}

#[test]
fn test_domain_to_ascii_strict_empty_string() {
    let domain = "";
    let _ = domain_to_ascii_strict(domain);
}

#[test]
fn test_domain_to_ascii_strict_max_length() {
    let domain = "a".repeat(253); // creates a domain consisting of 253 'a's
    let _ = domain_to_ascii_strict(&domain);
}

#[test]
fn test_domain_to_ascii_strict_invalid_hyphen_first() {
    let domain = "-invalid.com";
    let _ = domain_to_ascii_strict(domain);
}

#[test]
fn test_domain_to_ascii_strict_invalid_hyphen_last() {
    let domain = "invalid-.com";
    let _ = domain_to_ascii_strict(domain);
}

#[test]
fn test_domain_to_ascii_strict_invalid_hyphen_third() {
    let domain = "in-valid-example.com";
    let _ = domain_to_ascii_strict(domain);
}

#[test]
fn test_domain_to_ascii_strict_invalid_hyphen_fourth() {
    let domain = "inva-lid-.com";
    let _ = domain_to_ascii_strict(domain);
}

#[test]
fn test_domain_to_ascii_strict_invalid_characters() {
    let domain = "invalid@domain.com";
    let _ = domain_to_ascii_strict(domain);
}

