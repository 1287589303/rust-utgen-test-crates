// Answer 0

#[test]
fn test_is_match_end_crlf_case_1() {
    let look = Look::EndCRLF;
    let haystack = b"Hello World\r";
    let at = haystack.len() - 1;
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_end_crlf_case_2() {
    let look = Look::EndCRLF;
    let haystack = b"Hello\r";
    let at = haystack.len() - 1;
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_end_crlf_case_3() {
    let look = Look::EndCRLF;
    let haystack = b"A\r";
    let at = haystack.len() - 1;
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_end_crlf_case_4() {
    let look = Look::EndCRLF;
    let haystack = b"SingleLine\r";
    let at = haystack.len() - 1;
    look.is_match(haystack, at);
}

