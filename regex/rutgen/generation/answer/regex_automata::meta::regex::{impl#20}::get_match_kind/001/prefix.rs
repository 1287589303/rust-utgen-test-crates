// Answer 0

#[test]
fn test_get_match_kind_all() {
    let config = Config::new().match_kind(MatchKind::All);
    let _ = config.get_match_kind();
}

#[test]
fn test_get_match_kind_leftmost_first() {
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let _ = config.get_match_kind();
}

#[test]
fn test_get_match_kind_default() {
    let config = Config::new();
    let _ = config.get_match_kind();
}

