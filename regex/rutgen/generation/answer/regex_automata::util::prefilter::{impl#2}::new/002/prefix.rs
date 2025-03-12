// Answer 0

#[test]
fn test_choice_new_empty_needles() {
    let result = Choice::new(MatchKind::All, &[]);
}

#[test]
fn test_choice_new_needles_with_empty_string() {
    let needles = [""];
    let result = Choice::new(MatchKind::All, &needles);
}

