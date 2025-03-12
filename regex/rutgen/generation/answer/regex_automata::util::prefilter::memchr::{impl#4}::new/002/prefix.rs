// Answer 0

#[test]
fn test_new_valid_input() {
    let needles: [&[u8]; 3] = [&[b'a'], &[b'b'], &[b'c']];
    let result = Memchr3::new(MatchKind::All, &needles);
}

#[test]
fn test_new_valid_input_leftmost_first() {
    let needles: [&[u8]; 3] = [&[b'x'], &[b'y'], &[b'z']];
    let result = Memchr3::new(MatchKind::LeftmostFirst, &needles);
}

#[test]
fn test_new_valid_boundary_case() {
    let needles: [&[u8]; 3] = [&[b'A'], &[b'B'], &[b'C']];
    let result = Memchr3::new(MatchKind::All, &needles);
}

