// Answer 0

#[test]
fn test_http_whitespace_non_whitespace_char() {
    let c = 'a';
    http_whitespace(c);
}

#[test]
fn test_http_whitespace_special_character() {
    let c = '$';
    http_whitespace(c);
}

#[test]
fn test_http_whitespace_digit() {
    let c = '1';
    http_whitespace(c);
}

#[test]
fn test_http_whitespace_punctuation() {
    let c = '!';
    http_whitespace(c);
}

