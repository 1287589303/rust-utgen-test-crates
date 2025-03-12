// Answer 0

#[test]
fn test_match_kind_all() {
    let config = Config::new();
    let updated_config = config.match_kind(MatchKind::All);
}

#[test]
fn test_match_kind_leftmost_first() {
    let config = Config::new();
    let updated_config = config.match_kind(MatchKind::LeftmostFirst);
}

