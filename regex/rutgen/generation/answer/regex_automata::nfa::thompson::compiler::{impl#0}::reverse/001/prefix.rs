// Answer 0

#[test]
fn test_config_reverse_true() {
    let config = Config::new()
        .reverse(true);
}

#[test]
fn test_config_reverse_false() {
    let config = Config::new()
        .reverse(false);
}

#[test]
fn test_config_nfa_size_limit_none() {
    let config = Config::new()
        .nfa_size_limit(None);
}

#[test]
fn test_config_nfa_size_limit_zero() {
    let config = Config::new()
        .nfa_size_limit(Some(0));
}

#[test]
fn test_config_nfa_size_limit_one() {
    let config = Config::new()
        .nfa_size_limit(Some(1));
}

#[test]
fn test_config_nfa_size_limit_max() {
    let config = Config::new()
        .nfa_size_limit(Some(usize::MAX));
}

#[test]
fn test_config_shrink_true() {
    let config = Config::new()
        .shrink(true);
}

#[test]
fn test_config_shrink_false() {
    let config = Config::new()
        .shrink(false);
}

#[test]
fn test_config_which_captures_all() {
    let config = Config::new()
        .which_captures(WhichCaptures::All);
}

#[test]
fn test_config_which_captures_implicit() {
    let config = Config::new()
        .which_captures(WhichCaptures::Implicit);
}

#[test]
fn test_config_which_captures_none() {
    let config = Config::new()
        .which_captures(WhichCaptures::None);
}

#[test]
fn test_config_look_matcher() {
    let look_matcher = LookMatcher { lineterm: DebugByte {} };
    let config = Config::new()
        .look_matcher(look_matcher);
}

