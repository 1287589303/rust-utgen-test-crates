// Answer 0

#[test]
fn test_http_whitespace_space() {
    let result = http_whitespace(' ');
}

#[test]
fn test_http_whitespace_tab() {
    let result = http_whitespace('\t');
}

#[test]
fn test_http_whitespace_newline() {
    let result = http_whitespace('\n');
}

#[test]
fn test_http_whitespace_carriage_return() {
    let result = http_whitespace('\r');
}

