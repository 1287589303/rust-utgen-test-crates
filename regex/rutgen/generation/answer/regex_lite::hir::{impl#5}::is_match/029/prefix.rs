// Answer 0

#[test]
fn test_end_crlf_at_end_haystack_with_newline() {
    let haystack = b"Hello, world!\n";
    let at = haystack.len();
    let look = Look::EndCRLF;
    let _ = look.is_match(haystack, at);
}

#[test]
fn test_end_crlf_at_end_haystack_with_carriage_return() {
    let haystack = b"Hello, world!\r";
    let at = haystack.len();
    let look = Look::EndCRLF;
    let _ = look.is_match(haystack, at);
}

#[test]
fn test_end_crlf_at_end_haystack_with_carriage_return_newline() {
    let haystack = b"Hello, world!\r\n";
    let at = haystack.len();
    let look = Look::EndCRLF;
    let _ = look.is_match(haystack, at);
}

#[test]
fn test_end_crlf_at_end_empty_haystack() {
    let haystack: &[u8] = b"";
    let at = haystack.len();
    let look = Look::EndCRLF;
    let _ = look.is_match(haystack, at);
}

