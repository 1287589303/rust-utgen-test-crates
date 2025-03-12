// Answer 0

#[test]
fn test_match_kind_leftmost_first() {
    let config = Config::new()
        .match_kind(MatchKind::LeftmostFirst);
}

#[test]
fn test_match_kind_all() {
    let config = Config::new()
        .match_kind(MatchKind::All);
}

#[test]
fn test_match_kind_chained() {
    let config = Config::new()
        .match_kind(MatchKind::LeftmostFirst)
        .match_kind(MatchKind::All);
}

