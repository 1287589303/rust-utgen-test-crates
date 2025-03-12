// Answer 0

#[test]
fn test_is_match_endlf_true() {
    let look = Look::EndLF;
    let haystack: Vec<u8> = b"Hello\nWorld".to_vec();
    let at = haystack.len() - 1;
    let _ = look.is_match(&haystack, at);
}

#[test]
fn test_is_match_endlf_false() {
    let look = Look::EndLF;
    let haystack: Vec<u8> = b"HelloWorld".to_vec();
    let at = haystack.len() - 1; // Since haystack doesn't end with `\n`, expect false
    let _ = look.is_match(&haystack, at);
}

