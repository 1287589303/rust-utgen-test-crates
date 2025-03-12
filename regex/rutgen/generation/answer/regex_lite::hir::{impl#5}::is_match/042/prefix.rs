// Answer 0

#[test]
fn test_lookup_startlf_at_zero() {
    let look = Look::StartLF;
    let haystack: &[u8] = b"\nHello, World!";
    let at = 0;
    let _ = look.is_match(haystack, at);
}

#[test]
fn test_lookup_startlf_at_one() {
    let look = Look::StartLF;
    let haystack: &[u8] = b"\nHello, World!";
    let at = 1;
    let _ = look.is_match(haystack, at);
}

#[test]
fn test_lookup_startlf_with_empty_haystack() {
    let look = Look::StartLF;
    let haystack: &[u8] = b"";
    let at = 0;
    let _ = look.is_match(haystack, at);
}

