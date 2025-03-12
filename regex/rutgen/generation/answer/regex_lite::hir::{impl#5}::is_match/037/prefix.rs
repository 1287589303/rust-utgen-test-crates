// Answer 0

#[test]
fn test_start_crlf() {
    struct TestLook(Look);
    
    let look = TestLook(Look::StartCRLF);
    let haystack: Vec<u8> = vec![b'A', b'B', b'C', b'\n'];
    let at: usize = 0;
    
    let _ = look.0.is_match(&haystack, at);
}

#[test]
fn test_start_crlf_non_newline() {
    struct TestLook(Look);
    
    let look = TestLook(Look::StartCRLF);
    let haystack: Vec<u8> = vec![b'A', b'B', b'C', b'\n'];
    let at: usize = 0;

    let _ = look.0.is_match(&haystack, at);
}

