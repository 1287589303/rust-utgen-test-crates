// Answer 0

#[test]
fn test_continue_past_first_match_all() {
    let match_kind = MatchKind::All;
    let result = match_kind.continue_past_first_match();
}

#[test]
fn test_continue_past_first_match_leftmost_first() {
    let match_kind = MatchKind::LeftmostFirst;
    let result = match_kind.continue_past_first_match();
}

