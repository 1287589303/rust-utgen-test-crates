// Answer 0

#[test]
fn test_set_scheme_valid_transition() {
    let mut url = Url::parse("custom://example.com/path?query#fragment").unwrap();
    url.set_scheme("another").unwrap();
}

#[test]
fn test_set_scheme_valid_transition_2() {
    let mut url = Url::parse("ftp://example.com/path?query#fragment").unwrap();
    url.set_scheme("gopher").unwrap();
}

#[test]
fn test_set_scheme_valid_transition_multiple() {
    let mut url = Url::parse("mailto:user@example.com").unwrap();
    url.set_scheme("news").unwrap();
}

#[test]
fn test_set_scheme_change_scheme() {
    let mut url = Url::parse("smtp://example.com").unwrap();
    url.set_scheme("pop").unwrap();
} 

#[test]
fn test_set_scheme_valid_edge_case() {
    let mut url = Url::parse("custom://example.com").unwrap();
    url.set_scheme("custom").unwrap();
} 

#[test]
fn test_set_scheme_non_special_to_non_special() {
    let mut url = Url::parse("market://example.com/path?query#fragment").unwrap();
    url.set_scheme("content").unwrap();
}

