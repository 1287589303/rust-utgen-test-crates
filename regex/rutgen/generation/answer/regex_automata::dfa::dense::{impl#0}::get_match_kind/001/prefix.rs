// Answer 0

#[test]
fn test_get_match_kind_none() {
    let config = Config::new();
    let match_kind = config.get_match_kind();
}

#[test]
fn test_get_match_kind_all() {
    let config = Config::new().match_kind(MatchKind::All);
    let match_kind = config.get_match_kind();
}

#[test]
fn test_get_match_kind_leftmost_first() {
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let match_kind = config.get_match_kind();
}

