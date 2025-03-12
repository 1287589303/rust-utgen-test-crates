// Answer 0

#[test]
fn test_captures_with_yes_true() {
    let config = Config::new()
        .utf8(true)
        .reverse(false)
        .nfa_size_limit(None)
        .shrink(false)
        .which_captures(WhichCaptures::All);
    let config_result = config.captures(true);
}

#[test]
fn test_captures_with_yes_true_shrink_enabled() {
    let config = Config::new()
        .utf8(true)
        .reverse(false)
        .nfa_size_limit(None)
        .shrink(true)
        .which_captures(WhichCaptures::All);
    let config_result = config.captures(true);
}

#[test]
fn test_captures_with_yes_true_nfa_size_limit() {
    let config = Config::new()
        .utf8(true)
        .reverse(false)
        .nfa_size_limit(Some(1024))
        .shrink(false)
        .which_captures(WhichCaptures::All);
    let config_result = config.captures(true);
}

