// Answer 0

#[test]
fn test_get_match_kind_none() {
    let config = Config::new();
    let result = config.get_match_kind();
}

#[test]
fn test_get_match_kind_leftmost_first() {
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let result = config.get_match_kind();
}

#[test]
fn test_get_match_kind_all() {
    let config = Config::new().match_kind(MatchKind::All);
    let result = config.get_match_kind();
}

