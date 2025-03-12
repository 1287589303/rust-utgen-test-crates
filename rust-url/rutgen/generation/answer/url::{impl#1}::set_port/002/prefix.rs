// Answer 0

#[test]
fn test_set_port_error_due_to_file_scheme() {
    let mut url = Url::parse("file://example.com/").unwrap();
    let result = url.set_port(Some(80));
}

#[test]
fn test_set_port_none_error_due_to_file_scheme() {
    let mut url = Url::parse("file://example.com/").unwrap();
    let result = url.set_port(None);
}

#[test]
fn test_set_port_error_due_to_file_scheme_with_initial_port() {
    let mut url = Url::parse("file://example.com:8080/").unwrap();
    let result = url.set_port(Some(80));
}

