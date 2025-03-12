// Answer 0

#[test]
fn test_get_match_kind_default() {
    let config = Config::default();
    let _result = config.get_match_kind();
}

#[test]
fn test_get_match_kind_all() {
    let config = Config::new().match_kind(MatchKind::All);
    let _result = config.get_match_kind();
}

#[test]
fn test_get_match_kind_leftmost_first() {
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let _result = config.get_match_kind();
}

