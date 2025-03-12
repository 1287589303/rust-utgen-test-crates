// Answer 0

#[test]
fn test_start_crlf_at_start() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"\n";
    let at = 0;
    look.is_match(haystack, at);
}

#[test]
fn test_start_crlf_with_newline_before() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"\r\n";
    let at = 1; // Position right after '\r'
    look.is_match(haystack, at);
}

#[test]
fn test_start_crlf_with_carriage_return_before() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"\r";
    let at = 1; // Position after '\r'
    look.is_match(haystack, at);
}

#[test]
fn test_start_crlf_with_valid_new_line() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"test\r\ntest";
    let at = 4; // Position right after '\r'
    look.is_match(haystack, at);
}

#[test]
fn test_start_crlf_with_valid_carriage_return() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"test\rtest";
    let at = 4; // Position right after '\r'
    look.is_match(haystack, at);
}

