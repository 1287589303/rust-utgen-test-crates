// Answer 0

#[test]
fn test_new_needles_length_zero() {
    let result = Memchr::new(MatchKind::All, &[]);
}

#[test]
fn test_new_needles_length_two() {
    let needles: [&[u8]; 2] = [&[1], &[2]];
    let result = Memchr::new(MatchKind::LeftmostFirst, &needles);
}

#[test]
fn test_new_needles_length_three() {
    let needles: [&[u8]; 3] = [&[1], &[2], &[3]];
    let result = Memchr::new(MatchKind::All, &needles);
}

