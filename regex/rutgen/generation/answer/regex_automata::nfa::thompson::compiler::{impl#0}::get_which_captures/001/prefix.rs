// Answer 0

#[test]
fn test_get_which_captures_none() {
    let config = Config::new();
    assert_eq!(config.get_which_captures(), WhichCaptures::All);
}

#[test]
fn test_get_which_captures_implicit() {
    let config = Config::new().which_captures(WhichCaptures::Implicit);
    assert_eq!(config.get_which_captures(), WhichCaptures::Implicit);
}

#[test]
fn test_get_which_captures_all() {
    let config = Config::new().which_captures(WhichCaptures::All);
    assert_eq!(config.get_which_captures(), WhichCaptures::All);
}

#[test]
fn test_get_which_captures_none_explicit() {
    let config = Config::new().which_captures(WhichCaptures::None);
    assert_eq!(config.get_which_captures(), WhichCaptures::None);
}

#[test]
fn test_get_which_captures_with_utf8() {
    let config = Config::new().utf8(true).which_captures(WhichCaptures::Implicit);
    assert_eq!(config.get_which_captures(), WhichCaptures::Implicit);
}

#[test]
fn test_get_which_captures_with_reverse() {
    let config = Config::new().reverse(true).which_captures(WhichCaptures::All);
    assert_eq!(config.get_which_captures(), WhichCaptures::All);
}

#[test]
fn test_get_which_captures_with_nfa_size_limit() {
    let config = Config::new().nfa_size_limit(Some(1)).which_captures(WhichCaptures::None);
    assert_eq!(config.get_which_captures(), WhichCaptures::None);
}

#[test]
fn test_get_which_captures_with_shrink() {
    let config = Config::new().shrink(true).which_captures(WhichCaptures::Implicit);
    assert_eq!(config.get_which_captures(), WhichCaptures::Implicit);
}

#[test]
fn test_get_which_captures_with_unanchored_prefix() {
    let config = Config::new().unanchored_prefix(true).which_captures(WhichCaptures::All);
    assert_eq!(config.get_which_captures(), WhichCaptures::All);
}

