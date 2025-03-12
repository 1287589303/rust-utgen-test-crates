// Answer 0

#[test]
fn test_match_kind_all() {
    let mut config = Config::new();
    let kind = MatchKind::All;
    config.match_kind(kind);
}

#[test]
fn test_match_kind_leftmost_first() {
    let mut config = Config::new();
    let kind = MatchKind::LeftmostFirst;
    config.match_kind(kind);
}

