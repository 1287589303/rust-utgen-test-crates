// Answer 0

#[test]
fn test_match_kind_all() {
    let config = Config::new().match_kind(MatchKind::All);
    let _result = config;
}

#[test]
fn test_match_kind_leftmost_first() {
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let _result = config;
}

