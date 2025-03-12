// Answer 0

#[test]
fn test_memchr3_new_invalid_length() {
    let needles = vec![b"a", b"b"];
    let kind = MatchKind::All;
    let result = Memchr3::new(kind, &needles);
}

#[test]
fn test_memchr3_new_invalid_element_length() {
    let needles = vec![b"a", b"b", b"cd"];
    let kind = MatchKind::All;
    let result = Memchr3::new(kind, &needles);
}

