// Answer 0

#[test]
fn test_end_crlf_match_case_1() {
    let look = Look::EndCRLF;
    let haystack = b"Hello\r";
    let at = 5; // at == haystack.len()
    look.is_match(haystack, at);
}

#[test]
fn test_end_crlf_match_case_2() {
    let look = Look::EndCRLF;
    let haystack = b"\r\n";
    let at = 2; // at == haystack.len()
    look.is_match(haystack, at);
}

#[test]
fn test_end_crlf_match_case_3() {
    let look = Look::EndCRLF;
    let haystack = b"Hello\r\n";
    let at = 6; // at == haystack.len()
    look.is_match(haystack, at);
}

#[test]
fn test_end_crlf_match_false_case() {
    let look = Look::EndCRLF;
    let haystack = b"Hello\n";
    let at = 5; // at == haystack.len() is false
    look.is_match(haystack, at);
}

