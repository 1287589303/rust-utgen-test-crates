// Answer 0

#[test]
fn test_start_crlf_at_zero_empty_haystack() {
    let look = Look::StartCRLF;
    let haystack: Vec<u8> = vec![];
    let at = 0;
    let _ = look.is_match(&haystack, at);
}

#[test]
fn test_start_crlf_at_zero_haystack_with_newline() {
    let look = Look::StartCRLF;
    let haystack: Vec<u8> = vec![b'\n'];
    let at = 0;
    let _ = look.is_match(&haystack, at);
}

#[test]
fn test_start_crlf_at_zero_haystack_with_carriage_return() {
    let look = Look::StartCRLF;
    let haystack: Vec<u8> = vec![b'\r'];
    let at = 0;
    let _ = look.is_match(&haystack, at);
}

#[test]
fn test_start_crlf_at_zero_haystack_with_mixed_line_ending() {
    let look = Look::StartCRLF;
    let haystack: Vec<u8> = vec![b'\r', b'\n'];
    let at = 0;
    let _ = look.is_match(&haystack, at);
}

#[test]
fn test_start_crlf_at_zero_haystack_with_mixed_content() {
    let look = Look::StartCRLF;
    let haystack: Vec<u8> = vec![b'a', b'\r', b'b'];
    let at = 0;
    let _ = look.is_match(&haystack, at);
}

