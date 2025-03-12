// Answer 0

#[test]
fn test_find_cap_ref_braced_invalid_utf8() {
    let rep: &[u8] = b"{\xFF"; // Invalid UTF-8 content
    let i = 2; // i points to the character after '{'
    assert_eq!(find_cap_ref_braced(rep, i), None);
}

#[test]
fn test_find_cap_ref_braced_invalid_utf8_multiple_bytes() {
    let rep: &[u8] = b"{\xFF\xFE\xFD"; // Invalid UTF-8 content
    let i = 4; // i points to the character after '{'
    assert_eq!(find_cap_ref_braced(rep, i), None);
}

#[test]
fn test_find_cap_ref_braced_invalid_utf8_surrogate() {
    let rep: &[u8] = b"{\xED\xA0\x80"; // Invalid UTF-8 content (surrogate pair)
    let i = 3; // i points to the character after '{'
    assert_eq!(find_cap_ref_braced(rep, i), None);
}

