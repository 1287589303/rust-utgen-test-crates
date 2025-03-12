// Answer 0

#[test]
fn test_is_match_start_crlf_with_at_1() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"\r\n";
    let at = 1;
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_start_crlf_with_at_2() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"\r\n";
    let at = 2;
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_start_crlf_with_at_1_non_newline() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"abc";
    let at = 1;
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_start_crlf_with_at_3_non_newline() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"abc";
    let at = 3;
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_start_crlf_with_at_4() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"\r\nabc";
    let at = 4;
    look.is_match(haystack, at);
}

