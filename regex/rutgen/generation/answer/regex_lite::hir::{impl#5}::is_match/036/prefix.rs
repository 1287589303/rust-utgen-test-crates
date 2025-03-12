// Answer 0

#[test]
fn test_start_crlf_case_with_empty_haystack() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"";
    let at = 0;
    let _ = look.is_match(haystack, at);
}

#[test]
fn test_start_crlf_case_with_haystack_end() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"abc\n";
    let at = haystack.len();
    let _ = look.is_match(haystack, at);
}

#[test]
fn test_start_crlf_case_with_haystack_non_newline() {
    let look = Look::StartCRLF;
    let haystack: &[u8] = b"abc";
    let at = haystack.len();
    let _ = look.is_match(haystack, at);
}

