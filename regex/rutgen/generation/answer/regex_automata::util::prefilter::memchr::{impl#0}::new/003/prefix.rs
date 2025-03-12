// Answer 0

#[test]
fn test_memchr_new_valid_input() {
    let needles: &[&[u8; 1]] = &[b"A"];
    let result = Memchr::new(MatchKind::All, needles);
}

#[test]
fn test_memchr_new_valid_input_leftmost_first() {
    let needles: &[&[u8; 1]] = &[b"B"];
    let result = Memchr::new(MatchKind::LeftmostFirst, needles);
}

