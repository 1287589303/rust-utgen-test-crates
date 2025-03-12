// Answer 0

#[test]
fn test_end_crlf_match_with_front_boundary() {
    let look = Look::EndCRLF;
    let haystack: &[u8] = b"Hello\nWorld";
    let at: usize = 9; // Position just after "Hello\nWo"
    look.is_match(haystack, at);
}

#[test]
fn test_end_crlf_match_with_back_boundary() {
    let look = Look::EndCRLF;
    let haystack: &[u8] = b"Hello\nWorld";
    let at: usize = 10; // Position at the end of "Hello\nWorld"
    look.is_match(haystack, at);
}

#[test]
fn test_end_crlf_no_match_with_newline() {
    let look = Look::EndCRLF;
    let haystack: &[u8] = b"Hello\r\nWorld";
    let at: usize = 7; // Position at "\r" in "Hello\r\nWorld"
    look.is_match(haystack, at);
}

#[test]
fn test_end_crlf_no_match_with_non_boundary() {
    let look = Look::EndCRLF;
    let haystack: &[u8] = b"HelloWorld";
    let at: usize = 5; // Position in the middle of "Hello"
    look.is_match(haystack, at);
}

#[test]
fn test_end_crlf_no_match_with_invalid_before() {
    let look = Look::EndCRLF;
    let haystack: &[u8] = b"Hello\n";
    let at: usize = 5; // Position after "Hello\n"
    look.is_match(haystack, at);
}

