// Answer 0

#[test]
fn test_memchr2_new_valid_input() {
    let kind = MatchKind::All;
    let needles: [&[u8]; 2] = [&[b'a'], &[b'b']];
    let result = Memchr2::new(kind, &needles);
}

#[test]
fn test_memchr2_new_valid_input_alternate() {
    let kind = MatchKind::LeftmostFirst;
    let needles: [&[u8]; 2] = [&[b'c'], &[b'd']];
    let result = Memchr2::new(kind, &needles);
}

