// Answer 0

#[test]
fn test_get_captures_all() {
    let config = Config::new()
        .which_captures(WhichCaptures::All);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_implicit() {
    let config = Config::new()
        .which_captures(WhichCaptures::Implicit);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_none() {
    let config = Config::new()
        .which_captures(WhichCaptures::None);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_with_utf8() {
    let config = Config::new()
        .utf8(true)
        .which_captures(WhichCaptures::All);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_without_utf8() {
    let config = Config::new()
        .utf8(false)
        .which_captures(WhichCaptures::All);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_with_reverse() {
    let config = Config::new()
        .reverse(true)
        .which_captures(WhichCaptures::All);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_without_reverse() {
    let config = Config::new()
        .reverse(false)
        .which_captures(WhichCaptures::All);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_with_nfa_size_limit() {
    let config = Config::new()
        .nfa_size_limit(Some(1024))
        .which_captures(WhichCaptures::All);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_with_shrink() {
    let config = Config::new()
        .shrink(true)
        .which_captures(WhichCaptures::Implicit);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_without_shrink() {
    let config = Config::new()
        .shrink(false)
        .which_captures(WhichCaptures::Implicit);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_with_look_matcher() {
    let look_matcher = LookMatcher { lineterm: DebugByte::default() };
    let config = Config::new()
        .look_matcher(look_matcher)
        .which_captures(WhichCaptures::All);
    let _ = config.get_captures();
} 

#[test]
fn test_get_captures_with_unanchored_prefix() {
    let config = Config::new()
        .unanchored_prefix(true)
        .which_captures(WhichCaptures::All);
    let _ = config.get_captures();
}

#[test]
fn test_get_captures_without_unanchored_prefix() {
    let config = Config::new()
        .unanchored_prefix(false)
        .which_captures(WhichCaptures::All);
    let _ = config.get_captures();
}

