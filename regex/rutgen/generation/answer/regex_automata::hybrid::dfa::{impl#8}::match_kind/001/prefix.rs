// Answer 0

#[test]
fn test_match_kind_all() {
    let config = Config::new().match_kind(MatchKind::All);
}

#[test]
fn test_match_kind_leftmost_first() {
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
}

#[test]
fn test_match_kind_multiple_calls() {
    let mut config = Config::new();
    config = config.match_kind(MatchKind::All);
    config = config.match_kind(MatchKind::LeftmostFirst);
}

