// Answer 0

#[test]
fn test_default_port_empty_string() {
    let result = default_port("");
}

#[test]
fn test_default_port_unknown_scheme() {
    let result = default_port("unknown");
}

#[test]
fn test_default_port_non_http_protocol() {
    let result = default_port("smtp");
}

#[test]
fn test_default_port_unsupported_protocol() {
    let result = default_port("telnet");
}

#[test]
fn test_default_port_long_string() {
    let result = default_port("this_is_a_really_long_scheme_name");
}

#[test]
fn test_default_port_special_characters() {
    let result = default_port("http://example.com");
}

