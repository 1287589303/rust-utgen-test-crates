// Answer 0

#[test]
fn test_is_match_end_crlf_1() {
    let look = Look::EndCRLF;
    let haystack = vec![b'a', b'\n'];
    let at = 1;
    look.is_match(&haystack, at);
}

#[test]
fn test_is_match_end_crlf_2() {
    let look = Look::EndCRLF;
    let haystack = vec![b'\r', b'\n'];
    let at = 1;
    look.is_match(&haystack, at);
}

#[test]
fn test_is_match_end_crlf_3() {
    let look = Look::EndCRLF;
    let haystack = vec![b'b', b'\n'];
    let at = 1;
    look.is_match(&haystack, at);
}

